#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# ----------------------------------------------------------------
# IMPORTS
# ----------------------------------------------------------------

import os
from pathlib import Path

# ----------------------------------------------------------------
# EXPORTS
# ----------------------------------------------------------------

__all__ = [
    "clear_dir_if_exists",
    "create_dir_if_not_exists",
    "create_file_if_not_exists",
    "remove_dir_if_exists",
    "remove_file_if_exists",
]

# ----------------------------------------------------------------
# METHODS
# ----------------------------------------------------------------


def create_dir_if_not_exists(path: str):
    p = Path(path)
    p.mkdir(parents=True, exist_ok=True)
    return


def create_file_if_not_exists(path: str):
    create_dir_if_not_exists(os.path.dirname(path))
    p = Path(path)
    # ----------------
    # NOTE: set
    # 1. read+write access (=6) for user
    # 2. read+write access (=6) for group
    # 3. read access (=4) for others
    # ----------------
    p.touch(mode=0o664, exist_ok=True)
    return


def clear_dir_if_exists(
    path: str,
    recursive: bool = True,
):
    p = Path(path)
    if not (p.exists() and p.is_dir()):
        return
    for filename in os.listdir(path):
        p_ = Path(path, filename)
        path_ = p_.as_posix()
        if p_.is_file():
            os.remove(path_)
        elif recursive:
            clear_dir_if_exists(path_, recursive=recursive)
    return


def remove_dir_if_exists(path: str):
    p = Path(path)
    if not p.exists():
        return True
    clear_dir_if_exists(path=path, recursive=True)
    if p.is_dir():
        os.rmdir(path)
    return not p.exists()


def remove_file_if_exists(path: str) -> bool:
    p = Path(path)
    if not p.exists(path):
        return True
    os.remove(path)
    return not p.exists()
