#!/usr/bin/env bash
set -euo pipefail

# Small wrapper script used when landing the feature to update repository assets.
# It calls the tools/migrate-level-indices helper to convert tile indices across
# files under assets/levels and leaves backups in place.

prog_dir="$(dirname "$0")/.."
tool_bin="${prog_dir}/tools/migrate-level-indices/target/debug/migrate-level-indices"

if [[ ! -x "$tool_bin" ]]; then
  echo "migration tool not built: $tool_bin"
  echo "Build it with: (cd tools/migrate-level-indices && cargo build)"
  exit 1
fi

if [[ $# -lt 3 ]]; then
  echo "Usage: $0 --from <old-index> --to <new-index> [--backup]"
  echo "Example: $0 --backup --from 3 --to 20 assets/levels/*.ron"
  exit 2
fi

"$tool_bin" "$@"

echo "Migration finished. Review .bak files and commit updated assets/levels/"
