---
description: "Task list for Level Navigation Bricks (Bricks 50 & 54)"
---

# Tasks: Level Navigation Bricks (Bricks 50 & 54)

**Input**: Design documents from `/specs/024-level-navigation-bricks/` **Prerequisites**: plan.md (required), spec.md, research.md, data-model.md, contracts/events.md, quickstart.md

**Tests**: Tests are MANDATORY for all user stories.
Each story MUST include unit tests and feature-level acceptance tests.
Tests MUST be written and committed first, verified to FAIL (red), and then approved before implementation begins; record the failing-test commit hash in each test task.

**Bevy 0.17 compliance**: When generating tasks for ECS/rendering/UI work, include explicit tasks (or acceptance criteria within test tasks) to ensure compliance with the constitution's Bevy 0.17 mandates & prohibitions (no panicking queries, filtered queries, `Changed<T>` for reactive UI, message vs event correctness, asset handle reuse, and correct hierarchy APIs).
For this feature, explicitly validate Message vs Event usage, idempotent level-loading behavior, and multi-frame persistence.

**Note**: No new test level files are needed.
Level 014 already exists and should be used for gameplay/manual verification.

---

## Phase 1: Setup (Shared Infrastructure)

- [x] T001 Confirm existing level 014 is used for manual testing (no new level files) in specs/024-level-navigation-bricks/quickstart.md

---

## Phase 2: Foundational (Blocking Prerequisites)

- [x] T002 Add brick type constants `BRICK_50` and `BRICK_54` in src/level_format/mod.rs
- [x] T003 Add `LevelSwitchSource::Brick` variant (if missing) in src/systems/level_switch.rs
- [x] T004 Add 0-point mappings for brick types 50 and 54 in src/systems/scoring.rs (function `brick_points`)

---

## Phase 3: User Story 1 - Level Up Brick (Brick 50) (Priority: P1) 🎯 MVP

**Goal**: Brick 50 advances to the next level on ball collision, awards 0 points, and shows victory screen when on final level.

**Independent Test**: Spawn a brick 50 and ball in a test app, trigger collision, and verify `LevelSwitchRequested` message (Next) plus `CurrentLevel` change; on final level, verify victory screen and no transition.
Must persist across 10+ frames.

### Tests for User Story 1 (REQUIRED) ⚠️

- [x] T005 [P] [US1] Add failing integration tests for brick 50 transitions in tests/brick_50_level_up.rs (record failing-test commit hash: RED_COMMIT_TBD)
  - Must assert `LevelSwitchRequested { source: Brick, direction: Next }` is emitted (MessageWriter/Reader usage, not observers)
  - Must assert `BrickDestroyed` message emitted with `brick_type = 50` and scoring awards 0 points
  - Must verify boundary: final level → victory screen + `GameProgress.finished = true`, no transition
  - Must verify multi-frame persistence: `CurrentLevel.number` unchanged across 10+ `app.update()` cycles after transition
  - Bevy 0.17 checks: no panicking queries, filtered queries, and Message-Event separation documented in test comments

### Implementation for User Story 1

- [ ] T006 [US1] Emit `LevelSwitchRequested` (Next) for brick 50 collisions in src/lib.rs (`mark_brick_on_ball_collision`), then mark brick for despawn
- [ ] T007 [US1] Handle final-level boundary for brick 50 in src/level_loader.rs (`process_level_switch_requests`) by spawning victory screen and setting `GameProgress.finished = true`

**Checkpoint**: Brick 50 story passes tests and is independently functional.

---

## Phase 4: User Story 2 - Level Down Brick (Brick 54) (Priority: P2)

**Goal**: Brick 54 returns to the previous level on ball collision, awards 0 points, and does nothing on level 1.

**Independent Test**: Spawn a brick 54 and ball in a test app, trigger collision, verify `LevelSwitchRequested` message (Previous) and `CurrentLevel` changes when level > 1; no transition on level 1.
Must persist across 10+ frames.

### Tests for User Story 2 (REQUIRED) ⚠️

- [x] T008 [P] [US2] Add failing integration tests for brick 54 transitions in tests/brick_54_level_down.rs (record failing-test commit hash: RED_COMMIT_TBD)
  - Must assert `LevelSwitchRequested { source: Brick, direction: Previous }` is emitted (MessageWriter/Reader usage, not observers)
  - Must assert `BrickDestroyed` message emitted with `brick_type = 54` and scoring awards 0 points
  - Must verify boundary: level 1 → no transition (brick destroyed, level remains 1)
  - Must verify multi-frame persistence: `CurrentLevel.number` unchanged across 10+ `app.update()` cycles after transition
  - Bevy 0.17 checks: no panicking queries, filtered queries, and Message-Event separation documented in test comments

### Implementation for User Story 2

- [ ] T009 [US2] Emit `LevelSwitchRequested` (Previous) for brick 54 collisions in src/lib.rs (`mark_brick_on_ball_collision`), then mark brick for despawn

**Checkpoint**: Brick 54 story passes tests and is independently functional.

---

## Phase 5: User Story 3 - Unique Audio Feedback for Navigation Bricks (Priority: P3)

**Goal**: Brick 50 and 54 play unique destruction sounds once; fallback to generic brick sound if assets missing.

**Independent Test**: Destroy brick 50 and 54 in a test app and assert distinct sound mapping via audio system hooks.

### Tests for User Story 3 (REQUIRED) ⚠️

- [x] T010 [P] [US3] Add failing audio mapping tests in tests/level_navigation_audio.rs (record failing-test commit hash: RED_COMMIT_TBD)
  - Must assert brick 50 maps to `SoundType::Brick50LevelUp`
  - Must assert brick 54 maps to `SoundType::Brick54LevelDown`
  - Must assert fallback to `SoundType::BrickDestroy` when audio assets missing
  - Bevy 0.17 checks: Message-Event separation (audio triggered by `BrickDestroyed` messages)

### Implementation for User Story 3

- [ ] T011 [US3] Add `SoundType::Brick50LevelUp` and `SoundType::Brick54LevelDown` variants and mapping in src/systems/audio.rs
- [ ] T012 [US3] Load optional audio assets for brick 50/54 (or confirm fallback path) in src/systems/audio.rs startup loader

**Checkpoint**: Unique audio feedback works independently of other stories.

---

## Phase 6: Polish & Cross-Cutting Concerns

- [ ] T013 [P] Update docs/ or README references if any mention brick type listings (paths: docs/ or README.md as applicable)
- [ ] T014 Run quickstart validation steps and ensure level 014 is referenced for manual testing in specs/024-level-navigation-bricks/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: Depend on Foundational phase completion; implement in priority order (US1 → US2 → US3)
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: Depends on Foundational tasks T002–T004
- **US2 (P2)**: Depends on US1 completion for shared collision logic in src/lib.rs
- **US3 (P3)**: Depends on Foundational tasks T002–T004; can run after US1 if desired

### Within Each User Story

- Tests MUST be written first and verified to FAIL before implementation begins; record failing-test commit hash in the task description.
- Tests MUST be approved before implementation proceeds.
- Implementations MUST comply with Bevy 0.17 mandates: fallible systems, no panicking queries, filtered queries, correct Message vs Event usage, asset handle reuse, and hierarchy safety.

---

## Parallel Execution Examples

### User Story 1

- T005 (tests) can run in parallel with documentation prep tasks (T001) in different files.

### User Story 2

- T008 (tests) can run in parallel with US3 tests (T010) once Foundational tasks are complete.

### User Story 3

- T010 (tests) can run in parallel with US2 tests (T008) after Foundational completion.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Setup (T001)
2. Complete Foundational tasks (T002–T004)
3. Complete US1 tests (T005) → fail → get approval
4. Implement US1 (T006–T007) → pass tests
5. Stop and validate MVP

### Incremental Delivery

1. US1 complete → validate
2. US2 tests (T008) → implement (T009) → validate
3. US3 tests (T010) → implement (T011–T012) → validate
4. Polish tasks (T013–T014)

---

## Notes

- [P] tasks = different files, no dependencies
- Tests are mandatory and must be committed before implementation
- Level 014 exists for manual gameplay verification; no new level files are required
- Record failing-test commit hashes directly in T005/T008/T010 task descriptions
- All paths are repository-relative and should match the project structure in plan.md
