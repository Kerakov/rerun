# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import (
    Archetype,
)

__all__ = ["Transform3D"]


@define(str=False, repr=False)
class Transform3D(Archetype):
    """A 3D transform."""

    transform: components.Transform3DArray = field(
        metadata={"component": "primary"},
        converter=components.Transform3DArray.from_similar,  # type: ignore[misc]
    )
    """
    The transform
    """

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
