# Quickstart: Sea Mine Brick

## Prerequisites

- Rust 1.89 toolchain installed.
- Project builds on Bevy 0.17.3.
- `bevy_hanabi` 0.17.0 is added to `Cargo.toml` for explosion particles.

## Enable the Feature

1. Open a level file in `assets/levels/`.
2. Place brick index `31` where the sea mine brick should appear.
3. Run the game and destroy that brick with the ball.

## Expected Behavior

- Brick 31 is destroyed and spawns one sea mine hazard at the brick position.
- The sea mine moves in an arbitrary XZ direction and visibly spins.
- The sea mine keeps at least 3.0 u/s linear speed and 180 deg/s spin until it detonates.
- The mine detonates on contact with a wall, the paddle, or a brick with index greater than 90.
- The detonation removes balls and the paddle inside a 30-unit radius.
- Paddle destruction records exactly one life loss.
- A Hanabi explosion burst appears at the detonation point.

## Test Workflow

1. Run the focused tests for the sea mine feature.
2. Confirm the spawn test fails before implementation, then passes after implementation.
3. Verify the motion floor test across at least 10 `app.update()` calls.
4. Verify the Hanabi burst is emitted when detonation resolves.

## Notes

- The visual effect should reuse a loaded Hanabi effect asset instead of creating a new asset per explosion.
- Keep the gameplay messages and the particle burst separate: messages resolve state, the observer handles the visual burst.
- This feature assumes the project is built with Rust 1.89 or newer because Hanabi 0.17.0 is not compatible with Rust 1.81.
