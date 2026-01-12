# Implementation Plan: Pause and Resume System

**Branch**: `004-pause-system` | **Date**: 2025-11-28 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/004-pause-system/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a pause/resume system that freezes physics simulation when ESC is pressed, displays a pause overlay, switches from fullscreen to windowed mode during pause, and resumes gameplay with a screen click.
The system must preserve all game state during pause and handle window mode transitions gracefully.

## Technical Context

**Language/Version**: Rust 1.81 (Rust 2021 edition via rustup) **Primary Dependencies**: Bevy 0.16 (ECS, input handling, window management, Time, rendering), bevy_rapier3d 0.31 (physics simulation control) **Storage**: N/A (runtime state only, no persistence) **Testing**: cargo test (unit tests for state transitions), manual testing (gameplay validation) **Target Platform**: Native (Linux/Windows/macOS) + WASM **Project Type**: Single project (game) **Performance Goals**: 60 FPS maintained during pause/resume transitions, <16ms pause activation latency, <100ms window mode switching **Constraints**: Must not corrupt game state during pause, window mode changes must be graceful (handle failures), input debouncing required **Scale/Scope**: Single pause state, 3 window mode states (fullscreen/windowed/unknown), 2 input types (ESC key, mouse click)

## Constitution Check

*GATE: Must pass before Phase 0 research.*
                                                                                                                                                                                                                                                                                                                         *Re-check after Phase 1 design.*

### I. Entity-Component-System Architecture (ECS-First)

- ✅ **PASS**: Pause state will be implemented as ECS resource, overlay as entity with components, systems for input handling and physics control
- ✅ **PASS**: No mutable state outside ECS, all pause logic expressed as systems operating on components/resources

### II. Physics-Driven Gameplay

- ✅ **PASS**: Feature controls physics simulation (freeze/resume) via Rapier3D time scaling, does not bypass physics engine
- ✅ **PASS**: Preserves physics state (velocities, positions) during pause by halting simulation, not by manual state manipulation

### III. Modular Feature Design

- ✅ **PASS**: Pause system is independently testable module with clear component markers (PauseState resource, PauseOverlay component)
- ✅ **PASS**: Event-driven communication via Bevy input events (KeyboardInput, MouseButtonInput), no tight coupling to gameplay systems

### IV. Performance-First Implementation

- ✅ **PASS**: Performance targets defined (16ms pause latency, 100ms window switching, 60 FPS maintained)
- ✅ **PASS**: Minimal allocations (pause overlay spawned once, state transitions use enums), leverages Bevy's parallel execution

### V. Cross-Platform Compatibility

- ⚠️ **CONDITIONAL PASS**: Window mode switching requires platform-specific handling (FR-013: graceful failure handling)
- ✅ **PASS**: ESC/click input supported on native and WASM (Bevy abstracts input)
- **ACTION REQUIRED**: Research WASM window mode API limitations during Phase 0

**Overall Status**: ✅ PASS with research requirement for WASM window mode handling

**Re-evaluation After Phase 1 (2025-11-28)**:

- ✅ **ECS Purity Maintained**: Final design uses Resource (`PauseState`), Component (`PauseOverlay`), and Systems (input handlers, physics control, UI management).
  No mutable state outside ECS.
- ✅ **Cross-Platform Constraints Met**: WASM window mode switching resolved via conditional compilation (`#[cfg(target_arch = "wasm32")]`).
  Native supports fullscreen ↔ windowed switching.
  WASM remains in windowed mode (documented in quickstart).
- ✅ **Physics-Driven Gameplay Preserved**: Pause controls `RapierConfiguration::physics_pipeline_active` (does not bypass physics engine).
  State preservation guaranteed.
- ✅ **Modularity Confirmed**: Pause system is independently testable plugin with clear boundaries.
  No tight coupling to gameplay systems.
- ✅ **Performance Targets Defined**: <16ms pause/resume latency, <100ms window switching, 60 FPS maintained (documented in success criteria).

**Final Verdict**: ✅ **CONSTITUTION COMPLIANT** - All principles satisfied by final design

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
├── main.rs              # App setup, plugin registration (add PausePlugin)
├── level_loader.rs      # Existing level loading (integrate with pause blocking)
├── pause.rs             # NEW: Pause system (PauseState, PausePlugin, input systems)
├── ui/
│   └── pause_overlay.rs # NEW: Pause overlay UI (spawn/despawn overlay entity)
└── systems/
    ├── mod.rs
    └── grid_debug.rs    # Existing debug system

tests/
└── pause_tests.rs       # NEW: Unit tests for pause state transitions

assets/
└── fonts/               # NEEDS CLARIFICATION: Font for pause message text
```

**Structure Decision**: Single project (game) structure.
New `pause.rs` module contains core pause logic (state resource, input systems, physics control).
New `ui/pause_overlay.rs` handles visual overlay.
Tests in dedicated `tests/pause_tests.rs` file.
Existing `level_loader.rs` will be modified to respect pause state during transitions (FR-012).

## Complexity Tracking

No constitutional violations requiring justification.
