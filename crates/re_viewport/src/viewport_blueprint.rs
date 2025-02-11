use std::collections::BTreeMap;

use ahash::HashMap;
use arrow2_convert::field::ArrowField;

use re_data_store::{EntityPath, StoreDb};
use re_log_types::{Component, DataCell, DataRow, RowId, SerializableComponent, TimePoint};
use re_viewer_context::{
    CommandSender, Item, SpaceViewId, SystemCommand, SystemCommandSender, ViewerContext,
};

use crate::{
    blueprint_components::{
        AutoSpaceViews, SpaceViewComponent, SpaceViewMaximized, ViewportLayout, VIEWPORT_PATH,
    },
    space_info::SpaceInfoCollection,
    space_view::SpaceViewBlueprint,
    space_view_heuristics::default_created_space_views,
};

// ----------------------------------------------------------------------------

/// Describes the layout and contents of the Viewport Panel.
#[derive(Clone)]
pub struct ViewportBlueprint<'a> {
    /// The StoreDb used to instantiate this blueprint
    blueprint_db: &'a StoreDb,

    /// Where the space views are stored.
    ///
    /// Not a hashmap in order to preserve the order of the space views.
    pub space_views: BTreeMap<SpaceViewId, SpaceViewBlueprint>,

    /// The layouts of all the space views.
    pub tree: egui_tiles::Tree<SpaceViewId>,

    /// Show one tab as maximized?
    pub maximized: Option<SpaceViewId>,

    /// Set to `true` the first time the user messes around with the viewport blueprint.
    pub has_been_user_edited: bool,

    /// Whether or not space views should be created automatically.
    pub auto_space_views: bool,
}

impl<'a> ViewportBlueprint<'a> {
    /// Reset the blueprint to a default state using some heuristics.
    pub fn reset(&mut self, ctx: &mut ViewerContext<'_>, spaces_info: &SpaceInfoCollection) {
        re_tracing::profile_function!();

        let ViewportBlueprint {
            blueprint_db: _,
            space_views,
            tree,
            maximized,
            has_been_user_edited,
            auto_space_views,
        } = self;

        *space_views = Default::default();
        *tree = Default::default();
        *maximized = Default::default();
        *has_been_user_edited = Default::default();
        *auto_space_views = Default::default();

        for space_view in default_created_space_views(ctx, spaces_info) {
            self.add_space_view(space_view);
        }
    }

    pub fn space_view_ids(&self) -> impl Iterator<Item = &SpaceViewId> + '_ {
        self.space_views.keys()
    }

    pub fn space_view(&self, space_view: &SpaceViewId) -> Option<&SpaceViewBlueprint> {
        self.space_views.get(space_view)
    }

    pub fn space_view_mut(
        &mut self,
        space_view_id: &SpaceViewId,
    ) -> Option<&mut SpaceViewBlueprint> {
        self.space_views.get_mut(space_view_id)
    }

    pub(crate) fn remove(&mut self, space_view_id: &SpaceViewId) -> Option<SpaceViewBlueprint> {
        let Self {
            blueprint_db: _,
            space_views,
            tree,
            maximized,
            has_been_user_edited,
            auto_space_views: _,
        } = self;

        *has_been_user_edited = true;

        if *maximized == Some(*space_view_id) {
            *maximized = None;
        }

        if let Some(tile_id) = tree.tiles.find_pane(space_view_id) {
            tree.tiles.remove(tile_id);
        }

        space_views.remove(space_view_id)
    }

    /// If `false`, the item is referring to data that is not present in this blueprint.
    pub fn is_item_valid(&self, item: &Item) -> bool {
        match item {
            Item::ComponentPath(_) => true,
            Item::InstancePath(space_view_id, _) => space_view_id
                .map(|space_view_id| self.space_view(&space_view_id).is_some())
                .unwrap_or(true),
            Item::SpaceView(space_view_id) => self.space_view(space_view_id).is_some(),
            Item::DataBlueprintGroup(space_view_id, data_blueprint_group_handle) => {
                if let Some(space_view) = self.space_view(space_view_id) {
                    space_view
                        .data_blueprint
                        .group(*data_blueprint_group_handle)
                        .is_some()
                } else {
                    false
                }
            }
        }
    }

    pub fn mark_user_interaction(&mut self) {
        self.has_been_user_edited = true;
    }

    pub fn add_space_view(&mut self, mut space_view: SpaceViewBlueprint) -> SpaceViewId {
        let space_view_id = space_view.id;

        // Find a unique name for the space view
        let mut candidate_name = space_view.display_name.clone();
        let mut append_count = 1;
        let unique_name = 'outer: loop {
            for view in &self.space_views {
                if candidate_name == view.1.display_name {
                    append_count += 1;
                    candidate_name = format!("{} ({})", space_view.display_name, append_count);

                    continue 'outer;
                }
            }
            break candidate_name;
        };

        space_view.display_name = unique_name;

        self.space_views.insert(space_view_id, space_view);

        if self.has_been_user_edited {
            // Try to insert it in the tree, in the top level:
            if let Some(root_id) = self.tree.root {
                let tile_id = self.tree.tiles.insert_pane(space_view_id);
                if let Some(egui_tiles::Tile::Container(container)) =
                    self.tree.tiles.get_mut(root_id)
                {
                    container.add_child(tile_id);
                } else {
                    self.tree = Default::default(); // we'll just re-initialize later instead
                }
            }
        } else {
            // Re-run the auto-layout next frame:
            self.tree = Default::default();
        }

        space_view_id
    }

    pub fn space_views_containing_entity_path(&self, path: &EntityPath) -> Vec<SpaceViewId> {
        self.space_views
            .iter()
            .filter_map(|(space_view_id, space_view)| {
                if space_view.data_blueprint.contains_entity(path) {
                    Some(*space_view_id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn sync_viewport_blueprint(
        &self,
        snapshot: &ViewportBlueprint<'_>,
        command_sender: &CommandSender,
    ) {
        let mut deltas = vec![];

        let entity_path = EntityPath::from(VIEWPORT_PATH);

        // TODO(jleibs): Seq instead of timeless?
        let timepoint = TimePoint::timeless();

        if self.auto_space_views != snapshot.auto_space_views {
            let component = AutoSpaceViews(self.auto_space_views);
            add_delta_from_single_component(&mut deltas, &entity_path, &timepoint, component);
        }

        if self.maximized != snapshot.maximized {
            let component = SpaceViewMaximized(self.maximized);
            add_delta_from_single_component(&mut deltas, &entity_path, &timepoint, component);
        }

        if self.tree != snapshot.tree || self.has_been_user_edited != snapshot.has_been_user_edited
        {
            let component = ViewportLayout {
                space_view_keys: self.space_views.keys().cloned().collect(),
                tree: self.tree.clone(),
                has_been_user_edited: self.has_been_user_edited,
            };

            add_delta_from_single_component(&mut deltas, &entity_path, &timepoint, component);
        }

        // Add any new or modified space views
        for id in self.space_view_ids() {
            if let Some(space_view) = self.space_view(id) {
                sync_space_view(&mut deltas, space_view, snapshot.space_view(id));
            }
        }

        // Remove any deleted space views
        for space_view_id in snapshot.space_view_ids() {
            if self.space_view(space_view_id).is_none() {
                clear_space_view(&mut deltas, space_view_id);
            }
        }

        command_sender.send_system(SystemCommand::UpdateBlueprint(
            self.blueprint_db.store_id().clone(),
            deltas,
        ));
    }
}

// ----------------------------------------------------------------------------

// TODO(jleibs): Move this helper to a better location
pub fn add_delta_from_single_component<C: SerializableComponent>(
    deltas: &mut Vec<DataRow>,
    entity_path: &EntityPath,
    timepoint: &TimePoint,
    component: C,
) {
    let row = DataRow::from_cells1_sized(
        RowId::random(),
        entity_path.clone(),
        timepoint.clone(),
        1,
        [component].as_slice(),
    );

    deltas.push(row);
}

// ----------------------------------------------------------------------------

pub fn load_space_view_blueprint(
    path: &EntityPath,
    blueprint_db: &re_data_store::StoreDb,
) -> Option<SpaceViewBlueprint> {
    re_tracing::profile_function!();
    blueprint_db
        .store()
        .query_timeless_component::<SpaceViewComponent>(path)
        .map(|c| c.space_view)
}

pub fn load_viewport_blueprint(blueprint_db: &re_data_store::StoreDb) -> ViewportBlueprint<'_> {
    let space_views: HashMap<SpaceViewId, SpaceViewBlueprint> = if let Some(space_views) =
        blueprint_db
            .entity_db
            .tree
            .children
            .get(&re_data_store::EntityPathPart::Name(
                SpaceViewComponent::SPACEVIEW_PREFIX.into(),
            )) {
        space_views
            .children
            .values()
            .filter_map(|view_tree| load_space_view_blueprint(&view_tree.path, blueprint_db))
            .map(|sv| (sv.id, sv))
            .collect()
    } else {
        Default::default()
    };

    re_tracing::profile_function!();
    let auto_space_views = blueprint_db
        .store()
        .query_timeless_component::<AutoSpaceViews>(&VIEWPORT_PATH.into())
        .unwrap_or_else(|| {
            // Only enable auto-space-views if this is the app-default blueprint
            AutoSpaceViews(
                blueprint_db
                    .store_info()
                    .map_or(false, |ri| ri.is_app_default_blueprint()),
            )
        });

    let space_view_maximized = blueprint_db
        .store()
        .query_timeless_component::<SpaceViewMaximized>(&VIEWPORT_PATH.into())
        .unwrap_or_default();

    let viewport_layout: ViewportLayout = blueprint_db
        .store()
        .query_timeless_component::<ViewportLayout>(&VIEWPORT_PATH.into())
        .unwrap_or_default();

    let unknown_space_views: HashMap<_, _> = space_views
        .iter()
        .filter(|(k, _)| !viewport_layout.space_view_keys.contains(k))
        .map(|(k, v)| (*k, v.clone()))
        .collect();

    let known_space_views: BTreeMap<_, _> = space_views
        .into_iter()
        .filter(|(k, _)| viewport_layout.space_view_keys.contains(k))
        .collect();

    let mut viewport = ViewportBlueprint {
        blueprint_db,
        space_views: known_space_views,
        tree: viewport_layout.tree,
        maximized: space_view_maximized.0,
        has_been_user_edited: viewport_layout.has_been_user_edited,
        auto_space_views: auto_space_views.0,
    };
    // TODO(jleibs): It seems we shouldn't call this until later, after we've created
    // the snapshot. Doing this here means we are mutating the state before it goes
    // into the snapshot. For example, even if there's no visibility in the
    // store, this will end up with default-visibility, which then *won't* be saved back.
    for (_, view) in unknown_space_views {
        viewport.add_space_view(view);
    }

    viewport
}

// ----------------------------------------------------------------------------

pub fn sync_space_view(
    deltas: &mut Vec<DataRow>,
    space_view: &SpaceViewBlueprint,
    snapshot: Option<&SpaceViewBlueprint>,
) {
    if snapshot.map_or(true, |snapshot| space_view.has_edits(snapshot)) {
        let entity_path = EntityPath::from(format!(
            "{}/{}",
            SpaceViewComponent::SPACEVIEW_PREFIX,
            space_view.id
        ));

        // TODO(jleibs): Seq instead of timeless?
        let timepoint = TimePoint::timeless();

        let component = SpaceViewComponent {
            space_view: space_view.clone(),
        };

        add_delta_from_single_component(deltas, &entity_path, &timepoint, component);
    }
}

pub fn clear_space_view(deltas: &mut Vec<DataRow>, space_view_id: &SpaceViewId) {
    let entity_path = EntityPath::from(format!(
        "{}/{}",
        SpaceViewComponent::SPACEVIEW_PREFIX,
        space_view_id
    ));

    // TODO(jleibs): Seq instead of timeless?
    let timepoint = TimePoint::timeless();

    let cell =
        DataCell::from_arrow_empty(SpaceViewComponent::name(), SpaceViewComponent::data_type());

    let row = DataRow::from_cells1_sized(RowId::random(), entity_path, timepoint, 0, cell);

    deltas.push(row);
}
