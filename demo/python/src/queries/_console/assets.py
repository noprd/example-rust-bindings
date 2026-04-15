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
    _prog = "scripts/assets.py"
    _part = "APPLICATION"

    def create_parser(self) -> ArgumentParser:
        parser = self.base_parser
        parser.add_argument(
            "--owner",
            type=str,
            required=True,
            help="github repository owner",
        )
        parser.add_argument(
            "--repo",
            type=str,
            required=True,
            help="github repository name",
        )
        parser.add_argument(
            "--module",
            type=str,
            required=True,
            help="name of module as imported in python",
        )
        parser.add_argument(
            "--tag",
            type=str,
            required=True,
            help="github release tag",
        )
        parser.add_argument(
            "--os",
            type=str,
            choices=["linux", "osx", "win"],
            help="operating system",
            default="linux",
        )
        parser.add_argument(
            "--cpu",
            type=str,
            choices=["x86_64", "aarch64"],
            required=True,
            help="cpu architecture",
            default="x86_64",
        )
        parser.add_argument(
            "-o",
            "--output",
            type=str,
            help="path to output directory for assets",
            default="dist",
        )
        parser.add_argument(
            "--env",
            type=str,
            help="path to environment file",
            default=".env",
        )
        return parser
