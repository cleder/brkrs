# Tasks: Paddle Shrink Visual Feedback

**Input**: Design documents from `/specs/008-paddle-shrink-feedback/` **Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Integration tests included (following existing patterns in tests/ directory)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify existing infrastructure supports the feature

- [ ] T001 Verify Bevy 0.17.3 and bevy_rapier3d 0.32.0 in Cargo.toml
- [ ] T002 Verify existing PaddleGrowing component in src/lib.rs
- [ ] T003 Verify existing respawn systems in src/systems/respawn.rs
- [ ] T004 Verify existing LifeLostEvent message in src/systems/respawn.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core components that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T005 Add apply_paddle_shrink system stub to src/systems/respawn.rs
- [ ] T006 Register apply_paddle_shrink in RespawnSystems::Detect set in src/systems/respawn.rs
- [ ] T007 Ensure system runs after detect_ball_loss in src/systems/respawn.rs

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Paddle Shrinks on Ball Loss (Priority: P1) 🎯 MVP

**Goal**: Paddle provides immediate visual feedback by shrinking when ball is lost, running concurrently with respawn delay

**Independent Test**: Can be fully tested by playing the game, intentionally losing the ball, and observing the paddle shrink animation before the standard 1-second respawn delay and subsequent paddle regrowth

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T008 [P] [US1] Create tests/paddle_shrink.rs test file structure
- [ ] T009 [P] [US1] Implement test_paddle_shrinks_on_ball_loss in tests/paddle_shrink.rs
- [ ] T010 [P] [US1] Implement test_shrink_reaches_minimum_scale in tests/paddle_shrink.rs
- [ ] T011 [P] [US1] Implement test_paddle_remains_visible_during_shrink in tests/paddle_shrink.rs

### Implementation for User Story 1

- [ ] T012 [US1] Implement apply_paddle_shrink system logic in src/systems/respawn.rs
  - Query paddles without PaddleGrowing component
  - Read LifeLostEvent messages
  - Add PaddleGrowing with target_scale Vec3::splat(0.01)
  - Set timer duration from RespawnSchedule.timer.duration()
- [ ] T013 [US1] Add rustdoc documentation to apply_paddle_shrink system in src/systems/respawn.rs
- [ ] T014 [US1] Verify update_paddle_growth system handles shrink (target_scale < current scale) in src/lib.rs
- [ ] T015 [US1] Run tests T009-T011 and verify they pass
- [ ] T016 [US1] Manual verification per quickstart.md Section 1 (Basic Shrink Behavior)
- [ ] T017 [US1] Manual verification per quickstart.md Section 3 (Input Locking)

**Checkpoint**: At this point, User Story 1 should be fully functional - paddle shrinks on ball loss with proper visual feedback

---

## Phase 4: User Story 2 - Shrink Animation Timing Integration (Priority: P2)

**Goal**: Shrink animation integrates seamlessly with existing respawn system timing, running concurrently with fadeout overlay

**Independent Test**: Can be tested by measuring the total time from ball loss to full paddle regrowth completion, verifying smooth transitions and concurrent execution

### Tests for User Story 2

- [ ] T018 [P] [US2] Implement test_shrink_duration_matches_respawn_delay in tests/paddle_shrink.rs
- [ ] T019 [P] [US2] Implement test_shrink_concurrent_with_fadeout_overlay in tests/paddle_shrink.rs
- [ ] T020 [P] [US2] Implement test_paddle_scale_interpolation_smooth in tests/paddle_shrink.rs
- [ ] T021 [P] [US2] Implement test_rapid_consecutive_losses in tests/paddle_shrink.rs

### Implementation for User Story 2

- [ ] T022 [US2] Verify shrink timer initialization uses RespawnSchedule.timer.duration() in src/systems/respawn.rs
- [ ] T023 [US2] Verify concurrent execution: shrink starts same frame as RespawnScheduled event in src/systems/respawn.rs
- [ ] T024 [US2] Add timing validation to ensure no additional delay added in src/systems/respawn.rs
- [ ] T025 [US2] Run tests T018-T021 and verify they pass
- [ ] T026 [US2] Manual verification per quickstart.md Section 2 (Timing Integration)
- [ ] T027 [US2] Manual verification per quickstart.md Section 5 (Rapid Consecutive Losses)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work - shrink timing perfectly integrated with respawn system

---

## Phase 5: User Story 3 - Multiple Ball Scenarios (Priority: P3)

**Goal**: Handle edge cases including shrink during level transition, game over, and multi-paddle future-proofing

**Independent Test**: Can be tested by triggering edge cases (level transition during shrink, game over during shrink) and verifying graceful handling

### Tests for User Story 3

- [ ] T028 [P] [US3] Implement test_shrink_interrupts_growth_animation in tests/paddle_shrink.rs
- [ ] T029 [P] [US3] Implement test_shrink_during_level_transition in tests/paddle_shrink.rs
- [ ] T030 [P] [US3] Implement test_game_over_during_shrink in tests/paddle_shrink.rs
- [ ] T031 [P] [US3] Implement test_only_associated_paddle_shrinks in tests/paddle_shrink.rs

### Implementation for User Story 3

- [ ] T032 [US3] Verify apply_paddle_shrink skips paddles already with PaddleGrowing in src/systems/respawn.rs
- [ ] T033 [US3] Add guard to prevent shrink application if paddle already shrinking in src/systems/respawn.rs
- [ ] T034 [US3] Verify respawn_executor naturally overrides shrink state via component replacement in src/systems/respawn.rs
- [ ] T035 [US3] Add logging for edge case handling (optional) in src/systems/respawn.rs
- [ ] T036 [US3] Run tests T028-T031 and verify they pass
- [ ] T037 [US3] Manual verification per quickstart.md Section 4 (Edge Case: Shrink During Level Transition)
- [ ] T038 [US3] Manual verification per quickstart.md Section 6 (Edge Case: Game Over During Shrink)

**Checkpoint**: All user stories should now be independently functional with edge cases handled

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final quality checks, documentation, and performance validation

- [ ] T039 [P] Run full test suite: `cargo test`
- [ ] T040 [P] Run formatting: `cargo fmt --all`
- [ ] T041 [P] Run linting: `cargo clippy --all-targets --all-features`
- [ ] T042 [P] Run Bevy linting: `bevy lint`
- [ ] T043 Build for WASM: `cargo build --release --target wasm32-unknown-unknown`
- [ ] T044 Test WASM in browser (manual verification per quickstart.md WASM Performance section)
- [ ] T045 [P] Verify frame rate remains 60 FPS during shrink (manual with bevy diagnostics)
- [ ] T046 [P] Update CHANGELOG.md with feature summary
- [ ] T047 Run complete acceptance criteria checklist per quickstart.md
- [ ] T048 Final review of rustdoc documentation for apply_paddle_shrink system

**Checkpoint**: Feature complete, tested, and ready for pull request

---

## Dependencies

### User Story Completion Order

```text
Phase 1 (Setup) → Phase 2 (Foundation)
                       ↓
              Phase 3 (US1: Basic Shrink) 🎯 MVP
                       ↓
              Phase 4 (US2: Timing Integration)
                       ↓
              Phase 5 (US3: Edge Cases)
                       ↓
              Phase 6 (Polish)
```

**MVP Scope**: Phase 3 (User Story 1) delivers minimum viable product

- Paddle shrinks on ball loss
- Concurrent with respawn delay
- Smooth animation with proper timing

**Incremental Delivery**:

- After Phase 3: Basic feature works, can ship if needed
- After Phase 4: Timing perfected, production-ready
- After Phase 5: All edge cases covered, future-proof

### Parallel Execution Opportunities

#### Phase 3 (US1) Parallelization

```text
T008, T009, T010, T011 (all test creation) → run in parallel
     ↓
T012 (implementation)
     ↓
T013, T014 (documentation/verification) → run in parallel
     ↓
T015, T016, T017 (validation) → run sequentially
```

#### Phase 4 (US2) Parallelization

```text
T018, T019, T020, T021 (all test creation) → run in parallel
     ↓
T022, T023, T024 (implementation verification) → run in parallel
     ↓
T025, T026, T027 (validation) → run sequentially
```

#### Phase 5 (US3) Parallelization

```text
T028, T029, T030, T031 (all test creation) → run in parallel
     ↓
T032, T033, T034, T035 (implementation) → some can run in parallel
     ↓
T036, T037, T038 (validation) → run sequentially
```

#### Phase 6 (Polish) Parallelization

```text
T039, T040, T041, T042 (automated checks) → run in parallel
     ↓
T043, T044, T045, T046 (builds and verification) → T046 can run parallel with others
     ↓
T047, T048 (final checks) → run in parallel
```

---

## Implementation Strategy

### MVP-First Approach

**Phase 3 (User Story 1)** represents the MVP:

- Delivers core value: visual feedback on ball loss
- Independently testable and deployable
- ~4-6 hours implementation time

**Incremental Enhancements**:

- Phase 4 adds timing perfection (~2-3 hours)
- Phase 5 adds robustness for edge cases (~2-3 hours)
- Phase 6 adds polish and validation (~1-2 hours)

**Total Estimated Time**: 9-14 hours (including testing)

### Risk Mitigation

**Low Risk Areas**:

- Reusing existing PaddleGrowing component (proven pattern)
- Integration with existing respawn system (well-defined events)
- ECS architecture (natural fit for animation state)

**Medium Risk Areas**:

- Timing synchronization with fadeout overlay (requires careful testing)
- Edge case handling during level transitions (needs verification)

**Mitigation**:

- Write tests first (TDD approach)
- Manual verification at each checkpoint
- Frequent testing on both native and WASM

---

## Task Summary

**Total Tasks**: 48

- Phase 1 (Setup): 4 tasks
- Phase 2 (Foundation): 3 tasks
- Phase 3 (US1 - MVP): 10 tasks (4 test + 6 implementation)
- Phase 4 (US2): 10 tasks (4 test + 6 implementation)
- Phase 5 (US3): 11 tasks (4 test + 7 implementation)
- Phase 6 (Polish): 10 tasks

**Parallelizable Tasks**: 25 tasks marked with [P]

**User Story Mapping**:

- User Story 1 (P1): 10 tasks → MVP delivery
- User Story 2 (P2): 10 tasks → Timing perfection
- User Story 3 (P3): 11 tasks → Edge case handling

**Independent Test Criteria**:

- US1: Play game, lose ball, observe shrink animation
- US2: Measure timing, verify concurrent execution
- US3: Trigger edge cases, verify graceful handling

---

## Validation Checklist

Before marking feature complete:

- [ ] All functional requirements (FR-001 through FR-013) verified
- [ ] All success criteria (SC-001 through SC-007) met
- [ ] All acceptance scenarios from spec.md pass
- [ ] Manual verification per quickstart.md complete
- [ ] Automated tests pass: `cargo test paddle_shrink`
- [ ] Code quality checks pass: fmt, clippy, bevy lint
- [ ] WASM build successful and tested in browser
- [ ] Frame rate maintains 60 FPS during animation
- [ ] Documentation complete (rustdoc for new systems)
- [ ] No regressions in existing respawn functionality
