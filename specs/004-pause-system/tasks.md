# Tasks: Pause and Resume System

**Input**: Design documents from `/specs/004-pause-system/` **Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/ **Feature Branch**: `004-pause-system` **Date**: 2025-11-28

**Tests**: No explicit test requirements in specification.
Manual testing scenarios provided in quickstart.md.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create new module structure for pause system

- [x] T001 [P] Create `src/pause.rs` module file for pause system core logic
- [x] T002 [P] Create `src/ui/` directory for UI components
- [x] T003 [P] Create `src/ui/pause_overlay.rs` module file for overlay UI
- [x] T004 [P] Create `src/ui/mod.rs` to export pause_overlay module
- [x] T005 Add `pub mod pause;` declaration to `src/lib.rs`
- [x] T006 Add `pub mod ui;` declaration to `src/lib.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core pause state resource and plugin structure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 Define `PauseState` enum resource in `src/pause.rs` (Active | Paused with platform-specific window mode storage)
- [x] T008 Implement `Default` trait for `PauseState` in `src/pause.rs` (defaults to Active)
- [x] T009 Define `PausePlugin` struct in `src/pause.rs`
- [x] T010 Implement `Plugin` trait for `PausePlugin` in `src/pause.rs` (register resource, systems)
- [x] T011 Define `PauseOverlay` marker component in `src/ui/pause_overlay.rs`
- [x] T012 Add `PausePlugin` to app in `src/lib.rs` or `src/main.rs` (app.add_plugins(PausePlugin))

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Pause Game with ESC Key (Priority: P1) 🎯 MVP

**Goal**: Player can pause game by pressing ESC, freezing physics and displaying overlay

**Independent Test**: Run game, press ESC during gameplay, verify ball freezes and "PAUSED" overlay appears

### Implementation for User Story 1

- [x] T013 [US1] Implement `handle_pause_input` system in `src/pause.rs` (detects ESC key, transitions to Paused state, captures window mode on native)
- [x] T014 [US1] Implement `apply_pause_to_physics` system in `src/pause.rs` (sets RapierConfiguration::physics_pipeline_active based on PauseState)
- [x] T015 [US1] Implement `spawn_pause_overlay` function in `src/ui/pause_overlay.rs` (spawns UI entity with "PAUSED\\nClick to Resume" text)
- [x] T016 [US1] Register `handle_pause_input` system in `PausePlugin::build` in `src/pause.rs` (Update schedule)
- [x] T017 [US1] Register `apply_pause_to_physics` system in `PausePlugin::build` in `src/pause.rs` (Update schedule, after input handling)
- [x] T018 [US1] Register `spawn_pause_overlay` system in `PausePlugin::build` in `src/pause.rs` (Update schedule, run_if paused)
- [x] T019 [US1] Add run condition to `handle_pause_input` to check `LevelAdvanceState` in `src/pause.rs` (FR-012: prevent pause during level transitions)

**Manual Test (from quickstart.md)**:

- Run `cargo run`, start gameplay, press ESC
- Verify: Physics frozen, overlay visible
- Press ESC again → verify ignored (still paused)

**Checkpoint**: At this point, User Story 1 should be fully functional (pause works, physics freezes, overlay displays)

---

## Phase 4: User Story 2 - Resume Game with Screen Click (Priority: P1) 🎯 MVP

**Goal**: Player can resume game by clicking anywhere on screen, removing overlay and restarting physics

**Independent Test**: Manually pause game (ESC), click anywhere on screen, verify physics resumes and overlay disappears

### Implementation for User Story 2

- [x] T020 [US2] Implement `handle_resume_input` system in `src/pause.rs` (detects left mouse click, transitions to Active state when paused)
- [x] T021 [US2] Implement `despawn_pause_overlay` function in `src/ui/pause_overlay.rs` (queries and despawns entities with PauseOverlay marker)
- [x] T022 [US2] Register `handle_resume_input` system in `PausePlugin::build` in `src/pause.rs` (Update schedule)
- [x] T023 [US2] Register `despawn_pause_overlay` system in `PausePlugin::build` in `src/pause.rs` (Update schedule, run_if active)

**Manual Test (from quickstart.md)**:

- Pause game (ESC), click anywhere on screen
- Verify: Overlay disappears, physics resumes, ball continues from exact position
- Click on overlay text directly → verify also resumes
- With game active, click screen → verify no pause (ignored)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work (complete pause/resume cycle functional)

---

## Phase 5: User Story 3 - Window Mode Switching on Pause (Priority: P2)

**Goal**: When pausing from fullscreen, game switches to windowed mode; on resume, restores fullscreen (native only)

**Independent Test**: Run game in fullscreen, press ESC (verify windowed), click to resume (verify fullscreen restored)

### Implementation for User Story 3

- [x] T024 [US3] Implement `apply_pause_to_window_mode` system in `src/pause.rs` (switches window mode based on PauseState, native only via #[cfg])
- [x] T025 [US3] Add conditional compilation for native window mode switching in `apply_pause_to_window_mode` in `src/pause.rs` (#[cfg(not(target_arch = "wasm32"))])
- [x] T026 [US3] Add WASM no-op variant for `apply_pause_to_window_mode` in `src/pause.rs` (#[cfg(target_arch = "wasm32")])
- [x] T027 [US3] Register `apply_pause_to_window_mode` system in `PausePlugin::build` in `src/pause.rs` (Update schedule, after state change, before overlay)
- [x] T028 [US3] Add logic to restore window mode from `PauseState::Paused { window_mode_before_pause }` in `apply_pause_to_window_mode` in `src/pause.rs`
- [x] T029 [US3] Add logic to handle windowed→windowed case (no change per FR-010) in `apply_pause_to_window_mode` in `src/pause.rs`

**Manual Test (from quickstart.md - Native)**:

- Launch game (starts fullscreen), press ESC → verify switches to windowed
- Click to resume → verify switches back to fullscreen
- Manually switch to windowed before pausing, press ESC → verify stays windowed
- Pause from fullscreen, manually switch to fullscreen before clicking resume → verify respects manual change

**Manual Test (WASM)**:

- Launch game in browser, press ESC → verify window mode unchanged (stays windowed)
- Click to resume → verify window mode still unchanged

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Integration, edge case handling, documentation

- [x] T030 [P] Add system ordering constraints in `PausePlugin::build` in `src/pause.rs` (use .chain() to ensure: input → physics → window → UI)
- [x] T031 [P] Add graceful window mode switching failure handling in `apply_pause_to_window_mode` in `src/pause.rs` (FR-013: silently handle display limitations)
- [x] T032 Verify `just_pressed()` provides adequate debouncing for ESC key in `handle_pause_input` in `src/pause.rs` (FR-014: test rapid ESC presses)
- [x] T033 [P] Add inline documentation comments to `PauseState`, `PauseOverlay`, and all systems in `src/pause.rs` and `src/ui/pause_overlay.rs`
- [x] T034 [P] Update README.md with pause controls (ESC to pause, click to resume, keyboard/mouse only, native vs WASM differences)
- [ ] T035 Run full manual testing scenarios from `specs/004-pause-system/quickstart.md` (all user stories + edge cases)
- [x] T036 Run `cargo test` to verify no regressions in existing tests
- [x] T037 Run `cargo clippy --all-targets --all-features` to verify no linting issues
- [x] T038 Run `bevy lint` to verify Bevy-specific best practices (N/A - bevy lint not available as standalone command)
- [x] T039 [P] Test WASM build (`cargo build --target wasm32-unknown-unknown --release`) to verify platform compatibility
- [ ] T040 Verify 10 consecutive pause/resume cycles work without state corruption (SC-009 validation) - MANUAL TEST REQUIRED
- [ ] T041 Verify physics state preservation by pausing for 10+ seconds and checking ball position unchanged (SC-005 validation) - MANUAL TEST REQUIRED
- [x] T042 Update `.github/copilot-instructions.md` if not already updated by `update-agent-context.sh` script

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (US1 → US2 → US3)
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - Complements US1 but independently testable (can manually trigger pause without ESC)
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - Enhances US1/US2 but independently testable (window mode switching is separate concern)

### Within Each User Story

#### User Story 1 (Pause)

1. T013: Input handling (no dependencies)
2. T014: Physics control (no dependencies)
3. T015: UI overlay (no dependencies)
4. T016-T018: System registration (depends on T013-T015)
5. T019: Level transition blocking (depends on T016)

#### User Story 2 (Resume)

1. T020: Input handling (no dependencies within US2, but logically follows US1)
2. T021: UI cleanup (no dependencies)
3. T022-T023: System registration (depends on T020-T021)

#### User Story 3 (Window Mode)

1. T024: Window mode switching logic (no dependencies within US3)
2. T025-T026: Platform-specific compilation (depends on T024)
3. T027: System registration (depends on T024-T026)
4. T028-T029: Window mode restoration logic (depends on T027)

### Parallel Opportunities

#### Phase 1 (Setup) - All Parallel

- T001-T006 can all run in parallel (creating different files)

#### Phase 2 (Foundational) - Limited Parallelism

- T007-T008 (PauseState) must be sequential
- T009-T010 (PausePlugin) must be sequential after T007-T008
- T011 (PauseOverlay) can run parallel to T007-T010
- T012 (Plugin registration) must be last

#### Phase 3 (User Story 1) - Parallel Implementation

- T013 (input), T014 (physics), T015 (UI) can run in parallel
- T016-T018 (registration) must follow implementation
- T019 (blocking condition) must follow T016

#### Phase 4 (User Story 2) - Parallel Implementation

- T020 (input), T021 (UI) can run in parallel
- T022-T023 (registration) must follow implementation

#### Phase 5 (User Story 3) - Sequential within Story

- T024-T029 must be mostly sequential (building on window mode logic)

#### Phase 6 (Polish) - Many Parallel

- T030 (ordering), T031 (error handling), T033 (docs), T034 (README), T039 (WASM), T042 (copilot) can run in parallel
- T032, T035-T038, T040-T041 (testing/validation) can run in parallel after implementation complete

---

## Parallel Example: Phase 1 (Setup)

```bash
# Launch all setup tasks together:
Task: "Create src/pause.rs module file"
Task: "Create src/ui/ directory"
Task: "Create src/ui/pause_overlay.rs module file"
Task: "Create src/ui/mod.rs"
Task: "Add pub mod pause to src/lib.rs"
Task: "Add pub mod ui to src/lib.rs"
```

## Parallel Example: User Story 1 (Core Implementation)

```bash
# Launch core systems in parallel:
Task: "Implement handle_pause_input system in src/pause.rs"
Task: "Implement apply_pause_to_physics system in src/pause.rs"
Task: "Implement spawn_pause_overlay function in src/ui/pause_overlay.rs"

# Then register all systems (after implementation complete)
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 Only)

**Minimum Viable Product**: Complete pause/resume cycle

1. Complete Phase 1: Setup (T001-T006)
2. Complete Phase 2: Foundational (T007-T012) - CRITICAL blocker
3. Complete Phase 3: User Story 1 (T013-T019) - Pause functionality
4. Complete Phase 4: User Story 2 (T020-T023) - Resume functionality
5. **STOP and VALIDATE**: Test pause/resume cycle independently
6. Run basic testing: `cargo run`, press ESC (pause), click (resume), verify physics freeze/resume
7. MVP is ready - fully functional pause system (without window mode switching)

**MVP Scope**: 23 tasks (T001-T023)

### Incremental Delivery

1. **Foundation** (Setup + Foundational): T001-T012 → Foundation ready
2. **MVP** (US1 + US2): T013-T023 → Test independently → Deploy/Demo (pause/resume works!)
3. **Enhancement** (US3): T024-T029 → Test independently → Deploy/Demo (window mode switching added)
4. **Polish**: T030-T042 → Final validation → Production ready

Each increment adds value without breaking previous functionality.

### Parallel Team Strategy

With multiple developers:

1. **Team completes Setup + Foundational together** (T001-T012)
2. **Once Foundational is done (after T012)**:
   - Developer A: User Story 1 (T013-T019) - Pause
   - Developer B: User Story 2 (T020-T023) - Resume (can start parallel if familiar with codebase)
   - Developer C: User Story 3 (T024-T029) - Window Mode (can start parallel, independent concern)
3. **Integration checkpoint**: Verify all stories work together
4. **Polish together**: Divide Phase 6 tasks (many can run parallel)

**Note**: US2 depends logically on US1 (need pause before resume), but can be developed in parallel if developers coordinate on PauseState interface.

---

## Implementation Notes

### Run Conditions

Several systems use Bevy run conditions to optimize execution:

- `spawn_pause_overlay`: Only runs when entering Paused state
- `despawn_pause_overlay`: Only runs when entering Active state
- `handle_pause_input`: Blocked during level transitions (checks LevelAdvanceState)

Use `.run_if(resource_equals(PauseState::Paused))` or custom run condition functions.

### Platform-Specific Code

Window mode switching (US3) requires conditional compilation:

```rust
#[cfg(not(target_arch = "wasm32"))]
fn apply_pause_to_window_mode_native(/* ... */) { /* native implementation */ }

#[cfg(target_arch = "wasm32")]
fn apply_pause_to_window_mode_wasm() { /* no-op */ }
```

### System Ordering

Use `.chain()` in Phase 6 (T030) to ensure correct execution order:

```rust
app.add_systems(Update, (
    handle_pause_input,
    handle_resume_input,
    apply_pause_to_physics,
    apply_pause_to_window_mode,
    spawn_pause_overlay,
    despawn_pause_overlay,
).chain());
```

### Integration Points

- **LevelAdvanceState** (`src/level_loader.rs`): Check in `handle_pause_input` run condition (T019)
- **RapierConfiguration**: Mutate `physics_pipeline_active` field (T014)
- **Window**: Mutate `mode` field via `Single<&mut Window, With<PrimaryWindow>>` (T024-T029)

---

## Task Summary

**Total Tasks**: 42

**By Phase**:

- Phase 1 (Setup): 6 tasks
- Phase 2 (Foundational): 6 tasks
- Phase 3 (US1 - Pause): 7 tasks
- Phase 4 (US2 - Resume): 4 tasks
- Phase 5 (US3 - Window Mode): 6 tasks
- Phase 6 (Polish): 13 tasks

**By User Story**:

- User Story 1 (Pause): 7 tasks (T013-T019)
- User Story 2 (Resume): 4 tasks (T020-T023)
- User Story 3 (Window Mode): 6 tasks (T024-T029)

**Parallel Opportunities**:

- Phase 1: 6 parallel tasks (all setup)
- Phase 2: 1 parallel task (T011)
- Phase 3: 3 parallel tasks (T013-T015)
- Phase 4: 2 parallel tasks (T020-T021)
- Phase 6: 6 parallel tasks (T030, T031, T033, T034, T039, T042)

**MVP Scope**: 23 tasks (T001-T023) - Pause and resume functionality without window mode switching

**Independent Test Criteria**:

- **US1**: Press ESC → game freezes, overlay appears
- **US2**: Click screen → game resumes, overlay disappears
- **US3**: Pause from fullscreen → switches to windowed, resume → restores fullscreen (native only)

---

## Format Validation

✅ All tasks follow required format: `- [ ] [ID] [P?] [Story?] Description with file path` ✅ Task IDs sequential (T001-T042) ✅ [P] markers on parallelizable tasks (different files, no dependencies) ✅ labels on user story phase tasks (US1, US2, US3) ✅ File paths included in all implementation task descriptions ✅ Tasks organized by user story for independent implementation ✅ Clear checkpoints after each user story phase ✅ Dependencies documented in dedicated section ✅ Parallel execution examples provided

**Tasks file ready for implementation!**
