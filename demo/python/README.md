[![Python v3.14](https://img.shields.io/badge/python%20v3.14-1464b4.svg)](https://www.python.org)

# Demonstration - EXAMPLE-PACKAGE #

This provides an example repository set up to import the `EXAMPLE-PACKAGE` tool as a python dependency.

## System Requirements ##

- [python3](https://www.python.org) (development was performed primarily with `v3.14`, but this should work with all versions after `v3.10`)
- [bash](https://gitforwindows.org)
- the [justfile](https://github.com/casey/just?tab=readme-ov-file#installation) tool

## Usage ##

1. Ensure you have a `PAT` set up.

2. Create files from templates

    ```bash
    just setup
    ```

3. Adjust the newly created .env file.

4. Build using

    ```bash
    # build python code base
    just build
    just build "first-party"     # build with EXAMPLE-PACKAGE

    # the build contains the following steps to load the .whl files:
    just build-venv
    just build-requirements-basic # adds minimal dependencies to venv to execute scripts/assets.py
    just load-assets "example_package" # runs scripts/assets.py
    ```

5. Run the example script using

    ```bash
    just run-examples --help
    just run-examples # runs the example
    ```

6. Optionally, create a file `dev.py` locally within this folder with your own methods.

   ```bash
   just dev [args]
   ```
