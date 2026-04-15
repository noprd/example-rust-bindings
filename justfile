# ----------------------------------------------------------------
# NOTE: Setting shell does not work!
# For GitHub-actions we need "bash", but
# for Windows we need "sh".
# The solution is to ensure tasks are written with bash-shebang
# if they involve bash-syntax, e.g. 'if [[ ... ]] then else fi'.
# ----------------------------------------------------------------
# set shell := [ "bash", "-c" ]

_default:
    @- just --unsorted --list

menu:
    @- just --unsorted --choose

# ----------------------------------------------------------------
# Justfile
# Recipes for various workflows.
# ----------------------------------------------------------------

set dotenv-load := true
set positional-arguments := true

# --------------------------------
# VARIABLES
# --------------------------------

PATH_ROOT := justfile_directory()
CURRENT_DIR := invocation_directory()
OS := if os_family() == "windows" { "windows" } else { "linux" }
PYVENV_ON := if os_family() == "windows" { ". .venv/Scripts/activate" } else { ". .venv/bin/activate" }
PYVENV := if os_family() == "windows" { "python" } else { "python3" }
PYLINTING := "ruff"
RUST_TO_PY_BINDINGS := "maturin"
ZIG_FLAG := if os_family() == "windows" { "" } else { "--zig" }

# --------------------------------
# Macros
# --------------------------------

_clean-all-files path pattern:
    #!/usr/bin/env bash
    find {{path}} -type f -name "{{pattern}}" -exec basename {} \; 2> /dev/null
    find {{path}} -type f -name "{{pattern}}" -exec rm {} \; 2> /dev/null
    exit 0;

_clean-all-folders path pattern:
    #!/usr/bin/env bash
    find {{path}} -type d -name "{{pattern}}" -exec basename {} \; 2> /dev/null
    find {{path}} -type d -name "{{pattern}}" -exec rm -rf {} \; 2> /dev/null
    exit 0;

_check-tool tool name:
    #!/usr/bin/env bash
    success=false
    {{tool}} --version >> /dev/null 2> /dev/null && success=true;
    {{tool}} --help >> /dev/null 2> /dev/null && success=true;
    # NOTE: if exitcode is 251 (= help or print version), then render success.
    if [[ "$?" == "251" ]]; then success=true; fi
    # FAIL tool not installed
    if ( $success ); then
        echo -e "Tool \x1b[2;3m{{name}}\x1b[0m installed correctly.";
        exit 0;
    else
        echo -e "Tool \x1b[2;3m{{tool}}\x1b[0m did not work." >> /dev/stderr;
        echo -e "Ensure that \x1b[2;3m{{name}}\x1b[0m (-> \x1b[1mjust build\x1b[0m) installed correctly and system paths are set." >> /dev/stderr;
        exit 1;
    fi

_check-python-tool tool name:
    @just _check-tool "{{PYVENV}} -m {{tool}}" "{{name}}"

_rust_path_to_module path:
    #!/usr/bin/env bash
    path="{{path}}";
    path="${path%.*}";
    name="${path//[\/\\]/::}";
    name="${name#*::}";
    echo "${name}";
    exit 0;

_rust_path_to_test_module path:
    #!/usr/bin/env bash
    name="$(just _rust_path_to_module "{{path}}")";
    pref="";
    if [[ "$name" == *::* ]]; then
        parts_init="${name%::*}";
        name="${name##*::}";
        pref="${parts_init}::";
    fi
    if [[ "$name" != tests_* ]]; then
        name="tests_${name}";
    fi
    echo "${pref}${name}";
    exit 0;

# ----------------------------------------------------------------
# TARGETS
# ----------------------------------------------------------------

# --------------------------------
# TARGETS: build
# --------------------------------

[group("build")]
setup:
    @echo "TASK: SETUP"
    - cp -n "templates/template.env" ".env"
    @rustup toolchain install stable
    @rustup update
    @rustup override set stable

[group("build")]
build module="Main" build="zigbuild":
    @echo "Build using 'cargo {{build}}'"
    @just build-requirements
    @just build-compile "{{build}}"
    @# just build-binary "{{module}}" "{{build}}"
    @just check-system-requirements

[group("build")]
build-develop module="Main":
    @just build "{{module}}" "build"

[group("build")]
build-requirements:
    @- cargo +stable install --locked cargo-zigbuild 2> /dev/null

[group("build")]
build-binary module="Main" build="zigbuild":
    @rustup override set stable
    @cargo +stable {{build}} --target-dir "target" --release --bin "{{module}}"
    @cp "target/release/{{module}}" dist

[group("build")]
build-compile build="zigbuild":
    @rustup override set stable
    @cargo +stable {{build}} --target-dir "target" --release --lib

[group("build/python")]
build-py:
    @echo "TASK: BUILD BINDINGS FOR PYTHON"
    @- just build-py-venv
    @just build-py-requirements
    @just build-py-bindings

[group("build/python")]
build-py-venv:
    @echo "SUBTASK: create venv if not exists"
    @${PYTHON_PATH} -m venv .venv

[group("build/python")]
build-py-requirements:
    @echo "SUBTASK: build requirements"
    @just build-py-requirements-basic
    @just build-py-requirements-dependencies

[group("build/python")]
build-py-requirements-basic:
    @- {{PYVENV_ON}} && {{PYVENV}} -m pip install --upgrade pip 2> /dev/null
    @{{PYVENV_ON}} && pip install ruff uv

[group("build")]
build-py-requirements-dependencies:
    @echo "BUILD PACKAGE with dev dependencies"
    @{{PYVENV_ON}} && {{PYVENV}} -m uv sync \
        --verbose \
        --active \
        --compile-bytecode \
        --no-managed-python \
        --no-python-downloads

[group("build/python")]
build-py-bindings:
    @echo "Build compiled bindings."
    @{{PYVENV_ON}} && {{PYVENV}} -m {{RUST_TO_PY_BINDINGS}} build {{ZIG_FLAG}} \
        --bindings pyo3 \
        --ignore-rust-version \
        --release \
        --target "${CARGO_BUILD_TARGET}" \
        --out "dist"

# --------------------------------
# TARGETS: execution
# --------------------------------

[group("exec")]
run-rust module="${DEFAULT_MODULE:-UNKNOWN}" *args:
    @echo "Not yet implemented"
    @just build-binary "{{module}}"
    @cargo run --bin "{{module}}"
    @# "./target/release/{{module}}" {{args}}

# --------------------------------
# TARGETS: development
# --------------------------------

[group("dev")]
dev-rust *args:
    @echo "Run development script for rust"
    @echo "Not yet implemented"

[group("dev")]
dev-py *args:
    @echo "Run development script for python"
    @{{PYVENV_ON}} && {{PYVENV}} -m dev {{args}}

# --------------------------------
# TARGETS: tests
# --------------------------------

[group("tests")]
tests:
    @just tests-unit

[group("tests")]
tests-logs:
    @just tests

[group("tests")]
test-unit path *args:
    @cargo +stable zigbuild --tests
    @echo "run unit tests in $( just _rust_path_to_test_module "{{path}}")"
    @cargo +stable test --lib "$( just _rust_path_to_test_module "{{path}}")" {{args}} -- --nocapture
    @# echo "run unit tests in $( just _rust_path_to_module "{{path}}")"
    @# cargo +stable test --lib "$( just _rust_path_to_module "{{path}}")" {{args}} -- --nocapture

[group("tests")]
test-unit-optimised path *args:
    @cargo +stable zigbuild --tests --release
    @echo "run unit tests in $( just _rust_path_to_test_module "{{path}}")"
    @cargo +stable test --lib "$( just _rust_path_to_test_module "{{path}}")" {{args}} -- --nocapture
    @# echo "run unit tests in $( just _rust_path_to_module "{{path}}")"
    @# cargo +stable test --lib "$( just _rust_path_to_module "{{path}}")" {{args}} -- --nocapture

[group("tests")]
tests-unit *args:
    @# cargo +stable zigbuild --tests
    @cargo +stable test --lib {{args}} -- --nocapture

[group("tests")]
tests-unit-optimised *args:
    @# cargo +stable zigbuild --tests --release
    @cargo +stable test --release --bin {{args}} -- --nocapture

# --------------------------------
# TARGETS: prettify
# --------------------------------

[group("linting")]
lint path:
    @{{PYVENV_ON}} && {{PYVENV}} -m {{PYLINTING}} check \
        --respect-gitignore \
        --show-fixes \
        --no-unsafe-fixes \
        --exit-zero \
        --fix \
        "{{path}}"
    @{{PYVENV_ON}} && {{PYVENV}} -m {{PYLINTING}} format \
        --respect-gitignore \
        "{{path}}"

[group("linting")]
lint-dry path:
    @{{PYVENV_ON}} && {{PYVENV}} -m {{PYLINTING}} check \
        --respect-gitignore \
        --no-unsafe-fixes \
        --exit-zero \
        --diff \
        "{{path}}"

[group("linting")]
lint-check path:
    @{{PYVENV_ON}} && {{PYVENV}} -m {{PYLINTING}} check \
        --respect-gitignore \
        --no-unsafe-fixes \
        --exit-zero \
        --verbose \
        "{{path}}"

[group("linting")]
prettify:
    @echo "Force format not activated - running dry format instead"
    @- just prettify-dry 2> /dev/null
    @# cargo fmt --verbose
    @# cargo +nightly fmt --all --verbose -- --config-path rustfmt.toml
    @- just lint "${MODULE_NAME}.pyi" 2> /dev/null

[group("linting")]
prettify-dry:
    @# cargo fmt --verbose --check
    @cargo +nightly fmt --all --verbose --check -- --config-path rustfmt.toml
    @- just lint-dry "${MODULE_NAME}.pyi" 2> /dev/null

# --------------------------------
# TARGETS: clean
# --------------------------------

[group("clean")]
clean:
    @just clean-venv
    @just clean-basic

[group("clean")]
clean-basic:
    @echo "All system artefacts will be force removed."
    @- just _clean-all-files "." ".DS_Store" 2> /dev/null
    @echo "All build artefacts will be force removed."
    @cargo clean
    @- rm -rf "target"
    @just _clean-all-files "." "*.rs.bk"
    @- rm -rf ".venv" 2> /dev/null
    @- rm -rf "target" 2> /dev/null

[group("clean")]
clean-venv:
    @echo "VENV will be removed."
    @- just _delete-if-folder-exists ".venv" 2> /dev/null

# --------------------------------
# TARGETS: requirements
# --------------------------------

[group("system")]
check-system:
    @echo "Operating System detected:  {{os_family()}}"
    @echo "cargo command:              $( cargo +stable --version )"
    @echo "Rustc command:              $( rustc --version )"
    @echo "cargo Zigbuild:             $( cargo-zigbuild --version )"

[group("system")]
check-system-requirements:
    @just _check-tool "cargo" "cargo"
    @# just _check-tool "cargo +stable fmt -- --force" "cargo +stable fmt"
    @just _check-tool "cargo-zigbuild" "cargo-zigbuild"
