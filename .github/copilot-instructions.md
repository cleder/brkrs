# brkrs Development Guidelines

Auto-generated from all feature plans.
Last updated: 2025-11-24

## Active Technologies

- N/A (in-memory ECS state only) (008-paddle-shrink-feedback)

- Rust 1.81 (Rust 2021 edition) + Bevy 0.17.3, bevy_rapier3d 0.32.0, serde 1.0, ron 0.8 (007-level-metadata)
- RON files in `assets/levels/` directory (007-level-metadata)
- ECS architecture with Bevy (all features)

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

- 008-paddle-shrink-feedback: Added Rust 1.81 (Rust 2021 edition) + Bevy 0.17.3, bevy_rapier3d 0.32.0

- 007-level-metadata: Added Rust 1.81 (Rust 2021 edition) + Bevy 0.17.3, bevy_rapier3d 0.32.0, serde 1.0, ron 0.8

- 006-audio-system: Added Rust 1.81 (Rust 2021 edition) + Bevy 0.17 (AudioPlugin, AudioSource, observers), bevy_rapier3d

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
