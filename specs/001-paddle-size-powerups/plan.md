# Implementation Plan: Paddle Size Powerups

**Branch**: `001-paddle-size-powerups` | **Date**: 2025-12-12 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-paddle-size-powerups/spec.md`

## Summary

Implement paddle size modification mechanics triggered by special brick collisions (brick type 30: shrink to 70%, brick type 32: enlarge to 150%).
Effects are temporary (10 seconds), replaced when conflicting effects occur, and cleared on level advance/loss.
Visual feedback includes color tint (red for shrink, green for enlarge) with subtle glow outline.
Audio provides distinct sounds for each brick type.
Integrates with existing ECS architecture using component-based state management and collision-driven events.

## Technical Context

**Language/Version**: Rust 1.81 (Rust 2021 edition)  
**Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, serde 1.0, ron 0.8  
**Storage**: In-memory ECS state only (no persistent storage)  
**Testing**: cargo test (integration and unit tests)  
**Target Platform**: Native (Linux/Windows/macOS) + WASM (Web)  
**Project Type**: Game (multi-platform breakout/brickbreaker)  
**Performance Goals**: Maintain 60 FPS on native and WASM targets  
**Constraints**: <100ms visual feedback latency, no performance degradation across levels  
**Scale/Scope**: Single game instance with 40+ levels, component-based feature system

## Constitution Check

*GATE: All constitutional principles must be satisfied.*
                                                                                                                 *Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. ECS Architecture | ✅ PASS | Feature implemented as systems + components, no state outside ECS |
| II. Physics-Driven Gameplay | ✅ PASS | Collision detection triggers effects, paddle collider reflects game state changes |
| III. Modular Feature Design | ✅ PASS | Paddle size mechanics isolated as independent system set, event-driven with other systems |
| IV. Performance-First | ✅ PASS | Collision queries already optimized by Rapier, visual updates are immediate, 60 FPS target maintained |
| V. Cross-Platform Compatibility | ✅ PASS | Uses pure Bevy/Rust, no platform-specific APIs, WASM-compatible asset loading |

**Gate Result**: PASS - No violations detected.
Safe to proceed to Phase 0 research.

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
├── main.rs                          # Application entry point
├── lib.rs                           # Library root
├── pause.rs                         # Existing pause system
├── level_loader.rs                  # Existing level loading
├── level_format/                    # Level definition modules
├── systems/
│   ├── mod.rs
│   ├── paddle_size.rs              # NEW: Paddle size modification system
│   ├── paddle_size_effects.rs       # NEW: Effect timer and lifecycle management
│   ├── paddle_size_visual.rs        # NEW: Visual feedback (color, glow)
│   ├── paddle_size_audio.rs         # NEW: Audio feedback system
│   └── [existing systems...]
└── ui/                              # UI systems

tests/
├── paddle_shrink.rs                # Existing test file (reuse for shrink tests)
├── [existing test files...]
└── paddle_size_integration.rs      # NEW: Integration tests for all size mechanics
```

**Structure Decision**: Single project (monolithic game).
Paddle size mechanics implemented as four interdependent systems within existing `src/systems/` structure.
Follows established patterns used in `paddle_shrink.rs` test and existing ECS architecture.

## Complexity Tracking

No constitutional violations detected.
Feature aligns cleanly with ECS architecture, physics integration, modularity requirements, and performance standards.
