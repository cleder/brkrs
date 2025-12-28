
# Feature Specification: Ball Physics Config

**Feature Branch**: `015-ball-physics-config` **Created**: 2025-12-25 **Status**: Draft **Input**: User description: "Currently, restitution, friction, and damping and other values for the Ball are hardcoded directly into the spawn command in level_loader.rs.
This can lead to configuration drift, makes tuning harder, and increases the risk of inconsistent ball behavior if these values are changed in only some places.
Other entities may have similar issues, check and suggest if remediation should be done now or in a separate specification.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Consistent Ball Physics (Priority: P1)

As a developer or game designer, I want all balls in the game to use a single, centralized set of physics properties (restitution, friction, damping, etc.), so that tuning and maintaining ball behavior is easy and consistent.

**Why this priority**: Prevents configuration drift and ensures gameplay consistency, which is critical for both player experience and maintainability.

**Independent Test**: Can be fully tested by spawning multiple balls and verifying that all use the same physics values, and by confirming that changing the config updates all new balls.

**Acceptance Scenarios**:

1. **Given** a new ball is spawned, **When** inspecting its physics properties, **Then** restitution, friction, and damping match the centralized config.
2. **Given** the config is updated, **When** a new ball is spawned, **Then** the new values are applied to the ball's physics.
3. **Given** multiple balls are spawned in different ways, **When** inspecting their physics, **Then** all use the same config values.

---

### User Story 2 - No Hardcoded Physics Values (Priority: P2)

As a maintainer, I want to ensure that no restitution, friction, or damping values are hardcoded in the ball spawn logic, so that all changes are made in one place.

**Why this priority**: Reduces risk of bugs and makes future tuning straightforward.

**Independent Test**: Can be tested by code review and static analysis to confirm no hardcoded values remain in ball spawn code.

**Acceptance Scenarios**:

1. **Given** a codebase update, **When** reviewing ball spawn logic, **Then** no hardcoded restitution, friction, or damping values are present.

---

### User Story 3 - Documented and Maintainable Config (Priority: P3)

As a developer, I want the ball physics config to be documented and easy to update, so that future tuning is simple and safe.

**Why this priority**: Ensures long-term maintainability and knowledge transfer.

**Independent Test**: Can be tested by reviewing documentation and updating the config to see if changes are reflected in new balls.

**Acceptance Scenarios**:

1. **Given** a new developer joins the project, **When** they look for ball physics settings, **Then** they find clear documentation and a single config location.
2. **Given** the config is updated, **When** a new ball is spawned, **Then** the new values are used.

---

### Edge Cases

- What happens if the config is missing or invalid?
  The system MUST panic or halt the game with a clear, actionable error message, forcing an immediate fix and preventing silent misconfiguration.
- The configuration is not hot-reloadable and is encoded in source only; runtime updates or live tuning are not supported.
- All entities with physics properties (e.g., paddle, bricks) MUST have their physics configs centralized in this spec, eliminating hardcoded values for all such entities.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-008**: The physics configuration MUST NOT be hot-reloadable; it MUST be encoded in source code and only changeable via code updates.

- **FR-001**: System MUST provide a single, centralized configuration for ball physics properties (restitution, friction, damping, etc.).
- **FR-002**: All ball entities MUST use the centralized config for their physics properties when spawned.
- **FR-003**: No hardcoded restitution, friction, or damping values MAY exist in ball spawn logic.
- **FR-004**: The config location and usage MUST be documented for maintainability.
- **FR-005**: Changes to the config MUST be reflected in all subsequently spawned balls.
- **FR-006**: If the physics config is missing or invalid, the system MUST panic or halt the game with a clear, actionable error message.
- **FR-007**: The system MUST centralize physics configs for all entities with physics properties (e.g., paddle, bricks) in this specification, removing all hardcoded values for such entities.

### Key Entities

- **BallPhysicsConfig**: Represents the centralized set of physics properties for all balls.
  All gameplay-relevant fields (e.g., restitution, friction, linear_damping, angular_damping) MUST be explicitly listed and documented.
  Extension fields are allowed but must be justified and documented in code and spec.
- **Ball**: The game entity that uses BallPhysicsConfig for its physics properties when spawned.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of ball entities spawned use the centralized physics config (verified by test or inspection).
- **SC-002**: No hardcoded restitution, friction, or damping values exist in ball spawn code (verified by code review/static analysis).
- **SC-003**: Developers can update ball physics by changing a single config location, and changes are reflected in new balls within 5 minutes.
- **SC-004**: Documentation for ball physics config is clear and discoverable by new team members.
