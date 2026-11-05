# cargo-semver-python

Python bindings to the Rust [`semver`](https://crates.io/crates/semver) crate.
The `matches()` function uses **Cargo's interpretation** of version
requirements, which can differ from npm or Python package version constraints.

## Requirements

- Python 3.11+
- Rust, Cargo, and a native C compiler
- Git (for installation directly from GitHub)

## Install from GitHub

```sh
python -m pip install "cargo-semver-python @ git+https://github.com/tomokinakamaru/cargo-semver-python.git"
```

To install a specific release, append a Git tag such as `@v0.1.0` to the URL
after the `.git` suffix. The installer builds the Rust extension **locally**,
so Rust and a supported native build toolchain must be available. Python's
build isolation installs maturin automatically.

## Usage

```python
from cargo_semver_python import compare, matches, parse, validate

assert parse("1.2.3-alpha.1+build.5") == {
    "major": 1, "minor": 2, "patch": 3,
    "prerelease": "alpha.1", "build": "build.5",
}
assert validate("1.2.3")
assert not validate("1.2")
assert compare("1.2.3", "1.3.0") == -1
assert compare("1.0.0+abc", "1.0.0+def") == 0
assert matches("1.2.3", "^1.0")
assert matches("1.2.3", "1.2")
assert not matches("1.2.3-alpha", "^1.0")
```

- `parse(version)` returns a dictionary with integer `major`, `minor`, and
  `patch` fields and string-or-`None` `prerelease` and `build` fields.
  It raises `ValueError` for an invalid version.
- `validate(version)` returns `True` for valid full SemVer versions and
  `False` otherwise.
- `compare(left, right)` returns `-1`, `0`, or `1` according to SemVer
  precedence; build metadata does not affect the result.
- `matches(version, requirement)` checks a Cargo-style version requirement,
  including caret, tilde, comparison, wildcard, and prerelease rules.

`parse()`, `compare()`, and `matches()` raise `ValueError` on malformed inputs.

## Development

```sh
python -m pip install .
python -m unittest discover -s tests -v
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

The GitHub Actions workflow tests source installation on Python 3.11 and 3.13.

## Scope

This project provides Python bindings for Cargo-style Semantic Versioning.
It does not resolve dependencies or query the crates.io index.
