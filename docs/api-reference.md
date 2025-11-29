# API Reference

The brkrs API documentation is generated from source code using rustdoc.

## Rust API (rustdoc)

The API documentation is generated locally using `cargo doc`. See [Building Documentation Locally](#building-documentation-locally) below.

:::{note}
The embedded rustdoc is available when viewing documentation built by CI (see the [GitHub Actions artifacts](https://github.com/cleder/brkrs/actions/workflows/docs-main.yml)).
On Read the Docs, generate the docs locally using the instructions below.
:::

The rustdoc includes:

- All public modules, structs, enums, and traits
- Component definitions used by the ECS
- System function signatures
- Resource types for game state

## Module Overview

The crate is organized into the following modules:

| Module | Description |
|--------|-------------|
| `brkrs` | Main crate with game initialization and Bevy app setup |
| `level_format` | Level file parsing and RON deserialization |
| `level_loader` | Level loading, entity spawning, and grid management |
| `pause` | Pause system state machine and UI overlay |
| `systems` | Game systems (respawn, textures, level switching, debug) |
| `ui` | User interface components and palette definitions |

## Building Documentation Locally

To generate the rustdoc locally:

```bash
# Generate rustdoc
cargo doc --no-deps --all-features

# Open in browser
cargo doc --no-deps --open
```

To include rustdoc in the Sphinx documentation build:

```bash
# Stage rustdoc to docs/_static/rustdoc
./scripts/stage-rustdoc.sh

# Build Sphinx docs
cd docs && make html
```

## Version Compatibility

This API documentation corresponds to the version of brkrs you are viewing.
Use the version selector in the bottom-left corner to switch between versions.
