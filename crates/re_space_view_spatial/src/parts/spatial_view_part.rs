use crate::parts::UiLabel;

/// Common data struct for all spatial scene elements.
///
/// Each spatial scene element is expected to fill an instance of this struct with its data.
pub struct SpatialViewPartData {
    pub ui_labels: Vec<UiLabel>,
    pub bounding_box: macaw::BoundingBox,
}

impl SpatialViewPartData {
    pub fn extend_bounding_box(
        &mut self,
        other: macaw::BoundingBox,
        world_from_obj: glam::Affine3A,
    ) {
        self.bounding_box = self
            .bounding_box
            .union(other.transform_affine3(&world_from_obj));
    }

    pub fn extend_bounding_box_with_points(
        &mut self,
        points: impl Iterator<Item = glam::Vec3>,
        world_from_obj: glam::Affine3A,
    ) {
        re_tracing::profile_function!();
        self.extend_bounding_box(macaw::BoundingBox::from_points(points), world_from_obj);
    }

    pub fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for SpatialViewPartData {
    fn default() -> Self {
        Self {
            ui_labels: Vec::new(),
            bounding_box: macaw::BoundingBox::nothing(),
        }
    }
}
