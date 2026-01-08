# Implementation Plan: Map Format Change (22x22 to 20x20)

**Branch**: `003-map-format` | **Date**: 2025-11-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-map-format/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Change the game grid format from 22x22 to 20x20 cells while maintaining gameplay feel and fixing level transition timing issues.
The primary requirements are: (1) Update all grid dimension calculations from 22 to 20, (2) Fix level loading sequence so bricks spawn before ball physics activate, (3) Ensure fade-to-black transition shows bricks before gameplay begins, (4) Maintain exact mathematical cell sizing (PLANE_H/20 × PLANE_W/20).
Technical approach involves modifying constants (GRID_WIDTH/GRID_HEIGHT), updating level validation logic, adjusting level transition sequencing in spawn_bricks_only and finalize_level_advance systems, and updating all level .ron files to 20x20 format.

## Technical Context

**Language/Version**: Rust 1.81 (Rust 2021 edition via rustup) **Primary Dependencies**: Bevy 0.16 (ECS, scheduling, `Time`, asset system), bevy_rapier3d 0.31 (physics + collision), serde/ron (level asset parsing) **Storage**: File-based RON assets under `assets/levels/`; no runtime persistence **Testing**: `cargo test` (unit + integration tests), manual gameplay testing for transitions **Target Platform**: Native (Linux/Windows/macOS) + WASM (browser via wasm-bindgen) **Project Type**: Single Bevy game project with ECS architecture **Performance Goals**: 60 FPS on native and WASM; level transitions complete within 2 seconds **Constraints**: WASM embedded assets (no filesystem), cross-platform constant expressions, physics-driven gameplay **Scale/Scope**: Small game (2 levels currently, expandable); ~10-15 source files affected; grid overlay + level loader + embedded WASM level strings

## Constitution Check

*GATE: Must pass before Phase 0 research.*
  *Re-check after Phase 1 design.*

### I. ECS-First Architecture

**Status**: ✅ PASS **Analysis**: Grid format change modifies constants and level loading systems.
All changes remain within ECS paradigm (systems operating on components, no mutable state outside ECS).
Level transition sequence uses existing Bevy systems and resources.

### II. Physics-Driven Gameplay

**Status**: ✅ PASS **Analysis**: Ball physics freeze/activate behavior uses existing physics components (GravityScale, Velocity, BallFrozen marker).
No manual transform manipulation for gameplay.
Grid size change doesn't affect physics engine usage.

### III. Modular Feature Design

**Status**: ✅ PASS **Analysis**: Changes are localized to level loading module and grid overlay system.
Level transition timing is event-driven via LevelAdvanceState resource.
No new tight coupling introduced.

### IV. Performance-First Implementation

**Status**: ✅ PASS **Analysis**: Grid change from 22x22 to 20x20 actually reduces entity count (484 → 400 cells), improving performance.
Level transition sequence optimization (spawn bricks before physics) prevents wasted frame time on empty field rendering.
Must verify 60 FPS maintained on both native and WASM after changes.

### V. Cross-Platform Compatibility

**Status**: ⚠️ VERIFY **Analysis**: Changes affect WASM embedded level strings (include_str! for level_001.ron, level_002.ron).
Must update embedded_level_str() function and test WASM build.
Conditional compilation already in place, just needs data updates.

**Action Items**:

- Update embedded level RON strings for WASM (include_str! paths)
- Test WASM build after grid dimension changes
- Verify 60 FPS on both platforms with new grid size

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
├── lib.rs                  # Grid constants (GRID_WIDTH, GRID_HEIGHT, CELL_WIDTH, CELL_HEIGHT)
├── level_loader.rs         # Level validation, entity spawning, WASM embedded strings
└── systems/
    ├── grid_debug.rs       # Debug overlay rendering (grid dimensions)
    └── level_switch.rs     # Level transition state machine (timing changes)

assets/
└── levels/
    ├── level_001.ron       # Level data (22x22 → 20x20 matrix)
    └── level_002.ron       # Level data (22x22 → 20x20 matrix)

tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

```

**Structure Decision**: Single Rust project with Bevy ECS game engine.
Changes affect core grid constants (src/lib.rs), level loading and validation (src/level_loader.rs), debug visualization (src/systems/grid_debug.rs), and level transition timing (src/systems/level_switch.rs).
Asset files updated from 22x22 to 20x20 matrices (assets/levels/*.ron).

## Complexity Tracking

No constitution violations detected.
All principles pass or require verification testing only.
