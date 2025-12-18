# Tasks: Cheat Mode Safeguards

## Phase 1: Setup

- [x] T001 Create cheat mode module skeleton and plugin registration hook in src/systems/cheat_mode.rs

## Phase 2: Foundational

- [x] T002 Define CheatModeState resource and helper accessors in src/systems/cheat_mode.rs
- [x] T003 Add score reset helper callable from cheat mode transitions in src/systems/scoring.rs

## Phase 3: User Story 1 (Cheat Mode Activation with Score Reset) [P1]

- [x] T004 [US1] Implement 'g' key handling limited to gameplay states, toggling CheatModeState in src/systems/cheat_mode.rs
- [x] T005 [US1] Reset score to 0 on cheat mode activate/deactivate in src/systems/scoring.rs
- [x] T006 [US1] Ensure 'g' presses in non-gameplay states are ignored (no toggle) in src/systems/cheat_mode.rs
- [x] T007 [US1] Emit state change signal/event for UI indicator consumption in src/systems/cheat_mode.rs
- [x] T016 [US1] Reset `LivesState.lives_remaining` to 3 on cheat mode activate in src/systems/cheat_mode.rs (ensures player can resume after dying)
- [x] T017 [US1] Remove any active `GameOverOverlay` when cheat mode is toggled in src/systems/cheat_mode.rs (UI dismissal + tests in tests/cheat_mode.rs)

## Phase 4: User Story 3 (Cheat Mode Visual Indicator) [P1]

- [x] T008 [US3] Build cheat mode indicator UI (white "CHEAT MODE" text on semi-transparent dark background) anchored lower-right in src/ui/cheat_indicator.rs
- [x] T009 [US3] Wire indicator visibility to cheat mode state change events with show/hide within 100ms in src/ui/cheat_indicator.rs
- [x] T010 [US3] Guard indicator against obscuring critical HUD regions (apply padding/margins/layering) in src/ui/cheat_indicator.rs

## Phase 5: User Story 2 (Accidental Key Press Prevention) [P2]

- [ ] T011 [US2] Gate level control keys R/N/P to cheat mode active state; ignore when inactive and block actions in src/systems/level_controls.rs
- [ ] T012 [P] [US2] Remove existing 'P' texture picker binding so it can serve previous-level control in src/systems/editor_palette.rs
- [ ] T013 [US2] Enable level control actions (respawn/next/previous) when cheat mode active and ensure soft beep plays when blocked in src/systems/level_controls.rs

## Phase 6: Polish & Cross-Cutting

- [ ] T014 Add tracing events for cheat toggles and blocked level-control attempts in src/systems/cheat_mode.rs
- [ ] T015 Update quickstart instructions to reflect final key bindings and indicator behavior in specs/001-cheat-mode-safeguards/quickstart.md

## Dependencies (Story Order)

- Complete Phase 2 before all user stories.
- Story order: US1 → US3 (indicator depends on cheat state) → US2.

## Parallel Opportunities

- T012 can run in parallel with T011/T013 once module paths are known (separate file).
- T008 can start after T007 is stubbed (UI implementation mostly independent of gating logic).

## Implementation Strategy (MVP First)

- MVP: Finish US1 + US3 (cheat toggle, score reset, indicator).
  Defer US2 gating until MVP verified.
