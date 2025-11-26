# Quickstart: Ball Respawn System

## Prerequisites

- Rust 1.81 toolchain via `rustup` (matches repo toolchain file).
- Assets present under `assets/levels/` (grid must include `1` and `2`).
- Optional: add WASM target for browser smoke tests (`rustup target add wasm32-unknown-unknown`).

## Build & Test Commands

```bash
# Run unit + integration tests
cargo test

# Targeted regression tests for multi-ball respawns
cargo test multi_respawn

# Lint for regressions
cargo clippy --all-targets --all-features

# Validate Bevy schedule layout
bevy lint

# Launch the game with dynamic linking for faster reloads
cargo run --features bevy/dynamic_linking

# (Optional) WASM build to ensure timer logic is platform agnostic
cargo build --target wasm32-unknown-unknown --release
```

## Automated Coverage

- `tests/common/multi_respawn.rs` verifies that queued `LifeLostEvent`s process sequentially and game-over states halt new respawns. Run `cargo test multi_respawn` to execute only these cases when iterating on User Story 2.

## Manual Verification

1. **Respawn delay and positions**
   - Play level 001, allow the ball to hit the lower goal.
   - Observe the 1 second pause (ball + paddle hidden) and confirm both respawn exactly at the grid-defined transforms.

1. **Stationary until controls return (launch-input check)**
   - After respawn completes, keep your hands off the launch input (space/left click). The ball must remain frozen atop the paddle until movement controls unlock, then resume motion on its own the exact frame you regain control—no manual launch allowed.

1. **Controls locked**
   - Attempt to move the paddle during the respawn delay; input should be ignored until the timer completes and both the paddle and ball release together.

1. **Lives integration hook**
   - Enable debug logging (`RUST_LOG=info cargo run`). Lose a ball and confirm logs similar to:

   ```text
   life lost: ball=Entity(34) cause=LowerGoal spawn=(0.00, 2.00, 0.00)
   respawn scheduled: completes_at=12.45 remaining_lives=2
   ```

1. **Repeated losses + game-over skip**
   - Intentionally lose the ball multiple times in a row. Watch for log output indicating queued respawns and the queue length:

   ```text
   warn: respawn already pending; queued additional LifeLostEvent (queue_len=1)
   info: life lost: ... remaining_lives=1
   ```

   - When the final life is lost, expect a `GameOverRequested` log (remaining_lives=0) and no further respawn scheduling entries. Confirm at least 5 consecutive respawns complete without panics or timer drift.

1. **Multi-ball safety (if feature flag enabled)**
   - Spawn an extra ball (debug command). Lose only one ball and ensure the remaining ball stays active while only the lost ball respawns.

## Troubleshooting

- No respawn? Ensure the lower goal collider sets `Sensor` and `ActiveEvents::COLLISION_EVENTS` in `bevy_rapier3d` setup.
- Ball starts too early? Ensure `BallFrozen` is applied during respawn scheduling and that no system removes `InputLocked` prematurely (the automatic release only fires when both unlock together).
- Paddle still moveable during delay? Verify the input system checks for `InputLocked` before applying translations.
- Timer longer/shorter than 1 second? Inspect `RespawnSchedule.timer` for correct duration and confirm `Time` resource is not scaled for slow-mo when testing.
