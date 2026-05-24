# Tasks: Collision Particle Feedback

**Feature Branch**: `029-collision-particle-feedback` **Input**: `spec.md` + `plan.md` + `research.md` + `data-model.md` + `contracts/collision-feedback-observer.md` + `quickstart.md`

**Tests**: Mandatory (TDD-first).
For each story, tests must be written and committed in red phase before implementation tasks.

## Phase 1: Setup

**Purpose**: Create baseline module and wiring points for collision feedback work.

- [ ] T001 Create collision feedback module scaffold in `src/systems/collision_feedback.rs`
- [ ] T002 Register collision feedback module export in `src/systems/mod.rs`
- [ ] T003 Register collision feedback systems/observers in app wiring in `src/lib.rs`

## Phase 2: Foundational (Blocking)

**Purpose**: Add shared contracts/components/resources required by all stories.

- [ ] T004 Define `CollisionFeedbackTriggered` event payload and enums in `src/signals.rs`
- [ ] T005 Define `FeedbackProfile` resource and `FeedbackEffectInstance` component in `src/systems/collision_feedback.rs`
- [ ] T006 Implement profile defaults and validation guards (particles 8-16, lifetime 0.20-0.35) in `src/systems/collision_feedback.rs`
- [ ] T007 Implement exact-contact-point resolution helper with finite-value fallback in `src/systems/collision_feedback.rs`
- [ ] T008 Implement shared game-state gate helper for paused/non-playing suppression in `src/systems/collision_feedback.rs`
- [ ] T040 Add hierarchy-safety guard for collision VFX parent-child usage (if introduced), requiring `add_child`/`set_parent` APIs and forbidding manual Parent/Children mutation in `src/systems/collision_feedback.rs`

## Phase 3: User Story 1 - Immediate Hit Feedback (P1)

**Goal**: Spawn visible sparkly effects in the same frame for wall, paddle, and brick collisions at exact contact points.

**Independent Test**: Trigger one collision of each type and verify one visible effect appears at the recorded contact point in the same frame.

### Tests (Red Phase First)

- [ ] T009 [P] [US1] Add wall collision same-frame effect test in `tests/collision_particle_feedback.rs`
- [ ] T010 [P] [US1] Add paddle collision same-frame effect test in `tests/collision_particle_feedback.rs`
- [ ] T011 [P] [US1] Add brick collision same-frame effect test in `tests/collision_particle_feedback.rs`
- [ ] T012 [US1] Add exact contact-point spawn assertion test in `tests/collision_particle_feedback.rs`
- [ ] T013 [US1] Add brick-destroyed-on-impact still-spawns-effect test in `tests/collision_particle_feedback.rs`
- [ ] T014 [US1] Record red-phase failing test proof notes in `specs/029-collision-particle-feedback/checklists/requirements.md`

### Implementation

- [ ] T015 [US1] Emit `CollisionFeedbackTriggered` from wall collision path in `src/lib.rs`
- [ ] T016 [US1] Emit `CollisionFeedbackTriggered` from ball-brick collision path in `src/lib.rs`
- [ ] T017 [US1] Emit `CollisionFeedbackTriggered` from paddle collision path in `src/lib.rs`
- [ ] T018 [US1] Implement observer consumer to spawn one feedback effect per trigger in `src/systems/collision_feedback.rs`
- [ ] T019 [US1] Register observer and update scheduling order for immediate reaction in `src/lib.rs`

## Phase 4: User Story 2 - Clear and Non-Disruptive Effects (P2)

**Goal**: Keep effects brief/readable, enforce pause suppression, and guarantee cleanup.

**Independent Test**: Trigger repeated collisions and verify effects despawn in 0.20-0.35s, remain readable, and do not spawn/replay during pause.

### Tests (Red Phase First)

- [ ] T020 [P] [US2] Add lifetime window test (0.20-0.35 seconds) in `tests/collision_particle_feedback.rs`
- [ ] T021 [P] [US2] Add particle count window test (8-16 per collision) in `tests/collision_particle_feedback.rs`
- [ ] T022 [US2] Add pause suppression test (no spawn while paused) in `tests/collision_particle_feedback.rs`
- [ ] T023 [US2] Add no-replay-on-resume test in `tests/collision_particle_feedback.rs`
- [ ] T024 [US2] Add orphan cleanup test for repeated collisions in `tests/collision_particle_feedback.rs`
- [ ] T041 [US2] Add hierarchy-safety test/assertion that no manual Parent/Children mutation path is used for collision VFX in `tests/collision_particle_feedback.rs`

### Implementation

- [ ] T025 [US2] Implement effect lifetime update and despawn systems in `src/systems/collision_feedback.rs`
- [ ] T026 [US2] Implement particle count/lifetime sampling from `FeedbackProfile` in `src/systems/collision_feedback.rs`
- [ ] T027 [US2] Enforce paused/non-playing suppression in trigger producer paths in `src/lib.rs`
- [ ] T028 [US2] Ensure no backlog/replay state is introduced in `src/systems/collision_feedback.rs`

## Phase 5: User Story 3 - Consistent Feedback Across Surfaces (P3)

**Goal**: Keep one visual family across wall/paddle/brick with controlled variation and no missed supported collisions.

**Independent Test**: Compare sample collisions for all target kinds and verify same style family, controlled variation, and complete coverage.

### Tests (Red Phase First)

- [ ] T029 [P] [US3] Add cross-surface style consistency test in `tests/collision_particle_feedback.rs`
- [ ] T030 [P] [US3] Add controlled variation bounds test across target kinds in `tests/collision_particle_feedback.rs`
- [ ] T031 [US3] Add burst-collision one-effect-per-collision (no cap, no merge, no queue) test in `tests/collision_particle_feedback.rs`
- [ ] T032 [US3] Add supported-target coverage test (wall/paddle/brick all emit) in `tests/collision_particle_feedback.rs`

### Implementation

- [ ] T033 [US3] Implement target-kind style mapping with shared sparkly family and bounded variation in `src/systems/collision_feedback.rs`
- [ ] T034 [US3] Ensure burst collisions spawn independently with no per-frame cap in `src/systems/collision_feedback.rs`
- [ ] T035 [US3] Add tracing fields for source kind, contact point, particle count, and lifetime in `src/systems/collision_feedback.rs`

## Phase 6: Polish & Cross-Cutting

- [ ] T036 [P] Add module and function rustdoc for collision feedback flow in `src/systems/collision_feedback.rs`
- [ ] T037 [P] Update developer guidance for collision feedback behavior in `docs/developer-guide.md`
- [ ] T038 Run targeted feature tests and capture output notes in `specs/029-collision-particle-feedback/quickstart.md`
- [ ] T039 Run full validation (`cargo test`, `cargo fmt --all`, `cargo clippy --all-targets --all-features`, `bevy lint`) and record results in `specs/029-collision-particle-feedback/quickstart.md`

## Dependencies

- Setup (Phase 1) must complete before Foundational (Phase 2).
- Foundational (Phase 2) must complete before any user story implementation.
- User story order: US1 (P1) -> US2 (P2) -> US3 (P3).
- US2 depends on US1 trigger/observer baseline.
- US3 depends on US1 baseline and US2 lifecycle/suppression behavior.
- Polish begins after all target user stories are complete.

## Parallel Execution Examples

### US1

- Run T009, T010, and T011 in parallel (distinct tests in `tests/collision_particle_feedback.rs`).
- Run T015 and T017 in parallel once event contract task T004 is done (`src/lib.rs` collision paths).

### US2

- Run T020 and T021 in parallel (independent test cases in `tests/collision_particle_feedback.rs`).
- Run T025 and T026 in parallel after foundational tasks complete (`src/systems/collision_feedback.rs`).

### US3

- Run T029 and T030 in parallel (independent style assertions in `tests/collision_particle_feedback.rs`).
- Run T033 and T035 in parallel after T025-T028 complete (`src/systems/collision_feedback.rs`).

## Implementation Strategy

1. Deliver MVP by completing US1 first (immediate same-frame feedback for wall/paddle/brick).
2. Add safety and readability constraints in US2 (lifetime, particle bounds, pause suppression).
3. Finalize polish and consistency in US3 (shared style family + controlled variation).
4. Finish with cross-cutting docs and full validation.
