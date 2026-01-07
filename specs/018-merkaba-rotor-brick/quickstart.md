# Quickstart: Merkaba Rotor Brick

## Prerequisites

- Rust toolchain installed; repository builds successfully.
- Run tests and lints:
  - `cargo test`
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features`
  - `bevy lint`

## Enable Rotor Brick in a Level

1. Open a level RON file in `assets/levels/`.
2. Add a brick entry with `index = 36` at the desired position.
3. Ensure the level loads via the game.

## Behavior Overview

- Hitting rotor brick (36) emits a buffered spawn message; merkaba spawns at the brick position after 0.5s.
- Merkaba rotates around z-axis; initial direction is horizontal (y) ±20°; maintains ≥ 3.0 u/s y-speed.
- Collides and bounces off walls/bricks; despawns at goal.
- Paddle contact: lose 1 life; despawn all balls and all merkabas.
- Audio: distinct collision sounds for wall/brick/paddle; helicopter blade loop plays while at least one merkaba exists.

## Testing (TDD)

- Write failing tests first for each acceptance scenario.
- Verify message emission, delayed spawn, min y-speed enforcement, bounce behaviors, paddle contact consequences, and audio lifecycle.

## Notes

- Audio and texture assets may use placeholders; upgrade later without changing behavior.
