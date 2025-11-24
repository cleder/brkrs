# Implementation Plan: Brkrs Complete Game

**Branch**: `001-complete-game` | **Date**: 2025-11-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-complete-game/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a complete Breakout/Arkanoid-style game in Rust using Bevy 0.16.0 ECS engine and Rapier3D 0.31.0 physics. The game features mouse-controlled paddle with velocity-based movement and rotation, physics-driven ball mechanics with "english" steering, 37+ brick types with varied behaviors, 77 unique levels loaded from RON format, and full game state management (menu, playing, paused, game over). Technical approach leverages ECS-first architecture with physics-driven gameplay, modular plugin design, and cross-platform support (native + WASM) with 60 FPS target.

## Technical Context

**Language/Version**: Rust 2021 edition (toolchain managed by rustup)

**Primary Dependencies**:

- Bevy 0.16.0 (ECS game engine with dynamic_linking for dev builds)
- Rapier3D 0.31.0 (physics engine with simd-stable, debug-render-3d)
- Serde 1.0 with derive features (serialization)
- RON 0.8 (Rusty Object Notation for level files)

**Storage**: File-based level definitions (RON format) in `assets/levels/`, no persistent database

**Testing**: cargo test (unit + integration); manual gameplay testing required per constitution

**Target Platform**: Native (Linux/Windows/macOS) + WASM (web browsers via wasm-bindgen)

**Project Type**: Single standalone game project with modular plugin architecture

**Performance Goals**: 60 FPS on modern desktop (native) and moderate hardware (WASM); <100ms input latency

**Constraints**:

- 2D gameplay constrained to Y=2.0 plane with 3D rendering
- Mouse input required (velocity-based control)
- Window fullscreen on startup (user requirement)
- 22x22 grid layout (PLANE_H=30.0, PLANE_W=40.0)
- Cross-platform compatibility (native features like wireframe toggle optional on WASM)

**Scale/Scope**:

- 77 unique levels
- 37+ distinct brick types with unique behaviors
- 5 major game states (Menu, Playing, Paused, LevelTransition, GameOver/Victory)
- Single-player local gameplay (no networking)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Evidence/Notes |
|-----------|--------|----------------|
| **I. ECS-First Architecture** | ✅ PASS | All game logic implemented as Bevy systems (move_paddle, collision handlers, despawn systems); components (Paddle, Ball, Brick) mark entities; no mutable state outside ECS |
| **II. Physics-Driven Gameplay** | ✅ PASS | Rapier3D handles all collisions; physics forces/impulses drive ball movement; LockedAxes for Y-constraint; restitution/friction tuned via constants |
| **III. Modular Feature Design** | ✅ PASS | LevelLoaderPlugin separates concerns; systems use event-driven communication (WallHit, BrickHit, BallHit events); grid debug as separate module |
| **IV. Performance-First** | ✅ PASS | Profile configured (opt-level=3 for deps); dynamic_linking in dev; CCD enabled for ball; change detection not yet leveraged (Phase 2 optimization) |
| **V. Cross-Platform** | ✅ PASS | Conditional compilation for wireframe (native only); WASM target tested; assets loaded via include_str fallback; no platform-specific APIs without guards |

**Overall Assessment**: ✅ **APPROVED** - All constitutional principles satisfied. Current implementation aligns with ECS patterns, physics-driven design, and modularity goals.

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
├── main.rs              # App initialization, system registration, observers
├── level_loader.rs      # LevelLoaderPlugin, RON parsing, entity spawning
├── systems/
│   └── grid_debug.rs    # Debug grid overlay systems
├── components/          # [Phase 1 - to be created]
│   ├── paddle.rs        # Paddle marker, velocity tracking
│   ├── ball.rs          # Ball marker, BallType enum (golf/beach)
│   ├── brick.rs         # Brick, BrickType (37+ variants), Durability
│   ├── border.rs        # Border, LowerGoal markers
│   └── game_state.rs    # Lives, Score, CurrentLevel resources
├── systems/             # [Phase 1 - expand existing]
│   ├── paddle.rs        # move_paddle, paddle rotation, boundary clamping
│   ├── ball.rs          # velocity limits, respawn logic
│   ├── collision.rs     # event readers, physics response handlers
│   ├── brick.rs         # destruction, durability, special behaviors
│   ├── level.rs         # level completion detection, progression
│   └── input.rs         # mouse grab, keyboard controls
├── events.rs            # [Phase 1 - to be created]
│   └── WallHit, BrickHit, BallHit, LevelComplete, LifeLost
└── plugins/             # [Phase 2 - modular refactor]
    ├── gameplay.rs      # Core gameplay systems plugin
    ├── rendering.rs     # Visual effects, lighting, camera
    └── ui.rs            # HUD, menus, game state UI

assets/
├── levels/
│   ├── level_001.ron    # [Exists] First level definition
│   └── level_002..077.ron # [Phase 3+] Additional levels
└── textures/            # [Future] Brick textures, sprites

tests/
├── unit/                # [Phase 2] Component logic tests
└── integration/         # [Phase 2] System integration tests

target/                  # Build artifacts (gitignored)
wasm/                    # WASM build output
```

**Structure Decision**: Single Rust game project (Option 1). Current implementation has main.rs with systems module; Phase 1 will add components/, events.rs, and expand systems/ with feature-specific modules. Phase 2 will refactor into plugin architecture for better modularity. This aligns with Bevy's plugin pattern and constitutional modularity principle.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**No violations detected** - All constitutional principles satisfied by current design.
