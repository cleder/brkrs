---
description: "Task list for Refactor Entity Spawning"
---

# Tasks: Refactor Entity Spawning

**Input**: Design documents from `/specs/012-refactor-entity-spawning/` **Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are MANDATORY for all user stories.
Each story MUST include unit tests and feature-level acceptance tests (integration or contract tests as appropriate).
Tests MUST be written and committed first, verified to FAIL (red), and then approved before implementation begins; record the test-proof commit hash in the task description.

**Bevy 0.17 compliance**: When generating tasks for ECS/rendering/UI work, include explicit tasks (or acceptance criteria within test tasks) to ensure compliance with the constitution's Bevy 0.17 mandates & prohibitions (no panicking queries, filtered queries, `Changed<T>` for reactive UI, message vs event correctness, asset handle reuse, and correct hierarchy APIs).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create `src/systems/spawning.rs` module file

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

*No foundational tasks required for this refactor.*

## Phase 3: User Story 1 - Modular Entity Spawning (Priority: P1) 🎯 MVP

**Goal**: Extract entity spawning logic into dedicated, testable functions.

**Independent Test**: Verify that `spawn_camera`, `spawn_ground_plane`, and `spawn_light` systems exist and create the correct entities with expected components.

### Tests for User Story 1 (REQUIRED) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation; include failing-test commit hash in task**

- [ ] T002 [US1] Create unit test in `tests/spawning_systems.rs` to verify `spawn_camera` creates entity with `MainCamera` and `Camera3d` (Red state)
- [ ] T003 [US1] Create unit test in `tests/spawning_systems.rs` to verify `spawn_ground_plane` creates entity with `GroundPlane` and `Mesh3d` (Red state)
- [ ] T004 [US1] Create unit test in `tests/spawning_systems.rs` to verify `spawn_light` creates entity with `PointLight` (Red state)

### Implementation for User Story 1

- [ ] T005 [US1] Define `MainCamera` component in `src/systems/spawning.rs`
- [ ] T006 [US1] Implement `spawn_camera` system in `src/systems/spawning.rs`
- [ ] T007 [US1] Implement `spawn_ground_plane` system in `src/systems/spawning.rs`
- [ ] T008 [US1] Implement `spawn_light` system in `src/systems/spawning.rs`
- [ ] T009 [US1] Register `spawning` module in `src/systems/mod.rs`
- [ ] T010 [US1] Update `src/lib.rs` to register new systems and remove spawning logic from `setup`

**Checkpoint**: At this point, the game should compile and run with the new spawning systems.

## Phase 4: Polish & Cross-Cutting Concerns

**Purpose**: Final polish, cleanup, and non-functional requirements

- [ ] T011 Verify no visual regressions by running the game and comparing with `develop` branch
- [ ] T012 Run `cargo clippy` and `cargo fmt` to ensure code quality

## Dependencies

- US1 is independent.

## Parallel Execution Examples

- T006, T007, T008 can be implemented in parallel after T005 is done.

## Implementation Strategy

1. **Test First**: Write tests in `tests/spawning_systems.rs` that fail because the systems don't exist or don't do anything.
2. **Implement**: Create the module and systems.
3. **Integrate**: Wire them up in `lib.rs`.
4. **Verify**: Run the game and tests.
