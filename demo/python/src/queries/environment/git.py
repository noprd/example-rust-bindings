#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# ----------------------------------------------------------------
# IMPORTS
# ----------------------------------------------------------------

from pydantic import SecretStr

from .basic import *

# ----------------------------------------------------------------
# EXPORTS
# ----------------------------------------------------------------

__all__ = [
    "get_git_pat",
]

# ----------------------------------------------------------------
# METHODS
# ----------------------------------------------------------------


@add_environment
def get_git_pat(
    # DEV-NOTE: from decorator
    path: str,
    env: dict[str, str],
    # end decorator args
) -> SecretStr:
    """
    Gets the PAT for a git user
    """
    token = env["GIT_PAT"]
    return SecretStr(token)
