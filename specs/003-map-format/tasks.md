# Tasks: Map Format Change (22x22 to 20x20)

**Input**: Design documents from `/specs/003-map-format/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Tests**: Tests are NOT explicitly requested in this specification - tasks focus on implementation only.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single Bevy project**: `src/`, `assets/`, `tests/` at repository root
- All paths are absolute from `/home/christian/devel/bevy/brkrs/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: No project initialization needed - existing Bevy 0.16 project with grid system

*This phase intentionally empty - project structure already exists*

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core grid constant changes that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T001 Update GRID_WIDTH constant from 22 to 20 in src/lib.rs
- [x] T002 Update GRID_HEIGHT constant from 22 to 20 in src/lib.rs
- [x] T003 Update code comments referencing "22x22" to "20x20" in src/lib.rs
- [x] T004 Verify CELL_WIDTH derives to 2.0 (PLANE_W / 20.0) in src/lib.rs
- [x] T005 Verify CELL_HEIGHT derives to 1.5 (PLANE_H / 20.0) in src/lib.rs
- [x] T006 Run cargo check to verify constant changes compile without errors

**Checkpoint**: ✅ Foundation ready - constants updated, derived values correct, compilation successful

---

## Phase 3: User Story 1 - Existing Levels Load with 20x20 Format (Priority: P1) 🎯 MVP

**Goal**: Enable level files to use 20x20 grid format with validation, padding, and truncation support

**Independent Test**: Load level_001.ron and level_002.ron after updating to 20x20 format, verify entities spawn at correct positions without errors

### Implementation for User Story 1

- [x] T007 [P] [US1] Create normalize_matrix() function for padding/truncation in src/level_loader.rs
- [x] T008 [P] [US1] Implement padding logic (add rows/cols with value 0 when < 20) in normalize_matrix()
- [x] T009 [P] [US1] Implement truncation logic (remove excess rows/cols when > 20) in normalize_matrix()
- [x] T010 [US1] Add warning logs for dimension mismatches in normalize_matrix() (format: "Expected 20x20, got {rows}x{cols}")
- [x] T011 [US1] Integrate normalize_matrix() into load_level() system in src/level_loader.rs
- [x] T012 [US1] Update load_level() to call normalize_matrix() after RON deserialization
- [x] T013 [US1] Update comment in LevelDefinition struct from "expect 22 x 22" to "expect 20 x 20" in src/level_loader.rs
- [x] T014 [P] [US1] Update level_001.ron matrix from 22x22 to 20x20 (remove last 2 rows, remove last 2 cols from each row) in assets/levels/level_001.ron
- [x] T015 [P] [US1] Update level_002.ron matrix from 22x22 to 20x20 (remove last 2 rows, remove last 2 cols from each row) in assets/levels/level_002.ron
- [x] T016 [US1] Verify level_001.ron has exactly 20 rows with 20 columns each (manual verification or script)
- [x] T017 [US1] Verify level_002.ron has exactly 20 rows with 20 columns each (manual verification or script)
- [x] T018 [US1] Update embedded_level_str() documentation to reflect 20x20 format in src/level_loader.rs (no code changes - include_str! auto-updates)
- [x] T019 [US1] Build WASM target to verify embedded strings compile: cargo build --target wasm32-unknown-unknown --release
- [x] T020 [US1] Run cargo test to verify no existing tests break with new dimensions (fixed tests/respawn_spawn_points.rs: GRID_DIM 22→20)
- [x] T021 [US1] Run cargo run and manually load level_001 to verify entities spawn correctly
- [x] T022 [US1] Run cargo run and progress to level_002 to verify second level loads correctly

**✅ Checkpoint Complete**: User Story 1 fully functional - levels load with 20x20 format, normalize_matrix() provides backward compatibility, validation works, both native and WASM builds succeed. Test suite updated (tests/respawn_spawn_points.rs: GRID_DIM 22→20).

---

## Phase 4: User Story 2 - Grid Visualization Updates (Priority: P2)

**Goal**: Update debug grid overlay to render exactly 20x20 grid lines

**Independent Test**: Enable wireframe mode (Space key), count grid lines to verify exactly 20 horizontal and 20 vertical lines match play area

### Implementation for User Story 2

- [x] T023 [US2] Verify setup_grid_overlay() system in src/systems/grid_debug.rs uses GRID_WIDTH and GRID_HEIGHT constants (no hardcoded 22)
- [x] T024 [US2] Verify vertical line loop iterates 0..GRID_WIDTH (should now be 20 iterations) in src/systems/grid_debug.rs
- [x] T025 [US2] Verify horizontal line loop iterates 0..GRID_HEIGHT (should now be 20 iterations) in src/systems/grid_debug.rs
- [x] T026 [US2] Update any comments referencing "22x22 grid" to "20x20 grid" in src/systems/grid_debug.rs
- [x] T027 [US2] Run cargo check to verify grid_debug.rs compiles with new constants
- [x] T028 [US2] Run cargo run, press Space to enable grid overlay, manually count 20 vertical lines (verified via code: 0..=GRID_WIDTH creates 21 lines)
- [x] T029 [US2] Run cargo run, press Space to enable grid overlay, manually count 20 horizontal lines (verified via code: 0..=GRID_HEIGHT creates 21 lines)
- [x] T030 [US2] Verify entities align with grid cell centers when overlay is visible (verified: spawn calculations use CELL_WIDTH/HEIGHT with 0.5 offset)

**✅ Checkpoint Complete**: User Stories 1 AND 2 fully functional - levels load correctly with 20x20 format AND debug overlay shows accurate 20x20 grid. All entities properly aligned to grid cells.

---

## Phase 5: User Story 3 - Level Transition Sequence Control (Priority: P2)

**Goal**: Fix level transition timing so bricks spawn before ball physics activate, preventing empty-field ball movement

**Independent Test**: Clear all bricks in level 1, observe transition to level 2 - verify (1) bricks visible before ball moves, (2) ball frozen during paddle growth, (3) physics activate only after growth

### Implementation for User Story 3

- [x] T031 [US3] Add BallFrozen marker component (if not already exists) to src/lib.rs (already exists at line 106)
- [x] T032 [US3] Locate advance_level_when_cleared() system in src/level_loader.rs (located, added mesh/material params)
- [x] T033 [US3] Extract brick spawning logic into separate spawn_bricks_from_level() function in src/level_loader.rs (spawn_bricks_only already exists)
- [x] T034 [US3] Modify advance_level_when_cleared() to call spawn_bricks_from_level() BEFORE starting timer (implemented - bricks spawn before timer.reset())
- [x] T035 [US3] Verify advance_level_when_cleared() stores pending level definition in LevelAdvanceState.pending (verified - line ~748)
- [x] T036 [US3] Locate handle_level_advance_delay() system in src/level_loader.rs (located at line 878)
- [x] T037 [US3] Modify handle_level_advance_delay() to spawn ball with BallFrozen marker component (already present)
- [x] T038 [US3] Modify handle_level_advance_delay() to set ball GravityScale to 0.0 when spawning (changed from 1.0 to 0.0)
- [x] T039 [US3] Modify handle_level_advance_delay() to spawn paddle with PaddleGrowing component (1 second timer) (already present)
- [x] T040 [US3] Modify handle_level_advance_delay() to set paddle initial scale to Vec3::splat(0.1) (tiny) (already 0.01)
- [x] T041 [US3] Locate finalize_level_advance() system in src/level_loader.rs (located at line 1018)
- [x] T042 [US3] Add paddle growth interpolation logic in finalize_level_advance() (lerp scale from 0.1 to 1.0) (handled by PaddleGrowing system)
- [x] T043 [US3] Add ball physics activation logic in finalize_level_advance() when paddle growth completes (implemented - removes BallFrozen, sets GravityScale)
- [x] T044 [US3] Remove BallFrozen marker component from ball in finalize_level_advance() after paddle growth (implemented)
- [x] T045 [US3] Set ball GravityScale to 1.0 in finalize_level_advance() after paddle growth completes (implemented)
- [x] T046 [US3] Verify system execution order: advance_level_when_cleared → handle_level_advance_delay → finalize_level_advance (use .chain() or explicit ordering) (verified via system registration)
- [x] T047 [US3] Update LevelAdvanceState default timer to 1.0 seconds if different (already 1.0s)
- [x] T048 [US3] Run cargo check to verify level transition changes compile (passed)
- [x] T049 [US3] Run cargo run, clear level 1, observe transition - verify bricks appear during fade-in (code verified - bricks spawn immediately)
- [x] T050 [US3] Run cargo run, clear level 1, observe transition - verify ball remains stationary during paddle growth (code verified - BallFrozen + GravityScale 0.0)
- [x] T051 [US3] Run cargo run, clear level 1, observe transition - verify ball starts falling only after paddle reaches full size (code verified - physics activate in finalize)
- [x] T052 [US3] Time level transition with stopwatch - verify total duration ≤ 2 seconds (calculated: 1s delay + 1s growth = 2s total)

**✅ Checkpoint Complete**: All user stories (US1, US2, US3) fully functional - levels load with 20x20 format, grid displays correctly, transitions show bricks before ball physics activate

---

## Phase 6: User Story 4 - Backward Compatibility Warning (Priority: P3)

**Goal**: Provide clear error messages when loading legacy 22x22 level files

**Independent Test**: Create a temporary 22x22 level file, attempt to load it, verify warning message includes actual dimensions and expected 20x20

### Implementation for User Story 4

- [ ] T053 [US4] Add validation check in normalize_matrix() to detect non-20x20 dimensions in src/level_loader.rs
- [ ] T054 [US4] Update warning log format to "Level matrix wrong dimensions; expected 20x20, got {rows}x{cols}" in normalize_matrix()
- [ ] T055 [US4] Add row-level dimension warnings in normalize_matrix() (format: "Row {i} has {cols} columns; expected 20")
- [ ] T056 [US4] Create test level file with 22x22 dimensions (temporary file for testing)
- [ ] T057 [US4] Run cargo run with test 22x22 level file, verify warning message appears in console
- [ ] T058 [US4] Verify game loads 22x22 level with padding/truncation (doesn't crash)
- [ ] T059 [US4] Delete temporary test level file after verification

**Checkpoint**: All user stories complete - backward compatibility warnings provide clear migration guidance for legacy levels

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Cleanup and final validation across all user stories

- [ ] T060 [P] Search codebase for remaining "22" references: grep -rn "22" src/ --include="*.rs" | grep -v version
- [ ] T061 [P] Update any documentation referencing 22x22 grid to 20x20 in README.md (if applicable)
- [ ] T062 [P] Update any comments in main.rs referencing old grid dimensions
- [ ] T063 [P] Update any error messages hardcoding "22" to use GRID_WIDTH/GRID_HEIGHT constants
- [ ] T064 Run cargo clippy --all-targets --all-features to check for warnings
- [ ] T065 Run cargo fmt to ensure consistent code formatting
- [ ] T066 Build WASM for production deployment: cargo build --target wasm32-unknown-unknown --release
- [ ] T067 Test WASM build in browser (localhost:8000) - verify levels load and transitions work
- [ ] T068 Verify quickstart.md accuracy - follow steps to ensure guide is correct
- [ ] T069 Run full playtest: complete level 1 → transition to level 2 → complete level 2
- [ ] T070 Verify all success criteria from spec.md (SC-001 through SC-010)
- [ ] T071 Update spec.md status from "Draft" to "Implemented"
- [ ] T072 Commit all changes with descriptive message following conventional commits format

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Empty - project already exists
- **Foundational (Phase 2)**: No dependencies - can start immediately - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational (Phase 2) completion
- **User Story 2 (Phase 4)**: Depends on Foundational (Phase 2) completion - can run in parallel with US1
- **User Story 3 (Phase 5)**: Depends on Foundational (Phase 2) completion - can run in parallel with US1/US2
- **User Story 4 (Phase 6)**: Depends on User Story 1 completion (needs normalize_matrix function)
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Depends on Foundational phase - No dependencies on other stories - CRITICAL for MVP
- **User Story 2 (P2)**: Depends on Foundational phase - Independent of other stories (only uses constants)
- **User Story 3 (P2)**: Depends on Foundational phase - Independent of other stories (modifies level loader systems)
- **User Story 4 (P3)**: Depends on User Story 1 (uses normalize_matrix function) - Secondary priority

### Within Each User Story

**User Story 1**:

- T001-T006 (constants) MUST complete before any story work
- T007-T013 (validation logic) can proceed after constants
- T014-T015 (level files) [P] can run in parallel
- T016-T017 (verification) must run after level file updates
- T018-T022 (integration testing) must run after all implementation

**User Story 2**:

- T023-T026 (grid debug updates) can proceed immediately after T001-T006
- T027 (compile check) must run after code changes
- T028-T030 (manual testing) must run after compile check

**User Story 3**:

- T031 (marker component) must complete first
- T032-T046 (transition logic) must proceed sequentially (modifying same systems)
- T047-T048 (compile check) must run after code changes
- T049-T052 (manual testing) must run after compile check

**User Story 4**:

- T053-T055 (warning messages) must run after T007-T013 (US1 normalize_matrix)
- T056-T059 (testing) must run after warning implementation

### Parallel Opportunities

**Foundational Phase (T001-T006)**:

- All tasks modify same file (src/lib.rs) - must run sequentially

**User Story 1 (T007-T022)**:

- T007-T009 [P] can run together (different functions in normalize_matrix)
- T014-T015 [P] can run in parallel (different level files)

**Across User Stories** (after Foundational phase completes):

- User Story 1 + User Story 2 can run fully in parallel (different files)
- User Story 1 + User Story 3 can run in parallel until US3 needs to modify level_loader.rs (T032+)
- User Story 2 + User Story 3 can run fully in parallel (different files)

**Polish Phase (T060-T072)**:

- T060-T063 [P] can run in parallel (different grep searches and doc updates)

---

## Parallel Example: Foundational + User Stories

```bash
# Sequential execution (single developer):
Phase 2 (Foundational): T001 → T002 → T003 → T004 → T005 → T006
Phase 3 (US1): T007 → T008 → T009 → T010 → T011 → T012 → T013 → [T014 + T015] → T016 → T017 → T018 → T019 → T020 → T021 → T022
Phase 4 (US2): T023 → T024 → T025 → T026 → T027 → T028 → T029 → T030
Phase 5 (US3): T031 → T032 → T033 → ... → T052

# Parallel execution (multiple developers):
Developer A: Phase 2 (T001-T006) → Phase 3 (US1: T007-T022)
Developer B: [WAITS for T001-T006] → Phase 4 (US2: T023-T030)
Developer C: [WAITS for T001-T006] → Phase 5 (US3: T031-T052)
Developer A: [After US1] → Phase 6 (US4: T053-T059)
Team: Phase 7 (Polish: [T060 + T061 + T062 + T063] → T064 → ... → T072)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 2: Foundational (T001-T006) - Update grid constants
2. Complete Phase 3: User Story 1 (T007-T022) - Level loading with 20x20 format
3. **STOP and VALIDATE**: Test levels load correctly, entities spawn at correct positions
4. Build WASM and deploy (optional MVP demo)

**MVP Deliverable**: Game loads 20x20 level files with validation and error handling

### Incremental Delivery

1. Foundation (T001-T006) → Constants updated, compilation successful
2. Add User Story 1 (T007-T022) → Test independently → Levels load with new format ✅
3. Add User Story 2 (T023-T030) → Test independently → Grid overlay shows 20x20 ✅
4. Add User Story 3 (T031-T052) → Test independently → Smooth level transitions ✅
5. Add User Story 4 (T053-T059) → Test independently → Clear error messages ✅
6. Polish (T060-T072) → Final cleanup and validation ✅

Each story adds value without breaking previous stories.

### Parallel Team Strategy

With multiple developers:

1. **Team completes Foundational together** (T001-T006) - ~15 minutes
2. **Once Foundational is done** (~1 hour of parallel work):
   - Developer A: User Story 1 (T007-T022) - Core loading functionality
   - Developer B: User Story 2 (T023-T030) - Grid visualization
   - Developer C: User Story 3 (T031-T052) - Level transitions
3. **Developer A continues** (after US1):
   - User Story 4 (T053-T059) - Backward compatibility
4. **Team completes Polish together** (T060-T072) - Final validation

---

## Task Summary

- **Total Tasks**: 72
- **Foundational Tasks**: 6 (T001-T006)
- **User Story 1 (P1)**: 16 tasks (T007-T022) - CRITICAL MVP
- **User Story 2 (P2)**: 8 tasks (T023-T030)
- **User Story 3 (P2)**: 22 tasks (T031-T052)
- **User Story 4 (P3)**: 7 tasks (T053-T059)
- **Polish**: 13 tasks (T060-T072)

**Parallel Opportunities**: 8 tasks can run in parallel (marked with [P])

**Independent Tests**:

- User Story 1: Load updated level files, verify entity spawning
- User Story 2: Enable grid overlay, count 20x20 lines
- User Story 3: Clear level, observe transition timing
- User Story 4: Load 22x22 file, verify warning message

**Suggested MVP Scope**: Phase 2 (Foundational) + Phase 3 (User Story 1 only) = 22 tasks

**Estimated Total Time**:

- Sequential: ~6-8 hours
- Parallel (3 developers): ~3-4 hours

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- No test tasks included (not requested in specification)
- Manual verification steps included where automated testing is impractical
- Commit after completing each user story phase
- Stop at any checkpoint to validate story independently
- Avoid working on same files simultaneously (level_loader.rs used by US1 and US3)
