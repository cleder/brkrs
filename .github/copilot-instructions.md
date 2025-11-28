# brkrs Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-11-24

## Active Technologies

- Rust 1.81 (Rust 2021 edition via rustup) + Bevy 0.17 (ECS, input handling, window management, Time, rendering), bevy_rapier3d (physics simulation control)

## Project Structure

```text
src/
tests/
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

- 004-pause-system: Added Rust 1.81 (Rust 2021 edition via rustup) + Bevy 0.17 (ECS, input handling, window management, Time, rendering), bevy_rapier3d (physics simulation control)

- 003-map-format: Added Rust 1.81 (Rust 2021 edition via rustup) + Bevy 0.17 (ECS, scheduling, `Time`, asset system), bevy_rapier3d (physics + collision), serde/ron (level asset parsing)

- 002-ball-respawn: Added Rust 1.81 (Rust 2021 edition via rustup) + Bevy 0.17 (ECS, scheduling, `Time`), bevy_rapier3d (physics + collision sensors), serde/ron (level matrix assets)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
