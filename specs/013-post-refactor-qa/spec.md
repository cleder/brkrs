# Feature Specification: Post-Refactor QA & Sanitation

**Feature Branch**: `013-post-refactor-qa` **Created**: 2025-12-20 **Status**: Draft **Input**: User description: "Perform a final 'Post-Refactor QA & Sanitation' pass on the codebase..."

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).

### User Story 1 - Test Integrity Audit (Priority: P1)

As a developer, I want to ensure that all tests in the codebase are meaningful and actually verify logic, so that I can trust the test suite's green status.

**Why this priority**: "Fake tests" (comment-only or no-op assertions) provide false confidence and hide potential regressions.

**Independent Test**: Run a script or manual check to identify and fix/remove all hollow tests.
The test suite should still pass, but with fewer (or more meaningful) tests.

**Acceptance Scenarios**:

1. **Given** the `tests/` directory, **When** scanned for tests containing only comments or `assert!(true)`/`assert_eq!(1, 1)`, **Then** all such tests are identified.
2. **Given** a "fake test" that was identified, **When** it is evaluated, **Then** it is either rewritten to assert actual state changes (if valuable) or deleted (if valueless).
3. **Given** the cleaned test suite, **When** `cargo test` is run, **Then** all tests pass and no "fake tests" remain.

---

### User Story 2 - Constitution Compliance Sweep (Priority: P1)

As a maintainer, I want to ensure the codebase adheres to the project's constitution, specifically the Bevy 0.17 mandates and prohibitions, so that the code is maintainable, performant, and idiomatic.

**Why this priority**: Strict adherence to the constitution prevents technical debt and ensures long-term project health.

**Independent Test**: Manual code review or static analysis against the `constitution.md` rules.

**Acceptance Scenarios**:

1. **Given** the codebase, **When** audited against "Bevy 0.17 Mandates", **Then** all mandates (e.g., `#[require]`, `Changed<T>`, `ChildOf::parent()`) are followed.
2. **Given** the codebase, **When** audited against "Bevy 0.17 Prohibitions", **Then** no prohibited patterns (e.g., panicking queries, broad `EntityRef` queries, `assert!(true)`) are found.
3. **Given** any violations found, **When** fixed, **Then** the code compiles and passes all tests.

---

### User Story 3 - Code Review Fixes (Priority: P2)

As a developer, I want to address outstanding code review comments, so that the codebase meets the team's quality standards.

**Why this priority**: Addressing review comments is a standard part of the development lifecycle and improves code quality.

**Independent Test**: Verify visibility of constants and execution order of startup systems.

**Acceptance Scenarios**:

1. **Given** the constants `BALL_RADIUS`, `PADDLE_RADIUS`, `PADDLE_HEIGHT`, `PLANE_H`, and `PLANE_W`, **When** checked for visibility, **Then** they are restricted as much as possible (e.g., `pub(super)` or re-exported only where needed), avoiding unnecessary public API exposure.
2. **Given** the startup systems `spawn_camera`, `spawn_ground_plane`, and `spawn_light`, **When** registered in the app, **Then** they have explicit execution ordering (e.g., in a `StartupSet` or chained) relative to `setup` or each other, ensuring deterministic initialization.

### Edge Cases

- **False Positives in Test Audit**: A test might appear "fake" (e.g., no assertions) but implicitly test that code compiles or doesn't panic.
  These should be documented or refactored to be explicit (e.g., `assert!(result.is_ok())`).
- **Legitimate Public Constants**: If a constant is required by an integration test in `tests/`, it may need to remain `pub` but hidden from docs (`#[doc(hidden)]`) or restricted to the crate (`pub(crate)`).
- **Complex Startup Dependencies**: If startup systems have complex dependencies that cannot be resolved by simple ordering, they may need to be refactored into a state-machine or use `State` transitions.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST NOT contain any tests that pass trivially (e.g., `assert!(true)` without other logic).
- **FR-002**: The system MUST NOT contain any tests that consist solely of comments.
- **FR-003**: The system MUST adhere to all "Bevy 0.17 Mandates" defined in `constitution.md`.
- **FR-004**: The system MUST NOT violate any "Bevy 0.17 Prohibitions" defined in `constitution.md`.
- **FR-005**: The constants `BALL_RADIUS`, `PADDLE_RADIUS`, `PADDLE_HEIGHT`, `PLANE_H`, and `PLANE_W` MUST have the minimum necessary visibility.
- **FR-006**: The startup systems `spawn_camera`, `spawn_ground_plane`, and `spawn_light` MUST be explicitly ordered to ensure deterministic startup.

### Success Criteria

- **SC-001**: Zero "fake tests" remain in the codebase.
- **SC-002**: Zero violations of `constitution.md` mandates/prohibitions found in the audited code.
- **SC-003**: `cargo test` passes with all changes.
- **SC-004**: Startup systems execute in a deterministic order.
