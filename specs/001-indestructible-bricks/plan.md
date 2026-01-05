# Implementation Plan: Indestructible bricks (LevelDefinition)

**Branch**: `001-indestructible-bricks` | **Date**: 2025-11-28 | **Spec**: spec.md
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 1.81 **Primary Dependencies**: Bevy 0.17, bevy_rapier3d, serde (ron + serde), bevy_ecs **Storage**: Files (LevelDefinition assets stored in assets/levels/ *.ron) * *Testing* *: cargo test (unit + integration) and manual gameplay tests (native + WASM) * *Target Platform* *: Native desktop (Linux, macOS, Windows) and WASM for web builds * *Project Type* *: Single (game engine codebase) * *Performance Goals* *: Maintain 60 FPS on target platforms; low-latency collision handling for physics-driven gameplay * *Constraints* *: Must adhere to project's Constitution (ECS-first, physics-driven, cross-platform).*
*Changes should not add large allocations in hot loops or break WASM compatibility * *Scale/Scope**: Small feature-level change within the existing codebase — touching level parsing & brick systems, unit tests, sample assets

## Constitution Check

GATE: Must pass before Phase 0 research.
Re-check after Phase 1 design.

Checkpoints (must be satisfied or explicitly justified):

- ECS-first: Implementation MUST be expressed as systems + components. (OK)
- Physics-driven: Brick behaviour MUST be handled via collisions and physics events where applicable. (OK)
- Modular feature: Add new brick behaviour as a self-contained system, toggleable and testable. (OK)
- Performance: No additional per-frame allocations in hot loops, tests must confirm 60 FPS preserved. (OK - to be validated by profiling)

No gates are violated by the proposed plan — proceed to Phase 1.

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

<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: Project will use existing repository structure.
Key areas impacted:

- `src/level_loader.rs` — level parsing and mapping of tile indices
- `src/systems/` — new or updated system modules: `respawn.rs`, `level_switch.rs`, and a new `indestructible.rs` system file to coordinate brick behaviour
- `assets/levels/` — migration of levels under this directory during landing

The implementation will follow the existing layout and add unit/integration tests under `tests/`. directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
