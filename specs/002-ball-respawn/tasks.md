---

description: "Task list for Ball & Paddle Respawn + Level Flow"
---

# Tasks: Ball & Paddle Respawn + Level Flow

**Input**: Design documents from `/specs/002-ball-respawn/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/, quickstart.md

**Tests**: Manual gameplay verification steps are documented in quickstart.md. Automated coverage is now planned for every user story (US1 fallback positions, US2 soak/gravity, US3 animation + level flow).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish dependencies, module scaffolding, and sample assets that every story relies on.

- [ ] T001 Update `Cargo.toml` to add `bevy_tweening` and required Rapier/Serde feature flags for respawn metadata parsing.
- [ ] T002 [P] Scaffold module tree (`src/components/mod.rs`, `src/plugins/mod.rs`, `src/resources/mod.rs`, `src/events.rs`, `src/systems/mod.rs`) with placeholder exports matching the implementation plan.
- [ ] T003 [P] Annotate `assets/levels/level_001.ron` and `assets/levels/level_002.ron` with sample `gravity` and `respawn_overrides` blocks to unblock manual verification.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Data ingestion, resources, and events required before any user story systems can run.

- [ ] T004 Extend `src/level_loader.rs` to parse `gravity` + `respawn_overrides`, emit `SpawnPoint` components, and populate a `LevelOverrides` resource when levels load.
- [ ] T005 [P] Implement `LevelOverrides` and `RespawnSchedule` structs in `src/resources/level_overrides.rs` and `src/resources/respawn_schedule.rs`, then export via `src/resources/mod.rs`.
- [ ] T006 [P] Define gameplay events (`LifeLostEvent`, `RespawnQueuedEvent`, `RespawnCompleteEvent`, `LevelAdvanceEvent`, `LevelRestartRequested`) in `src/events.rs` with Debug derives for logging.
- [ ] T007 [P] Create `LevelProgress` and `RespawnCounters` data structures in `src/resources/progress.rs` plus constructors that seed default lives/brick counts.
- [ ] T008 Wire new modules into `src/main.rs` (resource initialization, plugin placeholders, schedule labels) so later story plugins can be registered cleanly.

**Checkpoint**: All core resources/events/hooks exist; user story work can now begin.

---

## Phase 3: User Story 1 - Ball Respawn After Loss (Priority: P1) 🎯 MVP

**Goal**: Detect lower-goal collisions and respawn the ball + paddle at matrix-defined positions after a 1-second delay while preserving gameplay state.

**Independent Test**: Follow Quickstart step “Life Loss Triggers Respawn” to ensure both entities return to start positions, the ball remains stationary until input, and lives decrement by one.

### Tests for User Story 1

- [ ] T009 [P] [US1] Add integration test `tests/integration/respawn_single.rs` that drives a life-loss event and asserts paddle/ball transforms match spawn markers after the one-second delay.
- [ ] T010 [P] [US1] Add regression test `tests/integration/respawn_fallback.rs` confirming fallback center positions are applied when spawn markers are missing or invalid.

### Implementation for User Story 1

- [ ] T011 [P] [US1] Implement a Rapier sensor observer in `src/systems/respawn.rs` that listens for lower-goal collisions and emits `LifeLostEvent` with ball/paddle entity IDs.
- [ ] T012 [US1] Build `RespawnSchedule` queue logic in `src/systems/respawn.rs` to capture spawn points, enforce the 1-second delay, and record timestamps for diagnostics.
- [ ] T013 [US1] Execute respawn actions in `src/systems/respawn.rs`: despawn old ball, reposition paddle, spawn new ball with default physics, apply fallback center positions if spawn markers are missing, and set `BallFrozen` while waiting for release.
- [ ] T014 [P] [US1] Add `RespawnPlugin` in `src/plugins/respawn.rs` and register it (plus system ordering) inside `src/main.rs` so detection, scheduling, and execution run in the correct Bevy stages.
- [ ] T015 [US1] Integrate lives management inside `src/systems/respawn.rs` by decrementing `LevelProgress.lives`, persisting score/brick progress, and logging each life loss.

**Checkpoint**: Single respawn loop is playable end-to-end and independently testable.

---

## Phase 4: User Story 2 - Multiple Respawn Handling (Priority: P2)

**Goal**: Sustain reliable behavior across repeated ball losses, handle last-life transitions, respect per-level gravity overrides, and support scenarios with multiple balls.

**Independent Test**: Repeat Quickstart “Repeated Respawns” flow to lose 100 balls in succession and confirm timers, gravity overrides, and lives-to-game-over transitions behave consistently.

### Tests for User Story 2

- [ ] T016 [P] [US2] Add gravity override test `tests/integration/respawn_gravity.rs` validating that per-level vectors apply immediately on load and zero out during respawn animation windows.
- [ ] T017 [P] [US2] Add soak test `tests/integration/respawn_flow.rs` simulating 100 sequential life losses to ensure timers/lives counters remain consistent across runs.

### Implementation for User Story 2

- [ ] T018 [P] [US2] Extend `LevelProgress` / `RespawnCounters` in `src/resources/progress.rs` to track loss streaks, consecutive respawn timestamps, and remaining lives for telemetry.
- [ ] T019 [US2] Add last-life handling in `src/systems/respawn.rs` that triggers GameOver state (or equivalent resource flag) instead of respawn when lives reach zero.
- [ ] T020 [US2] Update respawn execution in `src/systems/respawn.rs` to support multi-ball scenarios by only respawning the lost ball entity and leaving others untouched.
- [ ] T021 [P] [US2] Implement per-level + temporary gravity control in `src/systems/gravity.rs`, applying overrides from `LevelOverrides` and zeroing gravity during respawn animations.

**Checkpoint**: Respawn loop survives long play sessions and edge cases without manual intervention.

---

## Phase 5: User Story 3 - Respawn Visual Feedback (Priority: P3)

**Goal**: Provide clear visual/UX cues for respawn, including paddle growth animation, fade transitions, and manual level restart control.

**Independent Test**: Use Quickstart steps 2–5 to verify paddle scaling, ball freeze/unfreeze timing, fade overlay during level transitions, and `R` key restart behavior.

### Tests for User Story 3

- [ ] T022 [P] [US3] Add animation test `tests/integration/paddle_growth.rs` confirming the paddle scale tween follows the ease-out profile and that `BallFrozen` prevents motion until completion.
- [ ] T023 [P] [US3] Add level flow test `tests/integration/level_flow.rs` covering fade overlay opacity ramps and `R` key restart transitions.

### Implementation for User Story 3

- [ ] T024 [P] [US3] Implement `PaddleGrowth` component + `bevy_tweening` animator in `src/components/paddle.rs` and drive it from `src/systems/respawn.rs` when respawn completes.
- [ ] T025 [US3] Add `BallFrozen` handling in `src/components/ball.rs` and update respawn systems so the ball unfreezes only after the paddle growth tween reports completion.
- [ ] T026 [P] [US3] Build fade overlay entity/material and tween controllers in `src/systems/level_transition.rs`, listening for `LevelAdvanceEvent` to run fade-out/in cycles.
- [ ] T027 [US3] Implement `LevelFlowPlugin` in `src/plugins/level_flow.rs` plus `src/main.rs` wiring to coordinate fade overlays with level loading and respawn scheduling.
- [ ] T028 [P] [US3] Add keyboard input handling in `src/systems/level_transition.rs` so pressing `KeyCode::R` emits `LevelRestartRequested` and reuses the fade pipeline before reloading the current level.

**Checkpoint**: Players receive clear visual context for respawns, level transitions, and manual restarts.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Wrap-up tasks that span multiple stories and ensure cross-platform readiness.

- [ ] T029 Update `README.md` and `specs/002-ball-respawn/quickstart.md` with final instructions for gravity overrides, fade overlays, and restart controls.
- [ ] T030 [P] Add structured logging/telemetry in `src/systems/respawn.rs` and `src/systems/level_transition.rs` to flag slow respawn cycles (>2 ms) for profiling.
- [ ] T031 Validate native + WASM builds by running `cargo test`, `cargo clippy --all-targets --all-features`, and `cargo build --target wasm32-unknown-unknown --release`, recording results in `README.md`.

---

## Dependencies & Execution Order

1. **Phase 1 → Phase 2**: Setup must complete before foundational work (dependencies and module scaffolding in place).
2. **Phase 2 → All User Stories**: Resources/events/gravity parsing block every story; no user story work can begin before Phase 2 completion.
3. **User Story Order**: US1 (P1) must ship first for MVP; US2 and US3 can start after US1 if their dependent files diverge, but shared systems (respawn.rs) require coordination.
4. **Polish**: Runs after desired stories finish to avoid churn.

### User Story Dependencies

| Story | Depends On | Notes |
|-------|------------|-------|
| US1 | Setup + Foundational | MVP respawn loop |
| US2 | US1 | Reuses respawn pipeline but adds resiliency features |
| US3 | US1 | Needs respawn events to trigger animations and fades |

---

## Parallel Execution Examples

### User Story 1

- Run T011 (event detection) and T014 (plugin wiring) in parallel while T012/T013 focus on scheduling + execution.
- Manual and automated verification can start as soon as T009–T013 land.

### User Story 2

- T018 (counters) and T021 (gravity overrides) operate on separate files/resources, enabling parallel work.
- T017 (soak test) should start once T018–T021 compile.

### User Story 3

- T024 (paddle tween) and T026 (fade overlay) are independent; they integrate via T027 after both land.
- T028 (restart input) can proceed in parallel with T026 once the overlay entity exists, while T022–T023 tests run.

---

## Implementation Strategy

1. **MVP (US1 Only)**: Complete Phases 1–3, validate respawn loop manually, and ship a playable build.
2. **Incremental Delivery**:
   - Add US2 for resiliency and gravity overrides, verifying long-session stability via T017.
   - Layer in US3 for polish (animations + fades) once stability is proven.
3. **Parallel Collaboration**: After Phase 2, one developer can maintain respawn pipeline (US1/US2) while another focuses on UX polish (US3), using the parallel examples above to avoid merge conflicts.

---
