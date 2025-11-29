#!/usr/bin/env bash
# Stage rustdoc artifacts for Sphinx documentation build
#
# This script copies the generated rustdoc output from target/doc/
# to docs/_static/rustdoc/ so it can be included in the Sphinx build.
#
# Usage: ./scripts/stage-rustdoc.sh
#
# Prerequisites:
#   - Run `cargo doc --no-deps --all-features` first
#   - target/doc/ must exist with generated documentation

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

SOURCE_DIR="$REPO_ROOT/target/doc"
DEST_DIR="$REPO_ROOT/docs/_static/rustdoc"

# Check if rustdoc output exists
if [[ ! -d "$SOURCE_DIR" ]]; then
    echo "ERROR: Rustdoc output not found at $SOURCE_DIR"
    echo "Run 'cargo doc --no-deps --all-features' first."
    exit 1
fi

# Create destination directory if it doesn't exist
mkdir -p "$DEST_DIR"

# Remove old rustdoc artifacts
rm -rf "${DEST_DIR:?}"/*

# Copy rustdoc output to Sphinx static directory
cp -r "$SOURCE_DIR"/* "$DEST_DIR/"

echo "✓ Staged rustdoc artifacts from $SOURCE_DIR to $DEST_DIR"
echo "  Files copied: $(find "$DEST_DIR" -type f | wc -l)"
