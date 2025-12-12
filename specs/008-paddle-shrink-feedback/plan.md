# Implementation Plan: Paddle Shrink Visual Feedback

**Branch**: `008-paddle-shrink-feedback` | **Date**: 2025-12-12 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/008-paddle-shrink-feedback/spec.md`

## Summary

When a player loses the ball, the paddle provides immediate visual feedback by shrinking concurrently with the respawn delay.
The shrink animation runs during the existing 1-second respawn delay (matching the fadeout overlay timing), keeping the paddle visible throughout.
This feature reuses Bevy's existing animation components and integrates with the established respawn system without breaking current behavior or adding extra time to the respawn sequence.

## Technical Context

**Language/Version**: Rust 1.81 (Rust 2021 edition) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0 **Storage**: N/A (in-memory ECS state only) **Testing**: cargo test (integration tests in tests/ directory) **Target Platform**: Native (Linux/Windows/macOS) + WASM **Project Type**: Single game project with ECS architecture **Performance Goals**: 60 FPS on native, stable performance on WASM **Constraints**: Animation must complete within respawn delay duration (~1 second), no additional frame time overhead **Scale/Scope**: Single animation component, integrates with existing respawn system (~200 lines of code estimated)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Entity-Component-System Architecture (ECS-First)

- ✅ **PASS**: Feature implemented as ECS system operating on paddle entities
- ✅ **PASS**: Animation state stored in component (reusing PaddleGrowing pattern)
- ✅ **PASS**: System is pure function of query inputs (time, paddle transform, component state)
- ✅ **PASS**: Leverages Bevy's change detection and observers

### II. Physics-Driven Gameplay

- ✅ **PASS**: Does not interfere with physics; animation is visual only (scale transform)
- ✅ **PASS**: Maintains physics properties during animation

### III. Modular Feature Design

- ✅ **PASS**: Implemented as independent system in respawn module
- ✅ **PASS**: Uses clear component marker for shrinking state
- ✅ **PASS**: Event-driven communication with existing respawn system
- ✅ **PASS**: No tight coupling; integrates via existing LifeLostEvent

### IV. Performance-First Implementation

- ✅ **PASS**: Minimal performance impact (single lerp per paddle per frame during shrink)
- ✅ **PASS**: No allocations in animation loop
- ✅ **PASS**: Reuses existing timer infrastructure
- ✅ **PASS**: Tested on both native and WASM (per existing patterns)

### V. Cross-Platform Compatibility

- ✅ **PASS**: No platform-specific code required
- ✅ **PASS**: Animation uses standard Bevy transform system
- ✅ **PASS**: Testing covers both native and WASM builds

### VI. Comprehensive Rustdoc Documentation

- ✅ **PASS**: Public components and systems will be documented with rustdoc
- ✅ **PASS**: Focus on why/when to use, not implementation details

**GATE STATUS**: ✅ ALL CHECKS PASSED - Proceed to Phase 0

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
├── systems/
│   ├── respawn.rs           # Enhanced with paddle shrink logic
│   └── mod.rs               # Module declarations
├── lib.rs                   # Component definitions (PaddleShrinking)
└── main.rs                  # Game entry point

tests/
├── paddle_shrink.rs         # New: Integration tests for shrink behavior
└── common/
    └── paddle_shrink.rs     # Test helpers if needed
```

**Structure Decision**: Single Rust project following Bevy ECS patterns.
The paddle shrink feature integrates directly into the existing `src/systems/respawn.rs` module since it's tightly coupled with the respawn system lifecycle.
Component definitions added to `src/lib.rs` alongside existing game components.
Integration tests follow the established pattern in the `tests/` directory.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations detected.
All constitutional principles satisfied.

---

## Implementation Phases

### Phase 0: Research ✅ COMPLETE

**Deliverable**: `research.md`

**Key Decisions**:

- Reuse `PaddleGrowing` component for shrink animation (inverse semantics)
- Trigger shrink on `LifeLostEvent` with duration matching respawn delay
- Let respawn executor handle interruption naturally via component replacement
- Capture current scale implicitly via lerp (no explicit storage needed)
- Create integration tests following existing patterns

**Status**: Research complete, all technical unknowns resolved.

---

### Phase 1: Design & Contracts ✅ COMPLETE

**Deliverables**:

- `data-model.md` - ECS components, resources, events, state transitions
- `contracts/internal-contracts.md` - Component interfaces and system behaviors
- `quickstart.md` - Manual verification steps and testing guide
- Agent context updated (copilot-instructions.md)

**Key Artifacts**:

- Component schema: `PaddleGrowing { timer, target_scale }`
- Event contract: `LifeLostEvent` triggers shrink
- System contract: `apply_paddle_shrink` adds component
- State transition diagram: Playing → Ball Loss → Shrinking → Respawn → Regrowing → Playing
- Testing strategy: Integration tests verifying timing, concurrency, edge cases

**Status**: Design complete, ready for Phase 2 (task breakdown).

---

## Next Steps

**Command**: `/speckit.tasks`

This will:

1. Break down implementation into specific tasks
2. Map tasks to user stories and acceptance criteria
3. Define test cases for each task
4. Create `tasks.md` with detailed implementation checklist

**Estimated Effort**: ~4-6 hours implementation + 2-3 hours testing

**Implementation Order**:

1. Add shrink system to respawn module
2. Wire into existing event flow
3. Write integration tests
4. Manual verification against quickstart guide
5. Documentation and code review

---

## Post-Phase 1 Constitution Re-Check

Revisiting constitutional compliance after design:

### I. Entity-Component-System Architecture

- ✅ Design uses pure ECS patterns
- ✅ State in component (`PaddleGrowing`), logic in system (`apply_paddle_shrink`)
- ✅ Query-based entity selection

### II. Physics-Driven Gameplay

- ✅ No physics interference; scale is visual only

### III. Modular Feature Design

- ✅ Self-contained in respawn module
- ✅ Clean event-driven integration

### IV. Performance-First

- ✅ Minimal overhead (one lerp per frame per paddle)
- ✅ No allocations

### V. Cross-Platform

- ✅ Platform-agnostic implementation

### VI. Rustdoc Documentation

- ✅ Documentation plan established in quickstart

**FINAL GATE STATUS**: ✅ ALL CHECKS PASSED - Proceed to implementation
