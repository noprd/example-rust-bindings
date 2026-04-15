#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# ----------------------------------------------------------------
# IMPORTS
# ----------------------------------------------------------------

from math import sqrt
from typing import Literal

# ----------------------------------------------------------------
# EXPORTS
# ----------------------------------------------------------------

__all__ = [
    "FILE_SIZE_CLASSIFICATION",
    "MAX_DB_LIST_SIZE",
    "MAX_DB_TEXT_LENGTH",
    "MAX_ITERATIONS",
    "SIZE_1_KB",
    "SIZE_1_MB",
    "file_size_classifier",
]

# ----------------------------------------------------------------
# CONSTANTS
# ----------------------------------------------------------------

SIZE_1_KB = 2**10
SIZE_1_MB = 2**20
MAX_DB_TEXT_LENGTH = 1_000
MAX_DB_LIST_SIZE = 40
MAX_ITERATIONS = 1_000
FILE_SIZE_CLASSIFICATION = Literal[
    "UNKNOWN",
    "EMPTY",
    "SMALL",
    "MEDIUM",
    "LARGE",
    "HUGE",
]

# ----------------------------------------------------------------
# METHODS
# ----------------------------------------------------------------


def file_size_classifier(size_bytes: int | None, /) -> FILE_SIZE_CLASSIFICATION:
    """
    Classifies file sizes into categories

    - SMALL < ca. 30 MB
    - MEDIUM < ca. 150 MB
    - LARGE < ca. 300 MB
    - HUGE >= 300 MB
    """
    # handle edge cases
    match size_bytes:
        case 0:
            return "EMPTY"

        case -1 | None:
            return "UNKNOWN"

    size_mb = size_bytes / SIZE_1_MB
    threshold = sqrt(10)

    if size_mb < threshold * 10:
        return "SMALL"

    if size_mb < 0.5 * threshold * 100:
        return "MEDIUM"

    if size_mb < threshold * 100:
        return "LARGE"

    return "HUGE"
