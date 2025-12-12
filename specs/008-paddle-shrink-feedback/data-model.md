# Data Model: Paddle Shrink Visual Feedback

**Feature**: 008-paddle-shrink-feedback **Date**: 2025-12-12

## Overview

This document defines the ECS components, resources, events, and state transitions for the paddle shrink visual feedback feature.

## Components

### PaddleGrowing (Existing - Reused)

**Purpose**: Animates paddle scale over time (growth or shrink)

**Location**: `src/lib.rs`

**Structure**:

```rust
#[derive(Component)]
pub struct PaddleGrowing {
    pub timer: Timer,         // Duration of animation
    pub target_scale: Vec3,   // Final scale after animation
    pub start_scale: Vec3,    // Scale at the start of the animation
}
```

**Usage in Feature**:

- **For shrink**: `target_scale = Vec3::splat(0.01)`, timer duration matches respawn delay
- **For regrowth**: `target_scale = Vec3::ONE`, timer duration = PADDLE_GROWTH_DURATION (2.0s)

**Lifecycle**:

1. Added to paddle entity when shrink/growth starts
2. Updated each frame by `update_paddle_growth` system
3. Removed when `timer.is_finished()` returns true

---

### Paddle (Existing - Query Target)

**Purpose**: Marker component for paddle entities

**Location**: `src/lib.rs`

**Structure**:

```rust
#[derive(Component)]
pub struct Paddle;
```

**Role in Feature**: Target entity for shrink animation queries

---

### Transform (Bevy Built-in)

**Purpose**: Stores entity position, rotation, and scale

**Relevant Field**: `scale: Vec3`

**Animation**:

- Current implementation in `update_paddle_growth` interpolates scale:

  ```rust
  transform.scale = Vec3::splat(0.01).lerp(growing.target_scale, eased_progress);
  ```

- For shrink: lerps from current scale (typically Vec3::ONE) to Vec3::splat(0.01)

---

### InputLocked (Existing - State Marker)

**Purpose**: Prevents paddle input during animations

**Location**: `src/systems/respawn.rs`

**Structure**:

```rust
#[derive(Component)]
pub struct InputLocked;
```

**Role in Feature**: Already applied during respawn; remains active during shrink

---

## Resources

### RespawnSchedule (Existing)

**Purpose**: Tracks pending respawn operations and timing

**Location**: `src/systems/respawn.rs`

**Relevant Fields**:

```rust
#[derive(Resource)]
pub struct RespawnSchedule {
    pub timer: Timer,                         // 1.0 second default
    pub pending: Option<RespawnRequest>,
    pub queue: VecDeque<RespawnRequest>,
    pub last_loss: Option<Duration>,
}
```

**Role in Feature**:

- `timer.duration()` determines shrink animation duration
- Ensures shrink completes concurrently with respawn delay

---

### SpawnPoints (Existing)

**Purpose**: Stores paddle and ball spawn positions from level matrix

**Location**: `src/systems/respawn.rs`

**Role in Feature**: Used by respawn executor to reset paddle after shrink

---

## Events

### LifeLostEvent (Existing)

**Purpose**: Signals ball loss and initiates respawn sequence

**Location**: `src/systems/respawn.rs`

**Structure**:

```rust
#[derive(Message, Debug, Clone, Copy)]
pub struct LifeLostEvent {
    pub ball: Entity,
    pub cause: LifeLossCause,
    pub ball_spawn: SpawnTransform,
}
```

**Role in Feature**: Triggers paddle shrink animation in new system

---

### RespawnScheduled (Existing)

**Purpose**: Emitted when respawn is scheduled after life loss

**Location**: `src/systems/respawn.rs`

**Role in Feature**: Coincides with shrink start; used by visual overlay

---

### RespawnCompleted (Existing)

**Purpose**: Emitted when respawn executor finishes

**Role in Feature**: Marks end of shrink-delay-regrowth cycle

---

## System Sets

### RespawnSystems (Existing)

**Purpose**: Organize respawn-related systems execution order

**Location**: `src/systems/respawn.rs`

**Sets**:

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum RespawnSystems {
    Detect,    // Ball loss detection
    Schedule,  // Queue management
    Execute,   // Entity respawn
    Visual,    // Overlay animation
    Control,   // Input restoration
}
```

**Role in Feature**: New `apply_paddle_shrink` system runs in `Detect` set (after ball loss, before scheduling)

---

## State Transitions

### Complete Animation Cycle

```text
1. PLAYING
   - Paddle at full scale (Vec3::ONE)
   - Ball in play

2. BALL LOSS DETECTED
   - detect_ball_loss system removes ball
   - Emits LifeLostEvent
   - Paddle still at full scale

3. SHRINK INITIATED (NEW)
   - apply_paddle_shrink system reacts to LifeLostEvent
   - Adds PaddleGrowing { target_scale: Vec3::splat(0.01), timer: respawn_delay }
   - InputLocked already added by detect_ball_loss

4. SHRINKING (0.0s - 1.0s)
   - update_paddle_growth interpolates scale: 1.0 → 0.01
   - Runs concurrently with:
     * RespawnSchedule.timer counting down
     * RespawnFadeOverlay animating
   - Paddle remains visible throughout

5. SHRINK COMPLETE + RESPAWN QUEUED
   - Paddle reaches scale 0.01
   - PaddleGrowing component removed
   - RespawnSchedule.timer expires

6. RESPAWN EXECUTION
   - respawn_executor resets paddle transform
   - Sets scale explicitly to 0.01
   - Adds new PaddleGrowing { target_scale: Vec3::ONE, timer: 2.0s }
   - Spawns new ball with BallFrozen

7. REGROWING (0.0s - 2.0s)
   - update_paddle_growth interpolates scale: 0.01 → 1.0
   - InputLocked remains until growth completes

8. CONTROLS RESTORED
   - restore_paddle_control removes InputLocked
   - Ball unfrozen (BallFrozen removed)
   - Back to PLAYING state
```

### Edge Case: Shrink During Growth (Level Transition)

```text
1. LEVEL TRANSITION IN PROGRESS
   - Paddle growing from 0.01 → 1.0
   - PaddleGrowing attached, current scale ~0.5 (mid-animation)

2. BALL LOSS DURING GROWTH
   - LifeLostEvent emitted
   - apply_paddle_shrink replaces PaddleGrowing component
   - New target_scale: 0.01, timer: respawn_delay

3. SHRINK FROM PARTIAL SCALE
   - update_paddle_growth now interpolates 0.5 → 0.01
   - Animation completes smoothly from current scale

4. NORMAL RESPAWN SEQUENCE RESUMES
   - Respawn executor resets and regrows as usual
```

### Edge Case: Rapid Consecutive Losses

```text
1. FIRST BALL LOST
   - Shrink starts (timer: 1.0s)
   - Respawn queued

2. SECOND BALL LOST (before first respawn completes)
   - enqueue_respawn_requests adds to queue
   - First shrink continues undisturbed
   - No new shrink component added (paddle already shrinking)

3. FIRST RESPAWN EXECUTES
   - Resets paddle, starts regrowth
   - Queue has pending request

4. SECOND BALL LOST AGAIN (immediately)
   - New shrink started (interrupts regrowth)
   - Second respawn queued

5. QUEUE PROCESSES SEQUENTIALLY
   - Each loss gets full shrink-delay-regrowth cycle
```

---

## Query Patterns

### For Shrink System

```rust
Query<(Entity, &Transform), (With<Paddle>, Without<PaddleGrowing>)>
```

**Purpose**: Find paddles that need shrink animation (not already animating)

### For Growth System (Existing)

```rust
Query<(Entity, &mut Transform, &mut PaddleGrowing)>
```

**Purpose**: Animate all paddles with PaddleGrowing component (shrink or growth)

---

## Constants

### Animation Parameters

| Constant | Value | Location | Purpose |
|----------|-------|----------|---------|
| `PADDLE_GROWTH_DURATION` | 2.0 seconds | `src/lib.rs` | Duration of regrowth animation |
| Shrink duration | `RespawnSchedule.timer.duration()` | Runtime | Duration of shrink animation (typically 1.0s) |
| Min scale | `Vec3::splat(0.01)` | Hardcoded | Target for shrink, start for regrowth |
| Max scale | `Vec3::ONE` | Hardcoded | Target for regrowth, start for shrink |

---

## Implementation Notes

### Concurrency Guarantees

- Shrink runs in parallel with respawn delay timer and fadeout overlay
- No synchronization needed; each system operates on separate components
- ECS ensures no data races (mutable access is exclusive)

### Performance Characteristics

- Per-frame cost: One Vec3 lerp per paddle with PaddleGrowing
- Memory overhead: ~32 bytes per animating paddle (Timer + Vec3)
- No allocations in animation loop
- Negligible impact on 60 FPS target (<0.01ms per paddle)

---

## Testing Considerations

### State Verification

Tests should verify:

1. Component presence/absence (has PaddleGrowing after loss, removed after complete)
2. Scale values at key points (starts at 1.0, ends at 0.01)
3. Timing accuracy (shrink duration matches respawn delay)
4. Concurrent execution (overlay animates while paddle shrinks)

### Edge Case Coverage

Tests must cover:

1. Shrink interruption (loss during growth)
2. Queued respawns (multiple rapid losses)
3. Game over during shrink (animation completes, no regrowth)
4. Level transition during shrink (state cleanup)

---

## References

- ECS Architecture: Bevy 0.17.3 documentation
- Timer API: `bevy::time::Timer`
- Transform interpolation: `Vec3::lerp()`
- Easing functions: Cubic ease-out (existing implementation)
