# brkrs Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-11-24

## Active Technologies
- Rust 1.81 (Rust 2021 edition) + Bevy 0.17, bevy_rapier3d (physics/collision) (005-multi-hit-bricks)
- RON level files (existing format supports indices 10-13) (005-multi-hit-bricks)

- Rust 1.81 (project), Python 3.11 for docs toolchain + Sphinx, MyST-Parser, furo-theme, sphinx-rtd-theme-compat (if needed), `cargo doc` for rustdoc generation (001-sphinx-docs)
- N/A — documentation stored in repo under `/docs/` and `specs/` for plans (001-sphinx-docs)

- Rust 1.81 (Rust 2021 edition via rustup) + Bevy 0.17 (ECS, input handling, window management, Time, rendering), bevy_rapier3d (physics simulation control)

## Project Structure

```text
src/
tests/
docs/
assets/
```

## Commands

run these commands to verify your work:

- `cargo test`
- `cargo fmt --all`
- `cargo clippy --all-targets --all-features`
- `bevy lint`

## Code Style

Rust 2021 edition (toolchain managed by rustup): Follow standard conventions

## Recent Changes
- 005-multi-hit-bricks: Added Rust 1.81 (Rust 2021 edition) + Bevy 0.17, bevy_rapier3d (physics/collision)

- 001-sphinx-docs: Added Rust 1.81 (project), Python 3.11 for docs toolchain + Sphinx, MyST-Parser, furo-theme, sphinx-rtd-theme-compat (if needed), `cargo doc` for rustdoc generation

- 001-sphinx-docs: Added [if applicable, e.g., PostgreSQL, CoreData, files or N/A]


<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
