# Tasks: Sea Mine Brick

**Input**: Design documents from `/specs/067-add-seamine-brick/` **Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/gameplay-contract.md](contracts/gameplay-contract.md)

**Tests**: Tests are mandatory for all user stories.
Tests must be written first, must fail before implementation begins, and the failing-test commit hash must be recorded when the red phase is complete.

**Bevy 0.17 compliance**: This feature uses Messages for spawn/detonation/life-loss gameplay state and an Observer for the immediate Hanabi explosion burst.
Tasks explicitly include Message-Event Separation, hierarchy safety, motion floor persistence, and asset-handle reuse checks.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare shared dependencies, module scaffolding, and feature registration.

- [ ] T001 [P] Update the project toolchain baseline to Rust 1.89 where needed, add `bevy_hanabi` 0.17.0 to [Cargo.toml](../../Cargo.toml), and register the Hanabi plugin path in [src/lib.rs](../../src/lib.rs)
- [ ] T002 [P] Add the sea mine module scaffolding in [src/systems/sea_mine.rs](../../src/systems/sea_mine.rs) and [src/systems/particle_fx.rs](../../src/systems/particle_fx.rs), then export them from [src/systems/mod.rs](../../src/systems/mod.rs)
- [ ] T003 [P] Add buffered and observer-facing sea mine message/event types in [src/signals.rs](../../src/signals.rs) for spawn, detonation, and explosion burst trigger flow

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core brick registry and shared configuration that all user stories depend on.

**Checkpoint**: No user story work starts until this phase is complete.

- [ ] T004 [P] Define sea mine brick index `31` and helper predicates in [src/level_format/mod.rs](../../src/level_format/mod.rs), and update the reference entry in [docs/bricks.md](../../docs/bricks.md)
- [ ] T005 [P] Register sea mine brick loading and completion behavior in [src/level_loader.rs](../../src/level_loader.rs) so brick `31` loads as a destructible completion brick
- [ ] T006 [P] Add the sea mine texture/material profile to [assets/textures/manifest.ron](../../assets/textures/manifest.ron) and document the visual mapping in [docs/bricks.md](../../docs/bricks.md)

---

## Phase 3: User Story 1 - Release a Sea Mine From a Brick (Priority: P1) 🎯 MVP

**Goal**: Destroying brick `31` spawns exactly one sea mine hazard that starts with arbitrary motion, visible spin, and persistent minimum movement.

**Independent Test**: Destroy a sea mine brick in isolation and verify the brick despawns, one sea mine appears, and its motion/spin stay above the required floors across at least 10 `app.update()` cycles.

### Tests for User Story 1 (REQUIRED) ⚠️

> Write these tests first, confirm they fail, and record the failing-test commit hash in the task notes.

- [ ] T007 [P] [US1] Add the failing acceptance test in [tests/sea_mine_brick.rs](../../tests/sea_mine_brick.rs) that destroys brick `31` and asserts one sea mine spawn plus brick despawn
- [ ] T008 [P] [US1] Add the failing persistence/motion test in [tests/sea_mine_lifecycle.rs](../../tests/sea_mine_lifecycle.rs) that checks arbitrary launch direction, visible spin, and minimum `3.0 u/s` / `180 deg/s` floors across 10 frames

### Implementation for User Story 1

- [ ] T009 [US1] Implement `SpawnSeaMineMessage` consumption and sea mine entity spawning in [src/systems/sea_mine.rs](../../src/systems/sea_mine.rs), including hierarchy-safe child mesh setup with `add_child`/`set_parent`
- [ ] T010 [US1] Implement sea mine motion maintenance and rotation behavior in [src/systems/sea_mine.rs](../../src/systems/sea_mine.rs) so active mines never fall below the minimum speed/spin floors
- [ ] T011 [US1] Wire brick `31` ball-collision handling in [src/lib.rs](../../src/lib.rs) to emit the spawn message, despawn the brick, and keep the gameplay flow message-driven

**Checkpoint**: User Story 1 should now be independently playable and testable.

---

## Phase 4: User Story 2 - Detonate on Hazard Contact (Priority: P1)

**Goal**: Sea mines detonate on wall, paddle, or brick-index-greater-than-90 contact, destroying balls and the paddle within a 30-unit blast radius and spawning the Hanabi burst.

**Independent Test**: Spawn a sea mine near each valid trigger type and verify one detonation, radius-bound destruction, and a single Hanabi burst at the detonation point.

### Tests for User Story 2 (REQUIRED) ⚠️

> Write these tests first, confirm they fail, and record the failing-test commit hash in the task notes.

- [ ] T012 [P] [US2] Add the failing detonation test in [tests/sea_mine_particles.rs](../../tests/sea_mine_particles.rs) covering wall, paddle, and brick `> 90` triggers plus the 30-unit blast radius
- [ ] T013 [P] [US2] Add the failing one-shot/life-loss test in [tests/sea_mine_lifecycle.rs](../../tests/sea_mine_lifecycle.rs) that verifies non-trigger brick contacts do not detonate and paddle destruction records exactly one life loss

### Implementation for User Story 2

- [ ] T014 [US2] Implement detonation detection and buffered `SeaMineDetonationMessage` handling in [src/systems/sea_mine.rs](../../src/systems/sea_mine.rs) using Messages for gameplay state
- [ ] T015 [US2] Implement the immediate Hanabi explosion observer and shared effect resource in [src/systems/particle_fx.rs](../../src/systems/particle_fx.rs), and register the effect asset in [src/lib.rs](../../src/lib.rs)
- [ ] T016 [US2] Implement radius-based ball and paddle cleanup plus the single life-loss handoff in [src/systems/sea_mine.rs](../../src/systems/sea_mine.rs) and [src/systems/respawn.rs](../../src/systems/respawn.rs)

**Checkpoint**: User Story 2 should now independently detonate, destroy targets, and render the explosion burst.

---

## Phase 5: User Story 3 - Preserve Existing Progression Rules (Priority: P2)

**Goal**: Sea mine bricks participate in completion tracking and level loading without regressing existing level progression or death flow.

**Independent Test**: Load a level containing brick `31`, verify it counts toward completion, and confirm the explosion-driven life-loss path does not duplicate across frames.

### Tests for User Story 3 (REQUIRED) ⚠️

> Write these tests first, confirm they fail, and record the failing-test commit hash in the task notes.

- [ ] T017 [P] [US3] Add the failing level-loading/completion test in [tests/sea_mine_brick.rs](../../tests/sea_mine_brick.rs) that confirms brick `31` is authorable and counts toward level completion
- [ ] T018 [P] [US3] Add the failing regression test in [tests/sea_mine_lifecycle.rs](../../tests/sea_mine_lifecycle.rs) that confirms life loss from a sea mine explosion is not duplicated across later frames

### Implementation for User Story 3

- [ ] T019 [US3] Update level loading and completion marker logic in [src/level_loader.rs](../../src/level_loader.rs) and [src/level_format/mod.rs](../../src/level_format/mod.rs) so brick `31` behaves as the new sea mine brick
- [ ] T020 [US3] Update the feature-facing docs in [docs/bricks.md](../../docs/bricks.md) and [specs/067-add-seamine-brick/quickstart.md](quickstart.md) to describe brick `31`, the minimum-motion rules, and the Hanabi burst behavior

**Checkpoint**: User Stories 1, 2, and 3 should now all be independently functional.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final verification and repo-wide compliance checks.

- [ ] T021 [P] Run `cargo test`, `cargo fmt --all`, `cargo clippy --all-targets --all-features`, and `bevy lint`, then fix any sea-mine-specific issues in [src/systems/sea_mine.rs](../../src/systems/sea_mine.rs) and [src/systems/particle_fx.rs](../../src/systems/particle_fx.rs)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - blocks all user stories
- **User Stories (Phase 3+)**: Depend on the Foundational phase
- **Polish (Phase 6)**: Depends on the desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Starts after Foundational - no dependency on other stories
- **User Story 2 (P1)**: Starts after Foundational - independent of US1, though it reuses the same sea mine entity
- **User Story 3 (P2)**: Starts after Foundational - depends on the shared brick index and loader work only

### Within Each User Story

- Tests must be written first and confirmed to fail before implementation begins.
- The failing-test commit hash must be recorded in the task notes after the red phase.
- Messages are used for buffered gameplay state; the Hanabi burst uses an Observer.
- Models and shared resources before gameplay logic.
- Core behavior before documentation or polish.

### Parallel Opportunities

- Setup tasks T001-T003 can run in parallel.
- Foundational tasks T004-T006 can run in parallel.
- Tests within a user story can run in parallel when they touch different files.
- US1, US2, and US3 implementation can proceed in parallel after the foundation is in place.

---

## Parallel Example: User Story 1

```bash
Task: "Add the failing acceptance test in tests/sea_mine_brick.rs that destroys brick 31 and asserts one sea mine spawn plus brick despawn"
Task: "Add the failing persistence/motion test in tests/sea_mine_lifecycle.rs that checks arbitrary launch direction, visible spin, and minimum 3.0 u/s / 180 deg/s floors across 10 frames"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Stop and validate User Story 1 independently

### Incremental Delivery

1. Setup + Foundational → shared sea mine infrastructure ready
2. Add User Story 1 → brick spawn and motion floor are testable
3. Add User Story 2 → detonation, radius damage, and Hanabi burst are testable
4. Add User Story 3 → completion and loading integration are locked in

### Parallel Team Strategy

1. Team completes Setup + Foundational together
2. After the foundation is complete:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Merge only after the story-specific tests pass

---

## Notes

- `[P]` tasks operate on different files and do not depend on incomplete work.
- Each story remains independently testable.
- The sea mine brick index is 31.
- `bevy_hanabi` is used only for the explosion burst, not for gameplay state.
