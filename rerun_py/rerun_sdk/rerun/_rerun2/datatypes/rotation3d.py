# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, SupportsFloat, Union

import pyarrow as pa
from attrs import define, field

from .. import datatypes
from .._baseclasses import (
    BaseExtensionArray,
    BaseExtensionType,
)
from ._overrides import rotation3d_inner_converter  # noqa: F401

__all__ = ["Rotation3D", "Rotation3DArray", "Rotation3DArrayLike", "Rotation3DLike", "Rotation3DType"]


@define
class Rotation3D:
    """A 3D rotation."""

    inner: Union[datatypes.Quaternion, datatypes.RotationAxisAngle] = field(converter=rotation3d_inner_converter)
    """
    Quaternion (datatypes.Quaternion):
        Rotation defined by a quaternion.

    AxisAngle (datatypes.RotationAxisAngle):
        Rotation defined with an axis and an angle.
    """


if TYPE_CHECKING:
    Rotation3DLike = Union[Rotation3D, datatypes.Quaternion, datatypes.RotationAxisAngle, Sequence[SupportsFloat]]
    Rotation3DArrayLike = Union[
        Rotation3D,
        datatypes.Quaternion,
        datatypes.RotationAxisAngle,
        Sequence[Rotation3DLike],
    ]
else:
    Rotation3DLike = Any
    Rotation3DArrayLike = Any

# --- Arrow support ---


class Rotation3DType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.dense_union(
                [
                    pa.field("Quaternion", pa.list_(pa.field("item", pa.float32(), False, {}), 4), False, {}),
                    pa.field(
                        "AxisAngle",
                        pa.struct(
                            [
                                pa.field("axis", pa.list_(pa.field("item", pa.float32(), False, {}), 3), False, {}),
                                pa.field(
                                    "angle",
                                    pa.dense_union(
                                        [
                                            pa.field("Radians", pa.float32(), False, {}),
                                            pa.field("Degrees", pa.float32(), False, {}),
                                        ]
                                    ),
                                    False,
                                    {},
                                ),
                            ]
                        ),
                        False,
                        {},
                    ),
                ]
            ),
            "rerun.datatypes.Rotation3D",
        )


class Rotation3DArray(BaseExtensionArray[Rotation3DArrayLike]):
    _EXTENSION_NAME = "rerun.datatypes.Rotation3D"
    _EXTENSION_TYPE = Rotation3DType

    @staticmethod
    def _native_to_pa_array(data: Rotation3DArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError


Rotation3DType._ARRAY_TYPE = Rotation3DArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(Rotation3DType())
