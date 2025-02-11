# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    BaseDelegatingExtensionArray,
    BaseDelegatingExtensionType,
)

__all__ = ["Transform3DArray", "Transform3DType"]


class Transform3DType(BaseDelegatingExtensionType):
    _TYPE_NAME = "rerun.components.Transform3D"
    _DELEGATED_EXTENSION_TYPE = datatypes.Transform3DType


class Transform3DArray(BaseDelegatingExtensionArray[datatypes.Transform3DArrayLike]):
    _EXTENSION_NAME = "rerun.components.Transform3D"
    _EXTENSION_TYPE = Transform3DType
    _DELEGATED_ARRAY_TYPE = datatypes.Transform3DArray


Transform3DType._ARRAY_TYPE = Transform3DArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(Transform3DType())
