#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Example via interative console script
"""

# ----------------------------------------------------------------
# IMPORTS
# ----------------------------------------------------------------

import os
import sys
from pathlib import Path

os.chdir(Path(__file__).parent.parent.as_posix())
sys.path.insert(0, os.getcwd())

import logging
from lorem_text import lorem

from example_package import binary_search
from example_package import Pset
from example_package import PsetId
from example_package import Psets

from src.queries._console.example import *
from src.setup import *

# ----------------------------------------------------------------
# LOCAL CONSTANTS
# ----------------------------------------------------------------

PID = os.getpid()

# ----------------------------------------------------------------
# EXECUTION
# ----------------------------------------------------------------

if __name__ == "__main__":
    args = CliArguments(config.INFO).parse(*sys.argv[1:])

    # handle simple endpoints immediately
    if args.version:
        print(config.VERSION)
        exit(0)

    config.pid.set(PID)
    config.path_env.set(args.env)
    config.path_logging.set(args.log)
    config.initialise_application(
        name="app",
        verbose=args.verbose,
        serialise=False,
        log_to_files=(config.path_logging.get() is not None),
    )

    # demostrate search method
    logging.info("Demonstrate binary search")
    n = 0b100000000000000
    index = 0b10000
    data = lorem.words(n).split(" ")
    word = "%needle%"
    data[index] = word
    index_ = binary_search(data=data, element=word)
    assert isinstance(index_, int), "expected search algorithm to find element"
    assert data[index_] == word, "expected search algorithm to find index of element"
    assert index == index_, "expected algorithm to find unique instance of element"
    logging.info(f"found element '{word}' in list at index 0b{index_:0b}")

    # demonstrate bim classes
    x1 = PsetId(id_=4)
    assert x1.id_ == 4, "expected aspects to be computed correctly"

    x2 = Pset(id_=100, class_="Measure", value=8.923, value_type="number")
    assert x2.id_ == 100, "expected aspects to be computed correctly"
    assert x2.class_ == "Measure", "expected aspects to be computed correctly"
    assert x2.value == 8.923, "expected aspects to be computed correctly"
    assert x2.value_type == "number", "expected aspects to be computed correctly"

    x3 = Pset(id_=101, class_="CheckBox", value=True, value_type="boolean")
    assert x3.id_ == 101, "expected aspects to be computed correctly"
    assert x3.class_ == "CheckBox", "expected aspects to be computed correctly"
    assert x3.value == True, "expected aspects to be computed correctly"
    assert x3.value_type == "boolean", "expected aspects to be computed correctly"

    x4 = Psets.model_validate({
        "Ids": {
            "1": {"id": 4},
        },
        "Values": {
            "2": {"id": 100, "class": "Measure", "value": 8.923},
            "3": {"id": 101, "class": "CheckBox", "value": True},
        }
    })
    logging.info(x4)
