# brkrs Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-11-24

## Active Technologies
- Rust 1.81 (Rust 2021 edition) + Bevy 0.16 (PBR, asset system), bevy_rapier3d 0.31 (physics-driven gameplay), serde/ron for level parsing (001-apply-textures)
- File-based assets only (RON levels + PNG textures) (001-apply-textures)
- Rust 1.81 (Rust 2021 edition managed via rustup) (002-ball-respawn)
- File-based RON under `assets/levels/`; runtime-only ECS resources for respawn schedule and progress (no persistent storage) (002-ball-respawn)

- Rust 2021 edition (toolchain managed by rustup) (001-complete-game)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 2021 edition (toolchain managed by rustup): Follow standard conventions

## Recent Changes
- 002-ball-respawn: Added Rust 1.81 (Rust 2021 edition managed via rustup)
- 001-apply-textures: Added Rust 1.81 (Rust 2021 edition) + Bevy 0.16 (PBR, asset system), bevy_rapier3d 0.31 (physics-driven gameplay), serde/ron for level parsing

- 001-complete-game: Added Rust 2021 edition (toolchain managed by rustup)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
