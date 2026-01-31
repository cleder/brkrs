# Tasks: Ball Spawn Bricks

**Input**: Design documents from `/specs/025-ball-spawn-bricks/` **Prerequisites**: plan.md ✅, spec.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Tests**: Tests are MANDATORY for all user stories.
Each story MUST include unit tests and feature-level acceptance tests (integration or contract tests as appropriate).
Tests MUST be written and committed first, verified to FAIL (red), and then approved before implementation begins; record the test-proof commit hash in the task description.

**Bevy 0.17 compliance**: When generating tasks for ECS/rendering/UI work, include explicit tasks (or acceptance criteria within test tasks) to ensure compliance with the constitution's Bevy 0.17 mandates & prohibitions (no panicking queries, filtered queries, `Changed<T>` for reactive UI, message vs event correctness, asset handle reuse, and correct hierarchy APIs).
For any event-driven feature, tasks MUST specify which event system is used (Messages vs Observers), justify the choice, and include acceptance criteria for **Message-Event Separation** (e.g., verify buffered messages used for frame-agnostic logs and observers used for immediate UI/sound triggers) and **Hierarchy Safety** (verifying use of `commands.entity(parent).add_child(child)` or `EntityCommands::set_parent`).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

Single binary Bevy game project: `src/`, `tests/` at repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure for ball spawn bricks feature

- [ ] T001 Verify branch `025-ball-spawn-bricks` is checked out and up-to-date with latest develop
- [ ] T002 [P] Create test fixtures module in `tests/ball_spawn_bricks/fixtures.rs` with helper functions for spawning test balls, bricks, and test app setup
- [ ] T003 [P] Verify level_015 contains bricks 37, 38, 39 for manual testing (user confirmed level_015 already has required layout)

**Checkpoint**: Development environment ready, test infrastructure established

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Create `BrickSpawnConfig` resource in `src/systems/ball_spawn_bricks.rs` with HashMap mapping brick indices 37/38/39 to spawn rules (spawn_count, velocity_modifier, score_value=100)
- [ ] T005 Create `VelocityModifier` enum in `src/systems/ball_spawn_bricks.rs` with variants: `DespawnAll`, `Inverse`, `YShaped { angle_degrees: f32 }`
- [ ] T006 Verify `BrickDestroyed` message in `src/signals.rs` contains required fields: `brick_entity`, `brick_index`, `brick_position: Vec3`, `triggering_ball: Entity` (read-only verification, no code changes expected)
- [ ] T007 Create `BallSpawnBricksPlugin` struct in `src/systems/ball_spawn_bricks.rs` with plugin registration for config resource and systems
- [ ] T008 Register `BallSpawnBricksPlugin` in `src/lib.rs` with other game plugins

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Red 2 Brick: Spawn Additional Ball (Priority: P1) 🎯 MVP

**Goal**: When a player hits Red 2 brick (index 38), spawn one additional ball with inverse velocity, introducing controlled multi-ball gameplay

**Independent Test**: Place single Red 2 brick in level_015, hit it with one ball, verify exactly two balls exist with inverse velocity vectors

### Tests for User Story 1 (REQUIRED) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation; include failing-test commit hash in task metadata**

- [ ] T009 [P] [US1] Create test module `tests/ball_spawn_bricks.rs` with `mod red_2_tests` submodule
- [ ] T010 [P] [US1] Unit test `red_2_spawns_one_additional_ball` in `tests/ball_spawn_bricks.rs` - verify ball count increases from 1 to 2 after Red 2 brick destruction (Acceptance Scenario 1.1)
- [ ] T011 [P] [US1] Unit test `red_2_spawned_ball_has_inverse_velocity` in `tests/ball_spawn_bricks.rs` - verify spawned ball velocity equals `-triggering_velocity` (Acceptance Scenario 1.1)
- [ ] T012 [P] [US1] Unit test `red_2_spawns_from_multiple_balls` in `tests/ball_spawn_bricks.rs` - start with 3 balls, hit Red 2, verify 4 balls total and correct velocity inheritance (Acceptance Scenario 1.2)
- [ ] T013 [P] [US1] Unit test `red_2_spawns_at_brick_position` in `tests/ball_spawn_bricks.rs` - verify spawned ball appears at brick's XZ center (Acceptance Scenario 1.3)
- [ ] T014 [P] [US1] Multi-frame persistence test `red_2_spawned_ball_persists_10_frames` in `tests/ball_spawn_bricks.rs` - run `app.update()` 10 times, verify spawned ball still exists and position changes (physics applied) (Acceptance Scenario 1.4)
- [ ] T015 [P] [US1] Message-based test `red_2_uses_brick_destroyed_message` in `tests/ball_spawn_bricks.rs` - verify `BrickDestroyed` message written via `MessageWriter`, ball spawn system reads via `MessageReader` (Acceptance Scenario 1.5, Bevy 0.17 compliance)
- [ ] T016 [US1] Commit failing tests with message "test: add Red 2 brick spawn tests (red phase)" and record commit hash in T017 task description
- [ ] T017 [US1] Request approval from feature owner/requestor for Red 2 brick test specifications (blocking: must complete before T018)

### Implementation for User Story 1

- [ ] T018 [US1] Implement `ball_spawn_system` function in `src/systems/ball_spawn_bricks.rs` that reads `MessageReader<BrickDestroyed>` and dispatches to brick-specific handlers based on `brick_index` (test-proof commit: [HASH from T016])
- [ ] T019 [US1] Implement `spawn_ball` helper function in `src/systems/ball_spawn_bricks.rs` that creates Ball entity with Transform, Velocity, RigidBody, Collider components at specified position with specified velocity
- [ ] T020 [US1] Implement Red 2 brick logic in `ball_spawn_system`: match `brick_index == 38`, query triggering ball's velocity, spawn one ball with `-velocity`, verify no panicking queries (use `.ok()`)
- [ ] T021 [US1] Verify all T010-T015 tests pass with green status
- [ ] T022 [US1] Update `src/systems/mod.rs` to export `BallSpawnBricksPlugin` and relevant types
- [ ] T023 [US1] Manual test in level_015: hit Red 2 brick, observe spawned ball moving in opposite direction, verify 100 points awarded
- [ ] T024 [US1] Commit implementation with message "feat(US1): implement Red 2 brick spawn logic (green phase)"

**Checkpoint**: Red 2 brick fully functional - spawns one ball with inverse velocity, awards 100 points, testable independently

---

## Phase 4: User Story 2 - Red 3 Brick: Spawn Two Additional Balls (Priority: P2)

**Goal**: When a player hits Red 3 brick (index 39), spawn two additional balls in Y-shaped spread pattern, creating chaotic multi-ball scenarios

**Independent Test**: Hit single Red 3 brick in level_015, verify exactly three balls exist with Y-shaped velocity vectors (±30-45 degrees from original)

### Tests for User Story 2 (REQUIRED) ⚠️

- [ ] T025 [P] [US2] Create `mod red_3_tests` submodule in `tests/ball_spawn_bricks.rs`
- [ ] T026 [P] [US2] Unit test `red_3_spawns_two_additional_balls` in `tests/ball_spawn_bricks.rs` - verify ball count increases from 1 to 3 after Red 3 brick destruction (Acceptance Scenario 2.1)
- [ ] T027 [P] [US2] Unit test `red_3_spawns_y_shaped_pattern` in `tests/ball_spawn_bricks.rs` - verify spawned balls have velocities at approximately ±30-45 degrees from triggering ball's direction (Acceptance Scenario 2.1)
- [ ] T028 [P] [US2] Unit test `red_3_spawns_from_multiple_balls` in `tests/ball_spawn_bricks.rs` - start with 2 balls, hit Red 3, verify 4 balls total (Acceptance Scenario 2.2)
- [ ] T029 [P] [US2] Unit test `red_3_spawns_once_per_destruction` in `tests/ball_spawn_bricks.rs` - simulate multiple balls hitting Red 3 simultaneously, verify only 2 balls spawn (brick destroyed once) (Acceptance Scenario 2.3)
- [ ] T030 [P] [US2] Multi-frame persistence test `red_3_spawned_balls_persist_10_frames` in `tests/ball_spawn_bricks.rs` - verify both spawned balls exist and move independently for 10+ frames (Acceptance Scenario 2.4)
- [ ] T031 [US2] Commit failing tests with message "test: add Red 3 brick spawn tests (red phase)" and record commit hash in T033 task description
- [ ] T032 [US2] Request approval from feature owner/requestor for Red 3 brick test specifications (blocking: must complete before T033)

### Implementation for User Story 2

- [ ] T033 [US2] Implement `y_shaped_velocity` helper function in `src/systems/ball_spawn_bricks.rs` that takes base velocity and angle (default 45.0 degrees), returns tuple of (left_velocity, right_velocity) forming Y-shape in XZ plane (test-proof commit: [HASH from T031])
- [ ] T034 [US2] Implement Red 3 brick logic in `ball_spawn_system`: match `brick_index == 39`, query triggering ball's velocity, call `y_shaped_velocity`, spawn two balls with calculated velocities
- [ ] T035 [US2] Verify all T026-T030 tests pass with green status
- [ ] T036 [US2] Manual test in level_015: hit Red 3 brick, observe two spawned balls diverging in Y-pattern, verify 100 points awarded
- [ ] T037 [US2] Commit implementation with message "feat(US2): implement Red 3 brick spawn logic (green phase)"

**Checkpoint**: Red 2 AND Red 3 bricks both work independently - controlled and chaotic multi-ball gameplay available

---

## Phase 5: User Story 3 - Red 1 Brick: Reset to Single Ball (Priority: P3)

**Goal**: When a player hits Red 1 brick (index 37), despawn all balls except triggering ball, providing strategic relief from multi-ball chaos

**Independent Test**: Manually spawn 5 balls, hit Red 1 brick in level_015, verify exactly one ball remains

### Tests for User Story 3 (REQUIRED) ⚠️

- [ ] T038 [P] [US3] Create `mod red_1_tests` submodule in `tests/ball_spawn_bricks.rs`
- [ ] T039 [P] [US3] Unit test `red_1_despawns_all_except_triggering` in `tests/ball_spawn_bricks.rs` - start with 5 balls, hit Red 1, verify 1 ball remains (the triggering ball) (Acceptance Scenario 3.1)
- [ ] T040 [P] [US3] Unit test `red_1_with_single_ball_unchanged` in `tests/ball_spawn_bricks.rs` - start with 1 ball, hit Red 1, verify ball remains and is not despawned (Acceptance Scenario 3.2)
- [ ] T041 [P] [US3] Unit test `red_1_despawns_off_screen_balls` in `tests/ball_spawn_bricks.rs` - create 3 balls with different positions (some off-screen), hit Red 1 with on-screen ball, verify off-screen balls despawned (Acceptance Scenario 3.3)
- [ ] T042 [P] [US3] Multi-frame persistence test `red_1_no_respawn_after_despawn` in `tests/ball_spawn_bricks.rs` - despawn balls via Red 1, run 10+ frames, verify despawned balls do NOT respawn (Acceptance Scenario 3.4)
- [ ] T043 [US3] Commit failing tests with message "test: add Red 1 brick despawn tests (red phase)" and record commit hash in T045 task description
- [ ] T044 [US3] Request approval from feature owner/requestor for Red 1 brick test specifications (blocking: must complete before T045)

### Implementation for User Story 3

- [ ] T045 [US3] Implement Red 1 brick logic in `ball_spawn_system`: match `brick_index == 37`, query all Ball entities, despawn all except `triggering_ball` entity using `commands.entity(ball).despawn()` (test-proof commit: [HASH from T043])
- [ ] T046 [US3] Verify query uses `With<Ball>` filter and does NOT panic (Bevy 0.17 compliance: no `.unwrap()` on queries)
- [ ] T047 [US3] Verify all T039-T042 tests pass with green status
- [ ] T048 [US3] Manual test in level_015: spawn multiple balls (via Red 2/3 bricks), hit Red 1, observe all balls except one disappear, verify 100 points awarded
- [ ] T049 [US3] Commit implementation with message "feat(US3): implement Red 1 brick despawn logic (green phase)"

**Checkpoint**: All three user stories independently functional - full ball spawn/despawn mechanics available

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Integration, scoring, documentation, and edge case handling

- [ ] T050 [P] Verify scoring system awards exactly 100 points for brick indices 37, 38, 39 by checking `src/systems/scoring.rs::brick_index_to_points` function (read-only verification, no changes expected based on plan)
- [ ] T051 [P] Integration test `all_three_bricks_award_100_points` in `tests/ball_spawn_bricks.rs` - hit each brick type (37, 38, 39), verify 100 points awarded each time
- [ ] T052 [P] Integration test `rapid_consecutive_triggers` in `tests/ball_spawn_bricks.rs` - hit Red 2, then Red 3, then Red 1 within seconds, verify no errors and correct ball counts (Success Criterion SC-007)
- [ ] T053 [P] Add rustdoc comments to `BallSpawnBricksPlugin`, `BrickSpawnConfig`, `VelocityModifier`, `ball_spawn_system`, `spawn_ball`, `y_shaped_velocity` functions in `src/systems/ball_spawn_bricks.rs` (focus on WHY and WHEN, not HOW per constitution)
- [ ] T054 Verify WASM build compiles without errors: `cargo build --target wasm32-unknown-unknown`
- [ ] T055 Run full test suite: `cargo test` - verify no regressions in existing tests
- [ ] T056 Run native build and manual end-to-end test in level_015: hit all three brick types, verify behavior matches spec, check 60 FPS performance
- [ ] T057 Update `docs/bricks.md` to mark bricks 37, 38, 39 as implemented (change `|` to `✅️` in Status column)
- [ ] T058 Create pull request with title "feat: implement ball spawn bricks (Red 1/2/3)" and link to spec, plan, and tasks documents

**Final Checkpoint**: All acceptance criteria met, tests passing, documentation updated, ready for code review

---

## Dependencies & Parallel Execution

### User Story Dependency Graph

```text
Setup (Phase 1)
  ↓
Foundational (Phase 2) ← BLOCKING for all user stories
  ↓
  ├─→ User Story 1 (P1) - Red 2 Brick ← MVP, implement first
  ├─→ User Story 2 (P2) - Red 3 Brick ← Can implement in parallel with US1 after Phase 2
  └─→ User Story 3 (P3) - Red 1 Brick ← Can implement in parallel with US1/US2 after Phase 2
  ↓
Polish (Phase 6) ← Requires all user stories complete
```

**Key Insight**: User Stories 1, 2, and 3 are **independent** after Phase 2.
They can be implemented in parallel by different developers.

### Parallel Execution Opportunities

**Phase 1 (Setup)**:

- T002 and T003 can run in parallel (different concerns)

**Phase 2 (Foundational)**:

- T004 and T005 can run in parallel (same file, different structs)
- T006 is read-only verification (can run anytime)
- T007 and T008 depend on T004-T006

**Phase 3 (US1 - Red 2 Brick)**:

- T009-T015 all test tasks can run in parallel (different test functions)
- T019 (spawn_ball helper) can run in parallel with T020 (Red 2 logic) since they're in the same file but different functions
- T022 and T023 can run in parallel after T021 passes

**Phase 4 (US2 - Red 3 Brick)**:

- T025-T030 all test tasks can run in parallel
- Can implement **entire US2 phase in parallel with US1 phase** if multiple developers available

**Phase 5 (US3 - Red 1 Brick)**:

- T038-T042 all test tasks can run in parallel
- Can implement **entire US3 phase in parallel with US1/US2 phases** if multiple developers available

**Phase 6 (Polish)**:

- T050, T051, T052, T053, T057 can all run in parallel (different files/concerns)
- T054, T055, T056 must run sequentially (build dependencies)

### Parallel Implementation Example (3 developers)

```text
Developer A: Implements US1 (Red 2 brick) - Tasks T009-T024
Developer B: Implements US2 (Red 3 brick) - Tasks T025-T037
Developer C: Implements US3 (Red 1 brick) - Tasks T038-T049

All three can work simultaneously after Phase 2 completes (T004-T008).
Each developer commits their tests (red phase), gets approval, implements (green phase).
```

---

## Implementation Strategy

### MVP-First Approach

**Minimum Viable Product (MVP)**: User Story 1 (Red 2 Brick) only

- Deliverable: Players can hit Red 2 brick and spawn one additional ball with inverse velocity
- Value: Introduces core multi-ball mechanic with predictable behavior
- Effort: ~3-4 hours (T009-T024)
- Testing: Can be fully verified in level_015 without requiring other brick types

**Incremental Delivery**:

1. **Sprint 1**: Complete Phase 1-2 (Setup + Foundation) + US1 (Red 2) → **MVP Release**
2. **Sprint 2**: Add US2 (Red 3) → Escalated chaos mechanic
3. **Sprint 3**: Add US3 (Red 1) → Strategic reset mechanic
4. **Sprint 4**: Polish phase (integration tests, documentation, WASM verification)

### Risk Mitigation

**High-Risk Tasks** (require extra attention):

- T014, T030, T042: Multi-frame persistence tests (catch initialization system bugs per 020-gravity-bricks retrospective)
- T015: Message-based architecture verification (ensure no observer/message confusion)
- T020, T034, T045: Core spawn/despawn logic (critical game mechanics)
- T054: WASM build (platform-specific issues may arise)

**Mitigation Strategy**:

- Pair program on high-risk tasks
- Run multi-frame tests on every commit
- Early WASM builds during US1 implementation (don't wait for Phase 6)

---

## Verification Checklist

Before marking feature complete, verify:

- [ ] All tests in T009-T042 pass (red → green cycle documented in commit history)
- [ ] All three brick types (37, 38, 39) award exactly 100 points
- [ ] Spawned balls persist across 10+ frames without being reset
- [ ] `BrickDestroyed` message used correctly (buffered via `MessageWriter`/`MessageReader`)
- [ ] No panicking queries (all use `.ok()` or early returns)
- [ ] Queries use `With<Ball>` filter for specificity
- [ ] Spawned balls use simple `commands.spawn()` without parent complications
- [ ] WASM build compiles without errors
- [ ] Native build maintains 60 FPS with 5+ balls in play
- [ ] Manual test in level_015 confirms all behaviors match spec
- [ ] Rustdoc comments added to all public functions
- [ ] `docs/bricks.md` updated to mark bricks 37, 38, 39 as implemented ✅️

---

## Estimated Effort

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| Setup | T001-T003 | 0.5 hours |
| Foundational | T004-T008 | 1 hour |
| US1 (Red 2) | T009-T024 | 3-4 hours |
| US2 (Red 3) | T025-T037 | 2-3 hours |
| US3 (Red 1) | T038-T049 | 2-3 hours |
| Polish | T050-T058 | 2 hours |
| **Total** | 58 tasks | **10-13 hours** |

**Critical Path** (assuming single developer, sequential): ~12 hours

**Optimized Path** (assuming 3 developers, parallel US1/US2/US3): ~7 hours

---

## Summary

- **Total Tasks**: 58
- **User Stories**: 3 (US1=P1 MVP, US2=P2, US3=P3)
- **Tests Required**: 21 test tasks across all user stories
- **Parallel Opportunities**: US1, US2, US3 can be implemented simultaneously after Phase 2
- **MVP Scope**: User Story 1 only (Red 2 brick spawn mechanic)
- **Constitution Compliance**: All Bevy 0.17 mandates enforced in test and implementation tasks
- **Test Level**: level_015 (confirmed by user to contain bricks 37, 38, 39)
