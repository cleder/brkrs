# Tasks: Gravity Switching Bricks (020-gravity-bricks)

**Input**: Design documents from `/specs/020-gravity-bricks/` **Prerequisites**: spec.md ✅, plan.md ✅, data-model.md ✅, contracts/ ✅
**Branch**: `020-gravity-bricks` | **Date**: 2026-01-10

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no task dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- **File paths**: Exact paths in Rust crate (`src/`, `tests/`)

---

## Phase 1: Setup - Gravity Bricks Infrastructure

**Purpose**: Create message type, components, and resource definitions; register systems; create test module.

- [X] T001 Define `GravityChanged` message and validation in `src/systems/gravity/mod.rs` with `#[derive(Message)]`
- [X] T002 Define `GravityBrick` component marker in `src/components/gravity.rs` with index (21-25) and gravity vector
- [X] T003 Define `GravityConfiguration` resource in `src/physics_config.rs` with `current` and `level_default` fields
- [X] T004 Register `GravityChanged` message type in main app with `app.register_message::<GravityChanged>()`
- [X] T005 Create `tests/gravity_bricks.rs` test module with test harness and utilities (setup, assertions, helpers)

---

## Phase 2: Foundational - Core Systems & Level Loading

**Purpose**: Implement gravity configuration loading and system registration; enable all user stories to run.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T006 Implement `gravity_configuration_loader_system` in `src/systems/gravity/mod.rs` that loads `default_gravity` from `LevelDefinition` and initializes `GravityConfiguration` resource
- [X] T007 Register gravity systems in main app schedule: add `gravity_configuration_loader_system` to `Update`, `gravity_application_system` to `PhysicsUpdate`, `gravity_reset_on_life_loss_system` to `PostUpdate`
- [X] T008 Extend `LevelDefinition` struct in `src/level_format/mod.rs` to include optional `default_gravity: Option<Vec3>` field; add RON deserialization support with `None` fallback to `Vec3::ZERO`
- [X] T009 Verify level loading gracefully handles missing `default_gravity` field (backwards compatibility); test with existing levels that lack gravity config

**Checkpoint**: Foundation ready - gravity systems registered, level loading updated, tests can run independently.

---

## Phase 3: User Story 1 - Gravity Change on Brick Destruction (Priority: P1) 🎯 MVP

**Goal**: Ball destroys gravity brick → gravity immediately changes in game world → ball physics responds correctly.

**Independent Test**: Destroy one gravity brick in a test level, verify ball's vertical velocity changes according to brick's gravity setting (zero gravity → no acceleration, 10G → downward acceleration, etc.).

### Tests for US1 (REQUIRED) ⚠️

> **NOTE: Write these tests FIRST, verify they FAIL (red), record failing-test commit hash, then implement. Tests must pass before moving to next US.**

- [X] T010 [P] [US1] Write unit tests for gravity brick destruction detection in `tests/gravity_bricks.rs::test_gravity_brick_21_zero_gravity()` - assert brick 21 destruction sends `GravityChanged { gravity: (0.0, 0.0, 0.0) }`
- [X] T011 [P] [US1] Write unit tests for gravity brick 22 (2G) in `tests/gravity_bricks.rs::test_gravity_brick_22_moon_gravity()` - assert sends message with `(0.0, 2.0, 0.0)`
- [X] T012 [P] [US1] Write unit tests for gravity brick 23 (10G) in `tests/gravity_bricks.rs::test_gravity_brick_23_earth_gravity()` - assert sends message with `(0.0, 10.0, 0.0)`
- [X] T013 [P] [US1] Write unit tests for gravity brick 24 (20G) in `tests/gravity_bricks.rs::test_gravity_brick_24_high_gravity()` - assert sends message with `(0.0, 20.0, 0.0)`
- [X] T014 [P] [US1] Write unit tests for gravity application to ball physics in `tests/gravity_bricks.rs::test_gravity_applied_to_ball_velocity()` - assert ball's `Velocity` is affected by `GravityConfiguration::current` in next physics frame
- [X] T015 [US1] Write integration test for gravity brick destruction flow in `tests/gravity_bricks.rs::test_destroy_gravity_brick_changes_gravity()` - setup level with gravity brick, spawn ball, trigger destruction, assert `GravityConfiguration::current` updated, verify ball physics response
- [X] T016 [US1] Write test for message buffering/ordering in `tests/gravity_bricks.rs::test_multiple_gravity_bricks_sequential()` - destroy 3 bricks (21, 24, 22) in sequence, assert gravity transitions correctly (zero → high → light)

### Implementation for US1

- [X] T017 [P] [US1] Implement `brick_destruction_gravity_handler` system in `src/systems/gravity/mod.rs` that detects when brick entities with `GravityBrick` component are destroyed (via change detection or despawn event)
- [X] T018 [P] [US1] In `brick_destruction_gravity_handler`: Query for destroyed bricks with `GravityBrick` component, read gravity value from component, create `GravityChanged` message
- [X] T019 [US1] In `brick_destruction_gravity_handler`: Write `GravityChanged` message via `MessageWriter<GravityChanged>` (depends on T017, T018)
- [X] T020 [P] [US1] Implement `gravity_application_system` in `src/systems/gravity/mod.rs` that reads `MessageReader<GravityChanged>` and updates `GravityConfiguration::current` for each message received
- [X] T021 [US1] Implement physics gravity application: modify ball's physics to use `GravityConfiguration::current` (integrate with Rapier 3D gravity/forces; see `src/physics_config.rs` for existing gravity handling pattern)
- [X] T022 [US1] Add Bevy 0.17 compliance checks: verify no panicking queries (use `?` operator), verify queries filter with `With<Ball>`, `With<RigidBody>`, etc., verify no `.unwrap()` on physics entities
- [X] T023 [US1] Update level RON files to include gravity bricks (21-25) in test level maps for manual verification

**Checkpoint**: User Story 1 complete - gravity bricks (21-25) apply gravity changes immediately on destruction; physics responds correctly.

- [X] T023a [US1] Write test for ball-only gravity scope in `tests/gravity_bricks.rs::test_gravity_does_not_affect_paddle_physics()` - apply gravity change, verify paddle entity physics unchanged, verify enemies unaffected (proves FR-014: gravity applies to ball ONLY)

---

## Phase 4: User Story 2 - Gravity Reset on Ball Loss (Priority: P1)

**Goal**: Ball is lost (life decremented) → gravity resets to level default → next ball spawns with default gravity.

**Independent Test**: Change gravity to 20G by destroying a brick, then lose the ball, verify gravity returns to level default before next ball spawns.

### Tests for US2 (REQUIRED) ⚠️

- [X] T024 [P] [US2] Write unit test for ball loss detection in `tests/gravity_bricks.rs::test_gravity_reset_on_ball_loss()` - setup level with default gravity 10G, change to zero gravity, lose ball, assert `GravityConfiguration::current` reset to `(0.0, 10.0, 0.0)` before next ball spawn
- [X] T025 [P] [US2] Write test for zero gravity fallback in `tests/gravity_bricks.rs::test_gravity_reset_to_zero_gravity_fallback()` - load level without `default_gravity` field, change gravity, lose ball, assert reset to `(0.0, 0.0, 0.0)`
- [X] T026 [US2] Write integration test for full gravity lifecycle in `tests/gravity_bricks.rs::test_gravity_lifecycle_multiple_balls()` - destroy gravity bricks, lose balls, spawn new balls, verify gravity resets correctly each time

### Implementation for US2

- [X] T027 [P] [US2] Implement `gravity_reset_on_life_loss_system` in `src/systems/gravity/mod.rs` that detects ball loss (listen to existing ball loss event/signal from ball_lives system)
- [X] T028 [P] [US2] In `gravity_reset_on_life_loss_system`: Query `GravityConfiguration` resource, reset `current` to `level_default` value (depends on T027)
- [X] T029 [US2] Ensure gravity reset happens before next ball spawns (schedule dependency: gravity reset in `PostUpdate` before ball respawn system)
- [X] T030 [US2] Add logging/debug output for gravity reset events (optional: useful for debugging and gameplay feedback)
- [X] T031 [US2] Verify Bevy 0.17 compliance: ensure life loss detection doesn't panic on queries, uses change detection correctly

**Checkpoint**: User Story 2 complete - gravity resets to level default when ball is lost; gameplay flow works correctly.

---

## Phase 5: User Story 3 - Gravity Bricks Award Points (Priority: P1)

**Goal**: Destroying a gravity brick awards correct points (75-250 per brick type) to player score.

**Independent Test**: Destroy each gravity brick type individually, verify score increases by correct amount (21→125, 22→75, 23→125, 24→150, 25→250).

### Tests for US3 (REQUIRED) ⚠️

- [X] T032 [P] [US3] Write unit tests for gravity brick scores in `tests/gravity_bricks.rs::test_gravity_brick_21_score()` through `test_gravity_brick_25_score()` - assert each brick type awards correct points via existing score system
- [X] T033 [US3] Write integration test for score update on destruction in `tests/gravity_bricks.rs::test_score_updated_on_gravity_brick_destruction()` - destroy gravity brick, verify score component updated immediately

### Implementation for US3

- [X] T034 [P] [US3] Add score award logic to `brick_destruction_gravity_handler` (or existing brick destruction score system) - query destroyed brick with `GravityBrick` component, look up points from static map/table, send score message
- [X] T035 [US3] Create gravity brick score lookup in `src/systems/gravity/mod.rs` or `src/systems/brick_destruction.rs`: index 21→125 pts, 22→75 pts, 23→125 pts, 24→150 pts, 25→250 pts
- [X] T036 [US3] Ensure score updates integrate with existing score system (verify compatibility with `src/systems/[score-system]`)

**Checkpoint**: User Story 3 complete - gravity bricks award correct points on destruction.

---

## Phase 6: User Story 4 - Sequential Gravity Changes (Priority: P2)

**Goal**: Player can destroy multiple gravity bricks in sequence; gravity transitions through each value without glitches or message queue corruption.

**Independent Test**: Destroy 3+ gravity bricks of different types in rapid succession, verify gravity transitions through all values without physics state corruption or missed updates.

### Tests for US4 (REQUIRED) ⚠️

- [X] T037 [P] [US4] Write unit test for message queue buffering in `tests/gravity_bricks.rs::test_gravity_messages_buffered_in_order()` - send 3 `GravityChanged` messages, verify they process in order
- [X] T038 [P] [US4] Write test for gravity state consistency in `tests/gravity_bricks.rs::test_last_gravity_wins_sequential_destruction()` - destroy bricks (21, 24, 22) in sequence, assert final gravity is from brick 22 (last destroyed)
- [X] T039 [US4] Write integration test for rapid brick destruction in `tests/gravity_bricks.rs::test_rapid_multiple_brick_destruction()` - spawn ball, destroy 5 gravity bricks rapidly, verify physics remains stable and gravity applies correctly for each

### Implementation for US4

- [X] T040 [P] [US4] Verify message system handles multiple messages per frame correctly (no additional code needed if using standard Bevy messages; verify in tests T037-T039)
- [X] T041 [US4] Ensure `gravity_application_system` processes all buffered messages (MessageReader handles this automatically; verify with tests)

**Checkpoint**: User Story 4 complete - sequential gravity changes work correctly without state corruption.

---

## Phase 7: Queer Gravity RNG Implementation (Priority: P1) 🎯 Part of US1

**Goal**: Gravity brick 25 (Queer Gravity) generates random gravity within specified ranges on each destruction.

**Ranges**: X ∈ [-2.0, +15.0], Y = 0.0, Z ∈ [-5.0, +5.0]

### Tests for Queer Gravity (REQUIRED) ⚠️

- [X] T042 [P] Queer Gravity test in `tests/gravity_bricks.rs::test_gravity_brick_25_queer_gravity_random()` - destroy brick 25 multiple times, verify each generates gravity with X ∈ [-2.0, +15.0], **Y = 0.0 ALWAYS (no randomization on Y-axis)**, Z ∈ [-5.0, +5.0]
- [X] T043 [P] Queer Gravity X range test in `tests/gravity_bricks.rs::test_queer_gravity_x_range()` - assert all generated X values in [-2.0, +15.0]
- [X] T044 [P] Queer Gravity Y range test in `tests/gravity_bricks.rs::test_queer_gravity_y_zero()` - assert all generated Y values are exactly 0.0
- [X] T045 [P] Queer Gravity Z range test in `tests/gravity_bricks.rs::test_queer_gravity_z_range()` - assert all generated Z values in [-5.0, +5.0]
- [X] T046 [P] Queer Gravity RNG independence test in `tests/gravity_bricks.rs::test_queer_gravity_no_correlation()` - generate 20+ random gravities, verify no obvious bias or correlation between consecutive values

### Implementation for Queer Gravity

- [X] T047 [P] Implement RNG for Queer Gravity in `brick_destruction_gravity_handler` using `rand` crate - on brick 25 destruction, generate random Vec3 with specified ranges
- [X] T048 [P] Add `use rand::Rng` and create thread-local or function-scoped RNG for random gravity generation
- [X] T049 In `brick_destruction_gravity_handler`: Detect brick index 25, generate random gravity, send `GravityChanged` message with random value

**Checkpoint**: Queer Gravity fully functional with correct RNG behavior.

---

## Phase 8: Integration & Polish

**Purpose**: Integration testing, documentation updates, performance verification, WASM compatibility.

- [ ] T050 [P] Write combined integration test for full feature in `tests/gravity_bricks.rs::test_gravity_bricks_complete_gameplay_flow()` - load level with all 5 gravity bricks, destroy in sequence, lose balls, reset gravity, spawn new balls, verify complete lifecycle
- [ ] T051 [P] Update docs: Add gravity brick documentation to `docs/bricks.md` (already exists; verify gravity brick entries are complete)
- [ ] T052 [P] Update docs: Create `docs/gravity-system.md` explaining gravity mechanics, message flow, and level design guidelines
- [ ] T053 Profile gravity systems for performance (benchmark gravity system and message processing) - verify 60 FPS target maintained
- [ ] T054 Test WASM build compatibility - compile with `cargo build --target wasm32-unknown-unknown`, verify gravity systems work in WASM
- [ ] T055 Update level RON files with optional `default_gravity` field where appropriate - add gravity config to existing test levels
- [ ] T056 Manual gameplay testing - play through levels with gravity bricks, verify feel, difficulty, score awards

---

## Phase 9: Code Quality & Final Validation

**Purpose**: Lint, format, test coverage, final verification.

- [ ] T057 Run full test suite: `cargo test` - verify all tests pass (T010-T046)
- [ ] T058 Run clippy: `cargo clippy --all-targets --all-features` - fix any warnings
- [ ] T059 Run formatter: `cargo fmt --all` - ensure code style consistent
- [ ] T060 Verify no panicking queries - grep for `.unwrap()`, `.expect()` on physics queries; add error handling if needed
- [ ] T061 Verify Bevy 0.17 compliance checklist:
  - [ ] Messages vs Observers: `GravityChanged` uses Messages (buffered) ✅
  - [ ] Message-Event Separation: Correct derive (`Message` not `Event`) ✅
  - [ ] Queries use filters (`With<Ball>`, `With<RigidBody>`) ✅
  - [ ] No panicking on query results ✅
  - [ ] Asset handles stored in Resources (N/A for gravity feature) ✅
  - [ ] Hierarchy APIs (N/A for gravity feature) ✅
- [ ] T062 Final documentation review - verify plan.md, spec.md, data-model.md, quickstart.md are accurate and reflect final implementation

---

## Task Dependencies & Parallel Execution

### Critical Path (Sequential)

```text
T001-T005 (Setup)
    ↓
T006-T009 (Foundation) ← GATE: Must complete before user stories
    ↓
T010-T016 (US1 Tests - write first!)
    ↓
T017-T023 (US1 Implementation)
    ↓
T024-T031 (US2 Tests + Implementation)
    ↓
T032-T036 (US3 Tests + Implementation)
    ↓
T037-T041 (US4 Tests + Implementation)
    ↓
T042-T049 (Queer Gravity Tests + RNG)
    ↓
T050-T056 (Integration & Polish)
    ↓
T057-T062 (Quality & Validation)
```

### Parallel Opportunities (Can run in parallel within each phase)

**Phase 1 Setup**: T001-T005 (all parallel - different files) **Phase 2 Foundation**: T006, T008 (parallel - different files); T007 waits for T006; T009 depends on T008 **US1 Tests**: T010-T016 mostly parallel (different test cases) **US1 Implementation**: T017-T019 (parallel), T020 (parallel), T021 (sequential after T020), T022-T023 (parallel) **US2-US4**: Tests parallel with each other (different files), implementation sequential by user story **Queer Gravity**: T043-T046 parallel (RNG tests), T047-T049 sequential **Integration**: T050-T056 mostly parallel (doc updates, profiling, WASM can run in parallel) **Quality**: T057-T062 sequential (must run checks in order)

### Recommended Execution Order

1. **T001-T005**: Setup (prepare infrastructure)
2. **T006-T009**: Foundation (enable user story work)
3. **T010-T023**: US1 complete (core gravity mechanic)
   - T010-T016 (tests) → T017-T023 (implementation)
4. **Parallel T024-T046**: US2, US3, US4, Queer Gravity in parallel
   - Write tests first for each story (T024-T046)
   - Then implement in parallel (T027-T049)
5. **T050-T056**: Integration & Polish
6. **T057-T062**: Quality checks & Final validation

---

## Testing Strategy

**TDD Discipline** (Test-Driven Development):

1. **Red Phase**: Write failing tests first (T010-T046)
2. **Green Phase**: Implement code to make tests pass (T017-T049)
3. **Refactor Phase**: Improve code quality while keeping tests passing (T057-T062)

**Test Coverage Required**:

- ✅ Unit tests for each gravity brick type (21-25)
- ✅ Integration tests for gravity application to physics
- ✅ Message system tests (buffering, ordering, multiple messages)
- ✅ Gravity reset on ball loss
- ✅ Score awards for each brick type
- ✅ Sequential gravity changes without corruption
- ✅ Queer Gravity RNG within specified ranges
- ✅ Backwards compatibility (levels without default_gravity field)

**Test Artifacts**:

- Failing-test commit hash: Record in task description when tests first fail
- All tests must pass before final validation

---

## Bevy 0.17 Compliance Checklist

| Item | Status | Task(s) | Notes |
|------|--------|---------|-------|
| Message System | ✅ | T001 | `GravityChanged` message, not Event |
| Message-Event Separation | ✅ | T020, T061 | `MessageWriter/Reader`, not Observers |
| Query Safety (no panics) | ✅ | T022, T060 | Use `?` operator, not `.unwrap()` |
| Query Filters | ✅ | T017, T022 | `With<Ball>`, `With<RigidBody>`, etc. |
| Change Detection | ✅ | T017 | Use `Changed<>` or despawn detection |
| Physics Correctness | ✅ | T021, T053 | Gravity applies via Rapier 3D |
| No deprecated APIs | ✅ | T022 | Use Bevy 0.17 correct APIs |

---

## Success Criteria for Task Completion

All tasks complete when:

- ✅ All T001-T062 tasks marked as done
- ✅ All tests T010-T046 passing
- ✅ All 5 gravity brick types (21-25) working correctly
- ✅ Gravity resets on ball loss
- ✅ Scores awarded correctly
- ✅ Sequential changes work without glitches
- ✅ Queer Gravity RNG verified within ranges
- ✅ No Bevy 0.17 violations
- ✅ WASM build succeeds
- ✅ 60 FPS performance target maintained
- ✅ Manual gameplay testing verified

---

## Notes

- **Test Proof Commits**: Record failing-test commit hash when T010-T046 first run and fail.
  This proves tests were written before implementation.
- **Parallel Work**: Multiple developers can work on different user stories (T027-T049) in parallel after foundation (T006-T009) is complete.
- **Incremental Delivery**: Each user story can be deployed independently after completing tests + implementation for that story.
- **Performance**: Monitor frame time during T053 profiling; gravity system should add <0.1ms per frame.
- **WASM**: Test early (T054) to catch platform-specific issues; gravity should work identically on WASM and native.

---

**Status**: ✅ Task breakdown complete.
Ready for implementation.

**Next Step**: Begin Phase 1 Setup (T001-T005), then Phase 2 Foundation (T006-T009), then start User Story implementation following TDD discipline (write tests first, record failure, then implement).
