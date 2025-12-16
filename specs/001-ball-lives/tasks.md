---

description: "Task list for Ball Lives Counter feature implementation"
---

# Tasks: Ball Lives Counter

**Input**: Design documents from `/specs/001-ball-lives/`

- Required: [plan.md](plan.md), [spec.md](spec.md)
- Optional (available): [research.md](research.md), [data-model.md](data-model.md), [contracts/events.openapi.yaml](contracts/events.openapi.yaml), [quickstart.md](quickstart.md)

**Note on tests**: No new tests are included because the feature spec did not request a TDD approach.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Minimal shared setup required for this feature.

- [ ] T001 Verify Orbitron font asset exists at assets/fonts/Orbitron/Orbitron-VariableFont_wght.ttf
- [ ] T002 Add shared UI font resource in src/ui/fonts.rs (loads/stores Orbitron handle)
- [ ] T003 [P] Export new UI font module in src/ui/mod.rs
- [ ] T004 Wire Orbitron font loading in src/lib.rs (insert UiFonts resource at Startup)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Cross-cutting prerequisites that must be complete before user stories.

- [ ] T005 [P] Apply Orbitron font to pause overlay text in src/ui/pause_overlay.rs
- [ ] T006 [P] Apply Orbitron font to palette UI text in src/ui/palette.rs
- [ ] T007 [P] Apply Orbitron font to wireframe hint text in src/lib.rs
- [ ] T008 [P] Apply Orbitron font to "GAME COMPLETE" text in src/level_loader.rs

**Checkpoint**: All on-screen text uses Orbitron.

---

## Phase 3: User Story 1 - See Remaining Balls (Priority: P1) 🎯 MVP

**Goal**: Show the player how many balls/lives remain and keep the number accurate after each life loss.

**Independent Test**: Follow [quickstart.md](quickstart.md) "Verify manually" steps 1–4.

### Implementation

- [ ] T009 [US1] Reset lives to 3 on level restart in src/level_loader.rs (restart_level_on_key) by updating LivesState
- [ ] T010 [US1] Decrement lives on each LifeLostEvent in src/systems/respawn.rs (use ResMut<LivesState>, clamp at 0)
- [ ] T011 [P] [US1] Create lives counter HUD UI in src/ui/lives_counter.rs (spawn once, update on change, Orbitron)
- [ ] T012 [US1] Wire lives counter systems in src/lib.rs (add systems after RespawnSystems::Schedule)

**Checkpoint**: Lives counter shows 3 at start and decrements on life loss.

---

## Phase 4: User Story 2 - Game Over on Last Ball (Priority: P2)

**Goal**: When the last ball is lost, display a Game Over message.

**Independent Test**: Follow [quickstart.md](quickstart.md) "Verify manually" steps 5–6.

### Implementation

- [ ] T013 [US2] Emit GameOverRequested when lives reach 0 and skip respawn scheduling in src/systems/respawn.rs
- [ ] T014 [P] [US2] Add game-over overlay UI in src/ui/game_over_overlay.rs (spawn on GameOverRequested, Orbitron, no duplicates)
- [ ] T015 [US2] Wire game-over overlay systems in src/lib.rs (Update schedule)
- [ ] T016 [US2] Prevent pause overlay from obscuring game over in src/ui/pause_overlay.rs (gate spawn when game over is active)

**Checkpoint**: Game over message appears on the last life loss and remains visible at 0 lives.

---

## Phase 5: User Story 3 - Lives Never Go Negative (Priority: P3)

**Goal**: Lives never go below 0 and game-over remains stable under repeated loss events.

**Independent Test**: Trigger more LifeLostEvent occurrences than starting lives and confirm the counter stays at 0 and game-over stays visible.

### Implementation

- [ ] T017 [US3] Ensure lives decrement uses saturating/clamped behavior in src/systems/respawn.rs (no underflow)
- [ ] T018 [US3] Ensure game-over overlay spawns once and remains stable when additional LifeLostEvent occur at 0 lives in src/ui/game_over_overlay.rs
- [ ] T019 [US3] Ensure lives counter displays 0 and does not go negative in src/ui/lives_counter.rs

**Checkpoint**: Lives never negative; UI remains consistent.

---

## Phase 6: Polish & Cross-Cutting Concerns

- [ ] T020 [P] Run formatting and fix issues: `cargo fmt --all`
- [ ] T021 [P] Run clippy and fix relevant warnings: `cargo clippy --all-targets --all-features`
- [ ] T022 Run validation: `cargo test` and `bevy lint`
- [ ] T023 Run full manual verification checklist in specs/001-ball-lives/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 (Setup) → Phase 2 (Foundational) → User Stories (Phase 3–5) → Phase 6 (Polish)

### User Story Dependencies

- **US1 (P1)** depends on Phase 2.
- **US2 (P2)** depends on US1 (uses the same lives tracking) and Phase 2.
- **US3 (P3)** depends on US1/US2 implementations and Phase 2.

## Parallel Opportunities

- Phase 2 tasks T005–T008 can be executed in parallel (different files).
- Within US1, T011 can be done in parallel with T009–T010 once the UiFonts resource exists.
- Within US2, T014 can be done in parallel with T013 once the event semantics are finalized.

## Parallel Example: Phase 2 (Font rollout)

```bash
Task: "Apply Orbitron font to pause overlay text in src/ui/pause_overlay.rs"
Task: "Apply Orbitron font to palette UI text in src/ui/palette.rs"
Task: "Apply Orbitron font to wireframe hint text in src/lib.rs"
Task: "Apply Orbitron font to \"GAME COMPLETE\" text in src/level_loader.rs"
```

## MVP Suggestion

- MVP scope: **Phase 1–3 only (through US1)**.
  This delivers the core “lives counter shown and updated” value.
