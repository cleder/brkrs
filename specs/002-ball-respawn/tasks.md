---

description: "Task list for Ball Respawn System"
---

# Tasks: Ball Respawn System

**Input**: Design documents from `/specs/002-ball-respawn/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Automated tests are included where they materially reduce regressions for respawn timing; additional manual verification steps live in quickstart.md.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Ensure the workspace has the scaffolding and docs necessary to host the respawn systems.

- [X] T001 Create `RespawnPlugin` scaffold and placeholder system sets in `src/systems/respawn.rs`.
- [X] T002 Register `RespawnPlugin` in `src/systems/mod.rs` and `src/main.rs` so it loads with the app.
- [X] T003 Add `bevy lint` invocation to the Build & Test Commands block in `specs/002-ball-respawn/quickstart.md` to align with updated instructions.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core ECS data and Rapier plumbing required by all user stories.

- [ ] T004 Extend `src/level_loader.rs` to extract paddle/ball transforms from the level matrix into a `SpawnPoints` resource and attach `RespawnHandle` components (with fallback logging).
- [ ] T005 Define `LifeLostEvent`, `RespawnScheduled`, `RespawnCompleted`, and the `RespawnSchedule` resource structs in `src/systems/respawn.rs`.
- [ ] T006 Add a `LowerGoal` sensor entity setup (collider + `ActiveEvents::COLLISION_EVENTS`) in `src/level_loader.rs` to detect life loss.
- [ ] T007 Introduce `BallFrozen` and `InputLocked` marker components plus helpers in `src/systems/respawn.rs` so other systems can filter on them.
- [ ] T008 Create `tests/respawn_spawn_points.rs` verifying fallback center behavior and one-to-one spawn extraction using Bevy `App` tests.

**Checkpoint**: Spawn data, events, and markers exist; Rapier can emit the required collisions.

---

## Phase 3: User Story 1 - Ball Respawn After Loss (Priority: P1) 🎯 MVP

**Goal**: Automatically respawn ball and paddle at matrix positions with a 1s delay, stationary ball, and disabled controls until the player relaunches.

**Independent Test**: Run the game, let the ball cross the lower boundary, observe a 1s pause, confirm both entities respawn at their original transforms, ball stays stationary, and controls remain locked until launch input.

### Implementation for User Story 1

- [ ] T009 [US1] Implement `detect_ball_loss_system` in `src/systems/respawn.rs` to read `CollisionEvent::Started`, emit `LifeLostEvent`, and despawn the ball entity.
- [ ] T010 [US1] Add `life_loss_logging_system` in `src/systems/respawn.rs` to record cause+entity IDs (aids manual verification and SC-004 tracking).
- [ ] T011 [US1] Implement `respawn_scheduler_system` in `src/systems/respawn.rs` that consumes `LifeLostEvent`, populates `RespawnSchedule`, applies `BallFrozen`/`InputLocked`, and zeros velocities using Rapier components.
- [ ] T012 [US1] Implement `respawn_executor_system` in `src/systems/respawn.rs` that ticks the Bevy `Timer`, respawns/translates ball+paddle using `SpawnPoints`, emits `RespawnScheduled`/`RespawnCompleted`, and keeps ball stationary.
- [ ] T013 [US1] Add `ball_launch_input_system` in `src/systems/respawn.rs` that listens for launch input (e.g., Space) to remove `BallFrozen` and give the ball its initial impulse.
- [ ] T014 [US1] Update `specs/002-ball-respawn/quickstart.md` manual steps with the exact launch-input check to close the independent test loop.
- [ ] T015 [P] [US1] Add `tests/respawn_timer.rs` exercising `RespawnSchedule` timing tolerance (±16 ms) and verifying the ball remains stationary until launch.

**Checkpoint**: User Story 1 is fully functional and testable; qualifies as MVP.

---

## Phase 4: User Story 2 - Multiple Respawn Handling (Priority: P2)

**Goal**: Support consecutive losses, last-life game-over handling, and multi-ball scenarios without interfering with remaining balls.

**Independent Test**: Lose the ball repeatedly—including when lives reach zero—and verify each respawn schedules correctly, skips when out of lives, and only respawns the lost ball while others keep moving.

### Implementation for User Story 2

- [ ] T016 [US2] Extend `respawn_scheduler_system` in `src/systems/respawn.rs` to consult a `LivesState` resource and emit/consume `GameOverRequested` when `lives_remaining == 0`.
- [ ] T017 [US2] Implement per-entity respawn tracking in `src/systems/respawn.rs` so only the lost ball/paddle pair reinitializes (supporting multi-ball power-ups).
- [ ] T018 [US2] Add queuing/guard logic in `src/systems/respawn.rs` to warn when a respawn is already pending and defer new requests until completion (prevents timer overlap).
- [ ] T019 [P] [US2] Create `tests/multi_respawn.rs` validating two sequential `LifeLostEvent`s process correctly and that game over halts respawn scheduling.
- [ ] T020 [US2] Enhance `specs/002-ball-respawn/quickstart.md` repeated-loss manual test section with expected log output for lives decrement and game-over skip.

**Checkpoint**: User Stories 1 and 2 operate independently; repeated losses no longer block gameplay.

---

## Phase 5: User Story 3 - Respawn Visual Feedback (Priority: P3)

**Goal**: Provide a visible cue (fade/pause) during respawn so players understand the reset while controls remain disabled.

**Independent Test**: Trigger a respawn and visually confirm the overlay/pause effect plays during the 1s delay, then fades out before controls reactivate.

### Implementation for User Story 3

- [ ] T021 [US3] Add a simple fade overlay or pause tween implementation in `src/systems/respawn.rs` (or dedicated `src/systems/visuals.rs`) that listens to `RespawnScheduled` and animates opacity.
- [ ] T022 [US3] Gate control re-enabling in `src/systems/respawn.rs` on the visual feedback completion event to keep behavior consistent across native/WASM.
- [ ] T023 [P] [US3] Create `tests/respawn_visual.rs` (can be feature-flagged) to confirm the overlay entity transitions through expected phases when `RespawnScheduled` fires.
- [ ] T024 [US3] Document the visual indicator expectation in `specs/002-ball-respawn/quickstart.md` manual verification step 1 (include screenshot or description).

**Checkpoint**: All three user stories are independently testable and deliver polished UX.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Hardening, docs, and verification spanning all stories.

- [ ] T025 [P] Add structured logging hooks (e.g., `tracing::info!`) for `LifeLostEvent`, `RespawnScheduled`, and `GameOverRequested` inside `src/systems/respawn.rs` for observability.
- [ ] T026 [P] Run `cargo test`, `cargo clippy --all-targets --all-features`, `bevy lint`, and `cargo build --target wasm32-unknown-unknown --release` documenting outputs in `specs/002-ball-respawn/quickstart.md`.
- [ ] T027 Capture known limitations / future work for respawn (e.g., HUD lives display) in `specs/002-ball-respawn/spec.md` Edge Cases list once implementation gaps are observed.

---

## Dependencies & Execution Order

### Phase Dependencies

1. **Setup** → completes before Foundational.
2. **Foundational** → unlocks all user-story phases.
3. **User Story Phases (US1 → US2 → US3)** → may begin once Foundational is done; recommended to follow priority order though US2/US3 can run in parallel if resources allow.
4. **Polish** → final phase after desired user stories stabilize.

### User Story Dependencies

- **US1 (P1)**: Depends on Foundational data/resources; no other story dependencies.
- **US2 (P2)**: Depends on US1 systems being present (needs base respawn flow) plus Foundational.
- **US3 (P3)**: Depends on US1 (visual indicator hooks) and optionally US2 for consistent scheduling events.

### Parallel Opportunities

- Tasks marked [P] (T015, T019, T023, T025, T026) can run concurrently since they operate on separate files or solely on tests/build tooling.
- After Phase 2, US2 work (e.g., Lives handling) can begin in parallel with US1 test hardening if coordination ensures no overlapping file edits.
- Visual test implementation (T023) can run parallel to documentation updates (T024).

## Parallel Example: User Story 1

```bash
# Parallel test & documentation tasks for US1
- Run T015 to build automated timer tests in tests/respawn_timer.rs
- Run T014 to finalize manual verification notes in specs/002-ball-respawn/quickstart.md
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phases 1 & 2 (Setup + Foundational).
2. Implement all US1 tasks (T009–T015).
3. Validate via automated test (T015) plus manual quickstart steps.
4. Ship MVP enabling continuous gameplay after a single life loss.

### Incremental Delivery

- **Iteration 1**: MVP (US1) → proves respawn loop and unlocks gameplay beyond single life.
- **Iteration 2**: US2 → adds resilience for repeated losses and lives integration.
- **Iteration 3**: US3 → adds UX polish with visual feedback, ready for wider demos.
- **Polish Phase**: Logging, documentation, WASM validation.

### Parallel Team Strategy

- Developer A: Handles Phase 2 foundation + US1 systems.
- Developer B: Starts US2 lives/multi-ball logic once foundation ready.
- Developer C: Works on visual overlay (US3) and documentation after US1 signals (events) exist.

---

## Summary Metrics

- **Total Tasks**: 27
- **Per Story**: US1 → 7 tasks; US2 → 5 tasks; US3 → 4 tasks
- **Parallel Tasks**: T015, T019, T023, T025, T026 identified as parallel-friendly
- **Independent Tests**: Described per story (manual quickstart + automated test references)
- **MVP Scope**: Complete through US1 (Phases 1–3) before tackling later stories
- **Format Validation**: All tasks use required checkbox + ID format with explicit file paths
