# Task Breakdown: Add Scoring System

**Feature**: Add Scoring System | **Branch**: `009-add-scoring` | **Date**: 16 December 2025

**Specification**: [spec.md](spec.md) | **Design**: [data-model.md](data-model.md) | **Contracts**: [contracts/events.md](contracts/events.md)

---

## Executive Summary

**Total Tasks**: 19 | **Phases**: 5 | **User Stories**: 4 | **Parallel Opportunities**: 8

- **Phase 1 (Setup)**: 2 tasks
- **Phase 2 (Foundational)**: 3 tasks
- **Phase 3 (User Story 1 - P1)**: 3 tasks
- **Phase 4 (User Story 2 - P1)**: 5 tasks
- **Phase 5 (User Story 3 - P1)**: 3 tasks
- **Phase 6 (User Story 4 - P2)**: 3 tasks

**Recommended MVP Scope**: User Stories 1-3 (all P1) **Stretch Goal**: Add User Story 4 (score visibility) if time permits

**Parallel Execution Strategy**:

- Phase 1: Sequential (setup)
- Phase 2: Sequential (shared infrastructure)
- Phase 3: Sequential (initialization)
- Phase 4: T010-T011 parallelizable; T012-T014 sequential
- Phase 5: T015-T016 parallelizable; T017 sequential
- Phase 6: T018 blocks T019; T020 independent

---

## Dependency Graph

```text
Phase 1 (Setup)
    ↓
Phase 2 (Foundational Infrastructure)
    ├─→ Phase 3 (US1: Score Initialization)
    │       ↓
    │   Phase 4 (US2: Points on Destruction) ─┐
    │       ↓                                  │
    │   Phase 5 (US3: Milestone Balls)        │
    │       ↓                                  │
    └───→ Phase 6 (US4: Display Visibility) ◄─┘
```

**Completion Order for Feature**:

1. Complete Phase 1 (setup foundation)
2. Complete Phase 2 (shared infrastructure: ScoreState resource, BrickDestroyed event)
3. Complete Phase 3 (score initialization - enables all subsequent features)
4. **Parallel**: Complete Phase 4 & 5 (point award + milestone detection)
5. Complete Phase 6 (UI display)

---

## Phase 1: Setup & Project Initialization

Initialize feature branch structure and confirm prerequisites.

### Tests

*No tests required for setup phase.*

### Tasks

- [x] T001 Create source files for scoring module in `src/systems/scoring.rs` with module skeleton
- [x] T002 Create source files for score display module in `src/ui/score_display.rs` with module skeleton

---

## Phase 2: Foundational Infrastructure (Blocking Prerequisites)

Implement shared ECS infrastructure that all user stories depend on.

**Completion Gate**: All tasks in this phase MUST complete before starting user stories.

### Tests

*No tests required for Phase 2; tested by Phase 3+.*

### Tasks

- [x] T003 Define `ScoreState` resource in `src/systems/scoring.rs` with `current_score: u32` and `last_milestone_reached: u32` fields
- [x] T004 Define `BrickDestroyed` message struct in `src/systems/scoring.rs` with `brick_entity`, `brick_type`, `destroyed_by` fields
- [x] T005 Define `MilestoneReached` message struct in `src/systems/scoring.rs` with `milestone_tier`, `total_score` fields

---

## Phase 3: User Story 1 - Score Initialization (P1)

**Goal**: Player starts game with zero score displayed.

**Acceptance Criteria**:

- Score initializes to 0 when game starts
- Score persists unchanged until first brick destroyed
- Score display shows "0" on startup

**Independent Test**: Start game, verify score UI shows "0" and doesn't change until bricks destroyed

**Parallel Opportunities**: None (initialization must complete before point award)

### Tasks

- [x] T006 [US1] Register `ScoreState` resource in plugin with init value `(current_score: 0, last_milestone_reached: 0)` in `src/systems/scoring.rs`
- [x] T007 [US1] Create `ScoreDisplayUi` marker component in `src/ui/score_display.rs` for tagging score display entity
- [x] T008 [US1] Implement `spawn_score_display_system` in `src/ui/score_display.rs` that spawns score text with `ScoreDisplayUi` marker at startup

---

## Phase 4: User Story 2 - Points on Brick Destruction (P1)

**Goal**: Brick destruction awards points matching documented brick values.

**Acceptance Criteria**:

- Score increases by correct amount for each brick type (10-57)
- Question brick (53) awards 25-300 random points
- Extra Ball brick (41) awards 0 points (grants ball instead)
- Magnet bricks (55-56) award 0 points
- Points accumulate correctly (score is sum of all destroyed bricks)

**Independent Test**: Destroy single brick of known type, verify score increase matches brick_points value

**Parallel Opportunities**: T010-T011 can run in parallel (utility function + event integration); T012-T014 must sequence

### Tasks

- [ ] T009 [US2] [P] Implement `brick_points(brick_type: BrickType) -> u32` function in `src/systems/scoring.rs` with complete match covering indices 10-57 (use docs/bricks.md for values)
- [ ] T010 [US2] [P] Implement `award_points_system` in `src/systems/scoring.rs` that reads `BrickDestroyed` events and mutates `ScoreState.current_score` using `brick_points()` function
- [ ] T011 [US2] Modify brick destruction logic in `src/systems/bricks/destruction.rs` to emit `BrickDestroyed` event before despawning (coordinate with existing destruction system)
- [ ] T012 [US2] Register `award_points_system` in plugin with label that runs after brick destruction events are emitted and before milestone detection
- [ ] T013 [US2] Write integration test in `tests/scoring.rs` that destroys various brick types and verifies score accumulation (Simple Stone 25pts, Multi-hit 50pts, Question brick 25-300 range)

---

## Phase 5: User Story 3 - Milestone Balls (P1)

**Goal**: Extra ball awarded every 5000 points.

**Acceptance Criteria**:

- At exactly 5000 points, `MilestoneReached` event emitted, lives incremented by 1
- At 10000 points, second `MilestoneReached` event emitted, lives incremented by 1
- Milestone detection happens exactly once per threshold (no duplicate events)
- Lives counter UI automatically updates (existing system)

**Independent Test**: Accumulate 5000+ points (via brick destruction), verify `LivesState.lives_remaining` incremented and ball appears

**Parallel Opportunities**: T015-T016 can run in parallel (detection + ball award systems)

### Tasks

- [ ] T014 [US3] [P] Implement `detect_milestone_system` in `src/systems/scoring.rs` that detects when `current_score / 5000 > last_milestone_reached`, emits `MilestoneReached` event, updates `last_milestone_reached`
- [ ] T015 [US3] [P] Implement `award_milestone_ball_system` in `src/systems/respawn.rs` that reads `MilestoneReached` events and increments `LivesState.lives_remaining` by 1
- [ ] T016 [US3] Register systems in plugin with ordering: `award_points_system` → `detect_milestone_system` → `award_milestone_ball_system`

---

## Phase 6: User Story 4 - Score Display Visibility (P2)

**Goal**: Score visible and updating in real-time on UI.

**Acceptance Criteria**:

- Score display updates within 16ms of point award (one frame at 60 FPS)
- Score shows current value without truncation
- Display persists throughout game (no visual obstruction)

**Independent Test**: Destroy bricks, verify score display updates immediately with correct value

**Parallel Opportunities**: None (depends on all prior systems)

### Tasks

- [ ] T017 [US4] Implement `update_score_display_system` in `src/ui/score_display.rs` with change-detection query `Query<&mut Text, (With<ScoreDisplayUi>, Changed<ScoreState>)>` that updates text content to display `score_state.current_score`
- [ ] T018 [US4] Register `update_score_display_system` in plugin with label that runs after `award_points_system`
- [ ] T019 [US4] Write integration test in `tests/score_display.rs` that verifies UI text updates within same frame as score change using Bevy test utilities

---

## Cross-Cutting: Code Quality & Documentation

*Implement alongside development, validate at end:*

- [ ] T020 Add rustdoc comments to all public types and functions (ScoreState, BrickDestroyed, MilestoneReached, brick_points, systems)
- [ ] T021 Run quality checks: `cargo fmt --all`, `cargo clippy --all-targets`, `cargo test`, `bevy lint`
- [ ] T022 Create CHANGELOG entry documenting scoring system feature

---

## Verification Checklist

Complete after all tasks finished:

- [ ] All tests pass: `cargo test`
- [ ] Format check: `cargo fmt --all -- --check`
- [ ] Lint check: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Bevy lint: `bevy lint`
- [ ] Manual test: Launch game, verify score starts at 0
- [ ] Manual test: Destroy brick, verify score increases correctly
- [ ] Manual test: Accumulate 5000 points, verify lives/balls increase
- [ ] Manual test: Score updates displayed in real-time
- [ ] Code coverage: All brick types (10-57) tested
- [ ] Performance: Score updates complete in <16ms per frame

---

## Implementation Strategy

### MVP (Must Have - All P1 Stories)

**Estimated Effort**: 4-6 hours of focused development

**Deliverable**: Players can see score initialization, earn points, and receive bonus balls at milestones.

**Task Sequence**:

1. Phase 1 (Setup) - 15 min
2. Phase 2 (Infrastructure) - 30 min
3. Phase 3 (Initialization) - 45 min
4. Phase 4 (Points) - 90 min (parallelizable sections)
5. Phase 5 (Milestones) - 60 min (parallelizable sections)
6. Code Quality - 30 min

**Total**: ~4 hours

### Phase 2 Stretch (Display Polish - P2)

**Estimated Effort**: +1-2 hours

**Deliverable**: Score display optimized for visibility and performance.

**Task Sequence**:

1. Phase 6 (Display UI) - 60 min
2. Additional testing - 30 min

**Cumulative Total**: ~5-6 hours

### Future Enhancements (Out of Scope)

- Multiplier bricks (26-29) - Deferred per clarification
- Score persistence to disk - Out of scope (in-memory ECS only)
- Score logging/metrics - Out of scope for MVP
- Leaderboards/achievements - Future feature

---

## Testing Strategy

### Unit Tests (Per Task)

- T009: Test `brick_points()` with all brick types returns correct values
- T010: Test `award_points_system` with various BrickDestroyed events
- T014: Test `detect_milestone_system` triggers at 5000, 10000, etc.

### Integration Tests (Per Story)

- **US1**: Start game, verify score resource initialized and accessible
- **US2**: Destroy bricks, verify cumulative score matches sum of brick values
- **US3**: Accumulate 5000+ points, verify LivesState increments and events emitted
- **US4**: Change score, verify UI text updates within one frame

### Manual Verification (End-to-End)

All test scenarios in [quickstart.md](quickstart.md) (Scenarios 1-7)

---

## Dependencies & Constraints

### External Dependencies

- Bevy 0.17.3 (global message system for events)
- bevy_rapier3d 0.32.0 (collision detection, triggers BrickDestroyed events)
- `GlobalRng` from Bevy (for Question brick randomness)

### Internal Dependencies

- **Existing Brick System**: Must emit `BrickDestroyed` events (T011 modifies this)
- **Existing Lives System** (`LivesState`): Must be readable/writable for milestone ball award (T015)
- **Existing Level Loader**: Must NOT reset `ScoreState` on level transition (verified in T006)

### Performance Constraints

- Score updates must complete in <16ms per frame (60 FPS target)
- UI change detection prevents unnecessary text updates
- No per-frame allocations for score display

### Correctness Constraints

- All brick types 10-57 must have correct point values per docs/bricks.md
- Question brick (53) must use uniform distribution 25-300
- Milestone detection must not emit duplicate events
- Score must persist across levels but reset on new game

---

## Success Metrics

### Completion Criteria (Definition of Done)

1. ✅ All 22 tasks completed
2. ✅ All tests passing (`cargo test`)
3. ✅ Code quality checks passing (fmt, clippy, bevy lint)
4. ✅ Manual verification scenarios 1-4 passing (from quickstart.md)
5. ✅ Rustdoc comments complete for all public APIs

### Performance Metrics

- Score display updates within 16ms of point award
- 60 FPS gameplay maintained during scoring events
- No memory leaks or unbounded allocations

### Feature Completeness

- ✅ User Story 1: Score initialization (P1)
- ✅ User Story 2: Points on destruction (P1)
- ✅ User Story 3: Milestone balls (P1)
- ✅ User Story 4: Display visibility (P2)

---

## Notes for Implementation

### Key Design Decisions (From Phase 0 Research)

1. **ScoreState Resource**: Follows LivesState pattern; supports change detection
2. **Brick Points Function**: Compile-time match for exhaustiveness; efficient lookup
3. **Event-Driven**: Brick destruction → award points → detect milestone → award ball
4. **Lives Integration**: Milestone triggers lives increment, not direct ball spawn
5. **UI Change Detection**: Prevents unnecessary frame updates; meets performance goal

### Common Pitfalls to Avoid

- ❌ Resetting ScoreState on level transition (should only reset on game restart)
- ❌ Emitting MilestoneReached multiple times per threshold (track last_milestone_reached)
- ❌ Forgetting to emit BrickDestroyed event before despawning brick
- ❌ Updating UI every frame instead of using change detection
- ❌ Incorrect point values for brick types (validate against docs/bricks.md)

### Testing Tips

- Use `BrickDestroyed` constructor to create test events
- Mock `GlobalRng` for deterministic Question brick testing (seeded RNG)
- Verify LivesState changes in integration tests (not just ScoreState)
- Check UI text content directly (query TextBundle entities)

---

## Appendix: File Tree Reference

```text
src/
├── systems/
│   ├── scoring.rs       # NEW - ScoreState, events, systems
│   └── bricks/
│       └── destruction.rs  # MODIFY - Emit BrickDestroyed
└── ui/
    └── score_display.rs # NEW - ScoreDisplayUi, UI systems

tests/
├── scoring.rs           # NEW - Unit & integration tests
└── score_display.rs     # NEW - UI update tests
```
