# Research: Paddle Shrink Visual Feedback

**Feature**: 008-paddle-shrink-feedback **Date**: 2025-12-12 **Status**: Complete

## Overview

This document captures research and design decisions for implementing paddle shrink visual feedback on ball loss.
All technical unknowns from the planning phase are resolved here.

## Research Topics

### 1. Animation Component Design Pattern

**Question**: Should we create a new `PaddleShrinking` component or reuse the existing `PaddleGrowing` component?

**Research Findings**:

- Examined `src/systems/respawn.rs` and found `PaddleGrowing` component with:
  - `timer: Timer` for animation duration
  - `target_scale: Vec3` for the final scale
  - Used during respawn with cubic easing interpolation
- Current pattern: paddle spawns at 0.01 scale, grows to Vec3::ONE over PADDLE_GROWTH_DURATION (2.0 seconds)
- The component is removed when animation completes

**Decision**: Reuse `PaddleGrowing` component with inverse semantics

**Rationale**:

- `PaddleGrowing` already supports arbitrary target scales
- The animation system in `update_paddle_growth()` interpolates from current scale to `target_scale`
- For shrinking: set `target_scale` to Vec3::splat(0.01) instead of Vec3::ONE
- Reduces code duplication and testing surface
- Maintains consistency with existing animation patterns

**Alternatives Considered**:

- Create separate `PaddleShrinking` component → Rejected: Would duplicate timer logic, easing curves, and require parallel system
- Use a generic `PaddleAnimating` component → Rejected: Over-engineering for current needs; YAGNI principle

---

### 2. Timing Coordination with Respawn System

**Question**: How should paddle shrink timing coordinate with the existing RespawnSchedule and RespawnFadeOverlay?

**Research Findings**:

- `RespawnSchedule.timer` is 1.0 second by default (respawn delay)
- `RespawnFadeOverlay` spawns with duration matching `RespawnSchedule.timer.duration()`
- Overlay animates: fade in to 0.6 alpha (first half) then fade out (second half)
- From clarifications: shrink should match fadeout timing and run concurrently

**Decision**: Trigger paddle shrink immediately on `LifeLostEvent`, with duration matching `RespawnSchedule.timer.duration()`

**Rationale**:

- Concurrent execution keeps total respawn time unchanged (critical for gameplay pacing)
- Matches existing fadeout overlay pattern (visual consistency)
- User explicitly confirmed this timing model in clarifications
- `LifeLostEvent` is the earliest point where we know ball loss occurred

**Implementation Approach**:

1. In `detect_ball_loss` system (or immediately after): add `PaddleGrowing` component to paddle with `target_scale: Vec3::splat(0.01)`
2. Set timer duration from `RespawnSchedule.timer.duration()`
3. Existing `update_paddle_growth` system will animate the shrink automatically
4. When respawn executor runs, it will reset paddle and start regrowth as usual

**Alternatives Considered**:

- Sequential timing (shrink then delay) → Rejected: Adds extra time, contradicts clarification
- Different duration than fadeout → Rejected: Visual inconsistency, contradicts clarification

---

### 3. Handling Edge Cases

**Question**: How to handle shrink interruption during level transition or rapid consecutive losses?

**Research Findings**:

- Level transitions spawn paddle with `PaddleGrowing` component already attached
- `enqueue_respawn_requests` system handles queued ball losses
- Current respawn executor resets paddle transform and re-adds `PaddleGrowing` for regrowth
- Edge case spec states: "Animation is interrupted, paddle immediately begins shrinking from current scale"

**Decision**: Let respawn executor override shrink state naturally; add shrink before enqueue

**Rationale**:

- If paddle is growing and ball is lost: existing `PaddleGrowing` component gets replaced with new shrink target
- Respawn executor already handles paddle state reset, including removing/re-adding `PaddleGrowing`
- No special interruption logic needed; ECS component replacement is atomic
- Queued respawns handled by existing `RespawnSchedule.queue` mechanism

**Implementation Details**:

- Add shrink component in `detect_ball_loss` or new dedicated system right after
- Verify in testing that component replacement works smoothly (may need to update transform immediately to current scale before changing target)
- Ensure shrink timer starts fresh even if paddle was mid-growth

**Alternatives Considered**:

- Complex state machine tracking shrink/grow phases → Rejected: Over-engineered, existing component replacement sufficient
- Skip shrink if paddle is growing → Rejected: Loses visual feedback, contradicts spec edge case resolution

---

### 4. Visual Continuity During Scale Transitions

**Question**: How to ensure smooth visual transitions when paddle scale changes rapidly (current scale → 0.01 → respawn regrowth)?

**Research Findings**:

- `update_paddle_growth` uses cubic easing: `1.0 - (1.0 - progress).powi(3)` for smooth acceleration
- Interpolation formula: `Vec3::splat(0.01).lerp(target_scale, eased_progress)`
- Current pattern: spawns at 0.01, interpolates to 1.0
- For shrink: starts at current scale (likely 1.0), interpolates to 0.01

**Decision**: Capture current paddle scale when shrink starts, use as implicit starting point for lerp

**Rationale**:

- Bevy's lerp naturally handles any start scale
- When `PaddleGrowing` component is added, paddle has current transform scale
- System applies: `current_scale.lerp(target_scale, eased_progress)` each frame
- First frame: progress=0 → stays at current_scale
- Last frame: progress=1 → reaches target_scale (0.01)
- Respawn executor resets to 0.01 explicitly, then grows to 1.0

**Implementation Approach**:

- No special capture needed; lerp operates on current `Transform.scale` each frame
- Ensure shrink timer duration allows smooth animation (1.0 second is sufficient)
- Test edge case: paddle at partial growth (e.g., 0.5 scale) → shrinks from 0.5 to 0.01

**Alternatives Considered**:

- Store initial scale in component → Rejected: Unnecessary, lerp handles it
- Instant scale change → Rejected: Loses smooth animation requirement

---

### 5. Testing Strategy

**Question**: What testing approach ensures correctness across native and WASM platforms?

**Research Findings**:

- Existing tests use `test_app()` pattern with `MinimalPlugins`
- Timer tests in `tests/respawn_timer.rs` use `advance_time()` helper
- Visual tests in `tests/respawn_visual.rs` check overlay state
- Pattern: spawn entities, trigger events, advance time, assert component states

**Decision**: Create `tests/paddle_shrink.rs` following established patterns

**Test Cases**:

1. Paddle shrinks when ball lost (check scale reaches 0.01)
2. Shrink duration matches respawn delay
3. Shrink runs concurrently with fadeout overlay
4. Shrink interrupted by level transition (paddle reset works)
5. Rapid consecutive losses (queued shrinks)
6. Paddle remains visible throughout (no despawn)

**Rationale**:

- Mirrors existing test structure (maintainability)
- Time-based tests verify animation timing
- Component queries verify state transitions
- No WASM-specific code needed (Bevy abstracts platform differences)

**Alternatives Considered**:

- Manual testing only → Rejected: Insufficient coverage, regression risk
- Unit tests for animation math → Rejected: Integration tests more valuable for gameplay features

---

## Design Summary

### Component Structure

```rust
// Reuse existing component from lib.rs/respawn.rs
#[derive(Component)]
pub struct PaddleGrowing {
    pub timer: Timer,
    pub target_scale: Vec3,  // Vec3::ONE for growth, Vec3::splat(0.01) for shrink
}
```

### System Architecture

```text
detect_ball_loss (existing)
    ↓ emits LifeLostEvent
apply_paddle_shrink (new system, runs in RespawnSystems::Detect)
    ↓ adds PaddleGrowing with shrink target
update_paddle_growth (existing, runs in Update)
    ↓ animates scale over time
enqueue_respawn_requests (existing)
    ↓ queues respawn after delay
respawn_executor (existing)
    ↓ resets paddle, adds PaddleGrowing with growth target
```

### Key Constants

- Shrink target scale: `Vec3::splat(0.01)` (matches respawn spawn scale)
- Duration: `RespawnSchedule.timer.duration()` (typically 1.0 second)
- Easing: Cubic ease-out (existing in `update_paddle_growth`)

---

## Open Questions

None remaining.
All technical decisions finalized and ready for Phase 1 (data model and contracts).

---

## References

- Existing code: `src/systems/respawn.rs` (lines 399-430: `update_paddle_growth`)
- Existing code: `src/lib.rs` (PaddleGrowing component definition)
- Spec clarifications: Session 2025-12-12
- Constitution: ECS-First, Modular Feature Design principles
