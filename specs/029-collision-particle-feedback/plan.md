# Implementation Plan: Collision Particle Feedback

**Branch**: `029-collision-particle-feedback` | **Date**: 2026-05-24 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/029-collision-particle-feedback/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add immediate sparkly collision feedback for ball impacts against walls, paddle, and bricks.
Feedback must spawn at exact collision contact points, emit 8-16 particles per qualifying collision, complete within 0.20-0.35 seconds, remain active during burst collisions with no per-frame cap, and remain suppressed during pause/non-playing states without replay on resume.

Implementation will integrate with the existing collision flow in `src/lib.rs` (`detect_ball_wall_collisions`, `mark_brick_on_ball_collision`, and `read_character_controller_collisions`) using Bevy 0.17 observers for immediate VFX spawning and ECS-based short-lived effect entities for deterministic cleanup.

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1, rand 0.x **Storage**: N/A (in-memory ECS state; no persistent storage) **Testing**: `cargo test` (unit + integration in `tests/`), with TDD red/green commits required **Target Platform**: Native desktop (Linux/macOS/Windows) and WASM (`wasm32-unknown-unknown`) **Project Type**: Single Rust game project (Bevy + Rapier ECS app) **Performance Goals**: Maintain 60 FPS; spawn visual feedback in same frame as collision processing **Constraints**: Exact contact-point spawn; 8-16 particles/collision; 0.20-0.35s lifetime; no per-frame cap; suppress in paused/non-playing states; no replay backlog **Scale/Scope**: One new collision-feedback subsystem, integration at three collision trigger points, and focused integration tests for burst/pause/brick-destruction paths

## Constitution Check

*GATE: Must pass before Phase 0 research.*
*Re-check after Phase 1 design.*

This check MUST verify compliance with the constitution, including **Test-Driven Development (TDD)** gates:

- Tests are defined and committed prior to implementation efforts for each story/feature.
- A proof-of-failure commit (tests that FAIL) MUST exist in the branch history prior to implementation commits.
- Tests MUST be reviewed and approved by the feature owner or requestor before implementation begins.

This check MUST also verify compliance with **Bevy 0.17 mandates & prohibitions** (if the feature touches ECS, rendering, assets, or scheduling):

- **Bevy Event System Guidance:**
  - For any feature using events, messages, or observers, the plan MUST explicitly state which system is used (Messages vs Observers) and why, referencing the constitution's "Bevy 0.17 Event, Message, and Observer Clarification" section.
  - Justify the choice (e.g., "Messages for batchable, cross-frame work; Observers for immediate, reactive logic").
- **Coordinate System Guidance (if feature involves spatial movement/physics):**
  - Plan MUST specify which axes are used for movement (XZ plane for horizontal, Y for vertical, etc.).
  - Clarify whether directional terms (forward/backward) refer to Bevy's Transform API (-Z forward), gameplay perspective, or direct axis references.
  - Document any `LockedAxes` constraints and their relationship to camera orientation.
- **Initialization System Idempotence (if feature has loader/initializer systems in Update):**
  - Systems that initialize or load state in `Update` schedule MUST be idempotent.
  - Use a guard field (e.g., `last_level_number: Option<u32>`) to track whether initialization has occurred.
  - ONLY perform initialization when context changes (e.g., level transition), NOT every frame.
  - This prevents runtime state changes (e.g., gravity from brick destruction) from being overwritten.
  - See 020-gravity-bricks retrospective for the bug this pattern prevents.
- **Multi-Frame Persistence Testing (if feature modifies runtime state):**
  - Tests for runtime state changes MUST verify persistence across multiple `app.update()` cycles.
  - Tests MUST include ALL systems that write to the affected resource to catch per-frame overwrites.
  - Minimum 10 frames of persistence checking recommended.
- Systems are fallible (`Result`) and do not panic on query outcomes (`?`, no `.unwrap()` on queries).
- Queries use `With<T>`/`Without<T>` filters and `Changed<T>` where appropriate (especially UI).
- **Message-Event Separation**: Verify correct use of `MessageWriter/Reader` for buffered, frame-agnostic streams and observers/`Trigger<T>` for immediate, reactive logic (e.g., UI/sound triggers).
- Assets are loaded once and handles are stored in Resources (no repeated `asset_server.load()` in loops).
- Hierarchies use `ChildOf::parent()` and `add_children()`/`remove::<Children>()` patterns.

### Pre-Phase 0 Gate Review

**Gate status**: PASS

- TDD-first: Implementation will not begin until failing tests are authored and committed.
- Bevy event/message separation: Immediate collision visuals will use observers/trigger events; buffered messages remain for frame-agnostic streams.
- Coordinate system: Contact points and particle spread are expressed with direct axis/collision-normal references on gameplay XZ plane.
- Query safety: No panicking query patterns (`unwrap` on queries) are planned; filtered queries required.
- Hierarchy safety: Any parent/child VFX links (if needed) will use Bevy relationship APIs only.
- Asset loading: Effect materials/meshes will be loaded/initialized once and reused.
- Initialization idempotence: Feature does not require Update-schedule initialization loops; no unconditional per-frame state overwrite pattern introduced.
- Multi-frame persistence: Runtime behavior tests will cover repeated `app.update()` frames where applicable (effect lifetime, pause suppression, burst behavior).

### Post-Phase 1 Re-Check

**Gate status**: PASS

- Phase 0 research decisions preserve observer-first immediate reactions and avoid message misuse.
- Data model keeps runtime effect state in ECS components/resources (no static mutable/shared non-ECS state).
- Contracts define same-frame trigger semantics and pause suppression invariants.
- Quickstart enforces TDD ordering and specifies multi-frame checks for lifetime/pause behavior.

## Project Structure

### Documentation (this feature)

```text
specs/029-collision-particle-feedback/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── lib.rs                               # Existing collision entry points to integrate
├── signals.rs                           # Event/message contracts
└── systems/
    ├── mod.rs                           # System registration
    ├── sets.rs                          # Scheduling groups (if extension needed)
    └── collision_feedback.rs            # NEW: collision VFX observer + lifecycle systems

tests/
├── collision_particle_feedback.rs       # NEW: acceptance/integration behavior tests
└── multi_frame_persistence.rs           # Existing persistence pattern reference/extension

assets/
├── textures/                            # Reused materials/textures for sparkly feedback
└── levels/                              # Existing level assets used for integration tests
```

**Structure Decision**: Single-project Bevy game layout; add one focused system module under `src/systems/` and one dedicated integration test file under `tests/`.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |
