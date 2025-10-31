# Implementation Plan: Brkrs Complete Game

**Branch**: `001-complete-game` | **Date**: 2025-10-31 | **Spec**:
[spec.md](spec.md)
**Input**: Feature specification from `/specs/001-complete-game/spec.md`

## Summary

Build a complete Arkanoid/Breakout-style game with mouse-controlled paddle
movement (X/Z axes + rotation), physics-driven ball mechanics with steering
("english"), 77 levels arranged in a 22x22 brick grid, 37+ unique brick
types, and 3D rendering constrained to a 2D gameplay plane. The game uses
Bevy's ECS architecture with Rapier3D physics, supports both native and WASM
platforms, and targets 60 FPS performance.

## Technical Context

**Language/Version**: Rust 1.75+ (2021 edition)
**Primary Dependencies**: Bevy 0.16.0, bevy_rapier3d 0.31.0
**Storage**: File-based level definitions (RON/JSON format), local asset
files
**Testing**: Manual gameplay testing, `cargo test` for unit tests
**Target Platform**: Native (Linux/Windows/macOS) + WASM (web browsers)
**Project Type**: Single game project (standalone executable)
**Performance Goals**: 60 FPS on modern desktop (native), 60 FPS on moderate
hardware (WASM)
**Constraints**: All gameplay constrained to Y=2.0 plane, <100ms input
latency, WASM package size optimization
**Scale/Scope**: 77 levels, 37+ brick types, 22x22 grid layout, single-player
game

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Entity-Component-System Architecture (ECS-First)

✅ **PASS**: Game design fully embraces Bevy's ECS paradigm

- Paddle, Ball, Brick, Border entities with component-based properties
- Game logic implemented as systems (movement, collision handling, state
  transitions)
- Event-driven communication (collision events, state change events)
- No global mutable state; all state in ECS components

### II. Physics-Driven Gameplay

✅ **PASS**: Core mechanics rely on Rapier3D physics engine

- Ball movement and bouncing through physics simulation
- Restitution and friction for realistic collision response
- Impulses for paddle "english" effect and special brick behaviors
- Collision detection drives all game events (brick destruction, ball loss)

### III. Modular Feature Design

✅ **PASS**: Features designed as independent, testable modules

- Clear component markers (Paddle, Ball, Brick, Border)
- User stories (P1-P5) represent independently deliverable features
- Event-based communication between systems
- Features can be developed and tested separately (basic gameplay → state
  management → brick types → levels → visuals)

### IV. Performance-First Implementation

✅ **PASS**: Design targets 60 FPS with performance considerations

- Physics engine handles parallel collision detection
- Component queries for efficient system execution
- Asset optimization for WASM delivery
- Early testing on both native and WASM platforms planned
- Profiling planned for system performance monitoring

### V. Cross-Platform Compatibility

✅ **PASS**: Explicit support for native and WASM targets

- Conditional compilation for platform-specific features (wireframe mode)
- WASM as first-class target alongside native
- Platform compatibility tested throughout development
- Asset optimization for web delivery

**Constitution Compliance**: ✅ ALL GATES PASSED

No violations detected. The design aligns with all constitutional principles.

## Project Structure

### Documentation (this feature)

```text
specs/001-complete-game/
├── plan.md              # This file
├── research.md          # Phase 0 output (architecture decisions)
├── data-model.md        # Phase 1 output (ECS components & entities)
├── quickstart.md        # Phase 1 output (how to build, run, test)
├── contracts/           # Phase 1 output (event definitions, APIs)
│   └── events.md        # Game events and observers
└── checklists/          # Quality checklists
    └── requirements.md  # Spec validation checklist
```

### Source Code (repository root)

```text
src/
├── main.rs              # Application entry point, plugin registration
├── components/          # ECS component definitions
│   ├── mod.rs
│   ├── paddle.rs        # Paddle components
│   ├── ball.rs          # Ball components
│   ├── brick.rs         # Brick components and types
│   ├── border.rs        # Border/wall components
│   └── game_state.rs    # Game state components
├── systems/             # ECS systems
│   ├── mod.rs
│   ├── paddle_control.rs       # Mouse input → paddle movement
│   ├── ball_physics.rs         # Ball physics and steering
│   ├── collision.rs            # Collision event handling
│   ├── brick_behavior.rs       # Brick-specific collision logic
│   ├── state_management.rs     # Game state transitions
│   └── level_loading.rs        # Level definition loading
├── events/              # Custom game events
│   ├── mod.rs
│   ├── collision_events.rs     # Ball-brick, ball-paddle events
│   └── state_events.rs         # State transition events
├── resources/           # Global resources
│   ├── mod.rs
│   ├── game_config.rs          # Physics tuning, grid size
│   └── level_data.rs           # Level definitions
├── plugins/             # Feature modules as Bevy plugins
│   ├── mod.rs
│   ├── physics_plugin.rs       # Rapier3D configuration
│   ├── input_plugin.rs         # Mouse/keyboard input
│   ├── rendering_plugin.rs     # 3D rendering setup
│   └── ui_plugin.rs            # Menu, HUD, pause screen
└── lib.rs               # Library interface (optional, for testing)

assets/
├── levels/              # Level definition files
│   ├── level_001.ron
│   ├── level_002.ron
│   └── ...
├── models/              # 3D mesh files
│   ├── paddle.glb
│   ├── ball.glb
│   ├── bricks/
│   │   ├── standard.glb
│   │   ├── multi_hit.glb
│   │   └── ...
│   └── border.glb
├── textures/            # Texture files
│   └── uv_debug.png
└── fonts/               # UI fonts

wasm/
├── index.html           # WASM launcher page
└── restart-audio-context.js  # WASM audio workaround

target/                  # Build outputs (gitignored)
```

**Structure Decision**: Single Rust game project using Bevy's plugin
architecture for modularity. The `src/` directory is organized by ECS
concepts (components, systems, events, resources) with additional
`plugins/` for feature grouping. This aligns with Bevy best practices and
the project's ECS-first principle. Level data stored as RON files in
`assets/` for easy editing. WASM build artifacts in separate `wasm/`
directory.

## Complexity Tracking

> No constitutional violations detected. This section intentionally left
> empty.
