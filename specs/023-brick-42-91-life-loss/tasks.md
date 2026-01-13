# Implementation Tasks: Brick Types 42 & 91 — Paddle Life Loss

**Feature**: 023-brick-42-91-life-loss  
**Created**: 2026-01-13  
**Status**: READY FOR IMPLEMENTATION  
**Total Tasks**: 24

---

## Overview

This document breaks down the 4-phase implementation plan into **24 specific, actionable tasks**.
Each task is independently completable and includes:

- **Task ID**: Sequential identifier (T001–T024)
- **Phase**: 0–4 (Setup, Foundational, User Stories, Polish)
- **Story Label**: [US1], [US2], [US3] (User Story phases only)
- **Parallelizable**: [P] if independent of other in-flight tasks
- **Description**: Exact action with file paths

---

## Phase 0: Constants & Helpers (Tasks T001–T003)

**Goal**: Set up brick type constants and helper functions; ensure type 91 is recognized by the system.

**Prerequisites**: None  
**Acceptance**: `cargo clippy --all-targets` passes; helper functions compile without errors.

### Tasks

- [ ] T001 Add `HAZARD_BRICK_91` constant to [src/level_format/mod.rs](src/level_format/mod.rs) with value 91 and doc comment: "Brick type 91: indestructible hazard that causes life loss on paddle collision"
- [ ] T002 [P] Add `is_hazard_brick(type_id: u8) -> bool` helper function in [src/level_format/mod.rs](src/level_format/mod.rs) returning `true` for types 42 and 91; include unit test verifying both return `true`
- [ ] T003 Update level loader in [src/level_loader.rs](src/level_loader.rs) line ~120–150 to exclude type 91 from `CountsTowardsCompletion` marker insertion: replace `if brick_type_id != INDESTRUCTIBLE_BRICK { entity.insert(...); }` with `if brick_type_id != INDESTRUCTIBLE_BRICK && brick_type_id != HAZARD_BRICK_91 { entity.insert(...); }`

---

## Phase 1: Paddle-Brick Collision Life-Loss System (Tasks T004–T007)

**Goal**: Implement paddle collision detection for hazard bricks and emit `LifeLostEvent` with single-loss-per-frame guarantee.

**Prerequisites**: Phase 0 complete  
**Acceptance**: Paddle collision with type 42 or 91 emits exactly one `LifeLostEvent` per frame; multiple hazardous contacts in same frame emit only one event.

### Tasks

- [ ] T004 [P] [US2] Create frame-scoped life-loss tracking system by adding `clear_life_loss_frame_flag` system in [src/systems/respawn.rs](src/systems/respawn.rs) that:
  - Adds parameter `mut frame_flag: Local<bool>`
  - Resets `frame_flag` to `false` at start of each frame
  - Runs in `Update` schedule before `read_character_controller_collisions` (verify schedule insertion point)

- [ ] T005 [P] [US2] Extend paddle collision detection in [src/lib.rs](src/lib.rs) function `read_character_controller_collisions()` to:
  - Add `brick_types: Query<&BrickTypeId>` parameter
  - Add `mut life_lost_writer: MessageWriter<LifeLostEvent>` parameter
  - Add `frame_loss_flag: Local<bool>` to track per-frame losses
  - When paddle collides with entity, query brick type and check `is_hazard_brick(type_id)`
  - If hazard and flag is `false`: emit `LifeLostEvent` with first ball entity found and set flag to `true`
  - If hazard and flag is `true`: skip emission (already sent this frame)

- [ ] T006 [US2] Add `LifeLossCause::PaddleHazard` variant to enum in [src/systems/respawn.rs](src/systems/respawn.rs) (or document reuse of `LifeLossCause::LowerGoal` if extending not desired) and update `LifeLostEvent` handling to accept this cause without panicking

- [ ] T007 [P] [US2] Create integration test fixture in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs) named `test_paddle_brick_42_life_loss()` that:
  - Spawns a test level with one type 42 brick, paddle, and ball
  - Simulates paddle-brick collision
  - Verifies exactly one `LifeLostEvent` message is emitted
  - Verifies lives decremented by 1

---

## Phase 2: Ball-Brick Collision & Destruction (Tasks T008–T010)

**Goal**: Ensure type 42 is destroyed by ball collision and type 91 is indestructible; verify scoring integration.

**Prerequisites**: Phase 0 complete  
**Acceptance**: Type 42 destroyed on ball collision, scores 90 points, emits `BrickDestroyed`; type 91 remains on ball collision, awards 0 points, no event emitted.

### Tasks

- [ ] T008 [P] [US1] Extend ball-brick collision system in [src/lib.rs](src/lib.rs) function `mark_brick_on_ball_collision()` to:
  - Before marking brick for destruction, query brick type
  - If `is_hazard_brick(type_id) && type_id == HAZARD_BRICK_91`: skip marking (brick remains)
  - If `type_id == BRICK_TYPE_42` or other destructible: mark with `MarkedForDespawn` as normal
  - Verify `BrickDestroyed` message is emitted only for type 42

- [ ] T009 [P] [US1] Verify scoring system in [src/systems/scoring.rs](src/systems/scoring.rs) correctly maps type 42 to 90 points:
  - Locate `brick_points()` function and confirm it returns 90 for `brick_type == 42`
  - Add comment: `// Type 42: Destructible, awards 90 points`
  - If not already present, no code change needed (existing behavior)

- [ ] T010 [US1] Create integration test fixture in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs) named `test_ball_brick_42_destroyed_scores_90()` that:
  - Spawns a test level with one type 42 brick and ball
  - Simulates ball-brick collision
  - Verifies brick is removed from world (despawned)
  - Verifies score increased by exactly 90 points
  - Verifies `BrickDestroyed` message emitted with `brick_type == 42`

---

## Phase 3: Indestructibility & Level Completion (Tasks T011–T014)

**Goal**: Integrate type 91 indestructibility; verify level completion logic; add visual assets.

**Prerequisites**: Phases 0–2 complete  
**Acceptance**: Type 91 bricks do not block level completion; levels with only type 91 are immediately complete; visual representation consistent with indestructible bricks.

### Tasks

- [ ] T011 [P] [US3] Create integration test fixture in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs) named `test_ball_brick_91_indestructible()` that:
  - Spawns a test level with one type 91 brick and ball
  - Simulates ball-brick collision
  - Verifies brick remains in world (not despawned)
  - Verifies score unchanged (0 points awarded)
  - Verifies NO `BrickDestroyed` message emitted

- [ ] T012 [P] [US3] Create integration test fixture in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs) named `test_brick_42_contributes_to_completion()` that:
  - Spawns a test level with 2 type 42 bricks and 1 type 91 brick
  - Simulates destruction of all type 42 bricks via ball collision
  - Verifies level completion is triggered (count of `CountsTowardsCompletion` entities reaches 0)
  - Verifies type 91 brick remains visible

- [ ] T013 [US3] Verify level completion query in [src/level_loader.rs](src/level_loader.rs) line ~200–250:
  - Locate completion check: query entities with `With<CountsTowardsCompletion>`
  - Confirm it only counts type 42 (which have marker) and NOT type 91 (which don't)
  - Add comment: `// Type 91 bricks excluded from completion (no CountsTowardsCompletion marker)`
  - If no changes needed, document existing behavior

- [ ] T014 [P] [US3] Add texture/material support for type 91 in [assets/textures/manifest.ron](assets/textures/manifest.ron):
  - Add entry for brick type 91 with same material as type 90 (indestructible) or create distinct material
  - Include visual indicator (e.g., different color) that distinguishes from destructible bricks
  - Verify texture loads without errors: `cargo build`

---

## Phase 4: Testing & Validation (Tasks T015–T024)

**Goal**: Complete comprehensive test suite; validate all user stories; ensure no regressions.

**Prerequisites**: Phases 0–3 complete  
**Acceptance**: All 9 tests pass; no clippy/fmt errors; success criteria met; zero regressions.

### Tasks

- [ ] T015 [P] [US1] Create test `test_brick_42_ball_collision_awards_90_points()` in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs):
  - **Setup**: Spawn world with brick type 42 and ball
  - **Action**: Simulate ball-brick collision
  - **Assert**:
    - Brick is removed from world
    - Score increased by exactly 90
    - `BrickDestroyed` message emitted
  - **Acceptance**: Test passes consistently on 10 runs

- [ ] T016 [P] [US1] Create test `test_brick_42_multiple_destroys_score_correctly()` in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs):
  - **Setup**: Spawn world with 3 type 42 bricks and ball
  - **Action**: Simulate ball collisions with each brick sequentially
  - **Assert**:
    - Each destruction awards 90 points
    - Total score = 270
    - No double-scoring on single brick
  - **Acceptance**: Test passes; validates no scoring bugs with multiple bricks

- [ ] T017 [P] [US2] Create test `test_paddle_brick_42_life_loss()` in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs):
  - **Setup**: Spawn world with 3 lives, paddle, and type 42 brick
  - **Action**: Simulate paddle collision with brick
  - **Assert**:
    - Lives decremented to 2
    - `LifeLostEvent` message emitted
    - Standard respawn flow initiates
  - **Acceptance**: Test passes; validates paddle-brick 42 life loss integration

- [ ] T018 [P] [US2] Create test `test_paddle_brick_91_life_loss()` in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs):
  - **Setup**: Spawn world with 3 lives, paddle, and type 91 brick
  - **Action**: Simulate paddle collision with brick
  - **Assert**:
    - Lives decremented to 2
    - `LifeLostEvent` message emitted
    - Type 91 brick remains (not destroyed)
  - **Acceptance**: Test passes; validates paddle-brick 91 life loss integration

- [ ] T019 [P] [US2] Create test `test_single_life_loss_per_frame_multi_contact()` in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs):
  - **Setup**: Spawn world with 3 lives, paddle, 1 type 42 brick, and 1 type 91 brick positioned to contact paddle simultaneously
  - **Action**: Simulate single-frame paddle collision with both bricks
  - **Assert**:
    - Lives decremented by exactly 1 (not 2)
    - Exactly one `LifeLostEvent` message emitted
    - Both bricks' collision data processed (one loss only)
  - **Acceptance**: Test passes; validates multi-contact policy (one loss per frame max)

- [ ] T020 [P] [US2] Create test `test_life_loss_frame_flag_resets()` in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs):
  - **Setup**: Spawn world with 3 lives, paddle, and type 42 brick
  - **Action**:
    - Frame 0: Simulate paddle collision with brick → expect 1 `LifeLostEvent`
    - Frame 1: Simulate paddle collision with same brick again → expect 1 `LifeLostEvent` (not blocked by previous frame)
  - **Assert**:
    - Lives decremented by 2 (once per frame)
    - Frame flag resets between frames correctly
  - **Acceptance**: Test passes; validates per-frame flag behavior

- [ ] T021 [P] [US3] Create test `test_brick_91_indestructible_ball_collision()` in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs):
  - **Setup**: Spawn world with type 91 brick and ball
  - **Action**: Simulate ball-brick collision
  - **Assert**:
    - Brick remains in world (not despawned)
    - Score unchanged (0 points awarded)
    - No `BrickDestroyed` message emitted
  - **Acceptance**: Test passes; validates indestructibility contract

- [ ] T022 [P] [US3] Create test `test_level_completion_with_type_91_present()` in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs):
  - **Setup**: Spawn test level with 2 type 42 bricks and 3 type 91 bricks
  - **Action**: Destroy all type 42 bricks via ball collision
  - **Assert**:
    - Level completion triggered
    - Type 91 bricks remain visible
    - Completion not blocked by type 91 presence
  - **Acceptance**: Test passes; validates level completion integration

- [ ] T023 [P] [US1] [US2] [US3] Create test `test_score_and_lives_persist_multi_frame()` in [tests/brick_42_91_life_loss.rs](tests/brick_42_91_life_loss.rs):
  - **Setup**: Spawn world with 3 lives, type 42 brick, type 91 brick, paddle, and ball
  - **Action**:
    - Destroy type 42 via ball collision (score += 90)
    - Collide paddle with type 91 (lives -= 1)
    - Run 10 additional frames without collisions
  - **Assert**:
    - Score remains 90 across all 10 frames (no overwrites, no resets)
    - Lives remain 2 across all 10 frames
    - Multi-frame persistence confirmed
  - **Acceptance**: Test passes; validates no frame-boundary regressions

- [ ] T024 Run full test suite, linting, and formatting checks:
  - `cargo test --all` in [/home/christian/devel/bevy/brkrs](/)
  - `cargo clippy --all-targets --all-features` — must pass with no warnings
  - `cargo fmt --all` — must have no formatting changes required
  - `bevy lint` — if available, must pass
  - **Acceptance**: All 9 tests (T015–T023) pass; no clippy errors; code formatted correctly

---

## Summary & Execution Order

### Task Count by Phase

| Phase | Tasks | Count | Goal |
|-------|-------|-------|------|
| Phase 0: Constants & Helpers | T001–T003 | 3 | Setup brick type constants; add helper functions |
| Phase 1: Paddle Collision | T004–T007 | 4 | Implement life-loss on paddle collision; per-frame limit |
| Phase 2: Ball Collision | T008–T010 | 3 | Ensure type 42 destroyed, type 91 indestructible; verify scoring |
| Phase 3: Completion & Assets | T011–T014 | 4 | Verify level completion; add textures |
| Phase 4: Testing & Validation | T015–T024 | 10 | Complete test suite; validate all behaviors |
| **TOTAL** | **T001–T024** | **24** | **Full implementation & validation** |

### Parallelization Opportunities

**Can run in parallel** (independent file modifications):

- T001 + T002 + T003 (Phase 0 — all in different sections of same files or separate files)
- T007 + T010 + T011 + T012 + T014 (Test creation — independent test functions)
- T015–T023 (Test creation — independent test functions)
- T004 + T005 + T006 (Phase 1 — different systems/files)
- T008 + T009 (Phase 2 — different systems/files)
- T013 (Verification — read-only)

**Sequential dependencies**:

1. Phase 0 (T001–T003) must complete before Phase 1–4
2. Phase 1 (T004–T007) and Phase 2 (T008–T010) should complete before Phase 3
3. Phase 3 (T011–T014) should complete before Phase 4
4. Phase 4 (T015–T024) depends on all previous phases

### Recommended Execution Path

**Day 1 — Phase 0 & 1** (fastest feedback):

1. T001 + T002 (parallel) — add constants & helpers (10 min)
2. T003 — update level loader (5 min)
3. T004 + T005 + T006 (parallel) — implement paddle collision (30 min)
4. T007 — first integration test (15 min)
5. Run `cargo test T007` to verify phase 1 (5 min)

**Day 2 — Phase 2 & 3** (core feature):

1. T008 + T009 (parallel) — ball collision & scoring (15 min)
2. T010 + T011 + T012 (parallel) — more tests (20 min)
3. T013 + T014 (parallel) — verification & assets (10 min)
4. Run `cargo test brick_42_91_life_loss` to validate (5 min)

**Day 3 — Phase 4** (comprehensive validation):

1. T015–T023 (create all tests in parallel) — 60 min
2. T024 — run full suite, clippy, fmt (20 min)
3. Final validation: all tests pass, no warnings

### Success Criteria Mapping

Each task is tied to one or more success criteria from spec.md:

- **T001–T003**: Foundational; enable all subsequent tasks
- **T004–T007**: Address SC-002 (paddle life loss occurs, persists across frames)
- **T008–T010**: Address SC-001 (ball destroys type 42, awards 90 points)
- **T011–T014**: Address SC-003 (type 91 indestructible, level completes)
- **T015–T024**: Validate SC-001 through SC-004 (all success criteria met)

---

## Files Modified Summary

### New Files Created

- `tests/brick_42_91_life_loss.rs` — 9 integration tests (T015–T023)

### Files Modified

1. **[src/level_format/mod.rs](src/level_format/mod.rs)**
   - Add constant `HAZARD_BRICK_91` (T001)
   - Add function `is_hazard_brick()` (T002)

2. **[src/level_loader.rs](src/level_loader.rs)**
   - Update `spawn_level_entities()` to exclude type 91 from `CountsTowardsCompletion` (T003)
   - Verify level completion logic (T013)

3. **[src/lib.rs](src/lib.rs)**
   - Extend `read_character_controller_collisions()` for paddle-brick collisions (T005)
   - Extend `mark_brick_on_ball_collision()` to skip type 91 destruction (T008)

4. **[src/systems/respawn.rs](src/systems/respawn.rs)**
   - Add `clear_life_loss_frame_flag` system (T004)
   - Add/update `LifeLossCause` enum (T006)

5. **[src/systems/scoring.rs](src/systems/scoring.rs)**
   - Verify type 42 → 90 points mapping (T009, no code change expected)

6. **[assets/textures/manifest.ron](assets/textures/manifest.ron)**
   - Add type 91 texture entry (T014)

---

## Validation Checkpoints

### After Phase 0 (T001–T003)

```bash
cargo clippy --all-targets
# Expected: Passes with no errors; constants and helpers compile
```

### After Phase 1 (T004–T007)

```bash
cargo test brick_42_91_life_loss::test_paddle_brick_42_life_loss
# Expected: Passes; paddle-brick collision emits LifeLostEvent
```

### After Phase 2 (T008–T010)

```bash
cargo test brick_42_91_life_loss::test_ball_brick_42_destroyed_scores_90
# Expected: Passes; ball destroys type 42 and scores 90 points
```

### After Phase 3 (T011–T014)

```bash
cargo test brick_42_91_life_loss::test_level_completion_with_type_91_present
# Expected: Passes; level completes despite type 91 presence
```

### After Phase 4 (T015–T024)

```bash
cargo test --all
cargo clippy --all-targets --all-features
cargo fmt --all --check
# Expected: All pass; 9 tests pass; zero warnings; code formatted
```

---

## Notes for Implementation Team

1. **Local<bool> Frame Flag**: Use Bevy's `Local<T>` system parameter for per-frame state; it automatically resets on system re-run.
   Reset to `false` explicitly in `clear_life_loss_frame_flag` system at Update schedule start.

2. **Message System**: Use Bevy Messages (not Observers) for `LifeLostEvent` and `BrickDestroyed`.
   These are already defined in codebase; reuse them.

3. **Bevy 0.17 Compliance**:
   - Use `With<BrickTypeId>` and `Without<MarkedForDespawn>` filters in queries (no panicking)
   - Use hierarchy APIs correctly; despawn via `commands.entity().despawn_recursive()` if needed
   - No use of removed `Observers`; use `MessageReader<T>` for event consumption

4. **Test Fixtures**: Use existing testing patterns from other brick tests (e.g., `tests/brick_destroy_dedupe.rs`).
   Set up world with `World::default()`, insert components, and run systems in isolation.

5. **Scoring Verification**: Type 42 likely already has a 90-point entry in `brick_points()` function.
   If not found, add it.
   Verify by searching for `42 =>` in `src/systems/scoring.rs`.

6. **Level Loader Integration**: Level completion query already filters by `CountsTowardsCompletion`.
   No changes to query needed; just ensure type 91 doesn't get inserted with that marker.
