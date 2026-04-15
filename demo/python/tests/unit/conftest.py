#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# ----------------------------------------------------------------
# IMPORTS
# ----------------------------------------------------------------

from unittest import TestCase

from pytest import fixture

# ----------------------------------------------------------------
# FIXTURES - GENERAL
# ----------------------------------------------------------------


@fixture(scope="session", autouse=True)
def test() -> TestCase:
    return TestCase()
