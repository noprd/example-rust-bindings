#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# ----------------------------------------------------------------
# IMPORTS
# ----------------------------------------------------------------

import re
from typing import Any
from unittest import TestCase

from pytest import mark

from src._core.utils.basic import *

# ----------------------------------------------------------------
# TESTS
# ----------------------------------------------------------------


@mark.parametrize(
    ("text", "args", "kwargs", "expected"),
    [
        (
            r"hello {} {food} {time}",
            ("bob",),
            {"time": 10.18, "pet": "cat", "food": "chicken"},
            r"hello bob chicken 10.18",
        ),
        (
            r"hello {} {food} {time}",
            (),
            {
                "time": 10.18,
                "pet": "cat",
            },
            r"hello {} {food} 10.18",
        ),
        (
            r"hello {0} {food} {time}",
            (),
            {
                "time": 10.18,
                "pet": "cat",
            },
            r"hello {0} {food} 10.18",
        ),
        (
            r"hello {0} {4} {food} {time}",
            (),
            {
                "time": 10.18,
                "pet": "cat",
            },
            r"hello {0} {4} {food} 10.18",
        ),
        (
            r"hello {food} {time} {0} {4}",
            (),
            {
                "time": 10.18,
                "pet": "cat",
            },
            r"hello {food} 10.18 {0} {4}",
        ),
        (
            r"hello {food} {time} {3} {9}",
            (),
            {
                "time": 10.18,
                "pet": "cat",
            },
            r"hello {food} 10.18 {3} {9}",
        ),
    ],
)
def test_safe_format_string_CASES(
    # fixtures
    test: TestCase,
    # parameters
    text: str,
    args: tuple,
    kwargs: dict[str, Any],
    expected: str,
):
    result = safe_format_string(text, *args, **kwargs)
    test.assertEqual(result, expected)


@mark.parametrize(
    ("X", "expected"),
    [
        (
            [[0, 1], ["a"]],
            [0, 1, "a"],
        ),
        (
            [[0, 1], [0, 1]],
            [0, 1, 0, 1],
        ),
        (
            [[0, 1], ["a"], ["bc", 4]],
            [0, 1, "a", "bc", 4],
        ),
    ],
)
def test_flatten_CASES(
    test: TestCase,
    # parameters
    X: list,
    expected: list[list],
):
    result = flatten(X)
    test.assertEqual(result, expected)


@mark.parametrize(
    ("X", "expected"),
    [
        (
            [],
            [],
        ),
        (
            [[]],
            [],
        ),
        (
            [[0]],
            [0],
        ),
    ],
)
def test_flatten_EDGE_CASES(
    test: TestCase,
    # parameters
    X: list,
    expected: list[list],
):
    result = flatten(X)
    test.assertEqual(result, expected)


@mark.parametrize(
    ("X", "expected"),
    [
        (
            [[0, 1], [0]],
            [0, 1, 0],
        ),
        (
            [[0, 1], [[0]]],
            [0, 1, [0]],
        ),
        (
            [[0, 1, [2]], [[0], 3]],
            [0, 1, [2], [0], 3],
        ),
    ],
)
def test_flatten_NESTING(
    test: TestCase,
    # parameters
    X: list,
    expected: list[list],
):
    result = flatten(X)
    test.assertEqual(result, expected)


def test_create_regex_from_prefix_pattern_CASES_EDGE(
    test: TestCase,
):
    pattern = create_regex_from_prefix_pattern(None)
    test.assertIsNone(pattern)

    pattern = create_regex_from_prefix_pattern("")
    test.assertIsNone(pattern)

    pattern = create_regex_from_prefix_pattern(" ")
    test.assertIsNone(pattern)

    pattern = create_regex_from_prefix_pattern("    ")
    test.assertIsNone(pattern)

    pattern = create_regex_from_prefix_pattern("*")
    test.assertEqual(pattern, r"^(.*)$")

    pattern = create_regex_from_prefix_pattern(".*")
    test.assertEqual(pattern, r"^(.*)$")

    pattern = create_regex_from_prefix_pattern("A")
    test.assertEqual(pattern, r"^(A\b.*)$")

    pattern = create_regex_from_prefix_pattern("A*")
    test.assertEqual(pattern, r"^(A\b.*)$")

    pattern = create_regex_from_prefix_pattern("A.*")
    test.assertEqual(pattern, r"^(A\b.*)$")

    pattern = create_regex_from_prefix_pattern("^A.*")
    test.assertEqual(pattern, r"^(A\b.*)$")


@mark.parametrize(
    ("text", "positive", "negative"),
    [
        (
            r"H-AB*",
            [
                "H-AB-01-00036",
                "H-AB-XY-00036",
            ],
            [
                "",
                "H-ABX",
                "H-ABX-01-00001",
            ],
        ),
        (
            r"H-AB.*",
            [
                "H-AB-01-00036",
                "H-AB-XY-00036",
            ],
            [
                "",
                "K-AB-01-00036",
                "H-ABX",
                "H-ABX-01-00001",
            ],
        ),
        (
            r"H-AB*,R*",
            [
                "H-AB-01-00036",
                "H-AB-XY-00036",
                "R-AB-XY-00036",
            ],
            [
                "",
                "K-AB-01-00036",
                "H-ABX",
                "H-ABX-01-00001",
                "RX",
                "RX-AB",
                "RX-AB-01-00001",
            ],
        ),
        (
            r"H-AB*,R.*",
            [
                "H-AB-01-00036",
                "H-AB-XY-00036",
                "R-AB-XY-00036",
            ],
            [
                "",
                "K-AB-01-00036",
                "H-ABX",
                "H-ABX-01-00001",
                "RX",
                "RX-AB",
                "RX-AB-01-00001",
            ],
        ),
        (
            r"H-AB.*,R*",
            [
                "H-AB-01-00036",
                "H-AB-XY-00036",
                "R-AB-XY-00036",
            ],
            [
                "",
                "K-AB-01-00036",
                "H-ABX",
                "H-ABX-01-00001",
                "RX",
                "RX-AB",
                "RX-AB-01-00001",
            ],
        ),
        (
            r"H-AB.*,R.*",
            [
                "H-AB-01-00036",
                "H-AB-XY-00036",
                "R-AB-XY-00036",
            ],
            [
                "",
                "K-AB-01-00036",
                "H-ABX",
                "H-ABX-01-00001",
                "RX",
                "RX-AB",
                "RX-AB-01-00001",
            ],
        ),
        (
            r"(H-AB,R)*",
            [
                "H-AB-01-00036",
                "H-AB-XY-00036",
                "R-AB-XY-00036",
            ],
            [
                "",
                "K-AB-01-00036",
                "H-ABX",
                "H-ABX-01-00001",
                "RX",
                "RX-AB",
                "RX-AB-01-00001",
            ],
        ),
        (
            r"(H-AB,R).*",
            [
                "H-AB-01-00036",
                "H-AB-XY-00036",
                "R-AB-XY-00036",
            ],
            [
                "",
                "K-AB-01-00036",
                "H-ABX",
                "H-ABX-01-00001",
                "RX",
                "RX-AB",
                "RX-AB-01-00001",
            ],
        ),
        (
            r"H-AB*,R*,K-BC*",
            [
                "H-AB-01-00036",
                "H-AB-XY-00036",
                "R-AB-XY-00036",
                "K-BC-XY-00012",
            ],
            [
                "",
                "K-AB-01-00036",
                "H-ABX",
                "H-ABX-01-00001",
                "RX",
                "RX-AB",
                "RX-AB-01-00001",
                "K-BCX",
                "K-BCX-XY-00012",
            ],
        ),
    ],
)
def test_create_regex_from_prefix_pattern_CASES_BASIC(
    test: TestCase,
    # parameters
    text: str,
    positive: str,
    negative: str,
):
    pattern = create_regex_from_prefix_pattern(text)
    pattern_compiled = re.compile(pattern)
    for text_ in positive:
        m = re.match(pattern=pattern_compiled, string=text_)
        test.assertIsNotNone(m, f"text {text_} should match pseudopattern {pattern}")

    for text_ in negative:
        m = re.match(pattern=pattern_compiled, string=text_)
        test.assertIsNone(m, f"text {text_} should not match pseudopattern {pattern}")
