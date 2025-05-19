# -*- coding: utf-8 -*-

# ----------------------------------------------------------------
# IMPORTS
# ----------------------------------------------------------------

from __future__ import annotations

from typing import Any
from typing import Generator
# from typing import Self
from pydantic import BaseModel
from pydantic import ConfigDict
from pydantic import Field
from pydantic import RootModel
from pydantic import SkipValidation

# ----------------------------------------------------------------
# EXPORTS
# ----------------------------------------------------------------

class PsetId(BaseModel):
    """
    Class structure for Pset verbose expansion
    """

    model_config = ConfigDict(
        extra="forbid",
        populate_by_name=True,
    )
    id_: int = Field(alias="id")

    @staticmethod
    def model_validate(value: Any, /) -> PsetId:
        ...

class Pset(BaseModel):
    """
    Class structure for Pset verbose expansion
    """

    model_config = ConfigDict(
        # NOTE: This should be unnecessary, but just set allow to anticipate future additions to IfcOpenShell!
        extra="allow",
        populate_by_name=True,
        arbitrary_types_allowed=True,
    )
    id_: int = Field(alias="id")
    class_: str = Field(alias="class")
    value: SkipValidation[Any] = Field(alias="value")
    # NOTE: This is a new field since ifcopenshell==0.8.1. It is unclear if it is always set
    value_type: str | None = Field(default=None, alias="value-type")

    @staticmethod
    def model_validate(value: Any, /) -> Pset:
        ...

class Psets(RootModel[Pset | PsetId | dict[str, Psets | Any]]):
    """
    Basic part of pset
    """

    model_config = ConfigDict(
        populate_by_name=True,
    )
    root: Pset | PsetId | dict[str, Psets | Any] = {}

    @staticmethod
    def model_validate(value: Any, /) -> Psets:
        ...

    def flatten(
        self,
        /,
        *,
        delimiter: str = ":",
    ) -> dict[str, Pset | PsetId | Any]:
        ...

    def __iter__(self) ->  Generator[
        tuple[str | None, Pset | PsetId | Any],
        None,
        None,
    ]:
        ...
