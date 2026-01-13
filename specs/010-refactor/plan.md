# Implementation Plan: UI Constitution Refactor

**Branch**: `010-refactor` | **Date**: 2025-12-19 | **Spec**: [specs/010-refactor/spec.md](spec.md)
**Input**: Feature specification from `/specs/010-refactor/spec.md`

## Summary

Bring `src/ui` into compliance with the Brkrs Constitution by:

- Producing a complete compliance audit artifact for `src/ui`
- Refactoring UI systems to satisfy Bevy 0.17 mandates/prohibitions (fallible systems, change detection, asset handle reuse, required components)
- Preserving all player-facing UI behavior (score/lives/pause/cheat indicator/level label/game over)

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1 **Storage**: N/A (in-memory ECS state only) **Testing**: `cargo test` (integration tests in `tests/`) **Target Platform**: Native + WASM **Project Type**: Single project (`src/`, `tests/`) **Performance Goals**: 60 FPS (avoid per-frame UI updates where data unchanged) **Constraints**: Minimal scope outside `src/ui` (supporting edits only when required) **Scale/Scope**: One module subtree (`src/ui`) + minimal supporting changes

## Constitution Check

*GATE: Must pass before implementation tasks begin.*
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 *Re-check after each story.*

**TDD gates (Constitution VII)**

- Tests are authored and committed before any implementation changes for each user story.
- A proof-of-failure commit (tests FAIL) exists in branch history before implementation commits.
- Tests are reviewed/approved by the requestor before implementation proceeds.

**Bevy 0.17 gates (Constitution VIII)**

- Systems are fallible (`Result`), do not panic on query outcomes (`?`, no `.unwrap()` on queries).
- UI updates use `Changed<T>` and are not executed every frame without data changes.
- Message vs Event usage is correct (`MessageReader/Writer` vs observers).
- Asset handles are loaded once and cached in Resources (no repeated `asset_server.load()` during toggles/spawns).
- Marker components use `#[require(Transform, Visibility)]` where appropriate.

## Project Structure

### Documentation (this feature)

```text
specs/010-refactor/
├── plan.md
├── spec.md
├── compliance-audit.md
├── refactoring-plan.md
├── checklists/
│   ├── requirements.md
│   ├── compliance.md
│   └── compliance-lightweight.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── ui/
└── ...

tests/
├── ...
└── editor_palette.rs
```

**Structure Decision**: Single Rust crate with Bevy ECS; changes are primarily within `src/ui`, with minimal supporting edits allowed outside when required.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |
