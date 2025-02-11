# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from typing import Sequence, Union

import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseExtensionArray,
    BaseExtensionType,
)
from ._overrides import label_native_to_pa_array  # noqa: F401

__all__ = ["Label", "LabelArray", "LabelArrayLike", "LabelLike", "LabelType"]


@define
class Label:
    """A String label component."""

    value: str = field(converter=str)

    def __str__(self) -> str:
        return str(self.value)


LabelLike = Union[Label, str]

LabelArrayLike = Union[Label, Sequence[LabelLike], str, Sequence[str]]


# --- Arrow support ---


class LabelType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.utf8(), "rerun.label")


class LabelArray(BaseExtensionArray[LabelArrayLike]):
    _EXTENSION_NAME = "rerun.label"
    _EXTENSION_TYPE = LabelType

    @staticmethod
    def _native_to_pa_array(data: LabelArrayLike, data_type: pa.DataType) -> pa.Array:
        return label_native_to_pa_array(data, data_type)


LabelType._ARRAY_TYPE = LabelArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(LabelType())
