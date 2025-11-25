# Quickstart: Ball Respawn System

## Prerequisites

- Rust 1.81 toolchain via `rustup` (matches repo toolchain file).
- Assets present under `assets/levels/` (grid must include `1` and `2`).
- Optional: add WASM target for browser smoke tests (`rustup target add wasm32-unknown-unknown`).

## Build & Test Commands

```bash
# Run unit + integration tests
cargo test

# Lint for regressions
cargo clippy --all-targets --all-features

# Launch the game with dynamic linking for faster reloads
cargo run --features bevy/dynamic_linking

# (Optional) WASM build to ensure timer logic is platform agnostic
cargo build --target wasm32-unknown-unknown --release
```

## Manual Verification

1. **Respawn delay and positions**
   - Play level 001, allow the ball to hit the lower goal.
   - Observe the 1 second pause (ball + paddle hidden) and confirm both respawn exactly at the grid-defined transforms.
2. **Stationary ball**
   - After respawn, verify the ball remains frozen atop the paddle until you press the launch input. No drift should occur.
3. **Controls locked**
   - Attempt to move the paddle during the respawn delay; input should be ignored until the timer completes.
4. **Lives integration hook**
   - Enable debug logging (`RUST_LOG=info cargo run`). Lose a ball and confirm a `LifeLostEvent` log followed by either respawn scheduling or game-over skip when lives reach zero.
5. **Repeated losses**
   - Intentionally lose the ball multiple times in a row. Verify timers reset correctly and the system handles at least 5 consecutive respawns without panics or timer drift.
6. **Multi-ball safety (if feature flag enabled)**
   - Spawn an extra ball (debug command). Lose only one ball and ensure the remaining ball stays active while only the lost ball respawns.

## Troubleshooting

- No respawn? Ensure the lower goal collider sets `Sensor` and `ActiveEvents::COLLISION_EVENTS` in `bevy_rapier3d` setup.
- Ball keeps moving after respawn? Confirm `BallFrozen` marker exists and velocity is forced to zero when the respawn completes.
- Paddle still moveable during delay? Verify the input system checks for `InputLocked` before applying translations.
- Timer longer/shorter than 1 second? Inspect `RespawnSchedule.timer` for correct duration and confirm `Time` resource is not scaled for slow-mo when testing.
