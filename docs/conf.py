# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

import os
from pathlib import Path

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "brkrs"
copyright = "2025, Christian Ledermann"
author = "Christian Ledermann"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "myst_parser",
    "sphinx_copybutton",
]

# If using :github: option
github_username = "cleder"
github_repository = "brkrs"

# MyST-Parser configuration
myst_enable_extensions = [
    "colon_fence",
    "deflist",
    "fieldlist",
    "tasklist",
]

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

# Source file suffixes
source_suffix = {
    ".rst": "restructuredtext",
    ".md": "markdown",
}

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "furo"
html_static_path = ["_static"]

# Furo theme options
html_theme_options = {
    "source_repository": "https://github.com/cleder/brkrs",
    "source_branch": "develop",
    "source_directory": "docs/",
}

# Include rustdoc output in the build (only if directory exists)
# Set SPHINX_SKIP_RUSTDOC=1 to skip rustdoc embedding (for fast PR builds)
_rustdoc_path = Path(__file__).parent / "_static" / "rustdoc"
if not os.environ.get("SPHINX_SKIP_RUSTDOC") and _rustdoc_path.exists():
    html_extra_path = ["_static/rustdoc"]

# Suppress expected warnings
suppress_warnings = [
    # MyST cross-reference warnings for external HTML files (rustdoc links)
    "myst.xref_missing",
]
