# Implementation Plan: Paddle Shrink Visual Feedback

**Branch**: `008-paddle-shrink-feedback` | **Date**: 2025-12-12 | **Spec**: specs/008-paddle-shrink-feedback/spec.md
**Input**: Feature specification from `specs/008-paddle-shrink-feedback/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

When a player loses their last ball (life loss), the paddle provides immediate visual feedback by shrinking smoothly before the respawn sequence begins.
The shrink animation runs concurrently with the existing respawn delay and fadeout overlay, ensuring no additional time is added to the gameplay loop.

## Technical Context

**Language/Version**: Rust 1.81 (Rust 2021 edition) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, serde 1.0, ron 0.8 **Storage**: In-memory ECS state only (no persistent storage) **Testing**: cargo test (unit/integration), manual gameplay testing **Target Platform**: Native (Linux/Windows/macOS) + WASM **Project Type**: Game engine (Bevy ECS) **Performance Goals**: 60 FPS, smooth animation timing **Constraints**: ECS-first architecture, physics-driven gameplay, cross-platform compatibility **Scale/Scope**: Single system addition, ~200 LOC, affects paddle entity only

## Constitution Check

*GATE: Must pass before Phase 0 research.*
      *Re-check after Phase 1 design.*

### I. Entity-Component-System Architecture (ECS-First)

✅ **COMPLIES**: Feature implemented as ECS systems operating on components (`PaddleGrowing`, `Paddle`, `InputLocked`).
Uses existing `LifeLostEvent` for event-driven communication.
No mutable state outside ECS.

### II. Physics-Driven Gameplay

✅ **COMPLIES**: Visual feedback feature only.
Does not affect physics forces, collisions, or movement.
Paddle physics behavior unchanged during shrink animation.

### III. Modular Feature Design

✅ **COMPLIES**: Feature is independently testable, uses clear component markers, can be added/removed without breaking core gameplay.
Event-driven communication with existing respawn system.

### IV. Performance-First Implementation

✅ **COMPLIES**: Animation must maintain 60 FPS target.
Uses Bevy's built-in interpolation and timer systems.
No allocations in hot loops.
Tested on both native and WASM targets.

### V. Cross-Platform Compatibility

✅ **COMPLIES**: No platform-specific APIs.
Uses Bevy's cross-platform animation and timing systems.
Assets are already optimized for web delivery.

### VI. Comprehensive Rustdoc Documentation

✅ **COMPLIES**: All new public components, systems, and events will have rustdoc documentation explaining purpose and usage, not implementation details.

**GATE STATUS**: ✅ PASS - No violations detected.
Feature aligns with all constitutional principles.

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

## Project Structure

### Documentation (this feature)

```text
specs/008-paddle-shrink-feedback/
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
├── lib.rs               # Component definitions (Paddle, PaddleGrowing, InputLocked)
├── systems/
│   ├── respawn.rs       # Existing respawn system (modified for shrink integration)
│   └── [new file]       # New shrink system (apply_paddle_shrink)
└── level_loader.rs      # Existing level loading (no changes needed)

tests/
├── respawn_spawn_points.rs  # Existing tests (may need updates for shrink behavior)
└── [new test file]      # Integration tests for shrink animation timing
```

**Structure Decision**: Single project structure following existing Bevy ECS patterns.
New shrink system added to `systems/` directory alongside existing respawn system.
Components defined in `lib.rs` per project conventions.

## Phase 0: Outline & Research

**Status**: ✅ COMPLETE (research.md already exists and is comprehensive)

**Research Topics Resolved**:

1. ✅ Animation Component Design Pattern - Reuse `PaddleGrowing` with inverse semantics
2. ✅ Timing Coordination with Respawn System - Concurrent execution matching fadeout duration
3. ✅ Handling Edge Cases - Component replacement handles interruptions gracefully
4. ✅ ECS Integration Points - Event-driven system using existing `LifeLostEvent`

**Key Decisions**:

- Reuse existing `PaddleGrowing` component for shrink animation (target_scale = Vec3::splat(0.01))
- Shrink duration matches `RespawnSchedule.timer.duration()` (concurrent with fadeout)
- New `apply_paddle_shrink` system triggers on `LifeLostEvent` when no balls remain
- Component replacement handles animation interruption during level transitions

## Phase 1: Design & Contracts

**Status**: IN PROGRESS

### Data Model Design

**Entities & Components**:

1. **Paddle Entity** (Existing - Enhanced)
   - **Components**: `Paddle` (marker), `Transform` (scale), `PaddleGrowing` (animation state)
   - **Relationships**: Associated with ball entities through game state
   - **State Transitions**: Full size ↔ Shrinking ↔ Minimum size ↔ Regrowing
   - **Validation**: Scale must be Vec3::ONE when not animating, Vec3::splat(0.01) during shrink

2. **PaddleGrowing Component** (Existing - Extended Usage)
   - **Fields**:
   - `timer: Timer` - Animation duration and progress tracking
   - `target_scale: Vec3` - Final scale after animation (0.01 for shrink, 1.0 for growth)
   - **Validation**: `target_scale` must be either `Vec3::splat(0.01)` or `Vec3::ONE`
   - **Lifecycle**: Added on animation start, removed on completion

3. **InputLocked Component** (Existing - Extended Usage)
   - **Purpose**: Prevents paddle input during shrink animation
   - **Validation**: Must be present during shrink, absent during normal play

**Event Contracts**:

1. **LifeLostEvent** (Existing - Extended Consumer)
   - **Trigger**: Ball collides with lower goal boundary AND no balls remain in play
   - **Consumers**: `enqueue_respawn_requests` (existing), `apply_paddle_shrink` (new)
   - **Post-Conditions**: Paddle shrink animation begins immediately

**System Contracts**:

1. **apply_paddle_shrink** (New System)
   - **Purpose**: Apply shrink animation to paddle when life is lost
   - **Execution**: `RespawnSystems::Detect` set, after `detect_ball_loss`
   - **Inputs**: `EventReader<LifeLostEvent>`, paddle query without `PaddleGrowing`
   - **Outputs**: Adds `PaddleGrowing` component with shrink target
   - **Guarantees**: Only triggers when no balls remain, idempotent if already shrinking

### API Contracts Generation

**Internal ECS Contracts** (contracts/internal-contracts.md):

- Component interfaces and guarantees
- Event schemas and consumer expectations
- System behavior contracts and error handling

**External Contracts**: N/A (no external APIs, internal ECS feature only)

### Quickstart Documentation

**Target Audience**: Developers implementing or testing the feature

**Content Structure**:

1. Prerequisites and build setup
2. Manual verification steps for each acceptance scenario
3. Timing verification procedures
4. Edge case testing scenarios
5. Performance validation steps

### Agent Context Update

**Status**: ✅ COMPLETE - Updated GitHub Copilot context with:

- Language: Rust 1.81 (Rust 2021 edition)
- Framework: Bevy 0.17.3, bevy_rapier3d 0.32.0, serde 1.0, ron 0.8
- Database: In-memory ECS state only (no persistent storage)
- Project type: Game engine (Bevy ECS)

## Constitution Check (Post-Design)

*Re-evaluation after Phase 1 design completion*

### I. Entity-Component-System Architecture (ECS-First)

✅ **COMPLIES**: Design uses pure ECS patterns with components (`PaddleGrowing`, `InputLocked`), events (`LifeLostEvent`), and systems (`apply_paddle_shrink`).
No external state management.

### II. Physics-Driven Gameplay

✅ **COMPLIES**: Feature is purely visual feedback.
Does not interfere with physics forces, collisions, or Rapier3D integration.
Paddle remains physics-driven during shrink.

### III. Modular Feature Design

✅ **COMPLIES**: Feature is completely independent - can be added/removed without affecting core gameplay.
Uses event-driven communication.
Clear system boundaries.

### IV. Performance-First Implementation

✅ **COMPLIES**: Design leverages Bevy's built-in animation systems and timers.
No custom interpolation logic.
Maintains 60 FPS requirement through existing `update_paddle_growth` system.

### V. Cross-Platform Compatibility

✅ **COMPLIES**: Uses only Bevy's cross-platform APIs.
No platform-specific code.
Animation timing uses Bevy's `Timer` which works identically on native and WASM.

### VI. Comprehensive Rustdoc Documentation

✅ **COMPLIES**: All new public APIs (system functions, component structs) will include rustdoc explaining purpose and usage patterns, following project standards.

**GATE STATUS**: ✅ PASS - Design maintains constitutional compliance.
No violations introduced.

## Phase 2: Implementation Planning

**Status**: READY - All prerequisites complete.
Ready for `/speckit.tasks` command to generate implementation tasks.

**Deliverables Ready**:

- ✅ Feature specification (spec.md) - Updated with clarified life loss vs ball loss
- ✅ Technical research (research.md) - Complete with all design decisions
- ✅ Data model (data-model.md) - ECS entities, components, and state transitions defined
- ✅ API contracts (contracts/) - Internal ECS contracts documented
- ✅ Quickstart guide (quickstart.md) - Manual verification procedures
- ✅ Implementation plan (plan.md) - Complete technical approach
- ✅ Agent context updated - Copilot instructions include new technologies

**Next Steps**: Run `/speckit.tasks` to generate detailed implementation tasks from this plan.
