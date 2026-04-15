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

import json
import logging
import re
from argparse import ArgumentParser
from argparse import RawTextHelpFormatter

import httpx
from dotenv import dotenv_values
from dotenv import load_dotenv

# ----------------------------------------------------------------
# LOCAL CONSTANTS
# ----------------------------------------------------------------

PID = os.getpid()
# ensure azure logs are suppressed
try:
    logging.getLogger("httpx").setLevel(logging.WARNING)

except Exception as err:
    pass


class MediaTypes:
    JSON = "application/vnd.github+json"
    BYTES = "application/octet-stream"


BOUNDARY = r"[\b-_]"
LBOUNDARY = rf"(?:.*{BOUNDARY})?"
RBOUNDARY = rf"(?:{BOUNDARY}.*)?"
EXT_ARTEFACT = ".whl"
# cf. https://docs.github.com/en/rest/releases/assets
API_VERSION = "2026-03-10"
URL_LIST_ASSETS = "https://api.github.com/repos/{owner}/{repo}/releases/tags/{tag}"
URL_ASSET = "https://api.github.com/repos/{owner}/{repo}/releases/assets/{id}"

# ----------------------------------------------------------------
# CLASSES
# ----------------------------------------------------------------


def parse_cli_args(*args: str) -> ArgumentParser:
    parser = ArgumentParser(
        prog="assets",
        description="loads artefacts from git assets",
        formatter_class=RawTextHelpFormatter,
    )
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
    result = parser.parse_args(args)
    return result


def get_environment(path: str, /) -> dict[str, str]:
    """
    Loads environment variables from

    - bash session
    - a given .env file

    Values in session are ignored if empty-like.

    If a key is in both the session and file,
    then the session-value takes precedence,
    allowing users to change environments on-the-fly.
    """
    # values in file
    environ_file = dotenv_values(path)

    # load from session
    load_dotenv(dotenv_path=path)
    env_from_session = {
        key: value
        for key, value in os.environ.items()
        # NOTE: filter out empty/null values
        if value not in [None, ""]
    }

    # load from file
    env_from_file = {
        key: value
        for key, value in environ_file.items()
        # NOTE: allow file to include empty/null values
        # if value not in [None, ""]
    }

    # session env vars take precedence
    # NOTE: left-right = low to higher precedence
    env = env_from_file | env_from_session

    return dict(env)


# ----------------------------------------------------------------
# EXECUTION
# ----------------------------------------------------------------

if __name__ == "__main__":
    args = parse_cli_args(*sys.argv[1:])
    env = get_environment(args.env)
    module = args.module
    token = env["GIT_PAT"]

    """
    Get list of assets
    """

    logging.info("load list of assets")
    httpx.Headers()
    tag = re.sub(pattern=r"^v?(.*)$", repl=r"\1", string=args.tag)
    url = URL_LIST_ASSETS.format(owner=args.owner, repo=args.repo, tag=f"v{tag}")  # fmt: skip
    headers = {
        "Accept": MediaTypes.JSON,
        "X-GitHub-Api-Version": API_VERSION,
        "Authorization": f"Bearer {token}",
    }
    response = httpx.get(url, headers=headers, follow_redirects=True)
    response.raise_for_status()
    content = response.json()
    assets = content.get("assets", [])
    logging.info(f"found {len(assets)} assets")

    """
    Load assets
    """

    for index, asset in enumerate(assets):
        url = asset["url"]
        id = asset["id"]
        filename = asset["name"]
        basename, ext = os.path.splitext(filename)
        parts = basename.split("-")
        parts = [*parts, "", ""]
        module_, tag_ = parts[:2]

        if ext != EXT_ARTEFACT or tag != tag_ or module != module_:
            continue

        logging.info(f"load asset({index}) /{id}: '{basename}'")
        logging.debug(json.dumps(asset, indent=2))
        headers = {
            "Accept": MediaTypes.BYTES,
            "X-GitHub-Api-Version": API_VERSION,
            "Authorization": f"Bearer {token}",
        }
        response = httpx.get(url, headers=headers, follow_redirects=True)
        response.raise_for_status()
        content = response.read()

        """
        Store assets
        """

        p = Path(args.output, filename)
        logging.info(f"store to {p.as_posix()}")
        p.touch(mode=0o644, exist_ok=True)
        with open(p.as_posix(), "wb") as fp:
            fp.write(content)
