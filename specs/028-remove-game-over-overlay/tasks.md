# Tasks: Remove Legacy Game Over Overlay

**Input**: Design documents from `/specs/028-remove-game-over-overlay/` **Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/ui-overlay-removal.md`, `quickstart.md`

**Tests**: Tests are mandatory for all user stories.
For each story, write tests first, commit a failing red-test proof, and record the failing commit hash in the test task notes before implementation.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare shared test and verification scaffolding.

- [X] T001 Create feature integration test scaffold in `tests/game_over_overlay_removal.rs`
- [X] T002 [P] Add red/green commit-proof workflow notes in `specs/028-remove-game-over-overlay/quickstart.md`
- [X] T003 [P] Add contract verification checklist section for C1-C5 in `specs/028-remove-game-over-overlay/contracts/ui-overlay-removal.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Complete shared prerequisites that block user stories.

**CRITICAL**: No user story implementation starts until this phase is complete.

- [X] T004 Remove `GameOverOverlay` dependency gate from pause flow in `src/ui/pause_overlay.rs`
- [X] T005 [P] Remove overlay cleanup coupling from cheat flow in `src/systems/cheat_mode.rs`
- [X] T006 [P] Add Bevy 0.17 compliance guard assertions (fallible queries, `With`/`Without`) in `tests/game_over_overlay_removal.rs`
- [X] T007 Add hierarchy-safety verification checklist for touched UI paths in `specs/028-remove-game-over-overlay/contracts/ui-overlay-removal.md`
- [X] T008 [P] Add WASM validation command documentation in `specs/028-remove-game-over-overlay/quickstart.md`

**Checkpoint**: Foundational work complete.
User stories can proceed.

---

## Phase 3: User Story 1 - Clean Restart After Life Loss (Priority: P1)

**Goal**: Restart after game over never shows legacy overlay during gameplay.

**Independent Test**: Trigger life loss to game over, start new game, run multiple updates, verify zero `GameOverOverlay` entities.

### Tests for User Story 1 (REQUIRED)

- [X] T009 [US1] Add failing integration test `test_restart_after_game_over_has_no_legacy_overlay` in `tests/game_over_overlay_removal.rs` (record failing-test commit hash: `<hash>`)
- [X] T010 [US1] Add failing multi-frame test `test_no_legacy_overlay_reappears_after_restart_over_10_frames` in `tests/game_over_overlay_removal.rs` (record failing-test commit hash: `<hash>`)
- [X] T011 [US1] Add failing control regression test `test_new_game_and_gameplay_controls_work_after_overlay_removal` in `tests/game_over_overlay_removal.rs` (record failing-test commit hash: `<hash>`)
- [X] T012 [US1] Add failing unit test for buffered `GameOverRequested` message usage in `tests/game_over_overlay_removal.rs` (record failing-test commit hash: `<hash>`)
- [X] T013 [US1] Record explicit requestor approval for US1 red tests in `specs/028-remove-game-over-overlay/quickstart.md`

### Implementation for User Story 1

- [X] T014 [US1] Remove only legacy overlay spawn system registration/import wiring in `src/ui/mod.rs`
- [X] T015 [US1] Remove or retire legacy overlay spawning entry points in `src/ui/game_over_overlay.rs`
- [X] T016 [US1] Preserve message-event separation for `GameOverRequested` in `src/systems/respawn.rs`

**Checkpoint**: US1 is fully functional and independently testable.

---

## Phase 4: User Story 2 - No Legacy Overlay Artifacts In Any Flow (Priority: P2)

**Goal**: Ensure no legacy overlay artifacts appear on fresh launch, transitions, or repeated cycles.

**Independent Test**: Run fresh gameplay plus repeated game-over/restart cycles and verify no legacy overlay entities exist.

### Tests for User Story 2 (REQUIRED)

- [X] T017 [US2] Add failing integration test `test_no_legacy_overlay_on_fresh_launch_gameplay` in `tests/game_over_overlay_removal.rs` (record failing-test commit hash: `<hash>`)
- [X] T018 [US2] Add failing repeated-cycle test `test_no_legacy_overlay_across_10_game_over_restart_cycles` in `tests/game_over_overlay_removal.rs` (record failing-test commit hash: `<hash>`)
- [X] T019 [US2] Add failing hierarchy safety regression test (no manual `Parent`/`Children` mutation) in `tests/game_over_overlay_removal.rs` (record failing-test commit hash: `<hash>`)
- [X] T020 [US2] Add failing pause/cheat compatibility integration tests in `tests/ui_overlays.rs` (record failing-test commit hash: `<hash>`)
- [X] T021 [US2] Add failing pause/cheat compatibility integration tests in `tests/cheat_mode.rs` (record failing-test commit hash: `<hash>`)
- [X] T022 [US2] Record explicit requestor approval for US2 red tests in `specs/028-remove-game-over-overlay/quickstart.md`

### Implementation for User Story 2

- [X] T023 [US2] Remove only residual legacy overlay module exports/references in `src/ui/mod.rs` after US2 tests are green
- [X] T024 [US2] Finalize legacy overlay component/system removal in `src/ui/game_over_overlay.rs`
- [X] T025 [US2] Update pause overlay expectations to be marker-independent in `tests/ui_overlays.rs`
- [X] T026 [US2] Update cheat-mode overlay expectations to be marker-independent in `tests/cheat_mode.rs`

**Checkpoint**: US2 is independently testable with no legacy overlay artifacts.

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Execute full validation and documentation cleanup.

- [X] T027 [P] Update UI behavior documentation for overlay removal in `docs/ui-systems.md`
- [X] T028 Run validation commands (`cargo test`, `cargo fmt --all`, `cargo clippy --all-targets --all-features`, `bevy lint`, `cargo check --target wasm32-unknown-unknown`) and record results in `specs/028-remove-game-over-overlay/quickstart.md`
- [X] T029 Confirm no replacement game-over overlay exists by auditing active gameplay UI paths in `src/ui/mod.rs`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1**: No dependencies.
- **Phase 2**: Depends on Phase 1; blocks all user stories.
- **Phase 3 (US1)**: Depends on Phase 2.
- **Phase 4 (US2)**: Depends on Phase 2 and US1 module cleanup (`src/ui/mod.rs`).
- **Phase 5**: Depends on completed user stories.

### User Story Dependencies

- **US1 (P1)**: Starts after foundational phase.
- **US2 (P2)**: Starts after foundational phase and US1 shared module cleanup.

### Within Each User Story

- Write tests first and commit red proof before implementation.
- Record failing-test commit hash in each test task.
- Obtain explicit requestor approval before implementation tasks.
- Keep Bevy 0.17 message-event separation and hierarchy safety requirements satisfied.

---

## Parallel Execution Examples

## Parallel Example: Foundational

```bash
T005: src/systems/cheat_mode.rs
T006: tests/game_over_overlay_removal.rs
T008: specs/028-remove-game-over-overlay/quickstart.md
```

## Parallel Example: User Story 2 Tests

```bash
T020: tests/ui_overlays.rs
T021: tests/cheat_mode.rs
```

---

## Implementation Strategy

### MVP First (US1)

1. Complete Phase 1 and Phase 2.
2. Complete US1 red tests and approval.
3. Complete US1 implementation and validate independently.

### Incremental Delivery

1. Deliver MVP via US1.
2. Deliver all-flow cleanup via US2.
3. Execute Phase 5 quality gates and finalize docs.
