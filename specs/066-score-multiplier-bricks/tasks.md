# Tasks: Score Multiplier Bricks

**Input**: Design documents from `/specs/066-score-multiplier-bricks/` **Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md, contracts/gameplay-scoring-contract.md

**Tests**: Tests are mandatory for every user story.
Write tests first, verify they fail, and record the failing-test commit hash in the task notes or commit message before implementation starts.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare focused test surfaces for multiplier scoring and HUD behavior.

- [X] T001 Create feature integration test scaffold in /home/christian/devel/bevy/brkrs/tests/score_multiplier_bricks.rs for multiplier activation, replacement, reset, and level-transition scenarios
- [X] T002 [P] Extend UI test harness coverage in /home/christian/devel/bevy/brkrs/tests/score_display.rs and /home/christian/devel/bevy/brkrs/tests/change_detection.rs for multiplier-indicator assertions

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish shared multiplier state and app wiring that all stories depend on.

**⚠️ CRITICAL**: No user story work should begin until this phase is complete.

- [X] T003 Define shared `ScoreMultiplierState` resource, multiplier mapping helpers, and scoring helper APIs in /home/christian/devel/bevy/brkrs/src/systems/scoring.rs
- [X] T004 [P] Register multiplier state and scoring pipeline wiring in /home/christian/devel/bevy/brkrs/src/lib.rs so buffered `BrickDestroyed` message flow remains the single scoring entry path
- [X] T005 [P] Add shared UI component markers and layout anchors for the future multiplier indicator in /home/christian/devel/bevy/brkrs/src/ui/score_display.rs and /home/christian/devel/bevy/brkrs/src/ui/mod.rs

**Checkpoint**: Shared multiplier state and wiring are in place; story work can proceed.

---

## Phase 3: User Story 1 - Activate Multiplier on Brick Hit (Priority: P1) 🎯 MVP

**Goal**: Hitting bricks 27-29 activates a forward-only multiplier that scales later brick-destruction points.

**Independent Test**: Hit a multiplier brick, destroy a known-value brick, and verify the second brick's score uses the active factor while the triggering multiplier brick keeps its base score.

### Tests for User Story 1 (REQUIRED) ⚠️

- [X] T006 [P] [US1] Write failing unit tests for multiplier factor mapping, forward-only activation, and base-score handling of triggering bricks in /home/christian/devel/bevy/brkrs/tests/scoring.rs (record red commit hash)
- [X] T007 [P] [US1] Write failing integration tests for multiplier activation, buffered message consumption, and 10-frame persistence in /home/christian/devel/bevy/brkrs/tests/score_multiplier_bricks.rs (record red commit hash)
- [ ] T023 [US1] Obtain feature-owner approval for failing multiplier activation tests before implementation begins

### Implementation for User Story 1

- [X] T008 [US1] Implement multiplier activation and forward-only brick award calculation in /home/christian/devel/bevy/brkrs/src/systems/scoring.rs
- [X] T009 [US1] Chain the updated multiplier-aware scoring systems after brick destruction in /home/christian/devel/bevy/brkrs/src/lib.rs and preserve `MessageReader<BrickDestroyed>`-based scoring flow

**Checkpoint**: User Story 1 should support multiplier activation and multiplied follow-up brick scoring.

---

## Phase 4: User Story 2 - Replace Existing Multiplier with New One (Priority: P1)

**Goal**: The newest multiplier brick hit replaces the previously active multiplier, including explicit reset via brick 26.

**Independent Test**: Activate one multiplier, hit a different multiplier brick, and verify only the most recent factor applies to the next scored brick.

### Tests for User Story 2 (REQUIRED) ⚠️

- [X] T010 [P] [US2] Write failing unit tests for latest-hit-wins replacement semantics and brick-26 explicit reset in /home/christian/devel/bevy/brkrs/tests/scoring.rs (record red commit hash)
- [X] T011 [P] [US2] Write failing integration tests for sequential multiplier-brick hits and single-active-factor behavior in /home/christian/devel/bevy/brkrs/tests/score_multiplier_bricks.rs (record red commit hash)
- [ ] T024 [US2] Obtain feature-owner approval for failing replacement tests before implementation begins

### Implementation for User Story 2

- [X] T012 [US2] Implement replacement semantics for bricks 26-29 and enforce exactly one active factor at a time in /home/christian/devel/bevy/brkrs/src/systems/scoring.rs

**Checkpoint**: User Story 2 should deterministically replace the active multiplier with the most recent multiplier brick.

---

## Phase 5: User Story 3 - Reset Multiplier on Life Loss (Priority: P1)

**Goal**: Multiplier resets only when lives actually decrement, while non-life ball despawns and life-free level transitions preserve multiplier state.

**Independent Test**: Activate a multiplier, trigger life loss, then verify normal scoring resumes; separately verify non-life ball despawn and level transition keep the multiplier.

### Tests for User Story 3 (REQUIRED) ⚠️

- [X] T013 [P] [US3] Write failing reset-path tests for life-decrement-only semantics in /home/christian/devel/bevy/brkrs/tests/life_loss_flow.rs and /home/christian/devel/bevy/brkrs/tests/scoring.rs (record red commit hash)
- [X] T014 [P] [US3] Write failing integration tests for multi-ball non-reset behavior, level-transition persistence, and message-boundary compliance in /home/christian/devel/bevy/brkrs/tests/score_multiplier_bricks.rs (record red commit hash)
- [ ] T025 [US3] Obtain feature-owner approval for failing life-loss reset tests before implementation begins

### Implementation for User Story 3

- [X] T015 [US3] Implement multiplier reset on actual life decrement in /home/christian/devel/bevy/brkrs/src/systems/scoring.rs and /home/christian/devel/bevy/brkrs/src/systems/game_state_transitions.rs
- [X] T016 [US3] Preserve multiplier state across non-life ball despawns and level transitions in /home/christian/devel/bevy/brkrs/src/systems/scoring.rs and /home/christian/devel/bevy/brkrs/src/game_state.rs

**Checkpoint**: User Story 3 should reset multiplier only on real life loss and preserve it otherwise.

---

## Phase 6: User Story 4 - Show Active Multiplier Indicator (Priority: P2)

**Goal**: Display `x2`, `x3`, or `x4` beneath the score indicator when active, and hide the indicator at `1x`.

**Independent Test**: Activate each multiplier and verify the HUD shows the matching text beneath score; reset to `1x` and verify the indicator disappears without per-frame churn.

### Tests for User Story 4 (REQUIRED) ⚠️

- [X] T017 [P] [US4] Write failing unit tests for multiplier indicator text and visibility updates in /home/christian/devel/bevy/brkrs/tests/score_display.rs (record red commit hash)
- [X] T018 [P] [US4] Write failing integration and change-detection tests for hidden-at-1x behavior and stable multi-frame UI updates in /home/christian/devel/bevy/brkrs/tests/change_detection.rs and /home/christian/devel/bevy/brkrs/tests/score_display.rs (record red commit hash)
- [ ] T026 [US4] Obtain feature-owner approval for failing multiplier indicator tests before implementation begins
- [X] T028 [US4] Add hierarchy-safety verification for multiplier indicator spawn/update in /home/christian/devel/bevy/brkrs/tests/ui_compliance_audit.rs to confirm compliant UI relationship handling without manual `Parent` or `Children` mutation

### Implementation for User Story 4

- [X] T019 [US4] Implement multiplier indicator components, spawn layout beneath the score display, and text/visibility updates in /home/christian/devel/bevy/brkrs/src/ui/score_display.rs
- [X] T020 [US4] Wire multiplier indicator update scheduling with `Changed<T>`-style gating in /home/christian/devel/bevy/brkrs/src/ui/mod.rs and /home/christian/devel/bevy/brkrs/src/ui/score_display.rs

**Checkpoint**: User Story 4 should expose the active multiplier in the HUD and hide it at 1x.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final regression coverage, documentation, and validation across all stories.

- [X] T021 [P] Update multiplier brick and HUD indicator documentation in /home/christian/devel/bevy/brkrs/docs/bricks.md
- [X] T022 Run quickstart validation and repository verification commands from /home/christian/devel/bevy/brkrs/specs/066-score-multiplier-bricks/quickstart.md
- [X] T027 [P] Run WASM validation for score multiplier bricks with `cargo build --target wasm32-unknown-unknown`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup**: No dependencies.
- **Phase 2: Foundational**: Depends on Phase 1 and blocks all user stories.
- **Phase 3: US1**: Depends on Phase 2.
- **Phase 4: US2**: Depends on Phase 2 and builds on the multiplier state introduced for US1.
- **Phase 5: US3**: Depends on Phase 2 and the shared multiplier state from US1.
- **Phase 6: US4**: Depends on Phase 2 and can proceed after multiplier state is available.
- **Phase 7: Polish**: Depends on completion of all desired user stories.

### User Story Dependencies

- **US1 (P1)**: First MVP slice; no dependency on later stories.
- **US2 (P1)**: Requires shared multiplier state but is otherwise independently testable once implemented.
- **US3 (P1)**: Requires shared multiplier state and life-loss control path wiring; independently testable after implementation.
- **US4 (P2)**: Requires shared multiplier state and score UI surfaces; independently testable once the UI indicator is wired.

### Within Each User Story

- Tests first, confirmed failing, red commit hash recorded, and feature-owner approval obtained before implementation.
- Maintain Message/Event separation: use buffered messages for scoring/life streams and avoid observer-only scoring/reset shortcuts.
- Use filtered queries and change detection for HUD updates; no unconditional per-frame text rewrites.
- Complete implementation before moving to cross-story polish.

### Parallel Opportunities

- T001 and T002 can run in parallel.
- T004 and T005 can run in parallel after T003.
- Within each story, unit and integration test tasks marked `[P]` can run in parallel.
- US4 can begin after foundational work even if US2/US3 are still in progress, provided multiplier state contracts are stable.

---

## Parallel Example: User Story 1

```bash
# Run the red-first tests for User Story 1 in parallel:
Task: "Write failing unit tests for multiplier factor mapping, forward-only activation, and base-score handling of triggering bricks in tests/scoring.rs"
Task: "Write failing integration tests for multiplier activation, buffered message consumption, and 10-frame persistence in tests/score_multiplier_bricks.rs"
```

## Parallel Example: User Story 4

```bash
# Run the UI-focused red-first tests in parallel:
Task: "Write failing unit tests for multiplier indicator text and visibility updates in tests/score_display.rs"
Task: "Write failing integration and change-detection tests for hidden-at-1x behavior and stable multi-frame UI updates in tests/change_detection.rs and tests/score_display.rs"
```

---

## Implementation Strategy

### MVP First (US1)

1. Complete Setup and Foundational phases.
2. Deliver US1 multiplier activation and forward-only scoring.
3. Validate with red/green test history before continuing.

### Incremental Delivery

1. Add US1 for base multiplier behavior.
2. Add US2 for replacement semantics.
3. Add US3 for life-loss reset and persistence rules.
4. Add US4 for HUD feedback.
5. Finish with documentation and full quickstart validation.

### Team Strategy

1. One developer completes T003-T005 foundational work.
2. A second developer can prepare US1/US2 tests while foundational work lands.
3. UI work for US4 can proceed in parallel with US3 once `ScoreMultiplierState` is stable.
