---
description: "Task list for Ball, Paddle, Brick Physics Config centralization"
---

# Tasks: Ball, Paddle, Brick Physics Config

**Input**: Design documents from `/specs/015-ball-physics-config/` **Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are MANDATORY for all user stories.
Each story MUST include unit tests and feature-level acceptance tests (integration or contract tests as appropriate).
Tests MUST be written and committed first, verified to FAIL (red), and then approved before implementation begins; record the test-proof commit hash in the task description.

**Bevy 0.17 compliance**: All ECS/rendering/physics work must include explicit tasks to ensure compliance with the constitution's Bevy 0.17 mandates & prohibitions (no panicking queries, filtered queries, `Changed<T>` for reactive UI, message vs event correctness, asset handle reuse, and correct hierarchy APIs).

## Phase 1: Setup (Shared Infrastructure)

- [ ] T001 [P] Ensure Bevy 0.17.3, bevy_rapier3d 0.32.0, and tracing 0.1 are present in Cargo.toml
- [ ] T002 [P] Confirm project builds and tests pass (cargo test, cargo clippy, bevy lint)

---

## Phase 2: Foundational (Blocking Prerequisites)

- [ ] T003 [P] Create BallPhysicsConfig, PaddlePhysicsConfig, and BrickPhysicsConfig structs in src/physics_config.rs
- [ ] T004 [P] Register each config as a Bevy resource in src/lib.rs
- [ ] T005 [P] Add validation logic for config values (finite, non-negative, reasonable bounds) in src/physics_config.rs
- [ ] T006 [P] Add documentation for config usage and non-hot-reloadability in src/physics_config.rs

## Explicit runtime validation test for config values

- [ ] T006a [P] Add unit test in tests/unit/physics_config.rs to verify all config values are finite, non-negative, and within reasonable bounds at runtime

---

## Phase 3: User Story 1 - Consistent Ball Physics (Priority: P1) 🎯 MVP

**Goal**: All balls use a single, centralized set of physics properties for consistent, maintainable tuning.

**Independent Test**: Spawn multiple balls and verify all use the same config; changing the config updates all new balls.

### Tests for User Story 1 (REQUIRED)

- [ ] T007 [P] [US1] Write integration test in tests/ball_lives.rs to verify all spawned balls use BallPhysicsConfig values (commit: FAILING_HASH)
- [ ] T008 [P] [US1] Write or cross-reference static analysis test in tests/unit/physics_config.rs to ensure no hardcoded physics values in ball spawn logic (see T013; commit: FAILING_HASH)

### Implementation for User Story 1

- [ ] T009 [P] [US1] Refactor ball spawn logic in src/level_loader.rs to use BallPhysicsConfig resource
- [ ] T010 [US1] Ensure all ball spawn paths (including respawn, powerups) use BallPhysicsConfig
- [ ] T011 [US1] Add rustdoc to BallPhysicsConfig and usage sites
- [ ] T012 [US1] Add Bevy 0.17 compliance checks to tests/ball_lives.rs

---

## Phase 4: User Story 2 - No Hardcoded Physics Values (Priority: P2)

**Goal**: No restitution, friction, or damping values are hardcoded in ball, paddle, or brick spawn logic.

**Independent Test**: Code review and static analysis confirm no hardcoded values remain.

### Tests for User Story 2 (REQUIRED)

- [ ] T013 [P] [US2] Write static analysis test in tests/unit/physics_config.rs to detect hardcoded physics values in all spawn logic (commit: FAILING_HASH; cross-referenced by T008)

### Implementation for User Story 2

- [ ] T014 [P] [US2] Refactor paddle and brick spawn logic in src/systems/ and src/level_loader.rs to use PaddlePhysicsConfig and BrickPhysicsConfig
- [ ] T015 [US2] Add rustdoc to PaddlePhysicsConfig, BrickPhysicsConfig, and usage sites
- [ ] T016 [US2] Add Bevy 0.17 compliance checks to tests/unit/physics_config.rs

---

## Phase 5: User Story 3 - Documented and Maintainable Config (Priority: P3)

**Goal**: Ball, paddle, and brick physics configs are documented and easy to update for future tuning.

**Independent Test**: Documentation is clear, config is in a single location, and changes are reflected in new entities.

### Tests for User Story 3 (REQUIRED)

- [ ] T017 [P] [US3] Write test in tests/unit/physics_config.rs to verify config documentation and discoverability (commit: FAILING_HASH)

### Implementation for User Story 3

- [ ] T018 [P] [US3] Ensure quickstart.md and rustdoc document config location and update process
- [ ] T019 [US3] Add comments and doc links in all config usage sites

---

## Phase N: Polish & Cross-Cutting Concerns

- [ ] T020 [P] Update docs/ and specs/015-ball-physics-config/quickstart.md for config usage
- [ ] T021 [P] Refactor for code clarity and maintainability in src/physics_config.rs
- [ ] T022 [P] Run static analysis and linting to confirm no hardcoded values remain
- [ ] T023 [P] Add additional unit/integration tests for edge cases (missing/invalid config)
- [ ] T024 [P] Validate Bevy 0.17 compliance across all new/changed systems

---

## Dependencies & Execution Order

- Phase 1: Setup (T001–T002) → Phase 2: Foundational (T003–T006) → User Stories (T007–T019) → Polish (T020–T024)
- User stories are independent after foundational phase; all test tasks marked [P] can run in parallel

---

## Parallel Execution Examples

- T003, T004, T005, T006 can run in parallel
- T007, T008 can run in parallel
- T013, T014, T015, T016 can run in parallel
- T017, T018, T019 can run in parallel

---

## Implementation Strategy

- MVP: Complete User Story 1 (T007–T012) after setup/foundation, validate independently
- Incremental: Add User Story 2 (T013–T016), then User Story 3 (T017–T019), each with independent tests
- Polish: Finalize docs, refactor, and validate compliance (T020–T024)

---

*All tasks follow the strict checklist format: [ ] TXXX [P?] [US?] Description with file path*
