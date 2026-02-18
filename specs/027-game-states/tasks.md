---

description: "Task list for 027-game-states implementation"
---

# Tasks: Game States

**Input**: Design documents from /specs/027-game-states/ **Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/state-transitions.md, quickstart.md

**Tests**: Tests are MANDATORY for all user stories.
Each story MUST include unit tests and feature-level acceptance tests (integration or contract tests as appropriate).
Tests MUST be written and committed first, verified to FAIL (red), and then approved before implementation begins; record the failing-test commit hash in the task description.

**Bevy 0.17 compliance**: When generating tasks for ECS/rendering/UI work, include explicit tasks (or acceptance criteria within test tasks) to ensure compliance with the constitution’s Bevy 0.17 mandates & prohibitions (no panicking queries, filtered queries, Changed<T> for reactive UI, correct state transition usage, asset handle reuse, and correct hierarchy APIs).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create module files per plan in src/game_state.rs and src/systems/game_state_transitions.rs
- [x] T002 [P] Create UI module files in src/systems/ui/main_menu.rs and src/systems/ui/game_over.rs
- [x] T003 [P] Create test files tests/game_state_transitions.rs, tests/life_loss_flow.rs, tests/pause_state.rs, tests/main_menu.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

- [x] T004 Define GameState (States derive) and GameSession in src/game_state.rs
- [x] T005 Define StateTransitionContext (LifeLoss, LevelChange { target_level }, NewGame, ReturnToMenu) in src/game_state.rs
- [x] T006 Implement is_valid_transition helper (including LevelTransition) in src/systems/game_state_transitions.rs
- [x] T007 Register GameStatesPlugin with init_state, resources, and base schedules in src/game_state.rs
- [x] T008 Export GameStatesPlugin in src/lib.rs and register it in src/main.rs
- [x] T009 Add Bevy 0.17 compliance guardrails in state systems (fallible queries, With/Without filters, Changed<Interaction>) in src/systems/game_state_transitions.rs and src/systems/ui/*.rs

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Main Menu Navigation (Priority: P1) 🎯 MVP

**Goal**: Show Main Menu on launch with New Game and Quit buttons that transition or exit cleanly.

**Independent Test**: Launch app, verify MainMenu state active, press New Game to enter Playing, press Quit to exit.

### Tests for User Story 1 (REQUIRED) ⚠️

- [x] T010 [P] [US1] Add failing integration tests for MainMenu activation, New Game transition, and Quit exit in tests/main_menu.rs (record failing-test commit hash: TODO)
- [x] T011 [P] [US1] Add Bevy 0.17 compliance checks in tests/main_menu.rs (Changed<Interaction>, hierarchy safety on UI spawn) (record failing-test commit hash: TODO)

### Implementation for User Story 1

- [x] T012 [P] [US1] Implement spawn_main_menu and despawn_main_menu in src/systems/ui/main_menu.rs (use Node, Button, Text, and DespawnOnExit(GameState::MainMenu))
- [x] T013 [US1] Implement handle_main_menu_buttons in src/systems/ui/main_menu.rs using NextState<GameState> and AppExit
- [x] T014 [US1] Wire main menu systems into GameStatesPlugin in src/game_state.rs (OnEnter/OnExit + run_if)

**Checkpoint**: Main Menu flows are functional and independently testable

---

## Phase 4: User Story 2 - Active Gameplay (Priority: P1)

**Goal**: Playing state enables physics, input, collisions, and persists across frames.

**Independent Test**: Transition to Playing, verify physics and input run for 10+ frames.

### Tests for User Story 2 (REQUIRED) ⚠️

- [x] T015 [P] [US2] Add failing integration tests for Playing state system enablement and 10-frame persistence in tests/game_state_transitions.rs (record failing-test commit hash: TODO)

### Implementation for User Story 2

- [x] T016 [US2] Gate physics systems with run_if(in_state(GameState::Playing)) in src/main.rs or src/systems/mod.rs (per existing wiring)
- [x] T017 [US2] Gate input/collision systems to Playing in src/main.rs or src/systems/mod.rs; ensure no panicking queries

**Checkpoint**: Playing state enables gameplay only when active and persists across frames

---

## Phase 5: User Story 3 - Pause/Resume Gameplay (Priority: P1)

**Goal**: Pause freezes gameplay; resume restores state; pause only valid from Playing.

**Independent Test**: Pause from Playing, verify physics/input frozen; resume; invalid pause from MainMenu logs warning.

### Tests for User Story 3 (REQUIRED) ⚠️

- [x] T018 [P] [US3] Add failing tests for pause/resume behavior, invalid pause warnings, and 10-frame persistence in tests/pause_state.rs (record failing-test commit hash: TODO)

### Implementation for User Story 3

- [x] T019 [US3] Implement pause input handling with NextState<GameState> in src/pause.rs (or new system) with validation + warning logs
- [x] T020 [US3] Gate pause/resume input with in_state(GameState::Playing) / in_state(GameState::Paused) in src/main.rs or src/systems/mod.rs

**Checkpoint**: Pause/resume flows are functional and independently testable

---

## Phase 6: User Story 4 - Level Transition (Priority: P1)

**Goal**: FadeOut → LevelTransition → FadeIn → Playing, including next or previous level loads.

**Independent Test**: Trigger level change (next/previous), verify fade-out completes, level loads, fade-in completes, and old entities are cleaned up.

### Tests for User Story 4 (REQUIRED) ⚠️

- [x] T021 [P] [US4] Add failing tests for fade-out → level load → fade-in sequence and entity cleanup in tests/game_state_transitions.rs (record failing-test commit hash: TODO)
- [x] T022 [P] [US4] Add failing tests for previous-level navigation (brick 54) using LevelChange target_level in tests/game_state_transitions.rs (record failing-test commit hash: TODO)

### Implementation for User Story 4

- [x] T023 [US4] Implement FadeOverlay spawn/update systems with FadeTimer/FadeDirection in src/systems/game_state_transitions.rs
- [x] T024 [US4] Implement LevelTransition system to load target_level from StateTransitionContext in src/level_loader.rs (or new system) and clear previous level entities
- [x] T025 [US4] Wire FadeOut/LevelTransition/FadeIn schedules into GameStatesPlugin in src/game_state.rs

**Checkpoint**: Level transition sequence works for next and previous levels

---

## Phase 7: User Story 5 - Life Loss Handling (Priority: P1)

**Goal**: Life loss always enters FadeOut; lives check after fade decides FadeIn or GameOver.

**Independent Test**: With lives>0, ball loss triggers FadeOut→FadeIn with respawn; with lives=0, FadeOut→GameOver.

### Tests for User Story 5 (REQUIRED) ⚠️

- [x] T026 [P] [US5] Add failing tests for life-loss branching (FadeOut→FadeIn vs FadeOut→GameOver) in tests/life_loss_flow.rs (record failing-test commit hash: TODO)

### Implementation for User Story 5

- [x] T027 [US5] Trigger FadeOut with StateTransitionContext::LifeLoss on life-loss events (ball lost, paddle-brick 42/91, paddle-merkaba) in src/systems/game_state_transitions.rs (or integrate with existing handlers)
- [x] T028 [US5] Add OnEnter(GameState::FadeOut) system to despawn all merkabas and remaining balls in src/systems/game_state_transitions.rs
- [x] T029 [US5] Update check_fade_out_completion to branch on lives and set NextState (FadeIn/GameOver) in src/systems/game_state_transitions.rs
- [x] T030 [US5] Implement respawn logic and decrement lives in src/level_loader.rs or relevant respawn system

**Checkpoint**: Life loss flow behaves per spec with correct branching

---

## Phase 8: User Story 6 - Game Over State (Priority: P2)

**Goal**: Game Over disables gameplay and allows return to menu or new game.

**Independent Test**: Trigger GameOver, verify gameplay systems disabled; return to menu and start new game.

### Tests for User Story 6 (REQUIRED) ⚠️

- [x] T031 [P] [US6] Add failing tests for GameOver UI, disabled gameplay, and return-to-menu/new-game paths in tests/main_menu.rs (record failing-test commit hash: TODO)

### Implementation for User Story 6

- [x] T032 [P] [US6] Implement spawn_game_over and despawn_game_over in src/systems/ui/game_over.rs
- [x] T033 [US6] Implement GameOver UI interactions to set NextState and reset GameSession in src/systems/ui/game_over.rs
- [x] T034 [US6] Wire GameOver UI systems into GameStatesPlugin in src/game_state.rs (OnEnter/OnExit + run_if)

**Checkpoint**: GameOver flow is functional and independently testable

---

## Phase 9: Edge Cases & Polish

### Tests for Edge Cases (REQUIRED) ⚠️

- [x] T035 [P] [EC-001] Add failing tests for ignored transitions during FadeOut/FadeIn with warning logs in tests/game_state_transitions.rs (record failing-test commit hash: TODO)
- [x] T036 [P] [EC-002] Add failing tests for idempotent pause/resume across 3+ rapid requests in tests/pause_state.rs (record failing-test commit hash: TODO)
- [x] T037 [P] [EC-003] Add failing tests for deferred level-complete while Paused with trigger on resume in tests/game_state_transitions.rs (record failing-test commit hash: TODO)
- [x] T038 [P] [EC-004] Add failing tests for entity cleanup verification during LevelTransition with entity count assertions in tests/game_state_transitions.rs (record failing-test commit hash: TODO)
- [x] T039 [P] [EC-005] Add failing tests for invalid transition rejection with detailed error logs in tests/game_state_transitions.rs (record failing-test commit hash: TODO)

### Implementation for Edge Cases

- [x] T040 [EC-001] Implement transition guards in OnEnter(FadeOut) and OnEnter(FadeIn) to ignore NextState requests with warnings in src/systems/game_state_transitions.rs
- [x] T041 [EC-002] Ensure is_valid_transition enforces idempotence (Paused→Paused and Playing→Playing are no-ops) in src/systems/game_state_transitions.rs
- [x] T042 [EC-003] Implement deferred level-complete logic with flag in GameSession and trigger on Paused→Playing in src/systems/game_state_transitions.rs
- [x] T043 [EC-004] Implement OnExit(LevelTransition) system to despawn level entities with entity count logging in src/level_loader.rs or src/systems/game_state_transitions.rs
- [x] T044 [EC-005] Enhance is_valid_transition to return Result with error messages and log invalid attempts in src/systems/game_state_transitions.rs

### Cross-Cutting Concerns

- [ ] T045 [P] Update docs to align with States-based transitions (spec.md, plan.md, quickstart.md) if not already updated
- [ ] T046 Run cargo test, cargo clippy --all-targets --all-features, cargo fmt --all, bevy lint

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
- **Polish (Phase 9)**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: Can start after Foundational - No dependencies on other stories
- **US2 (P1)**: Can start after Foundational - Independent of US1
- **US3 (P1)**: Can start after Foundational - Independent of US1/US2
- **US4 (P1)**: Can start after Foundational - Independent but benefits from FadeOverlay utilities
- **US5 (P1)**: Can start after Foundational - Depends on FadeOut/FadeIn behavior (US4)
- **US6 (P2)**: Can start after Foundational - Independent UI layer

### Parallel Execution Examples

- US1 tests (T010–T011) in parallel with US2 tests (T015) and US3 tests (T018)
- UI implementation tasks T012 and T031 can run in parallel (different files)
- Fade systems (T023) can run in parallel with GameOver UI (T031)

---

## Implementation Strategy

- **MVP Scope**: User Story 1 (Main Menu) plus Foundational phase to reach Playable entry point
- **Incremental Delivery**: Implement each user story end-to-end (tests → implementation → verify) before moving to the next
- **Validation**: Each story must pass its independent test before proceeding

---
