---

description: "Task list for brick-type-decals (014)"
---

# Tasks: Brick Type Decals (014)

<!--
User Story Mapping:
US1 = User Story 1 (Recognize Brick Type Visually)
US2 = User Story 2 (Decal Embossing/Engraving)
-->

**Input**: Design documents from `/specs/014-brick-type-decals/` **Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are MANDATORY for all user stories.
Each story MUST include unit tests and feature-level acceptance tests (integration or contract tests as appropriate).
Tests MUST be written and committed first, verified to FAIL (red), and then approved before implementation begins; record the test-proof commit hash in the task description.

**Bevy 0.17 compliance**: All ECS/rendering/UI tasks include explicit acceptance criteria for Bevy 0.17 mandates & prohibitions (no panicking queries, filtered queries, `Changed<T>` for reactive UI, message vs event correctness, asset handle reuse, and correct hierarchy APIs).

## Phase 1: Setup (Shared Infrastructure)

- [ ] T001 Create project structure per implementation plan
- [ ] T002 [P] Add Bevy, bevy_rapier3d, tracing, serde, ron dependencies to Cargo.toml
- [ ] T003 [P] Configure cargo fmt, cargo clippy, and bevy lint

---

## Phase 2: Foundational (Blocking Prerequisites)

- [ ] T004 [P] Create BrickType enum and Decal struct in src/level_format/brick_types.rs
- [ ] T005 [P] Add asset loading system for decals and normal maps in src/systems/asset_loading.rs
- [ ] T006 [P] Add test assets for decals and normal maps in assets/textures/decals/
- [ ] T007 [P] Add BrickType and Decal to test level RON in assets/levels/test_decals.ron

---

## Phase 3: User Story 1 - Recognize Brick Type Visually (Priority: P1) 🎯 MVP

**Goal**: Player can recognize brick type by a visible, type-specific decal centered on the top side of each brick.

**Independent Test**: Display all brick types in a test level and verify each has a distinct, visible decal centered on the top side.

### Tests for User Story 1 (REQUIRED)

- [ ] T008 [P] [US1] Contract test: All brick types in test level have correct decals assigned in tests/contract/test_brick_decals.rs
- [ ] T009 [P] [US1] Integration test: Decals are visible and centered in tests/integration/test_decal_rendering.rs
- [ ] T010 [P] [US1] Bevy 0.17 compliance test: No panicking queries, correct With<T>/Without<T> filters, asset handle reuse in tests/ui_compliance_audit.rs

### Implementation for User Story 1

- [ ] T011 [P] [US1] Implement assign_brick_decals system in src/systems/brick_decals.rs
- [ ] T012 [US1] Integrate decal assignment into level loading in src/level_loader.rs
- [ ] T013 [US1] Add fallback/default decal logic for missing types/assets in src/systems/brick_decals.rs

---

## Phase 4: User Story 2 - Decal Embossing/Engraving (Priority: P2)

**Goal**: Player sees embossed/engraved effect on decals using normal/bump mapping.

**Independent Test**: Inspect bricks in test level and confirm decals have 3D embossed/engraved appearance using normal/bump mapping.

### Tests for User Story 2 (REQUIRED)

- [ ] T014 [P] [US2] Contract test: Normal/bump mapping is applied and visible under lighting in tests/contract/test_decal_normals.rs
- [ ] T015 [P] [US2] Integration test: 3D effect of decals is consistent from different angles in tests/integration/test_decal_normals.rs
- [ ] T016 [P] [US2] Bevy 0.17 compliance test: No per-frame UI updates without Changed<T>, correct asset handle reuse in tests/ui_compliance_audit.rs

### Implementation for User Story 2

- [ ] T017 [P] [US2] Implement normal/bump mapping for decals in src/systems/brick_decals.rs
- [ ] T018 [US2] Add/update normal map assets for decals in assets/textures/decals/
- [ ] T019 [US2] Integrate normal map assignment into assign_brick_decals system in src/systems/brick_decals.rs

---

## Phase 5: Polish & Cross-Cutting Concerns

- [ ] T020 [P] Documentation updates in docs/architecture.md and docs/ui-systems.md
- [ ] T021 Code cleanup and refactoring in src/systems/brick_decals.rs
- [ ] T022 Performance optimization for decal rendering in src/systems/brick_decals.rs
- [ ] T023 [P] Additional unit tests for edge cases in tests/unit/test_brick_decals.rs
- [ ] T024 Run quickstart.md validation steps

---

## Dependencies & Execution Order

### Phase Dependencies

- Setup (Phase 1): No dependencies
- Foundational (Phase 2): Depends on Setup completion
- User Stories (Phase 3+): Depend on Foundational phase completion; can proceed in parallel
- Polish (Final Phase): Depends on all user stories being complete

### User Story Dependencies

- User Story 1 (P1): Can start after Foundational (Phase 2)
- User Story 2 (P2): Can start after Foundational (Phase 2); independently testable

### Parallel Execution Examples

- Asset loading, enum/struct creation, and test asset setup (T004–T007) can be done in parallel
- All test tasks (T008–T010, T014–T016) can be written in parallel before implementation
- Implementation tasks for US1 and US2 can proceed in parallel after foundational phase

### MVP Scope

- User Story 1 (Recognize Brick Type Visually) and its tests/implementation

---

## Implementation Strategy

- MVP first: Deliver User Story 1 with full test coverage and Bevy 0.17 compliance
- Incremental: Add normal/bump mapping and polish in subsequent phases
- All tasks follow strict checklist format: `- [ ] TXXX [P] [USX] Description with file path`
