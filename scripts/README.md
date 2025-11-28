# Asset migration helper

This folder contains small helper scripts used when landing major asset-format changes.

scripts/migrate-assets.sh

- Wrapper for the `tools/migrate-level-indices` CLI that updates assets/levels/ files
  in-place and writes `*.ron.bak` backups when `--backup` is supplied.

How to use

1. Build the migration CLI:

```bash
cd tools/migrate-level-indices && cargo build
```

1. Run the wrapper (example):

```bash
./scripts/migrate-assets.sh --backup --from 3 --to 20 assets/levels/*.ron
```

Notes

- The script is intentionally small and designed for use when landing the feature. CI will run tests that exercise the migration logic.
