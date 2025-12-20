# Feature Specification: Refactor Entity Spawning

**Feature Branch**: `012-refactor-entity-spawning` **Created**: 2025-12-20 **Status**: Draft **Input**: User description: "The spawning of the main camera, ground plane, and related entities is performed directly within the setup function.
This approach reduces modularity and makes testing or reusing entity spawning logic more difficult.
Recommendation: Extract the entity spawning logic (e.g., camera, ground plane, light) into separate, dedicated functions.
This will improve maintainability, testability, and allow for easier extension or modification of the entity setup process. in src/lib.rs"

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).

### User Story 1 - Modular Entity Spawning (Priority: P1)

As a developer, I want the entity spawning logic to be modular so that I can easily maintain, test, and extend the game setup.

**Why this priority**: This is a foundational refactoring to improve code quality and maintainability.

**Independent Test**: Verify that the game scene (camera, light, ground) is identical before and after the refactor.

**Acceptance Scenarios**:

1. **Given** the game application is starting, **When** the `setup` system runs, **Then** a `PointLight` entity is spawned with the correct properties (intensity, range, shadow settings).
2. **Given** the game application is starting, **When** the `setup` system runs, **Then** a `GroundPlane` entity is spawned with the correct mesh and material.
3. **Given** the game application is starting, **When** the `setup` system runs, **Then** a `Camera3d` entity is spawned at the correct position and looking at the origin.
4. **Given** the code structure, **When** examining `src/lib.rs`, **Then** `spawn_camera`, `spawn_ground_plane`, and `spawn_light` functions exist and are called by `setup`.
5. **Given** the `MainCamera` component, **When** it is used in other systems, **Then** it is accessible (moved to module scope).

## Clarifications

### Session 2025-12-20

- Q: Where should the extracted functions be located? → A: Move to a new module (e.g., `src/systems/spawning.rs`) for better separation.
- Q: How should the new functions be invoked? → A: Register as individual startup systems (e.g., `.add_systems(Startup, (spawn_camera, ...))`).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST extract camera spawning logic into a dedicated `spawn_camera` function in a new module `src/systems/spawning.rs`.
- **FR-002**: The system MUST extract ground plane spawning logic into a dedicated `spawn_ground_plane` function in a new module `src/systems/spawning.rs`.
- **FR-003**: The system MUST extract light spawning logic into a dedicated `spawn_light` function in a new module `src/systems/spawning.rs`.
- **FR-004**: The `MainCamera` component struct definition MUST be moved to the new module (or appropriate shared location) and made public.
- **FR-005**: The application MUST register `spawn_camera`, `spawn_ground_plane`, and `spawn_light` as `Startup` systems (replacing the monolithic `setup` call for these parts).
- **FR-006**: The refactoring MUST NOT change the visual appearance or behavior of the game.
- **FR-007**: The `setup` function (or a renamed equivalent like `configure_physics`) MUST continue to set the gravity configuration.

### Assumptions

- The `setup` function signature might change or the new functions might need to accept `Commands`, `ResMut<Assets<Mesh>>`, etc.
- The `MainCamera` component is intended to be public or at least module-visible.

## Success Criteria

- **Measurable**: The `setup` function length is reduced by at least 20 lines.
- **Technology-agnostic**: The game scene initializes correctly with all required entities.
- **User-focused**: Developers can locate entity spawning logic in dedicated functions.
- **Verifiable**: Unit tests or integration tests confirm the existence of the entities.
