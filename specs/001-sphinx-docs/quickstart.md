# Quickstart: Building and previewing docs locally

This quickstart explains how to build the Sphinx + MyST documentation locally and preview the site.
It also covers generating rustdoc and bundling it into the Sphinx build for local verification.

Prerequisites

- Python 3.11+ and a virtual environment
- Node (optional for some assets), and [Rust toolchain] installed (rustup + cargo)

1. Create and activate a Python virtualenv

```bash
python -m venv .venv
source .venv/bin/activate
pip install -U pip setuptools
pip install -r docs/requirements.txt
```

1. Build rustdoc artifacts

```bash
# build docs for the crate without dependencies to speed things up
cargo doc --no-deps --all-features
# generated artifacts appear in target/doc/
```

1. Build the Sphinx site locally and stage rustdoc

```bash
# from repository root
rm -rf docs/_build
# copy rustdoc into docs/_static/rustdoc (example)
mkdir -p docs/_static/rustdoc
cp -r target/doc/* docs/_static/rustdoc/

# build the Sphinx site
sphinx-build -b html docs/ docs/_build/html

# open the site
python -m http.server --directory docs/_build/html 8000
# then open http://localhost:8000 in your browser
```

1. Fast PR checks

When authoring smaller doc changes, prefer running a fast check instead of a full rustdoc build.
Use the `docs-pr` script or CI job that validates links and runs `sphinx-build` with incremental builds.
