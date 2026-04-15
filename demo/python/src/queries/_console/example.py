#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# ----------------------------------------------------------------
# IMPORTS
# ----------------------------------------------------------------

from argparse import ArgumentParser

from ..._core.utils.misc import *
from ...models.application import *
from .basic import *

# ----------------------------------------------------------------
# EXPORTS
# ----------------------------------------------------------------

__all__ = [
    "CliArguments",
]

# ----------------------------------------------------------------
# METHODS
# ----------------------------------------------------------------


class CliArguments(CliArgumentsBase):
    _prog = "src/example.py"
    _part = "APPLICATION"

    def create_parser(self) -> ArgumentParser:
        parser = self.base_parser
        parser.add_argument(
            "--version",
            action="store_true",
            help="show version of programme"
        )
        parser.add_argument(
            "--env",
            nargs="?",
            type=str,
            help="path to environment file",
            default=".env",
        )
        parser.add_argument(
            "--log",
            nargs="?",
            type=str,
            help="path to files for logging",
        )
        parser.add_argument(
            "--verbose",
            action="store_true",
            help="more verbose console logging (force logging level to be DEBUG)",
        )
        return parser
