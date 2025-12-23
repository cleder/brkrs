# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

This feature adds visible, type-specific decals to the top of every brick in the game, supporting normal/bump mapping for embossed or engraved effects.
Decals are assigned via ECS systems during level loading, with assets managed in the standard Bevy way.
The approach leverages Bevy's material and asset systems, ensures all brick types are covered, and provides fallbacks for missing assets.
All work is TDD-first and Bevy 0.17 compliant.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1, serde 1.0, ron 0.8 **Storage**: N/A (in-memory ECS state only) **Testing**: cargo test, bevy lint, cargo clippy **Target Platform**: Linux, Windows, macOS, WASM **Project Type**: single (Bevy ECS game) **Performance Goals**: 60 FPS on target hardware, smooth decal rendering **Constraints**: No panicking queries, no per-frame UI updates without Changed : No panicking queries, no per-frame UI updates without Changed<T>, asset handles reused, ECS-only state, WASM compatible **Scale/Scope**: All brick types in game, extensible for future types

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**TDD Compliance:**

- All user stories have independently testable acceptance criteria.
- Tests must be written and committed before implementation (red phase required).
- A failing-test commit must exist in branch history before implementation.
- Tests must be reviewed and approved by feature owner/requestor before implementation.

**Bevy 0.17 Mandates & Prohibitions:**

- ECS-only state, no panicking queries, no unwraps on queries.
- Use With<T>/Without<T> and Changed<T> filters for queries.
- Message/Event distinction must be followed.
- Asset handles loaded once and reused from Resources.
- Hierarchies use ChildOf::parent() and add_children()/remove::<Children>().
- No per-frame UI updates without Changed<T>.
- No repeated asset loading in spawn systems.
- All code must be WASM compatible and cross-platform.

**Gate Status (Post-Design):** All gates remain satisfied after design.
No violations or clarifications required.
The plan is ready for implementation.

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

ios/ or android/

```text
src/
  level_loader.rs
  lib.rs
  main.rs
  pause.rs
  signals.rs
  level_format/
  systems/
  ui/
tests/
  (all test modules)
assets/
  levels/
  textures/
  fonts/
  audio/
specs/
  014-brick-type-decals/
    plan.md
    spec.md
    checklists/
```

**Structure Decision**: Single-project Bevy ECS game.
All feature code will be added to src/systems/ and src/ui/ as appropriate, with assets in assets/ and tests in tests/.

## Complexity Tracking

No violations or complexity justifications required; all constitution gates are satisfied.
