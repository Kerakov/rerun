//! User-facing data types, component types, and archetypes.
//!
//! The SDK is responsible for submitting component columns that conforms to these schemas. The
//! schemas are additionally documented in doctests.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
//!

use arrow2::{
    array::{FixedSizeListArray, MutableFixedSizeListArray, PrimitiveArray},
    datatypes::{DataType, Field},
};
use arrow2_convert::{
    deserialize::{ArrowArray, ArrowDeserialize},
    field::{ArrowEnableVecForType, ArrowField},
    serialize::ArrowSerialize,
};
use lazy_static::lazy_static;

mod arrow;
mod bbox;
mod class_id;
mod color;
pub mod context;
pub mod coordinates;
mod disconnected_space;
mod draw_order;
mod keypoint_id;
mod label;
mod linestrip;
mod mat;
mod mesh3d;
mod pinhole;
mod point;
mod quaternion;
mod radius;
mod rect;
mod scalar;
mod size;
mod tensor;
mod tensor_data;
mod text_box;
mod text_entry;
mod transform3d;
mod vec;

#[cfg(not(target_arch = "wasm32"))]
mod load_file;

#[cfg(feature = "arrow_datagen")]
pub mod datagen;

// ----------------------------------------------------------------------------
// TODO(emilk): split into modules, like we do in re_sdk/src/lib.rs

pub use self::{
    arrow::Arrow3D,
    bbox::Box3D,
    class_id::ClassId,
    color::ColorRGBA,
    context::{AnnotationContext, AnnotationInfo, ClassDescription},
    coordinates::ViewCoordinates,
    disconnected_space::DisconnectedSpace,
    draw_order::DrawOrder,
    keypoint_id::KeypointId,
    label::Label,
    linestrip::{LineStrip2D, LineStrip3D},
    mat::Mat3x3,
    mesh3d::{EncodedMesh3D, Mesh3D, MeshFormat, MeshId, RawMesh3D},
    pinhole::Pinhole,
    point::{Point2D, Point3D},
    quaternion::Quaternion,
    radius::Radius,
    rect::Rect2D,
    scalar::{Scalar, ScalarPlotProps},
    size::Size3D,
    tensor::{
        DecodedTensor, Tensor, TensorCastError, TensorData, TensorDataMeaning, TensorDimension,
        TensorId,
    },
    tensor_data::{TensorDataType, TensorDataTypeTrait, TensorElement},
    text_box::TextBox,
    text_entry::TextEntry,
    transform3d::{
        Angle, Rotation3D, RotationAxisAngle, Scale3D, Transform3D, Transform3DRepr,
        TranslationAndMat3, TranslationRotationScale3D,
    },
    vec::{Vec2D, Vec3D, Vec4D},
};

#[cfg(feature = "image")]
pub use self::tensor::{TensorImageLoadError, TensorImageSaveError};

#[cfg(not(target_arch = "wasm32"))]
pub use self::load_file::{data_cell_from_file_path, data_cell_from_mesh_file_path, FromFileError};

// This is a component
pub use re_log_types::InstanceKey;

// This is very convenient to re-export
pub use re_log_types::Component;

pub mod external {
    #[cfg(feature = "glam")]
    pub use glam;

    #[cfg(feature = "image")]
    pub use image;
}

// ----------------------------------------------------------------------------

lazy_static! {
    //TODO(john): use a run-time type registry
    static ref FIELDS: [Field; 29] = [
        <AnnotationContext as Component>::field(),
        <Arrow3D as Component>::field(),
        <Box3D as Component>::field(),
        <ClassId as Component>::field(),
        <ColorRGBA as Component>::field(),
        <DrawOrder as Component>::field(),
        <DisconnectedSpace as Component>::field(),
        <re_log_types::InstanceKey as Component>::field(),
        <KeypointId as Component>::field(),
        <Label as Component>::field(),
        <LineStrip2D as Component>::field(),
        <LineStrip3D as Component>::field(),
        <Mesh3D as Component>::field(),
        <Pinhole as Component>::field(),
        <Point2D as Component>::field(),
        <Point3D as Component>::field(),
        <Quaternion as Component>::field(),
        <Radius as Component>::field(),
        <Rect2D as Component>::field(),
        <Scalar as Component>::field(),
        <ScalarPlotProps as Component>::field(),
        <Size3D as Component>::field(),
        <Tensor as Component>::field(),
        <TextBox as Component>::field(),
        <TextEntry as Component>::field(),
        <Transform3D as Component>::field(),
        <Vec2D as Component>::field(),
        <Vec3D as Component>::field(),
        <ViewCoordinates as Component>::field(),
    ];
}

/// Iterate over the registered field types
pub fn iter_registered_field_types() -> impl Iterator<Item = &'static Field> {
    FIELDS.iter()
}

#[derive(thiserror::Error, Debug)]
pub enum FieldError {
    #[error("Encountered bad value")]
    BadValue,

    #[error("Slice over bad indices")]
    BadSlice(#[from] std::array::TryFromSliceError),
}

pub type Result<T> = std::result::Result<T, FieldError>;

/// `arrow2_convert` helper for fields of type `[T; SIZE]`
///
/// This allows us to use fields of type `[T; SIZE]` in `arrow2_convert`. Since this is a helper,
/// it must be specified as the type of the field using the `#[arrow_field(type = "FixedSizeArrayField<T,SIZE>")]` attribute.
///
/// ## Example:
/// ```
/// use arrow2_convert::{ArrowField, ArrowSerialize, ArrowDeserialize};
/// use re_components::FixedSizeArrayField;
///
/// #[derive(ArrowField, ArrowSerialize, ArrowDeserialize)]
/// pub struct ConvertibleType {
///     #[arrow_field(type = "FixedSizeArrayField<u32,2>")]
///     data: [u32; 2],
/// }
/// ```
pub struct FixedSizeArrayField<T, const SIZE: usize>(std::marker::PhantomData<T>);

impl<T, const SIZE: usize> ArrowField for FixedSizeArrayField<T, SIZE>
where
    T: ArrowField + ArrowEnableVecForType,
{
    type Type = [T; SIZE];

    #[inline]
    fn data_type() -> DataType {
        arrow2::datatypes::DataType::FixedSizeList(Box::new(<T as ArrowField>::field("item")), SIZE)
    }
}

impl<T, const SIZE: usize> ArrowSerialize for FixedSizeArrayField<T, SIZE>
where
    T: ArrowSerialize + ArrowEnableVecForType + ArrowField<Type = T> + 'static,
    <T as ArrowSerialize>::MutableArrayType: Default,
{
    type MutableArrayType = MutableFixedSizeListArray<<T as ArrowSerialize>::MutableArrayType>;
    #[inline]
    fn new_array() -> Self::MutableArrayType {
        Self::MutableArrayType::new_with_field(
            <T as ArrowSerialize>::new_array(),
            "item",
            <T as ArrowField>::is_nullable(),
            SIZE,
        )
    }

    fn arrow_serialize(
        v: &<Self as ArrowField>::Type,
        array: &mut Self::MutableArrayType,
    ) -> arrow2::error::Result<()> {
        let values = array.mut_values();
        for i in v.iter() {
            <T as ArrowSerialize>::arrow_serialize(i, values)?;
        }
        array.try_push_valid()
    }
}

pub struct FastFixedSizeArrayIter<'a, T, const SIZE: usize>
where
    T: arrow2::types::NativeType,
{
    offset: usize,
    end: usize,
    array: &'a FixedSizeListArray,
    values: &'a PrimitiveArray<T>,
}

impl<'a, T, const SIZE: usize> Iterator for FastFixedSizeArrayIter<'a, T, SIZE>
where
    T: arrow2::types::NativeType,
{
    type Item = Option<[T; SIZE]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.end {
            if let Some(validity) = self.array.validity() {
                if !validity.get_bit(self.offset) {
                    self.offset += 1;
                    return Some(None);
                }
            }

            let out: [T; SIZE] =
                array_init::array_init(|i: usize| self.values.value(self.offset * SIZE + i));
            self.offset += 1;
            Some(Some(out))
        } else {
            None
        }
    }
}

pub struct FastFixedSizeListArray<T, const SIZE: usize>(std::marker::PhantomData<T>);

#[cfg(not(target_os = "windows"))]
extern "C" {
    fn do_not_call_into_iter(); // we never define this function, so the linker will fail
}

impl<'a, T, const SIZE: usize> IntoIterator for &'a FastFixedSizeListArray<T, SIZE>
where
    T: arrow2::types::NativeType,
{
    type Item = Option<[T; SIZE]>;

    type IntoIter = FastFixedSizeArrayIter<'a, T, SIZE>;

    #[cfg(not(target_os = "windows"))]
    fn into_iter(self) -> Self::IntoIter {
        #[allow(unsafe_code)]
        // SAFETY:
        // This exists so we get a link-error if some code tries to call into_iter
        // Iteration should only happen via iter_from_array_ref.
        // This is a quirk of the way the traits work in arrow2_convert.
        unsafe {
            do_not_call_into_iter();
        }
        unreachable!();
    }

    // On windows the above linker trick doesn't work.
    // We'll still catch the issue on build in Linux, but on windows just fall back to panic.
    #[cfg(target_os = "windows")]
    fn into_iter(self) -> Self::IntoIter {
        panic!("Use iter_from_array_ref. This is a quirk of the way the traits work in arrow2_convert.");
    }
}

impl<T, const SIZE: usize> ArrowArray for FastFixedSizeListArray<T, SIZE>
where
    T: arrow2::types::NativeType,
{
    type BaseArrayType = FixedSizeListArray;

    fn iter_from_array_ref(b: &dyn arrow2::array::Array) -> <&Self as IntoIterator>::IntoIter {
        let array = b.as_any().downcast_ref::<Self::BaseArrayType>().unwrap();
        let values = array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<T>>()
            .unwrap();
        FastFixedSizeArrayIter::<T, SIZE> {
            offset: 0,
            end: array.len(),
            array,
            values,
        }
    }
}

impl<T, const SIZE: usize> ArrowDeserialize for FixedSizeArrayField<T, SIZE>
where
    T: arrow2::types::NativeType
        + ArrowDeserialize
        + ArrowEnableVecForType
        + ArrowField<Type = T>
        + 'static,
    <T as ArrowDeserialize>::ArrayType: 'static,
    for<'b> &'b <T as ArrowDeserialize>::ArrayType: IntoIterator,
{
    type ArrayType = FastFixedSizeListArray<T, SIZE>;

    #[inline]
    fn arrow_deserialize(
        v: <&Self::ArrayType as IntoIterator>::Item,
    ) -> Option<<Self as ArrowField>::Type> {
        v
    }
}
