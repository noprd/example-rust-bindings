#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# ----------------------------------------------------------------
# IMPORTS
# ----------------------------------------------------------------

import os
import re
from pathlib import Path

# ----------------------------------------------------------------
# EXPORTS
# ----------------------------------------------------------------

__all__ = [
    "get_mock_case_path",
    "get_module",
    "get_resource_path",
    "get_root_path",
    "get_source_path",
    "get_this_module",
]

# ----------------------------------------------------------------
# CONSTANTS
# ----------------------------------------------------------------

_root = Path(__file__).parent.parent.parent.as_posix()
_source = Path(__file__).parent.as_posix()

# ----------------------------------------------------------------
# METHODS
# ----------------------------------------------------------------


def get_path(root: str, *parts: str) -> str:
    return root if len(parts) == 0 else Path(root, *parts).as_posix()


def get_root_path(*parts: str) -> str:
    return get_path(_root, *parts)


def get_source_path(*parts: str) -> str:
    return get_path(_source, *parts)


def get_resource_path(*parts: str) -> str:
    """
    @returns path within `resource` folder
    """
    return get_source_path("resources", *parts)


def get_mock_case_path(*parts: str, case: int) -> str:
    """
    @returns path within `resources/mock_cases/mock_case{n}` folder
    """
    return get_resource_path("mock_cases", f"mock_case{case}", *parts)


def get_this_module(path: str, /) -> str:
    """
    Returns the module equivalent of current path
    """
    # remove extension
    path, _ = os.path.splitext(path)
    # get parts
    rel = Path(path).relative_to(_root)
    parts = list(rel.parts)
    # join to form module name
    name = ".".join(parts)
    return name


def get_module(
    path: str,
    /,
    *,
    root: str = "src",
    prefix: str = r"^tests?_(.*)",
    basename: str | None = None,
) -> str:
    """
    Replaces path to current file by corresponding module in source.
    """
    # remove extension
    path, _ = os.path.splitext(path)
    # get parts
    rel = Path(path).relative_to(_source)
    parts = list(rel.parts)
    # optionally replace final part
    if (basename or "") != "":
        parts[-1] = basename
    # remove test-prefixes
    parts = [re.sub(pattern=prefix, repl=r"\1", string=part) for part in parts]
    # join to form module name
    name = ".".join([root, *parts])
    return name
