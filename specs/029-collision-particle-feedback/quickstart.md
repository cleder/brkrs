# Quickstart: Collision Particle Feedback

## 1. Prerequisites

- Branch: `029-collision-particle-feedback`
- Spec: `specs/029-collision-particle-feedback/spec.md`
- Plan: `specs/029-collision-particle-feedback/plan.md`

## 2. TDD First (Required)

1. Add failing tests before implementation:
   - New integration tests in `tests/collision_particle_feedback.rs`
2. Cover, at minimum:
   - Wall collision spawns effect in same frame
   - Paddle collision spawns effect in same frame
   - Brick collision spawns effect in same frame
   - Brick destroyed on impact still spawns effect
   - Burst collisions spawn one effect per collision (no cap)
   - Pause suppresses spawns and does not replay on resume
   - Lifetime window is 0.20-0.35 seconds
   - Particle count is 8-16 per collision
   - Spawn point equals exact collision contact point
3. Run tests and confirm red phase:

```bash
cargo test collision_particle_feedback -- --nocapture
```

4. Commit red phase before implementation.

## 3. Implement Feature

1. Add collision feedback systems under `src/systems/` (planned file: `collision_feedback.rs`).
2. Register systems/observers in plugin setup via `src/systems/mod.rs` and `src/lib.rs`.
3. Integrate trigger emission with existing collision entry points:
   - `detect_ball_wall_collisions`
   - `mark_brick_on_ball_collision`
   - `read_character_controller_collisions`
4. Ensure all new effect instances:
   - Spawn at exact contact point
   - Use 8-16 particles
   - Despawn within 0.20-0.35 seconds

## 4. Validate Locally

```bash
cargo test
cargo fmt --all
cargo clippy --all-targets --all-features
bevy lint
```

## 5. Manual Verification Pass

1. Launch game and verify visible sparkly feedback for wall, paddle, and brick hits.
2. Trigger rapid collision sequences and confirm no capped suppression during active play.
3. Pause game and confirm no new effects appear while paused.
4. Resume and confirm suppressed effects were not replayed.
5. Destroy bricks on impact and confirm effect still appears at contact point.

## 6. Implementation Validation Notes (2026-05-24)

- `cargo check --lib`: PASS
- `cargo fmt --all`: PASS
- `cargo clippy --all-targets --all-features --no-deps`: FAIL (workspace-level test diagnostics)
  - New `tests/collision_particle_feedback.rs` API issues were reported and fixed:
    - `World::query` generic usage corrected to `query_filtered` where filters are required.
    - Helper now accepts `&mut World` for query state construction.
  - Remaining clippy output includes pre-existing warnings in unrelated test files.
- `cargo test collision_particle_feedback -- --nocapture`: BLOCKED by environment linker instability during full test-target build (`ld.lld` undefined hidden symbol errors while linking unrelated test binaries).
- `bevy lint`: BLOCKED by concurrent background cargo lock contention during this session.

Follow-up for full green validation:

1. Re-run `cargo test --test collision_particle_feedback -- --nocapture` in a clean terminal session with no concurrent cargo tasks.
2. Re-run `bevy lint` after cargo lock contention is clear.
3. Run full suite (`cargo test`, `cargo clippy --all-targets --all-features`) once linker issue is resolved.
