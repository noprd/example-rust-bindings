#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# ----------------------------------------------------------------
# IMPORTS
# ----------------------------------------------------------------

import logging
from contextvars import ContextVar
from pathlib import Path

import toml

from ..__paths__ import *
from .._core.logging import *
from .._core.utils.code import *
from ..models.application import *

# ----------------------------------------------------------------
# EXPORTS
# ----------------------------------------------------------------

__all__ = [
    "INFO",
    "VERSION",
]

# ----------------------------------------------------------------
# GLOBAL PROPERTIES
# ----------------------------------------------------------------

pid = ContextVar[int]("pid")  # fmt: skip
path_env = ContextVar[str]("path env", default=".env")  # fmt: skip
path_session = ContextVar[str]("path session", default=".session")  # fmt: skip
path_logging = ContextVar[str | None]("path logging", default=None)  # fmt: skip

# ----------------------------------------------------------------
# METHODS
# ----------------------------------------------------------------


def initialise_application(
    *,
    name: str,
    verbose: bool = False,
    title: str | None = None,
    serialise: bool = True,
    log_to_files: bool = False,
):
    """
    Initialises logging and displays information about pid, cpus.
    """
    level = "DEBUG" if verbose else "INFO"
    path = path_logging.get() if log_to_files else None
    configure_logging(level=level, path=path, serialise=serialise)  # fmt: skip
    logging.info(f"running {title or name} v{INFO.version} on PID {pid.get()}")
    return


# ----------------------------------------------------------------
# QUERIES
# ----------------------------------------------------------------


@compute_once
def load_repo_info() -> RepoInfo:
    path = Path(get_root_path(), "pyproject.toml").as_posix()
    with open(path, "r") as fp:
        config_repo = toml.load(fp)
        assets = config_repo.get("project", {})
        info = RepoInfo.model_validate(assets)
        return info


@compute_once
def get_version() -> str:
    info = load_repo_info()
    return info.version


# ----------------------------------------------------------------
# LAZY LOADED RESOURCES / PROPERTIES
# ----------------------------------------------------------------

INFO = load_repo_info()
VERSION = get_version()
