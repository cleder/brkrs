# Implementation Plan: Level Metadata (Description and Author)

**Branch**: `007-level-metadata` | **Date**: 2025-12-06 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/007-level-metadata/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add optional `description` and `author` fields to the `LevelDefinition` RON structure.
The description field enables level designers to document design intent, unique features, and gameplay characteristics.
The author field supports contributor attribution using either plain string names or Markdown link format (extracting name from `[Name](url)`).
Both fields are documentation-only (not displayed during gameplay) and maintain backward compatibility with existing level files.

## Technical Context

**Language/Version**: Rust 1.81 (Rust 2021 edition)  
**Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, serde 1.0, ron 0.8  
**Storage**: RON files in `assets/levels/` directory  
**Testing**: `cargo test` (unit and integration tests)  
**Target Platform**: Native (Linux/Windows/macOS) and WASM (web)  
**Project Type**: Single project (game)  
**Performance Goals**: 60 FPS gameplay, no measurable impact from metadata parsing  
**Constraints**: Backward compatible with existing level files, WASM-compatible  
**Scale/Scope**: ~10 level files currently, designed to scale to 100+ levels

## Constitution Check

*GATE: Must pass before Phase 0 research.*
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 *Re-check after Phase 1 design.*

✅ **I.**
**ECS-First Architecture**: This feature adds data fields to the `LevelDefinition` struct (deserialized from RON) which is consumed by existing ECS systems.
No new systems required - existing level loader systems will handle the new optional fields transparently.

✅ **II.**
**Physics-Driven Gameplay**: Not applicable - this feature adds metadata fields only, does not affect physics or gameplay mechanics.

✅ **III.**
**Modular Feature Design**: This feature extends the existing level loading module without introducing new dependencies.
Changes are isolated to the `LevelDefinition` struct and documentation.
Backward compatibility ensures existing levels load unchanged.

✅ **IV.**
**Performance-First Implementation**: Parsing two optional string fields during level load (one-time cost per level) has negligible performance impact.
No hot loop changes.
No new allocations during gameplay.

✅ **V.**
**Cross-Platform Compatibility**: RON deserialization with serde works identically on native and WASM.
No platform-specific code required.
Markdown parsing (if implemented) uses standard string operations available on all platforms.

✅ **VI.**
**Comprehensive Rustdoc**: Will document new fields on `LevelDefinition` struct explaining their purpose and format.
Update module-level docs in `level_loader.rs` to describe metadata fields.

**Result**: All constitution principles satisfied.
No violations to justify.

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
├── level_loader.rs          # Contains LevelDefinition struct - ADD description/author fields
├── level_format/            # Helper functions for level data
└── systems/                 # Game systems (unmodified for this feature)

assets/
└── levels/
    ├── README.md            # UPDATE with description/author field documentation
    ├── level_001.ron        # OPTIONALLY update with example metadata
    ├── level_002.ron
    └── ...

docs/
├── asset-format.md          # UPDATE with level metadata field documentation
├── developer-guide.md       # UPDATE with examples of adding level metadata
└── bricks.md                # OPTIONALLY reference metadata for level design

tests/
├── level_definition.rs      # ADD tests for new optional fields
└── integration/             # Existing integration tests (verify backward compatibility)

tools/
└── migrate-level-indices/   # REMOVED - no longer needed
```

**Structure Decision**: Single project structure.
Changes are minimal and isolated to:

1. `LevelDefinition` struct definition in `src/level_loader.rs`
2. Documentation in `assets/levels/README.md` (technical reference for level designers)
3. Documentation in `docs/asset-format.md` (user-facing documentation)
4. Documentation in `docs/developer-guide.md` (developer workflow examples)
5. New tests in `tests/level_definition.rs`
6. Optional migration tool update to handle new fields

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**No violations**: All constitution principles are satisfied.
No complexity tracking required.
