[![Rust v1.94](https://img.shields.io/badge/rust%20v1.94-black)](https://www.rust-lang.org)
[![Python v3.14](https://img.shields.io/badge/python%20v3.14-1464b4.svg)](https://www.python.org)
[![Zig v0.15](https://img.shields.io/badge/zig%20v0.15-gold)](https://ziglang.org)

[![qa manual:main](https://github.com/noprd/example-rust-bindings/actions/workflows/manual.yaml/badge.svg?branch=main)](https://github.com/noprd/example-rust-bindings/actions/workflows/manual.yaml)
[![qa manual:staging](https://github.com/noprd/example-rust-bindings/actions/workflows/manual.yaml/badge.svg?branch=staging)](https://github.com/noprd/example-rust-bindings/actions/workflows/manual.yaml)

[![qa auto:staging](https://github.com/noprd/example-rust-bindings/actions/workflows/auto.yaml/badge.svg?branch=staging)](https://github.com/noprd/example-rust-bindings/actions/workflows/auto.yaml)
[![qa auto:current](https://github.com/noprd/example-rust-bindings/actions/workflows/auto.yaml/badge.svg)](https://github.com/noprd/example-rust-bindings/actions/workflows/auto.yaml)

# Example: Rust Bindings for Python #

Modern packages for python, for which efficiency is pivotal,
tend not to be written in python, but rather compiled languages such as c++, rust, zig, etc.
Notable examples include the widely used packages
[numpy](https://numpy.org),
[pydantic](https://docs.pydantic.dev),
[polars](https://docs.pola.rs/api/python/stable/reference/index.html),
as well as
[pydust](https://pydust.fulcrum.so).
Once written in these languages, particular tools are applied to obtain bindings
for higher level language such as python.

In the case of Rust, [maturin](https://www.maturin.rs) can be utilised for this purpose.

In this repository, we provide an example of this.
The python package created is written in Rust and provides a few simple classes
for recursive structures that occur in BIM models known as Psets.
To achieve this, we use [PyO3](https://pyo3.rs),
rely on rust's serde package to parse arbitrary JSON-like objects,
and develop a few simple pure rust structs/traits to handle tree structures.

Moreover, we demonstrate the generation of prebuilt python artefacts ("wheels"),
as well as a (albeit suboptimal) way to install the pre-built wheels
in [demo/python](demo/python) from possibly private repositories on GitHub.

## System Requirements ##

- [rust](https://www.rust-lang.org) (currently uses `v1.94`) incl. cargo
- [python3](https://www.python.org) (development was performed primarily with `v3.14`,
    but this should work with all versions starting from `v3.10`)
- [bash](https://gitforwindows.org)
- the [justfile](https://github.com/casey/just?tab=readme-ov-file#installation) tool

> [!IMPORTANT]
> We also require an installation of [Zig](https://ziglang.org) for cross-compilation.
> This avoids gcc-compiler issues on your local machine,
> which the rust compiler requires to install some dependencies.

Ensure paths to the binaries have been set.

> [!TIP]
> To verify, open a bash terminal and call.
>
> ```bash
> just --version
> # rust
> rustup --version
> rustc --version
> cargo --version
> # zig
> zig version
> ```

## Usage ##

Create files from templates

```bash
just setup
```

Adjust the newly created .env file.
Then run

```bash
just build # builds the rust code base
just build-py # builds bindings for python
```

QA tasks are as follows

```bash
just prettify # to check and perform linting
just tests-unit # to run unit tests
just test-unit {path/to/[tests_]file.rs}
just test-unit-optimised # to run with code optimised for architecture set in .env
just test-unit-optimised {path/to/[tests_]file.rs}
```

## Authorship and Attribution ##

- No references to personal or company data is contained in this repository.

- The here presented code was written entirely by
  the owner of the repository and account,
  in the context of other private repositories.

- None of the source code (rust), stubs (python),
  configuration files / task scripts (.gitignore, toml files, justfile etc.),
  or documents (e.g. this README.md file),
  were generated via LLMs.
  Nor have they been lifted from other people's work.
