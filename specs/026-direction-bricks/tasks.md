# Tasks: Direction Bricks

**Feature Branch**: `026-direction-bricks` | **Input**: spec.md + plan.md + data-model.md + quickstart.md
**Prerequisites**: Bevy 0.17.3, bevy_rapier3d 0.32.0, rand 0.8, tracing 0.1 (all in Cargo.toml)

**Tests**: Tests are MANDATORY for all user stories.
All tests MUST be written first (red phase), committed, verified to fail, and only then approved for implementation (green phase).
Multi-frame persistence tests required per spec mandate (minimum 10 frames).

**Bevy 0.17 Compliance**: This feature touches ECS systems (Observer pattern), physics (LinearVelocity modification), and messaging.
All tasks include acceptance criteria verifying **Message-Event Separation** (Messages for scoring, Observers for immediate velocity impulses) and **coordinate system correctness** (X and Y modification, Z preserved).

---

## Overview: Feature Scope & Dependencies

**Feature**: Implement 7 destructible brick types (43-48, 52) that apply instantaneous velocity impulses to the ball during collision.

**Key Dependencies**:

- Existing brick destruction system (emits `BrickDestroyed` message)
- bevy_rapier3d `LinearVelocity` component for ball physics
- Existing scoring system
- Level loader (RON format)
- Observers pattern (Bevy 0.17 native)

**Independent Test Scope**: Each user story can be implemented and tested independently.

---

## Phase 1: Setup (No New Setup Required)

✅ **SKIP**: No new project initialization needed.
All infrastructure exists (brick destruction, level loading, scoring, physics).

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Create shared infrastructure for all direction brick types

### Add DirectionBrickEffect Trigger Type

- [ ] T001 Define `DirectionBrickEffect` trigger type in `src/signals.rs` with fields: `ball_entity: Entity`, `brick_type: u32`, `brick_position: Vec3`, `velocity_before: Vec3`

### Register Observer Plugin

- [ ] T002 [P] Create module `src/systems/brick_effects.rs` and register observer system in `src/systems/mod.rs`
- [ ] T003 [P] Verify existing brick destruction system can emit new `Trigger<DirectionBrickEffect>` events (check how `BrickDestroyed` message is currently emitted)

### Coordinate System & Physics Validation

- [ ] T004 Verify `LinearVelocity` from bevy_rapier3d is directly modifiable (unit test: spawn ball, modify X component, verify change persists across frames)
- [ ] T005 Document coordinate system constraints in module-level docs: XZ horizontal plane, Y vertical, Z-axis never modified by direction bricks

---

## Phase 3: User Story 1 - Cardinal Direction Bricks (Priority: P1) 🎯 MVP

**Goal**: Implement bricks 43-46 that apply instantaneous 5.0 units/sec velocity impulses in cardinal directions (down, left, right, up).

**Independent Test**: Can be tested by spawning a ball, triggering a single cardinal brick effect, and verifying velocity changes in correct axis by expected amount.

**Bevy 0.17 Checkpoint**: Verify Observer pattern correctly used (not Messages), velocity modification applied once per event (no per-frame accumulation), and Z-velocity unchanged.

### Tests for User Story 1 (REQUIRED) 🚀 RED PHASE FIRST

> **CRITICAL**: Write tests first, run `cargo test` to verify they FAIL, commit with hash, then implement.

- [ ] T006 [P] [US1] Create unit test for brick 43 (Down) in `tests/direction_bricks.rs`: spawn ball at (3.0, 2.0, 0.0), trigger brick 43, verify velocity becomes (3.0, -3.0, 0.0).
  **Acceptance**: Y-velocity decreased by exactly 5.0.
  **Bevy Check**: LinearVelocity component directly modified, not wrapped in Message.
- [ ] T007 [P] [US1] Create unit test for brick 44 (Left) in `tests/direction_bricks.rs`: spawn ball at (3.0, 2.0, 0.0), trigger brick 44, verify velocity becomes (-2.0, 2.0, 0.0).
  **Acceptance**: X-velocity decreased by exactly 5.0.
  **Bevy Check**: No Message-based intermediary, direct Observer reaction.
- [ ] T008 [P] [US1] Create unit test for brick 45 (Right) in `tests/direction_bricks.rs`: spawn ball at (-3.0, 2.0, 0.0), trigger brick 45, verify velocity becomes (2.0, 2.0, 0.0).
  **Acceptance**: X-velocity increased by exactly 5.0.
- [ ] T009 [P] [US1] Create unit test for brick 46 (Up) in `tests/direction_bricks.rs`: spawn ball at (3.0, -2.0, 0.0), trigger brick 46, verify velocity becomes (3.0, 3.0, 0.0).
  **Acceptance**: Y-velocity increased by exactly 5.0.
- [ ] T010 [P] [US1] Create unit test for additive impulses in `tests/direction_bricks.rs`: spawn ball at (-3.0, velocity.y), trigger brick 43, verify Y becomes -8.0 (additive, not replacement).
  **Acceptance**: Impulse applied on top of existing velocity, not replacing it.
  **Bevy Check**: Velocity accumulation correct (previous velocity + impulse).
- [ ] T011 [P] [US1] Create unit test for stationary ball in `tests/direction_bricks.rs`: spawn ball at (0.0, 0.0, 0.0), trigger brick 46, verify velocity becomes (0.0, 5.0, 0.0).
  **Acceptance**: Even stationary balls receive impulse.
  **Bevy Check**: No special case for zero velocity.
- [ ] T012 [P] [US1] Create unit test for Z-axis preservation in `tests/direction_bricks.rs`: spawn ball with Z-velocity 5.0, trigger brick 45, verify Z unchanged.
  **Acceptance**: Z-velocity remains exactly 5.0 after X modification.
  **Bevy Check**: Z-axis never touched by cardinal brick logic.
- [ ] T013 [US1] Create multi-frame persistence test in `tests/multi_frame_persistence_direction_bricks.rs`: spawn ball, trigger brick 45, run 10+ `app.update()` cycles, verify velocity persists and physics system applies on modified velocity.
  **Acceptance**: Velocity not reset or overwritten by any initialization system.
  **Bevy Check**: No per-frame unconditional velocity resets; physics system reads current velocity state.
- [ ] T014 [US1] Create unit test for tracing spans in `tests/direction_bricks.rs`: trigger brick 43, verify tracing span is emitted with brick_type, velocity_before, and velocity_after context.
  **Acceptance**: Span contains all 4 required fields.
  **Bevy Check**: Tracing instrumentation per spec FR-011.

**Commit tests first** (red phase): `git add tests/ && git commit -m "tests: Add failing tests for cardinal direction bricks (US1 red phase)"` **Record failing commit hash**: [commit_hash from `git rev-parse HEAD`]

### Implementation for User Story 1

- [ ] T015 [P] [US1] Implement cardinal impulse logic in `src/systems/brick_effects.rs` function `apply_direction_brick_effects`: match on brick_type 43-46, apply ±5.0 to correct X or Y axis.
  **Acceptance**: All 4 cardinal directions produce correct velocity deltas.
  **Bevy Check**: Uses Observer pattern (not Message), query has `With<Ball>` filter, uses `get_mut()` for fallible access.
- [ ] T016 [P] [US1] Add tracing instrumentation to `apply_direction_brick_effects`: emit `tracing::info_span!()` with brick_type, brick_position, velocity_before, velocity_after inside observer system.
  **Acceptance**: Span context logged to test output.
  **Bevy Check**: Structured logging per spec.
- [ ] T017 [US1] Modify brick destruction system (find exact location in `src/systems/`) to trigger `Trigger<DirectionBrickEffect>` when brick types 43-46 are destroyed.
  **Acceptance**: Effect event emitted immediately after `BrickDestroyed` message.
  **Bevy Check**: Trigger emission does not interfere with existing Message emission.
- [ ] T018 [US1] Extend scoring system match statement in `src/systems/` to award 75 points for brick types 43-46.
  **Acceptance**: Score increases by exactly 75 when each brick is destroyed.
  **Bevy Check**: Scoring reads `BrickDestroyed` message (not Observer), maintaining separation.
- [ ] T019 [US1] Run tests: `cargo test direction_bricks -- --nocapture` verify T006-T014 now PASS (green phase).
  **Acceptance**: All 9 tests pass.
  **Bevy Check**: No panicking queries, proper filters, Observer pattern confirmed.
- [ ] T020 [P] [US1] Create test level in `assets/levels/test_cardinal.ron` with bricks 43, 44, 45, 46 at different positions.
  **Acceptance**: Level loads without errors.
  **Bevy Check**: Level loader accepts brick_type IDs 43-46.
- [ ] T021 [P] [US1] Create integration test in `tests/direction_bricks.rs` for level loading: load test_cardinal.ron, verify all 4 bricks spawn with correct components.
  **Acceptance**: All 4 brick entities created with DirectionBrick component.

**Checkpoint**: User Story 1 complete—cardinal direction bricks fully functional and testable independently.

---

## Phase 4: User Story 2 - Diagonal Direction Bricks (Priority: P1)

**Goal**: Implement bricks 47-48 that apply simultaneous velocity impulses along two axes (up+right, up+left).

**Independent Test**: Can be tested by spawning a ball, triggering a single diagonal brick effect, and verifying velocity changes correctly in both axes.

**Bevy 0.17 Checkpoint**: Observer pattern used, both axis modifications applied atomically (same Trigger event), Z-velocity unchanged.

### Tests for User Story 2 (REQUIRED) 🚀 RED PHASE FIRST

- [ ] T022 [P] [US2] Create unit test for brick 47 (Up-Right) in `tests/direction_bricks.rs`: spawn ball at (1.0, 1.0, 0.0), trigger brick 47, verify velocity becomes (6.0, 6.0, 0.0).
  **Acceptance**: Both X and Y increased by 5.0.
  **Bevy Check**: Two modifications applied in single Observer call (atomicity).
- [ ] T023 [P] [US2] Create unit test for brick 48 (Up-Left) in `tests/direction_bricks.rs`: spawn ball at (3.0, 1.0, 0.0), trigger brick 48, verify velocity becomes (-2.0, 6.0, 0.0).
  **Acceptance**: X decreased by 5.0, Y increased by 5.0.
  **Bevy Check**: Both axes modified without intermediate state exposure.
- [ ] T024 [P] [US2] Create unit test for diagonal additive behavior in `tests/direction_bricks.rs`: spawn ball at existing velocity, apply diagonal brick, verify both changes stack correctly.
  **Acceptance**: Each axis modified independently and additively.
  **Bevy Check**: No replacement; both modifications are additions.
- [ ] T025 [US2] Create multi-frame persistence test in `tests/multi_frame_persistence_direction_bricks.rs`: spawn ball, trigger diagonal brick, run 10+ frames, verify both axis velocities persist.
  **Acceptance**: Both X and Y persist unchanged for 10+ frames.
  **Bevy Check**: Physics system reads and applies to both modified components.

**Commit tests first** (red phase): `git add tests/ && git commit -m "tests: Add failing tests for diagonal direction bricks (US2 red phase)"` **Record failing commit hash**: [commit_hash]

### Implementation for User Story 2

- [ ] T026 [P] [US2] Extend `apply_direction_brick_effects` in `src/systems/brick_effects.rs` to handle brick_type 47-48: match cases apply ±5.0 to both X and Y in single operation.
  **Acceptance**: Both axes modified per brick specification.
  **Bevy Check**: Single match arm modifies both components atomically.
- [ ] T027 [US2] Extend tracing span in observer to distinguish diagonal brick effects (different span name or context field).
  **Acceptance**: Diagonal bricks emit same tracing structure as cardinal.
  **Bevy Check**: Structured logging captures both axis changes.
- [ ] T028 [US2] Modify brick destruction system to emit `Trigger<DirectionBrickEffect>` for brick types 47-48.
  **Acceptance**: Trigger emitted on destruction of diagonals.
  **Bevy Check**: No interference with cardinal brick events.
- [ ] T029 [US2] Extend scoring system to award 100 points for brick types 47-48.
  **Acceptance**: Score increases by exactly 100 per brick.
  **Bevy Check**: Scoring matches extended requirements (75 for 43-46, 100 for 47-48).
- [ ] T030 [US2] Run tests: `cargo test direction_bricks -- --nocapture` verify T022-T025 now PASS.
  **Acceptance**: All diagonal tests pass.
  **Bevy Check**: Both axes correctly modified in all test scenarios.
- [ ] T031 [P] [US2] Extend test level in `assets/levels/test_cardinal.ron` or create new `assets/levels/test_diagonal.ron` with bricks 47, 48.
  **Acceptance**: Level loads with diagonal bricks.
  **Bevy Check**: Loader handles new brick types.
- [ ] T032 [P] [US2] Create integration test for diagonal bricks in `tests/direction_bricks.rs`: load level, spawn ball, destroy diagonal brick, verify both velocities correct.
  **Acceptance**: In-game destruction triggers correct impulses.

**Checkpoint**: User Story 2 complete—diagonal direction bricks fully functional and independently testable.

---

## Phase 5: User Story 3 - Randomization Brick (Priority: P1)

**Goal**: Implement brick 52 (Randomizer) that replaces ball velocity with random direction and magnitude (5.0-15.0 units/sec, 0-360° direction).

**Independent Test**: Can be tested by spawning a ball, triggering brick 52 multiple times, and verifying random velocities within bounds and different from each other.

**Bevy 0.17 Checkpoint**: Observer pattern used, velocity replaced (not additive), RNG seeding correct for test determinism, Z-velocity preserved.

### Tests for User Story 3 (REQUIRED) 🚀 RED PHASE FIRST

- [ ] T033 [P] [US3] Create unit test for brick 52 magnitude range in `tests/direction_bricks.rs`: trigger brick 52 multiple times (10+), verify all magnitudes fall in [5.0, 15.0] range.
  **Acceptance**: Every generated velocity has magnitude between 5.0 and 15.0 inclusive.
  **Bevy Check**: RNG generation per spec (direct range, no clamping).
- [ ] T034 [P] [US3] Create unit test for brick 52 direction distribution in `tests/direction_bricks.rs`: trigger brick 52 many times, verify angles span at least 180° (or statistical test for uniformity).
  **Acceptance**: Direction variation across 180+ degrees, not clustered.
  **Bevy Check**: `rand::gen_range(0.0..TAU)` for uniform distribution.
- [ ] T035 [P] [US3] Create unit test for brick 52 velocity replacement (not additive) in `tests/direction_bricks.rs`: spawn ball with velocity (10.0, 0.0, 0.0), trigger brick 52, verify new velocity is NOT (10.0+random_x, 0.0+random_y, 0.0).
  **Acceptance**: Previous velocity completely replaced, not modified.
  **Bevy Check**: `velocity.linvel = new_random_vector` assignment (not +=).
- [ ] T036 [P] [US3] Create unit test for brick 52 Z-preservation in `tests/direction_bricks.rs`: spawn ball with Z-velocity 5.0, trigger brick 52, verify Z unchanged.
  **Acceptance**: Z-velocity remains exactly 5.0.
  **Bevy Check**: Randomizer preserves Z-axis per spec edge case.
- [ ] T037 [US3] Create multi-frame persistence test for randomizer in `tests/multi_frame_persistence_direction_bricks.rs`: trigger brick 52, run 10+ frames, verify randomized velocity persists and is not reset.
  **Acceptance**: Random velocity persists across 10+ frames.
  **Bevy Check**: No per-frame overwrites; physics system applies to randomized velocity.

**Commit tests first** (red phase): `git add tests/ && git commit -m "tests: Add failing tests for randomization brick 52 (US3 red phase)"` **Record failing commit hash**: [commit_hash]

### Implementation for User Story 3

- [ ] T038 [P] [US3] Implement randomization logic in `src/systems/brick_effects.rs`: add case 52 to match statement that generates random magnitude (5.0..=15.0) and angle (0.0..TAU), then assigns `velocity.linvel = Vec3::new(mag * cos(angle), mag * sin(angle), velocity.linvel.z)`.
  **Acceptance**: Random velocity generated with correct bounds.
  **Bevy Check**: `rand::thread_rng()`, direct range generation (no clamping), Z preserved.
- [ ] T039 [P] [US3] Add RNG seeding for test determinism in `tests/direction_bricks.rs`: verify randomizer tests can be seeded for reproducible results if needed.
  **Acceptance**: Tests use `thread_rng()` (default) or seeded RNG for reproducibility.
  **Bevy Check**: RNG usage appropriate for game context.
- [ ] T040 [US3] Extend tracing in observer to capture randomized velocity parameters: log magnitude and angle for brick 52.
  **Acceptance**: Tracing span includes randomizer-specific context.
  **Bevy Check**: Observability per spec FR-011.
- [ ] T041 [US3] Modify brick destruction system to emit trigger for brick type 52.
  **Acceptance**: Trigger emitted on destruction.
  **Bevy Check**: No interference with cardinal/diagonal events.
- [ ] T042 [US3] Extend scoring system to award 125 points for brick type 52.
  **Acceptance**: Score increases by exactly 125.
  **Bevy Check**: Scoring covers all 7 brick types (75, 100, 125).
- [ ] T043 [US3] Run tests: `cargo test direction_bricks -- --nocapture` verify T033-T037 now PASS.
  **Acceptance**: All randomizer tests pass.
  **Bevy Check**: Randomization behavior correct, Z preserved, magnitude/direction in bounds.
- [ ] T044 [P] [US3] Create test level in `assets/levels/test_randomizer.ron` with brick 52 at one or more positions.
  **Acceptance**: Level loads.
  **Bevy Check**: Loader handles brick_type 52.
- [ ] T045 [P] [US3] Create integration test in `tests/direction_bricks.rs`: load randomizer level, destroy brick 52, verify velocity changed to random value.
  **Acceptance**: In-game destruction triggers randomization.

**Checkpoint**: User Story 3 complete—randomization brick fully functional and independently testable.

---

## Phase 6: User Story 4 - Scoring Integration (Priority: P1)

**Goal**: Verify all direction brick types (43-48, 52) award correct point values and integrate seamlessly with existing scoring system.

**Independent Test**: Can be tested by destroying each brick type individually and verifying score increases by correct amount, independent of velocity mechanics.

**Bevy 0.17 Checkpoint**: Scoring reads `BrickDestroyed` Message (not Observer), maintains Message-Event Separation, points awarded correctly.

### Tests for User Story 4 (REQUIRED) 🚀 RED PHASE FIRST

- [ ] T046 [P] [US4] Create unit tests for scoring in `tests/direction_bricks.rs`: 7 separate tests (one per brick type) that spawn ball + brick, trigger destruction, verify score increases by correct amount (75 for 43-46, 100 for 47-48, 125 for 52).
  **Acceptance**: Each brick type awards exact points.
  **Bevy Check**: Scoring system reads `BrickDestroyed` message, not Observer (proper separation).
- [ ] T047 [US4] Create integration test for scoring: load level with all 7 brick types, destroy each in sequence, verify total score matches sum of individual values.
  **Acceptance**: Score accumulation correct across all types.
  **Bevy Check**: Scoring system compatible with direction brick integration.

**Commit tests first** (red phase): `git add tests/ && git commit -m "tests: Add failing tests for direction brick scoring (US4 red phase)"` **Record failing commit hash**: [commit_hash]

### Implementation for User Story 4

- [ ] T048 [US4] Verify scoring system (in `src/systems/`) correctly extended with brick types 43-48, 52 in match statement.
  **Acceptance**: Scoring match statement includes all 7 brick types with correct point values.
  **Bevy Check**: Points awarded via `MessageReader<BrickDestroyed>`, not Observer (message pattern for buffered signals).
- [ ] T049 [US4] Run tests: `cargo test direction_bricks -- --nocapture` verify T046-T047 now PASS.
  **Acceptance**: All scoring tests pass.
  **Bevy Check**: Scoring integration confirmed.

**Checkpoint**: User Story 4 complete—scoring fully integrated for all 7 brick types.

---

## Phase 7: Multi-Frame Persistence & Edge Cases (Priority: P1)

**Goal**: Verify velocity modifications persist across multiple frames without being overwritten or reset by any system.

**Bevy 0.17 Mandate**: Multi-frame persistence required per spec (minimum 10 frames) to catch per-frame unconditional overwrites.

### Tests for Multi-Frame Persistence (REQUIRED)

- [ ] T050 [US1] Create multi-frame test in `tests/multi_frame_persistence_direction_bricks.rs`: spawn ball, apply cardinal impulse (brick 45), run exactly 10, 20, and 50 `app.update()` cycles, verify X-velocity persists unchanged in all frames.
  **Acceptance**: Velocity not reset or overwritten by any system in 50 frames.
  **Bevy Check**: No per-frame initialization systems unconditionally reset velocity.
- [ ] T051 [US1] Create multi-frame stacking test in `tests/multi_frame_persistence_direction_bricks.rs`: spawn ball, apply brick 45 (X+5), run 1 frame, apply brick 46 (Y+5) in same app context, run 10 frames, verify final velocity accumulates correctly without resets.
  **Acceptance**: Impulses stack correctly; velocity from first persists and receives second.
  **Bevy Check**: Physics system reads current velocity (not resetting to initial state).
- [ ] T052 [US2] Create multi-frame test for diagonal bricks in `tests/multi_frame_persistence_direction_bricks.rs`: apply brick 47, run 10 frames, verify both X and Y persist unchanged.
  **Acceptance**: Both axes persist together.
  **Bevy Check**: No partial resets; all components preserve state.
- [ ] T053 [US3] Create multi-frame test for randomizer in `tests/multi_frame_persistence_direction_bricks.rs`: apply brick 52, run 10 frames with physics enabled, verify random velocity persists and is modified only by physics (gravity, forces), not reset per frame.
  **Acceptance**: Random velocity persists; only physics-based modifications occur.
  **Bevy Check**: Randomizer not re-triggered; persistence not interrupted.
- [ ] T054 [P] Create edge case test for rapid succession (same-frame multiple bricks) in `tests/direction_bricks.rs`: trigger brick 45, then immediately (in same app update) trigger brick 46, verify both impulses apply and stack correctly.
  **Acceptance**: Multiple triggers in same frame accumulate correctly.
  **Bevy Check**: Observer system handles multiple events per frame.
- [ ] T055 [P] Create edge case test for stationary ball in `tests/direction_bricks.rs`: spawn ball with zero velocity, apply any impulse, verify non-zero velocity results.
  **Acceptance**: Even zero-velocity balls receive impulse.
  **Bevy Check**: No zero-velocity special case.

### Run All Tests

- [ ] T056 Run full test suite: `cargo test` verify all tests (T006-T055) PASS, including multi-frame persistence.
  **Acceptance**: 100% pass rate.
  **Bevy Check**: No regressions in existing tests.
- [ ] T057 Run with tracing enabled: `RUST_LOG=direction_bricks=info cargo test -- --nocapture` verify tracing spans appear for all brick effects.
  **Acceptance**: Tracing output visible in test output.
  **Bevy Check**: Observability per spec.

**Checkpoint**: All multi-frame and edge case tests pass—feature is robust and handles all scenarios.

---

## Phase 8: Integration Testing & Level Design (Priority: P1)

**Goal**: Verify direction bricks work correctly in realistic game scenarios with other systems.

### Integration Tests

- [ ] T058 [P] Create comprehensive test level in `assets/levels/test_all_bricks.ron` with all 7 brick types plus other brick types (standard, gravity, etc.) in one level.
  **Acceptance**: Level loads without errors, all entities spawn.
  **Bevy Check**: Level loader handles mixed brick types.
- [ ] T059 [US1-4] Create integration test in `tests/direction_bricks.rs`: load test_all_bricks.ron, spawn ball, destroy bricks in specific sequence, verify velocity and score accumulate correctly.
  **Acceptance**: All bricks behave correctly in shared level.
  **Bevy Check**: No conflicts with other brick types.
- [ ] T060 [P] Create test for gravity + direction bricks in `tests/direction_bricks.rs` (if gravity system exists): apply gravity brick, then direction brick, verify both systems' effects compose correctly.
  **Acceptance**: Velocity from both sources accumulates.
  **Bevy Check**: Physics system applies all modifications correctly.
- [ ] T061 [P] Create test for multi-ball scenario in `tests/direction_bricks.rs` (if applicable): spawn multiple balls, destroy direction brick, verify correct ball receives impulse (not all balls).
  **Acceptance**: Only the colliding ball gets modified.
  **Bevy Check**: Observer queries filter correctly to affect only Ball entities.

### Regression Testing

- [ ] T062 Run full game test suite: `cargo test` verify existing brick types still work (40+ existing tests pass).
  **Acceptance**: 100% existing tests pass.
  **Bevy Check**: No regressions in ball physics, collision, or other systems.
- [ ] T063 Run formatting & linting: `cargo fmt --all && cargo clippy --all-targets --all-features` verify no issues.
  **Acceptance**: No new warnings or errors.
  **Bevy Check**: Code style consistent with project.

**Checkpoint**: Integration testing complete—direction bricks work seamlessly with existing game systems.

---

## Phase 9: Documentation & Polish (Priority: P2)

**Goal**: Document implementation details and provide guidance for future brick type additions.

### Code Documentation

- [ ] T064 Add module-level documentation to `src/systems/brick_effects.rs`: explain observer pattern, impulse mechanics, coordinate system, and integration points.
  **Acceptance**: New developers can understand the system from docs alone.
  **File**: `src/systems/brick_effects.rs` (line 1-50).
- [ ] T065 Add function-level documentation to `apply_direction_brick_effects()`: document trigger payload fields, brick type mappings, randomizer algorithm, and tracing context.
  **Acceptance**: Function signature and behavior clear.
  **File**: `src/systems/brick_effects.rs` (apply_direction_brick_effects function).
- [ ] T066 Add brick type lookup documentation: create comments or inline const mapping brick_type → description (43→Down, 44→Left, ..., 52→Randomizer) in observer function.
  **Acceptance**: Easy to reference which brick ID does what.
  **File**: `src/systems/brick_effects.rs` (match statement).
- [ ] T067 Add tracing instrumentation documentation: explain which context fields are logged and why.
  **Acceptance**: Developers understand observability data.
  **File**: `src/systems/brick_effects.rs` (tracing span).

### Developer Guide Update

- [ ] T068 Update or create `docs/developer-guide.md` section on direction bricks: explain how to add new direction brick types (copy match case, add test, commit red→green).
  **Acceptance**: Clear steps for extension.
  **File**: `docs/developer-guide.md`.
- [ ] T069 Add impulse calculation reference to developer guide: document formula for each brick type, explain additive vs. replacement semantics.
  **Acceptance**: Designers can calculate expected behaviors.
  **File**: `docs/developer-guide.md`.
- [ ] T070 Add level design guide: show example RON syntax for placing direction bricks, explain positioning and combinations for level design.
  **Acceptance**: Level designers can use all 7 brick types effectively.
  **File**: `docs/developer-guide.md` or `docs/asset-format.md`.

### Test Documentation

- [ ] T071 Add test module documentation to `tests/direction_bricks.rs`: explain test structure, how to add new tests, and interpretation of tracing output.
  **Acceptance**: Test structure self-documenting.
  **File**: `tests/direction_bricks.rs` (line 1-20).
- [ ] T072 Add multi-frame persistence test documentation to `tests/multi_frame_persistence_direction_bricks.rs`: explain why 10-frame minimum, what to look for in logs, and how to debug failures.
  **Acceptance**: Rationale for multi-frame testing clear.
  **File**: `tests/multi_frame_persistence_direction_bricks.rs` (line 1-20).

### Final Code Review & Cleanup

- [ ] T073 Code review checklist: ✅ All tests pass (T056), ✅ Tracing visible (T057), ✅ No regressions (T062), ✅ Formatting correct (T063), ✅ Documentation complete (T064-T072).
  **Acceptance**: All items checked.
- [ ] T074 Performance verification: run game for 1 minute with direction bricks in level, verify 60 FPS maintained.
  **Acceptance**: No frame drops or stuttering.
  **Bevy Check**: Observer system doesn't cause performance issues.

**Checkpoint**: Documentation and polish complete—feature ready for merge.

---

## Phase 10: Acceptance & Merge (Priority: P1)

**Goal**: Verify feature meets all success criteria and prepare for merge to develop branch.

### Final Verification

- [ ] T075 Verify all 4 user stories independently testable and passing: US1 (cardinal), US2 (diagonal), US3 (randomizer), US4 (scoring).
  **Acceptance**: Each story has own test suite and passes independently.
  **File**: `tests/direction_bricks.rs` (organized by story).
- [ ] T076 Verify all acceptance scenarios from spec.md are covered by tests: 23 scenarios total (7 per US1-3, 7 per US4).
  **Acceptance**: Every scenario in spec has corresponding test.
  **Reference**: spec.md User Stories 1-4.
- [ ] T077 Verify success criteria from spec.md: ✅ SC-001 (all 7 types work), ✅ SC-002 (100% scoring), ✅ SC-003 (10+ frame persistence), ✅ SC-004 (randomizer range), ✅ SC-005 (lifecycle integration), ✅ SC-006 (no regressions).
  **Acceptance**: All 6 success criteria measured and verified.
- [ ] T078 Verify Bevy 0.17 compliance: ✅ Observer pattern used (not Messages) for impulses, ✅ Message pattern used for scoring (proper separation), ✅ Coordinate system correct (XZ horizontal, Y vertical, Z preserved), ✅ Multi-frame persistence tested (10+ frames), ✅ Query filters used (With <Ball>), ✅ No panicking unwraps.
  **Acceptance**: Constitution compliance confirmed.

### Merge Preparation

- [ ] T079 Create feature summary: list all new/modified files, brief description of each.
  **Acceptance**: Merge PR author has clear inventory.
  **File**: Git commit message for final merge.
- [ ] T080 Final Git commit: commit all changes with message summarizing feature completion (e.g., "feat: Implement direction bricks 43-48, 52 with Observer pattern").
  **Acceptance**: Clean Git history.
  **Command**: `git add -A && git commit -m "feat: ..."`.
- [ ] T081 Push feature branch: `git push origin 026-direction-bricks`.
  **Acceptance**: Feature branch ready for PR review.
- [ ] T082 Create pull request to develop: reference spec.md, plan.md, tasks.md; link acceptance tests.
  **Acceptance**: PR provides full context for reviewers.

**Checkpoint**: Feature complete and ready for review/merge.

---

## Parallel Execution Strategy

**Independent Tracks** (can run in parallel after foundational tasks T001-T005):

- **Track 1 (US1)**: T006-T021 (cardinal bricks) — 2-3 hours
- **Track 2 (US2)**: T022-T032 (diagonal bricks) — 1.5-2 hours
- **Track 3 (US3)**: T033-T045 (randomizer) — 2-2.5 hours
- **Track 4 (US4)**: T046-T049 (scoring) — 1 hour (can start after US1/US2/US3 tests written)
- **Parallel (T050-T055)**: Multi-frame & edge cases — can start once any US tests are implemented
- **Parallel (T058-T063)**: Integration & regression — after US1-US4 complete
- **Sequential (T064-T082)**: Documentation & merge — final phase

**Recommended MVP Scope**: Complete US1 (T006-T021) first (~2-3 hours), verify independently, then add US2-US3 in parallel.

---

## Task Statistics

**Total Tasks**: 82 **Mandatory Tests**: 28 tasks (T006-T049, T050-T055) **Implementation**: 31 tasks (T001-T005, T015-T045, T058-T063, T073-T082) **Documentation**: 8 tasks (T064-T072)

**Tasks by User Story**:

- **US1 (Cardinal)**: 16 tasks (T006-T021)
- **US2 (Diagonal)**: 11 tasks (T022-T032)
- **US3 (Randomizer)**: 13 tasks (T033-T045)
- **US4 (Scoring)**: 2 tasks (T046-T049)
- **Multi-Frame Persistence**: 7 tasks (T050-T055, T056-T057)
- **Integration**: 6 tasks (T058-T063)
- **Documentation/Merge**: 19 tasks (T064-T082)

---

## File Inventory

**New Files to Create**:

- `src/systems/brick_effects.rs` — Direction brick observer system
- `tests/direction_bricks.rs` — Unit & integration tests
- `tests/multi_frame_persistence_direction_bricks.rs` — Multi-frame persistence tests
- `assets/levels/test_cardinal.ron` — Test level with bricks 43-46
- `assets/levels/test_diagonal.ron` — Test level with bricks 47-48
- `assets/levels/test_randomizer.ron` — Test level with brick 52
- `assets/levels/test_all_bricks.ron` — Comprehensive test level

**Files to Modify**:

- `src/signals.rs` — Add `DirectionBrickEffect` trigger type
- `src/systems/mod.rs` — Register observer system
- `src/systems/[brick_destruction].rs` — Emit `Trigger<DirectionBrickEffect>` (location TBD)
- `src/systems/[scoring].rs` — Extend point values for bricks 43-48, 52 (location TBD)
- `docs/developer-guide.md` — Add direction brick section
- `Cargo.toml` — Verify `rand 0.8` present (should be)

---

## Acceptance Criteria (Final Gate)

✅ All 82 tasks completed ✅ All tests pass: `cargo test` returns 0 errors ✅ All acceptance scenarios from spec.md verified ✅ All success criteria (SC-001 through SC-006) measured ✅ Bevy 0.17 compliance confirmed (Observer pattern, message separation, coordinate system, multi-frame testing) ✅ No regressions in existing tests ✅ Code formatted: `cargo fmt --all` clean ✅ No clippy warnings: `cargo clippy --all-targets --all-features` clean ✅ Documentation complete (module docs, developer guide, test docs) ✅ Feature branch pushed and PR ready

---

## Notes

- **Red-Green-Refactor TDD**: Tests (red) committed first in each user story phase, then implementation (green) added to make tests pass.
- **Bevy 0.17 Mandate Compliance**: Every test includes acceptance criteria or annotations noting Bevy 0.17 requirements verified.
- **Multi-Frame Persistence**: Per 020-gravity-bricks retrospective, minimum 10 frames required; included in T050-T053.
- **Tracing Observability**: Every brick destruction emits structured span with context per FR-011.
- **Message-Event Separation**: Scoring uses Messages (buffered), direction effects use Observers (immediate) per spec clarification.
