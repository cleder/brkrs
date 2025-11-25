# Quickstart: Ball & Paddle Respawn + Level Flow

## Prerequisites

- Rust toolchain 1.81 installed via `rustup` (matches workspace `rust-toolchain.toml`).
- Fetch dependencies: `cargo fetch` at repo root.
- Ensure assets are available (`assets/levels/level_001.ron`, etc.).
- WASM target (`wasm32-unknown-unknown`) added if testing web build: `rustup target add wasm32-unknown-unknown`.

## Build & Test Commands

1. Native build + tests (Rust + Rapier + Bevy):

   ```bash
   cargo test && cargo clippy --all-targets --all-features
   ```

2. Run the game (native):

   ```bash
   cargo run --features bevy/dynamic_linking
   ```

3. WASM smoke test (optional, ensures cross-platform parity):

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

## Manual Verification Steps

1. **Life Loss Triggers Respawn**
   - Launch the game, let the ball fall past the lower goal.
   - Confirm both ball and paddle despawn, a 1-second pause occurs, then both respawn at the original grid-defined positions.
2. **Paddle Growth & Ball Freeze**
   - Observe paddle reappearing small and scaling up smoothly over ~2 seconds.
   - During the growth animation, confirm the ball remains stationary/frozen.
3. **Level-Specific Gravity**
   - Modify `assets/levels/level_002.ron` to include a non-default `gravity` vector (example in plan).
   - Load the level, confirm ball trajectory reflects the override while standard gravity resumes on other levels.
4. **Fade Overlay & Level Advance**
   - Clear all bricks (or temporarily reduce brick count) to trigger level completion.
   - Ensure a fade-out occurs, the next level loads after the delay, and fade-in restores gameplay.
5. **Manual Restart (`R` key)**
   - During play, press `R`.
   - Verify fade overlay replays, the current level reloads, and paddle/ball respawn at initial positions with counters reset appropriately.
6. **Repeated Respawns**
   - Lose the ball multiple times; ensure respawn timing and animations remain consistent and no stale timers accumulate.

## Troubleshooting

- If respawn never triggers, ensure lower-goal collider has `Sensor` + `ActiveEvents::COLLISION_EVENTS` enabled (check Rapier debug logs).
- If paddle keeps shrinking/growing instantly, verify `bevy_tweening` feature is enabled in `Cargo.toml` and the tween system runs in the main schedule.
- For gravity overrides not applying, confirm `LevelDefinition.gravity` exists in the RON file and `LevelOverrides` resource logs the new vector.
