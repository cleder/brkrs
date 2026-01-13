# Tasks: Paddle-Destroyable Brick (Type 57)

**Input**: Design documents from `/specs/022-paddle-destroyable-brick/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/events.md, quickstart.md

**Tests**: Tests are MANDATORY for all user stories. Each story MUST include integration tests. Tests MUST be written and committed first, verified to FAIL (red), and then approved before implementation begins; record the test-proof commit hash in the task description.

**Bevy 0.17 compliance**: Tasks include explicit acceptance criteria to ensure compliance with the constitution's Bevy 0.17 mandates & prohibitions:
- No panicking queries (use fallible patterns with early returns)
- Filtered queries (`With`/`Without` for marker components)
- Messages (via `MessageWriter`) for brick destruction (NOT observers)
- `despawn_recursive()` for hierarchy safety
- Multi-frame persistence tests (minimum 10 frames)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Verify existing project structure matches plan.md requirements (src/, tests/, assets/levels/)
- [ ] T002 Verify Rust 1.81 (edition 2021) with Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1 dependencies in Cargo.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T003 Add `is_paddle_destroyable_brick(brick_type: u8) -> bool` helper function in src/lib.rs (returns true for type 57)
- [ ] T004 Verify `brick_points(57, _) -> 250` mapping exists in src/systems/scoring.rs (already implemented per research.md Q2)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Paddle Destroys Brick on Contact (Priority: P1) 🎯 MVP

**Goal**: When paddle touches paddle-destroyable brick (type 57), brick is destroyed within 1 frame, 250 points awarded, brick counts toward level completion

**Independent Test**: Spawn a paddle-destroyable brick, move paddle to collide, verify brick despawned, 250 points added, level completion tracking updated

### Tests for User Story 1 (REQUIRED) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation; include failing-test commit hash in task**

- [ ] T005 [P] [US1] Create integration test file tests/paddle_destroyable_brick.rs with test module structure
- [ ] T006 [P] [US1] Acceptance test AS 1.1: Paddle collision despawns brick within 1 frame in tests/paddle_destroyable_brick.rs
- [ ] T007 [P] [US1] Acceptance test AS 1.2: Paddle contact awards exactly 250 points in tests/paddle_destroyable_brick.rs
- [ ] T008 [P] [US1] Acceptance test AS 1.3: Level completion when all paddle-destroyable bricks destroyed in tests/paddle_destroyable_brick.rs
- [ ] T009 [P] [US1] Acceptance test AS 1.4: Multi-frame persistence (10 frames) for score award in tests/paddle_destroyable_brick.rs
- [ ] T010 [P] [US1] Acceptance test AS 1.5: Brick destruction uses Messages via MessageWriter (not observers), verify BrickDestroyed message has destroyed_by=None for paddle destruction in tests/paddle_destroyable_brick.rs
- [ ] T011 [P] [US1] Acceptance test AS 1.6: Hierarchy safety - despawn uses despawn_recursive() in tests/paddle_destroyable_brick.rs
- [ ] T012 [US1] Commit failing tests with message "test(US1): add paddle destroys brick acceptance tests (red)" and record commit hash

**Bevy 0.17 Acceptance Criteria for Tests**:
- Test MUST verify no panicking queries (system continues if brick missing)
- Test MUST verify `MessageWriter<BrickDestroyed>` used (not observers)
- Test MUST verify `despawn_recursive()` used (check via hierarchy test)
- Test MUST include multi-frame persistence check (10+ `app.update()` cycles)

### Implementation for User Story 1

- [ ] T013 [US1] Extend `read_character_controller_collisions` system in src/lib.rs to detect paddle-brick type 57 collisions (add `brick_types: Query<&BrickTypeId, With<Brick>>` parameter)
- [ ] T014 [US1] Add type 57 check in paddle collision handler: if `brick_type.0 == 57`, insert `MarkedForDespawn` component in src/lib.rs
- [ ] T015 [US1] Add DEBUG logging for paddle-brick type 57 collisions using `debug!(target: "paddle_destroyable", ...)` in src/lib.rs
- [ ] T016 [US1] Verify `despawn_marked_entities` system emits `BrickDestroyed` message with `brick_type=57`, `destroyed_by=None` (no changes needed per data-model.md)
- [ ] T017 [US1] Verify `award_points_system` awards 250 points for `brick_type=57` via existing `brick_points()` function (no changes needed per research.md Q2)
- [ ] T018 [US1] Run all US1 acceptance tests to verify green (all 6 tests pass)
- [ ] T019 [US1] Commit implementation with message "feat(US1): implement paddle destroys brick type 57 (green)"

**Bevy 0.17 Implementation Requirements**:
- MUST use fallible query patterns: `if let Ok(brick_type) = brick_types.get(brick) { ... }`
- MUST use `With<Brick>` filter in brick_types query
- MUST NOT use `.unwrap()` on query results
- MUST use `commands.entity(brick).insert(MarkedForDespawn)` (existing despawn system handles `despawn_recursive()`)
- MUST use `MessageWriter<BrickDestroyed>` (already implemented in despawn_marked_entities - verify only)

**Checkpoint**: At this point, User Story 1 should be fully functional - paddle destroys brick, awards 250 points, counts toward completion

---

## Phase 4: User Story 2 - Ball Bounces Off Brick Without Destruction (Priority: P1)

**Goal**: When ball collides with paddle-destroyable brick, ball bounces off (physics-based reflection), brick remains intact, no points awarded

**Independent Test**: Spawn paddle-destroyable brick, launch ball to collide, verify ball bounces, brick persists 10+ frames, no score change

### Tests for User Story 2 (REQUIRED) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation; include failing-test commit hash in task**

- [ ] T020 [P] [US2] Acceptance test AS 2.1: Ball reflects at correct angle (physics-based) in tests/paddle_destroyable_brick.rs
- [ ] T021 [P] [US2] Acceptance test AS 2.2: Brick entity NOT despawned after ball collision in tests/paddle_destroyable_brick.rs
- [ ] T022 [P] [US2] Acceptance test AS 2.3: Zero points awarded on ball-brick collision in tests/paddle_destroyable_brick.rs
- [ ] T023 [P] [US2] Acceptance test AS 2.4: Multi-frame persistence (10 frames) - brick exists after ball collision in tests/paddle_destroyable_brick.rs
- [ ] T024 [P] [US2] Acceptance test AS 2.5: Ball collision uses bevy_rapier3d contact events (not custom observers) in tests/paddle_destroyable_brick.rs
- [ ] T025 [US2] Commit failing tests with message "test(US2): add ball bounces off brick acceptance tests (red)" and record commit hash

**Bevy 0.17 Acceptance Criteria for Tests**:
- Test MUST verify brick entity exists after 10 `app.update()` cycles (multi-frame persistence)
- Test MUST verify no `BrickDestroyed` message emitted for type 57 ball collisions
- Test MUST verify bevy_rapier3d `CollisionEvent` handling (existing system, no observers)

### Implementation for User Story 2

- [ ] T026 [US2] Add `is_paddle_destroyable_brick()` guard in `handle_collision_events` ball-brick collision handler in src/lib.rs
- [ ] T027 [US2] Add early return (`continue`) if `is_paddle_destroyable_brick(current_type) == true` to skip destruction logic in src/lib.rs
- [ ] T028 [US2] Verify ball physics reflection handled automatically by bevy_rapier3d collider (no code changes - verify via test)
- [ ] T029 [US2] Run all US2 acceptance tests to verify green (all 5 tests pass)
- [ ] T030 [US2] Commit implementation with message "feat(US2): ball bounces off paddle-destroyable brick (green)"

**Bevy 0.17 Implementation Requirements**:
- MUST use `is_paddle_destroyable_brick(current_type)` check before processing ball-brick collision
- MUST NOT mark brick with `MarkedForDespawn` for type 57 ball collisions
- MUST rely on bevy_rapier3d automatic collision response (no custom physics code)
- Ball bounce behavior verification via test only (no implementation code needed)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work - paddle destroys brick, ball bounces off brick

---

## Phase 5: User Story 3 - Brick Type Configuration in Level Files (Priority: P2)

**Goal**: Level designers can place paddle-destroyable bricks in RON level files using brick type 57, bricks spawn with correct components

**Independent Test**: Create test level file with type 57 bricks, load level, verify correct number spawned with required components (Transform, BrickTypeId, Collider, CountsTowardsCompletion)

### Tests for User Story 3 (REQUIRED) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation; include failing-test commit hash in task**

- [ ] T031 [P] [US3] Acceptance test AS 3.1: RON file with `{ brick_type: 57 }` spawns paddle-destroyable brick in tests/paddle_destroyable_brick.rs
- [ ] T032 [P] [US3] Acceptance test AS 3.2: Spawned brick has all required components (Transform, BrickTypeId(57), Collider, CountsTowardsCompletion) in tests/paddle_destroyable_brick.rs
- [ ] T033 [P] [US3] Acceptance test AS 3.3: Level with 3 type 57 bricks spawns exactly 3 entities in tests/paddle_destroyable_brick.rs
- [ ] T034 [P] [US3] Acceptance test AS 3.4: Multi-frame persistence (10 frames) - spawned bricks maintain properties in tests/paddle_destroyable_brick.rs
- [ ] T035 [US3] Commit failing tests with message "test(US3): add level file configuration acceptance tests (red)" and record commit hash

**Bevy 0.17 Acceptance Criteria for Tests**:
- Test MUST verify `CountsTowardsCompletion` component present on spawned bricks
- Test MUST use fallible queries to get components (`if let Ok(components) = query.get(entity) { ... }`)
- Test MUST verify brick properties persist across 10 `app.update()` cycles

### Implementation for User Story 3

- [ ] T036 [US3] Create test level file assets/levels/test_paddle_destroyable.ron with 3 paddle-destroyable bricks (type 57)
- [ ] T037 [US3] Run acceptance tests T031-T034 to verify level loader spawns type 57 bricks with `BrickTypeId(57)` component (no loader changes needed per research.md Q4)
- [ ] T038 [US3] Verify level loader adds `CountsTowardsCompletion` component to type 57 bricks (should be automatic for all typed bricks - verify via test)
- [ ] T039 [US3] Run all US3 acceptance tests to verify green (all 4 tests pass)
- [ ] T040 [US3] Commit implementation with message "feat(US3): add level file support for paddle-destroyable brick (green)"

**Bevy 0.17 Implementation Requirements**:
- Level loader MUST spawn type 57 bricks with same component pattern as other brick types
- Level loader MUST add `CountsTowardsCompletion` marker component
- No special handling needed (type 57 follows existing brick spawn pattern per data-model.md)

**Checkpoint**: All user stories should now be independently functional - paddle destroys, ball bounces, level files work

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

**Note**: Edge cases 4 (all bricks paddle-destroyable) and 5 (nested entities) are covered by US1 tests T008 and T011 respectively.

- [ ] T041 [P] Add edge case test: Simultaneous paddle+ball contact (paddle takes precedence) in tests/paddle_destroyable_brick.rs
- [ ] T042 [P] Add edge case test: Multiple paddle-destroyable bricks touched in one frame (each awards 250 points) in tests/paddle_destroyable_brick.rs
- [ ] T043 [P] Add edge case test: Brick spawns overlapping paddle (destroyed on first frame, 250 points awarded) in tests/paddle_destroyable_brick.rs
- [ ] T044 [P] Add edge case test: Ball inside brick collider when brick despawns (ball continues unaffected) in tests/paddle_destroyable_brick.rs
- [ ] T045 Run quickstart.md manual verification scenarios (6 test cases) and verify all pass
- [ ] T046 Run full test suite: `cargo test` and verify all tests pass
- [ ] T047 Run code quality checks: `cargo fmt --all`, `cargo clippy --all-targets --all-features`
- [ ] T048 Verify DEBUG logging output for paddle-brick collisions using `RUST_LOG=paddle_destroyable=debug cargo run`
- [ ] T049 [P] Update CHANGELOG.md with feature summary: "Added paddle-destroyable brick (type 57) - destroyed by paddle only, awards 250 points"
- [ ] T050 [P] Update docs/bricks.md with type 57 documentation: behavior, point value, level file syntax

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - verification only
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - User Story 1 (P1): Can start after Foundational - No dependencies on other stories
  - User Story 2 (P1): Can start after Foundational - May run in parallel with US1 (different collision handler code path)
  - User Story 3 (P2): Can start after Foundational - Independent of US1/US2 (level loader path)
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - Implements paddle collision handler extension
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - Implements ball collision handler guard
  - Can run in parallel with US1 if US1 starts first (different code sections in src/lib.rs)
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - Verifies level loader integration
  - Independent of US1/US2 implementation (test-only verification)

### Within Each User Story

- Tests MUST be written first and verified to FAIL before implementation begins; include failing-test commit in the branch and record its hash in the task description
- Tests MUST be approved by the feature owner/requestor before implementation proceeds
- Implementations MUST comply with Bevy 0.17 mandates:
  - Fallible systems: `if let Ok(x) = query.get() { ... }` not `.unwrap()`
  - Filtered queries: `Query<&BrickTypeId, With<Brick>>` not `Query<&BrickTypeId>`
  - Messages for destruction: `MessageWriter<BrickDestroyed>` (existing system)
  - Hierarchy safety: `despawn_recursive()` (handled by existing despawn system)
  - Multi-frame persistence tests: Minimum 10 `app.update()` cycles
- Tests (T005-T012) before implementation (T013-T019) for US1
- Tests (T020-T025) before implementation (T026-T030) for US2
- Tests (T031-T035) before implementation (T036-T040) for US3

### Parallel Opportunities

- **Phase 1**: T001 and T002 can run in parallel (independent verifications)
- **Phase 2**: T003 and T004 can run in parallel (different files)
- **User Story 1 Tests**: T006-T011 can all run in parallel (different test functions in same file)
- **User Story 2 Tests**: T020-T024 can all run in parallel (different test functions)
- **User Story 3 Tests**: T031-T034 can all run in parallel (different test functions)
- **Once Foundational complete**: All 3 user stories can start in parallel (different team members)
- **Phase 6 Polish**: T041-T044 (edge case tests) can run in parallel, T049-T050 (docs) can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
# T006: Acceptance test AS 1.1 - Paddle collision despawns brick
# T007: Acceptance test AS 1.2 - Paddle contact awards 250 points
# T008: Acceptance test AS 1.3 - Level completion condition
# T009: Acceptance test AS 1.4 - Multi-frame persistence
# T010: Acceptance test AS 1.5 - Messages (not observers)
# T011: Acceptance test AS 1.6 - Hierarchy safety

# All 6 test tasks can be written in parallel (same file, different functions)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T002) - Verification only
2. Complete Phase 2: Foundational (T003-T004) - Helper functions
3. Complete Phase 3: User Story 1 (T005-T019)
   - Write failing tests (T005-T012)
   - Get approval from feature owner
   - Implement paddle collision handler (T013-T017)
   - Verify integration with scoring (T016-T017)
   - Run tests and commit green (T018-T019)
4. **STOP and VALIDATE**: Test paddle-destroyable brick manually (destroy with paddle, verify 250 points)
5. Demo/review if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → **MVP: Paddle destroys brick, awards 250 points**
3. Add User Story 2 → Test independently → **Ball bounce behavior complete**
4. Add User Story 3 → Test independently → **Level file support complete**
5. Add Polish (edge cases, docs) → **Feature complete**

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together (T001-T004)
2. Once Foundational is done:
   - Developer A: User Story 1 (T005-T019) - Paddle collision handler
   - Developer B: User Story 2 (T020-T030) - Ball collision guard
   - Developer C: User Story 3 (T031-T040) - Level file verification
3. Stories complete independently, integrate naturally (same collision systems, non-conflicting code paths)

---

## Total Task Count

- **Phase 1 (Setup)**: 2 tasks
- **Phase 2 (Foundational)**: 2 tasks
- **Phase 3 (User Story 1)**: 15 tasks (7 tests + 7 implementation + 1 commit)
- **Phase 4 (User Story 2)**: 11 tasks (6 tests + 4 implementation + 1 commit)
- **Phase 5 (User Story 3)**: 10 tasks (5 tests + 4 implementation + 1 commit)
- **Phase 6 (Polish)**: 10 tasks
- **Total**: 50 tasks

### Task Breakdown by User Story

- **User Story 1**: 15 tasks (paddle destroys brick, awards points, level completion)
- **User Story 2**: 11 tasks (ball bounces off, no destruction, no points)
- **User Story 3**: 10 tasks (level file configuration, component verification)

### Parallel Opportunities Summary

- **Phase 1**: 2 tasks can run in parallel
- **Phase 2**: 2 tasks can run in parallel
- **User Story 1 tests**: 6 tasks can run in parallel (T006-T011)
- **User Story 2 tests**: 5 tasks can run in parallel (T020-T024)
- **User Story 3 tests**: 4 tasks can run in parallel (T031-T034)
- **All 3 user stories**: Can proceed in parallel after Foundational phase (if team capacity allows)
- **Polish phase**: 6 tasks can run in parallel (T041-T044, T049-T050)

---

## Notes

- [P] tasks = different files or independent code sections, no dependencies
- [Story] label (US1, US2, US3) maps task to specific user story for traceability
- Each user story independently completable and testable
- **TDD CRITICAL**: Write tests first (red), get approval, implement (green), commit
- **Bevy 0.17 CRITICAL**: All implementations must use fallible queries, Messages (not Observers), `despawn_recursive()`
- Multi-frame persistence tests mandatory (minimum 10 frames)
- Commit after each user story completion (T012, T025, T035, T040)
- Stop at any checkpoint to validate story independently
- Record failing-test commit hashes in task descriptions (T012, T025, T035)
