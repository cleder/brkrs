# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Centralize all physics configuration (restitution, friction, damping, etc.) for balls, paddles, and bricks in dedicated Bevy resource structs, defined in source code.
All spawn logic for these entities must use the relevant config resource, eliminating hardcoded values and ensuring maintainability and consistency.
No config is hot-reloadable or loaded from files.
Static analysis and integration tests enforce compliance.
Extension fields in configs are allowed but must be justified and documented.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1 **Storage**: N/A (in-memory ECS state only) **Testing**: cargo test, cargo clippy, bevy lint **Target Platform**: Linux, Windows, macOS, WASM (Chrome/Firefox) **Project Type**: single (game, ECS-based) **Performance Goals**: 60 FPS on target hardware (native + WASM) **Constraints**: No panicking queries, no hardcoded physics values, config not hot-reloadable, must follow Bevy 0.17 mandates **Scale/Scope**: Applies to all ball, paddle, and brick entities with physics properties in the game

## Constitution Check

*GATE: Must pass before Phase 0 research.*
                                                                                                                                                                                                                                                                                                                         *Re-check after Phase 1 design.*

**TDD Compliance Gates:**

- All tests for each user story/feature MUST be written and committed before implementation.
- A proof-of-failure commit (tests that FAIL) MUST exist in the branch history before implementation.
- Tests MUST be reviewed and approved by the feature owner or requestor before implementation begins.

**Bevy 0.17 Mandates & Prohibitions:**

- Systems must be fallible and must not panic on query outcomes (no `.unwrap()` on queries).
- Queries must use `With<T>`/`Without<T>` filters and `Changed<T>` where appropriate.
- Message-Event Separation: Use `MessageWriter` for buffered streams, `Trigger<T>` for immediate logic; never conflate.
- Assets must be loaded once and handles stored in Resources (no repeated `asset_server.load()` in loops).
- Hierarchies must use `ChildOf::parent()` and `add_children()`/`remove::<Children>()` patterns.
- No hardcoded physics values in spawn logic; all configs centralized per spec.
- No hot-reloadable config; all config encoded in source only.

All gates are derived from the current constitution (v1.3.2, ratified 2025-12-25).

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
├── level_loader.rs
├── lib.rs
├── main.rs
├── pause.rs
├── signals.rs
├── level_format/
├── systems/
├── ui/

tests/
├── ball_lives.rs
├── ball_material_startup.rs
├── scoring.rs
├── paddle_shrink.rs
├── paddle_size_powerups.rs
├── ...

assets/
├── levels/
├── audio/
├── fonts/
├── textures/

specs/015-ball-physics-config/
├── spec.md
├── plan.md
├── checklists/

```

**Structure Decision**: This feature will be implemented in the main game code under `src/`, with tests in `tests/`, and documentation/specs in `specs/015-ball-physics-config/`.
No new project or submodule is required; all changes are within the existing ECS-based game structure.

## Complexity Tracking

No Constitution Check violations.
All requirements and gates are satisfied by this plan and the feature specification.
