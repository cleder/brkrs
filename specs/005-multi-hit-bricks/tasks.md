# Tasks: Multi-Hit Bricks

**Input**: Design documents from `/specs/005-multi-hit-bricks/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Tests are included as this is a gameplay feature requiring verification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- Paths based on plan.md project structure

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add multi-hit brick constants and module structure

- [x] T001 Add multi-hit brick constants in `src/level_format/mod.rs`
- [x] T002 [P] Create `src/systems/multi_hit.rs` module file with module documentation
- [x] T003 [P] Export multi_hit module in `src/systems/mod.rs`
- [x] T004 [P] Create test level `assets/levels/test_multi_hit.ron` with bricks at indices 10, 11, 12, 13

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Define `MultiHitBrickHit` event struct in `src/systems/multi_hit.rs`
- [x] T006 Register `MultiHitBrickHit` event in app builder (in `src/lib.rs` or plugin)
- [x] T007 Add `is_multi_hit_brick()` helper function in `src/level_format/mod.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Hitting a Multi-Hit Brick (Priority: P1) 🎯 MVP

**Goal**: When ball hits multi-hit brick (10-13), brick transitions to next lower index with visual change

**Independent Test**: Launch ball at multi-hit brick, observe visual transition 13→12→11→10→20

### Tests for User Story 1

- [x] T008 [P] [US1] Create integration test file `tests/multi_hit_bricks.rs`
- [x] T009 [P] [US1] Add test `multi_hit_brick_transitions_on_collision` in `tests/multi_hit_bricks.rs`
- [x] T010 [P] [US1] Add test `multi_hit_brick_13_to_12_transition` in `tests/multi_hit_bricks.rs`
- [x] T011 [P] [US1] Add test `multi_hit_brick_10_to_20_transition` in `tests/multi_hit_bricks.rs`

### Implementation for User Story 1

- [x] T012 [US1] Modify `mark_brick_on_ball_collision` in `src/lib.rs` to check for multi-hit bricks before marking for despawn
- [x] T013 [US1] Add `BrickTypeId` mutation logic for indices 10-13 in `src/lib.rs` collision handler
- [x] T014 [US1] Handle special case: index 10 transitions to index 20 (simple stone) in `src/lib.rs`
- [x] T015 [US1] Emit `MultiHitBrickHit` event when multi-hit brick is damaged in `src/lib.rs`
- [x] T016 [P] [US1] Implement `update_brick_material` system in `src/systems/multi_hit.rs` using `Changed<BrickTypeId>`
- [x] T017 [US1] Register `update_brick_material` system in app Update schedule
- [x] T018 [P] [US1] Add type_variants for indices 10, 11, 12, 13 in `assets/textures/manifest.ron`

**Checkpoint**: Multi-hit bricks transition visually on each hit (13→12→11→10→20)

---

## Phase 4: User Story 2 - Destroying the Final Stage (Priority: P1)

**Goal**: Simple stone (index 20, after multi-hit transition) can be destroyed and awards points

**Independent Test**: Hit brick at index 10, observe it becomes index 20, hit again to destroy

### Tests for User Story 2

- [x] T019 [P] [US2] Add test `simple_stone_destroyed_after_multi_hit` in `tests/multi_hit_bricks.rs`
- [x] T020 [P] [US2] Add test `multi_hit_to_stone_keeps_counts_towards_completion` in `tests/multi_hit_bricks.rs`

### Implementation for User Story 2

- [x] T021 [US2] Verify existing despawn logic handles index 20 correctly in `src/lib.rs`
- [x] T022 [US2] Ensure `CountsTowardsCompletion` component is preserved during BrickTypeId mutation
- [x] T023 [US2] Add rustdoc comments for multi-hit collision handling in `src/lib.rs`

**Checkpoint**: Multi-hit bricks can be fully destroyed (transition to 20, then destroyed)

---

## Phase 5: User Story 3 - Audio Feedback for Multi-Hit Bricks (Priority: P2)

**Goal**: Sound 29 plays when hitting multi-hit bricks (10-13)

**Independent Test**: Hit multi-hit brick, verify Sound 29 plays

### Tests for User Story 3

- [x] T024 [P] [US3] Add test `multi_hit_event_emitted_on_collision` in `tests/multi_hit_bricks.rs`

### Implementation for User Story 3

- [x] T025 [US3] Document `MultiHitBrickHit` event for audio system integration in `src/systems/multi_hit.rs`
- [x] T026 [US3] Add placeholder observer/system for audio playback in `src/systems/multi_hit.rs` (stub if audio not implemented)

**Checkpoint**: Audio event is emitted on multi-hit collision (audio playback when audio system ready)

---

## Phase 6: User Story 4 - Level Completion with Multi-Hit Bricks (Priority: P2)

**Goal**: Level completes only when all multi-hit bricks are fully destroyed

**Independent Test**: Create level with only multi-hit bricks, verify completion requires full destruction

### Tests for User Story 4

- [x] T027 [P] [US4] Add test `level_not_complete_with_partial_multi_hit_bricks` in `tests/multi_hit_bricks.rs`
- [x] T028 [P] [US4] Add test `level_completes_after_all_multi_hit_destroyed` in `tests/multi_hit_bricks.rs`

### Implementation for User Story 4

- [x] T029 [US4] Verify bricks with indices 10-13 spawn with `CountsTowardsCompletion` in `src/level_loader.rs`
- [x] T030 [US4] Verify `advance_level_when_cleared` works correctly with multi-hit lifecycle in `src/level_loader.rs`

**Checkpoint**: Level completion logic works correctly with multi-hit bricks

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, cleanup, and validation

- [x] T031 [P] Add rustdoc module documentation to `src/systems/multi_hit.rs`
- [x] T032 [P] Update `docs/bricks.md` if implementation differs from spec
- [x] T033 Run `cargo fmt --all` and `cargo clippy --all-targets --all-features`
- [x] T034 Run `bevy lint` for Bevy-specific checks
- [x] T035 Run all tests with `cargo test`
- [x] T036 Manual validation: Run `BK_LEVEL=998 cargo run` with test level per `quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - US1 and US2 are both P1 - US2 depends on US1 (builds on transition to index 20)
  - US3 and US4 are P2 - can proceed after US1/US2 or in parallel
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Depends on User Story 1 (needs 10→20 transition to exist)
- **User Story 3 (P2)**: Depends on User Story 1 (needs event emission to exist)
- **User Story 4 (P2)**: Can start after Foundational - depends on US1/US2 for full lifecycle

### Within Each User Story

- Tests (included) should be written first and FAIL before implementation
- Core logic before visual updates
- Events before subscribers
- Implementation before documentation

### Parallel Opportunities

- T002, T003, T004 can run in parallel (different files)
- All US1 tests (T008-T011) can run in parallel
- T016, T018 can run in parallel with collision logic (different files)
- US3 and US4 tests can run in parallel
- All Polish tasks marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: T008 "Create integration test file tests/multi_hit_bricks.rs"
Task: T009 "Add test multi_hit_brick_transitions_on_collision"
Task: T010 "Add test multi_hit_brick_13_to_12_transition"
Task: T011 "Add test multi_hit_brick_10_to_20_transition"

# After collision logic (T012-T015), these can run in parallel:
Task: T016 "Implement update_brick_material system in src/systems/multi_hit.rs"
Task: T018 "Add type_variants for indices 10, 11, 12, 13 in assets/textures/manifest.ron"
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 - Core collision/transition logic
4. Complete Phase 4: User Story 2 - Full destruction lifecycle
5. **STOP and VALIDATE**: Test multi-hit bricks work end-to-end
6. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test transitions work → Demo (MVP!)
3. Add User Story 2 → Test full lifecycle → Demo
4. Add User Story 3 → Audio feedback ready → Demo
5. Add User Story 4 → Level completion verified → Demo
6. Each story adds value without breaking previous stories

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- US1 + US2 together form the core MVP (both P1 priority)
- US3 (audio) and US4 (level completion) are enhancements (P2 priority)
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
