# Feature Specification: Refactor Systems for Constitution Compliance

**Feature Branch**: `011-refactor-systems` **Created**: 2025-12-19 **Status**: Draft **Input**: Refactor src/systems to align with Constitution 1.3.0 mandates and prohibitions: fallible systems (Result+?), message/event separation, replace tuple chaining with System Sets, add change detection filters, enforce required components with # [require], error recovery patterns, unify BrickDestroyed and UiBeep into Messages, and improve query specificity and parallelism.

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.

  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Constitution-Compliant Systems (Priority: P1)

As a maintainer, I want all gameplay systems to comply with the Constitution so they are safe, predictable, and testable across platforms.

**Why this priority**: This refactor prevents regressions (panics, over-chained systems, uncontrolled per-frame work) and unblocks future feature work.

**Independent Test**: Run the test suite and targeted integration tests to verify no panics on expected missing data, change-driven updates, and correct message/event flows.

**Acceptance Scenarios**:

1. Fallible systems
   - Given a system that encounters expected missing data (e.g., `AssetServer` absent in a minimal test app), When the system runs, Then it returns gracefully without panic and the app continues updating.
2. Error recovery
   - Given respawn execution with no pending request, When `respawn_executor` runs, Then it performs no action and does not panic.
3. Change-driven updates
   - Given a paddle with an active size effect that does not change this frame, When `update_paddle_visual_feedback` runs, Then it does not modify materials this frame (no-op without changes).
4. No tuple `.chain()` inside system lists
   - Given configured schedules, When the app is constructed, Then no system registrations use tuple `.chain()`; ordering is expressed via system sets and `.after()`/`.before()` between sets only.

---

### User Story 2 - Clear Message vs Event Boundaries (Priority: P2)

As a maintainer, I want a single, consistent messaging pattern so cross-feature communication is unambiguous and testable.

**Why this priority**: Prevents dual semantics and runtime surprises, simplifies testing and tool integration.

**Independent Test**: Simulate message producers and verify consumers observe exactly one path (Message or Event) with no dual-derive types.

**Acceptance Scenarios**:

1. Ui beep flow
   - Given a blocked level switch keypress in cheat-disabled state, When the input system runs, Then exactly one `UiBeep` message is written and an audio consumer processes it (no `Event` observer path exists for this signal).
2. Brick destroyed flow
   - Given a brick-destroy producer, When it emits `BrickDestroyed`, Then scoring and audio consume the same unified Message type; there is not a duplicate Event type of the same name.

---

### User Story 3 - Performance-Safe Updates (Priority: P3)

As a player, I want smooth gameplay at target frame rates so interactions feel responsive.

**Why this priority**: Eliminates unnecessary per-frame work and leverages parallelism via correctly organized sets.

**Independent Test**: Measure that previously per-frame updates now run only on change and that system ordering preserves game behavior.

**Acceptance Scenarios**:

1. Grid overlay visibility
   - Given wireframe mode toggles, When `toggle_grid_visibility` runs, Then visibility changes only when wireframe state changes, not every frame.
2. Canonical materials application
   - Given canonical handles become ready, When they change from not-ready to ready, Then materials are applied once, not continuously each frame.

---

[Add more user stories as needed, each with an assigned priority]

### Edge Cases

- Missing or partial resources at startup (e.g., `AssetServer`, `Messages<AssetEvent<Image>>` unavailable in minimal test app) must not cause panics; systems no-op gracefully.
- WASM environment without file system must not attempt blocking file I/O; configuration and manifests load via supported mechanisms only.
- Multiple brick-destroy events in a single frame must result in exactly one score update per event and a single audio play per brick, respecting concurrency limits.

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001 (Fallible Systems)**: All systems in `src/systems/**` MUST be fallible and propagate errors using `anyhow::Result<()>` and early-return patterns; no panicking unwraps in system logic.
  Tests verify graceful no-op on expected missing resources. (Constitution VIII: Fallible Systems; Prohibitions: NO Panicking Queries)
- **FR-002 (Message/Event Separation)**: Each cross-feature signal MUST choose a single mechanism and be defined in a shared `crate::signals` module. `UiBeep` and `BrickDestroyed` MUST be Messages (buffered) and NOT derive or be consumed as Events.
  Tests verify only one mechanism is registered/consumed and that no duplicate definitions exist across modules. (Constitution VIII: Message vs Event Distinction)
- **FR-003 (System Set Organization)**: Systems MUST be organized into enums with `*Systems` suffix and ordered only via `.configure_sets()` and `.after()`/ `.before()` between sets; tuple `.chain()` inside a system list is not permitted.
  Tests assert expected ordering via observable state (not tuple chaining). (Constitution VIII: System Organization; Prohibitions: NO Over-Chaining Systems)
- **FR-004 (Change Detection)**: Systems updating UI/materials/components MUST use strict change-driven triggers (`Changed<T>`, `RemovedComponents<T>`, `OnAdd`) and MUST NOT use periodic fallback ticks.
  Tests confirm no changes occur when inputs are unchanged. (Constitution VIII: Change Detection; Prohibitions: NO Universal Query Updates)
- **FR-005 (Required Components)**: Marker components for core entities MUST require `Transform` and `Visibility` via attributes, and spawns MUST avoid redundant bundles.
  Apply now to: `Paddle`, `Ball`, `GridOverlay`, `Border`, `GroundPlane`.
  Tests ensure marker-only spawns include required components and existing spawns no longer redundantly add `Transform`/ `Visibility`. (Constitution VIII: Required Components; Prohibitions: NO Manual Component Bundles)
- **FR-006 (Error Recovery)**: Expected failures (missing optional data, optional parents) MUST use `let Some(..) = .. else { return Ok(()) }` or `let Ok(..) = .. else { return Ok(()) }`.
  Tests cover respawn executor, audio asset access, and input gating. (Constitution VIII: Error Recovery Patterns)
- **FR-007 (Query Specificity & Parallelism)**: Queries with `&mut Transform` MUST use specific filters (`With<T>`, `Without<T>`) to avoid conflicts; parallelizable systems grouped in same set.
  Verified by existing integration tests and absence of unsafe broad queries. (Constitution VIII: Query Specificity)
- **FR-008 (Asset Handle Reuse)**: Asset handles MUST be loaded once and reused via resources; no repeated `asset_server.load()` calls inside spawn loops.
  Tests check no per-spawn loading. (Constitution VIII: Asset Handle Reuse; Prohibitions: NO Repeated Asset Loading)
- **FR-009 (Engine Events via Observers)**: Engine-originated signals such as `AssetEvent<Image>` MUST be consumed as Events via observers, not Messages, and MUST NOT use `MessageReader` wrappers or guards.
  Tests verify observer reception and absence of Message registration. (Constitution VIII: Message vs Event Distinction)

### Key Entities *(include if feature involves data)*

- **Module: crate::signals** (implemented in `src/signals.rs`): Shared home for cross-feature signals (Messages only).
- **Signal: UiBeep (Message)**: Developer-intent signal for short UI feedback; buffered, consumed by audio.
- **Signal: BrickDestroyed (Message)**: Unified brick-destroy domain signal in `crate::signals`; consumed by scoring and audio.
- **System Sets**: `AudioSystems`, `PaddleSizeSystems`, `TextureOverrideSystems`, and `RespawnSystems` (existing) define ordered, parallelizable groups.

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: Running the full test suite demonstrates zero panics across systems when expected resources are absent; all new integration tests pass.
- **SC-002**: No systems run per-frame when their inputs have not changed (verified by targeted tests that observe no state changes over N frames without input changes).
- **SC-003**: Message flows for `UiBeep` and `BrickDestroyed` are single-path (messages only) and verified by producer/consumer tests; duplicate derivations do not exist.
- **SC-004**: Schedule configuration shows no tuple `.chain()` usage in system registrations; ordering validated by scenario-based tests (e.g., respawn visual trigger before animate).

## Assumptions

- Unification will standardize on Messages for gameplay-domain cross-feature signals to maximize buffered processing and testability.
- Required-component attributes will be added to marker components in their defining modules and referenced by systems (minor cross-module coordination).
- Success criteria are verified without requiring specific framework internals; they rely on behavior observation via tests.

## Clarifications

### Session 2025-12-19

- Q: What concrete error type should fallible systems return? → A: Use anyhow::Result<()>.
- Q: Where should `BrickDestroyed` live and by which mechanism? → A: Centralize as `#[derive(Message)]` in `crate::signals`; remove duplicates in `audio` and `scoring` and update all producers/consumers to the shared type.
- Q: How should `AssetEvent<Image>` be handled? → A: Treat as Event consumed via observers; remove MessageReader usage and related guards.
- Q: Should texture/material updates use periodic fallbacks? → A: No.
  Use strict change-driven triggers only (`Changed<T>`, `RemovedComponents<T>`, `OnAdd`).
- Q: Apply `#[require(Transform, Visibility)]` now and to which markers? → A: Apply now to `Paddle`, `Ball`, `GridOverlay`, `Border`, `GroundPlane`.
