# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Refactor the monolithic `setup` function in `src/lib.rs` by extracting entity spawning logic (camera, ground plane, light) into dedicated functions within a new module `src/systems/spawning.rs`.
These functions will be registered as individual startup systems, improving modularity and testability.

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0 **Storage**: N/A (In-memory ECS) **Testing**: `cargo test` **Target Platform**: Linux (Native) + WASM **Project Type**: Single project (Rust binary) **Performance Goals**: 60 FPS (Refactor must not degrade startup time significantly) **Constraints**: Bevy 0.17 mandates (fallible systems, no panics, strict query safety) **Scale/Scope**: Refactoring ~50 lines of code in `src/lib.rs` into a new module.

## Constitution Check

*GATE: Must pass before Phase 0 research.*
         *Re-check after Phase 1 design.*

This check MUST verify compliance with the constitution, including **Test-Driven Development (TDD)** gates:

- [x] Tests are defined and committed prior to implementation efforts for each story/feature.
- [x] A proof-of-failure commit (tests that FAIL) MUST exist in the branch history prior to implementation commits.
- [x] Tests MUST be reviewed and approved by the feature owner or requestor before implementation begins.

This check MUST also verify compliance with **Bevy 0.17 mandates & prohibitions** (if the feature touches ECS, rendering, assets, or scheduling):

- [x] Systems are fallible (`Result`) and do not panic on query outcomes (`?`, no `.unwrap()` on queries).
- [x] Queries use `With<T>`/`Without<T>` filters and `Changed<T>` where appropriate (especially UI).
- [x] Message vs Event usage follows Bevy 0.17 APIs (`MessageWriter/Reader` vs observers).
- [x] Assets are loaded once and handles are stored in Resources (no repeated `asset_server.load()` in loops).
- [x] Hierarchies use `ChildOf::parent()` and `add_children()`/`remove::<Children>()` patterns.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
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
├── lib.rs               # Modified: Remove spawning logic, register new systems
└── systems/
    ├── mod.rs           # Modified: Export spawning module
    └── spawning.rs      # New: Contains spawn_camera, spawn_ground_plane, spawn_light
```

**Structure Decision**: Single project (Rust binary) with modular systems.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
