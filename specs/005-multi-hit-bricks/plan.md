# Implementation Plan: Multi-Hit Bricks

**Branch**: `005-multi-hit-bricks` | **Date**: 2025-11-29 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/005-multi-hit-bricks/spec.md`

## Summary

Implement bricks that require multiple hits to destroy (indices 10-13), where each hit transforms the brick to the next lower index until it becomes a simple stone (index 20), which can then be destroyed.
The implementation leverages Bevy's ECS architecture with component-based state tracking and collision-driven state transitions.

## Technical Context

**Language/Version**: Rust 1.81 (Rust 2021 edition) **Primary Dependencies**: Bevy 0.17, bevy_rapier3d (physics/collision) **Storage**: RON level files (existing format supports indices 10-13) **Testing**: cargo test (unit + integration tests) **Target Platform**: Native (Linux/Windows/macOS) + WASM **Project Type**: Single Bevy game project **Performance Goals**: 60 FPS on target hardware (existing requirement) **Constraints**: No frame drops during brick state transitions; WASM compatibility **Scale/Scope**: 4 new brick variants (indices 10-13) with state machine behavior

## Constitution Check

*GATE: Must pass before Phase 0 research.*
            *Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. ECS Architecture | ✅ Pass | Multi-hit state stored as component (`BrickTypeId`), transitions via systems |
| II. Physics-Driven Gameplay | ✅ Pass | Ball-brick collisions trigger state transitions via existing Rapier collision events |
| III. Modular Feature Design | ✅ Pass | Multi-hit behavior implemented as separate system(s), uses event-driven communication |
| IV. Performance-First | ✅ Pass | No hot-loop allocations; simple component mutation on collision |
| V. Cross-Platform Compatibility | ✅ Pass | No platform-specific APIs; level format unchanged |
| VI. Comprehensive Rustdoc | ✅ Pass | All new public types/functions will be documented |

## Project Structure

### Documentation (this feature)

```text
specs/005-multi-hit-bricks/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (event contracts)
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
src/
├── lib.rs               # Main game module (existing collision systems)
├── level_format/
│   └── mod.rs           # Level format constants (add MULTI_HIT_BRICK_* constants)
├── level_loader.rs      # Level loading (existing, handles brick spawn with BrickTypeId)
├── systems/
│   ├── mod.rs           # Systems module (add multi_hit export)
│   ├── multi_hit.rs     # NEW: Multi-hit brick state transition system
│   └── textures/        # Texture manifest (add type variants for indices 10-13)
└── ...

tests/
├── multi_hit_bricks.rs  # NEW: Integration tests for multi-hit behavior
└── ...

assets/
├── levels/
│   └── test_multi_hit.ron  # NEW: Test level with multi-hit bricks
└── textures/
    └── manifest.ron     # Update with type_variants for indices 10-13
```

**Structure Decision**: Single project structure (existing).
Multi-hit brick logic added as a new system in `src/systems/multi_hit.rs` with integration tests in `tests/`.

## Complexity Tracking

No constitution violations.
Implementation follows established patterns:

- Uses existing `BrickTypeId` component for state
- Extends existing collision handling in `mark_brick_on_ball_collision`
- Follows existing texture manifest pattern for visual variants
