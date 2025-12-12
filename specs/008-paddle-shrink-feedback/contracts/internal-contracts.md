# API Contracts: Paddle Shrink Visual Feedback

**Feature**: 008-paddle-shrink-feedback **Date**: 2025-12-12

## Overview

This feature is a visual enhancement within a game engine (Bevy ECS).
It does not expose external HTTP APIs or network services.
This document defines the internal ECS "contracts" - component interfaces, event schemas, and system behaviors that other systems can depend on.

## Component Contracts

### PaddleGrowing Component

**Type**: Bevy Component **Visibility**: Public (other systems may query)

**Schema**:

```rust
#[derive(Component)]
pub struct PaddleGrowing {
    pub timer: Timer,        // Bevy timer tracking animation progress
    pub target_scale: Vec3,  // Final scale value (0.01 for shrink, 1.0 for growth)
}
```

**Contract Guarantees**:

- **Invariant**: `timer.duration()` matches animation length
- **Invariant**: `target_scale` is either `Vec3::splat(0.01)` or `Vec3::ONE` (shrink or growth)
- **Lifecycle**: Automatically removed when `timer.is_finished() == true`
- **Behavior**: Presence of this component signals active animation (shrink or growth)

**Consumer Expectations**:

- Systems querying `With<PaddleGrowing>` will see both shrinking and growing paddles
- Systems can differentiate by comparing `target_scale` to current `Transform.scale`
- Component will be replaced (not updated) when animation direction changes

---

## Event Contracts

### LifeLostEvent (Existing - Extended Usage)

**Type**: Bevy Message Event **Trigger**: Ball collides with lower goal boundary

**Schema**:

```rust
#[derive(Message, Debug, Clone, Copy)]
pub struct LifeLostEvent {
    pub ball: Entity,              // Entity ID of lost ball
    pub cause: LifeLossCause,      // LowerGoal collision
    pub ball_spawn: SpawnTransform, // Respawn position
}
```

**Contract Extension** (for this feature):

- **Post-Condition**: Paddle entity associated with lost ball will receive `PaddleGrowing` component with shrink target
- **Timing**: Shrink starts same frame as event emission
- **Guarantee**: Only paddles without existing `PaddleGrowing` receive shrink component

**Consumers**:

- `apply_paddle_shrink` system (new, added by this feature)
- `enqueue_respawn_requests` system (existing, unchanged)
- Logging systems (existing, unchanged)

---

## System Contracts

### apply_paddle_shrink (New System)

**Purpose**: Apply shrink animation to paddle when ball is lost

**Execution Set**: `RespawnSystems::Detect` **Run Order**: After `detect_ball_loss`, before `enqueue_respawn_requests`

**Inputs**:

- `MessageReader<LifeLostEvent>`: Ball loss events
- `Query<(Entity, &Transform), (With<Paddle>, Without<PaddleGrowing>)>`: Paddles eligible for shrink
- `Res<RespawnSchedule>`: For reading respawn delay duration

**Outputs**:

- Adds `PaddleGrowing` component to paddle entities
- No events emitted

**Behavior Guarantees**:

1. **Idempotency**: If paddle already has `PaddleGrowing`, do nothing (avoid interrupting existing animation)
2. **Duration**: Shrink timer duration set to `respawn_schedule.timer.duration()` (typically 1.0s)
3. **Target**: `target_scale = Vec3::splat(0.01)`
4. **No Side Effects**: Does not modify transform, velocity, or other paddle state directly

**Error Handling**:

- If no paddle entity found: Log warning, continue (edge case: paddle already despawned)
- If multiple paddles: Apply shrink to first matching paddle (future-proofing for multi-paddle)

---

### update_paddle_growth (Existing - Reused)

**Purpose**: Animate paddle scale over time (shrink or growth)

**Execution Set**: `Update` (Bevy default) **Run Order**: Independent, runs every frame

**Contract (Unchanged)**:

**Inputs**:

- `Query<(Entity, &mut Transform, &mut PaddleGrowing)>`: All animating paddles
- `Res<Time>`: For delta time

**Outputs**:

- Modifies `Transform.scale` via interpolation
- Removes `PaddleGrowing` component when finished
- Restores gravity (existing behavior for regrowth)

**Behavior for Shrink** (this feature):

1. **Interpolation**: `transform.scale = transform.scale.lerp(target_scale, eased_progress)`
2. **Easing**: Cubic ease-out `1.0 - (1.0 - progress).powi(3)`
3. **Completion**: When `timer.is_finished()`, set scale to exact target and remove component

---

## Integration Points

### With Respawn System

**Touchpoints**:

1. **detect_ball_loss** → emits `LifeLostEvent` → triggers shrink
2. **enqueue_respawn_requests** → queues respawn → runs after shrink starts
3. **respawn_executor** → resets paddle → replaces shrink with regrowth

**Guarantees**:

- Shrink completes before respawn executor runs (or is interrupted by it)
- Respawn executor resets paddle state regardless of shrink progress
- No mutual exclusion needed; systems operate on separate data

### With Visual Overlay

**Relationship**: Parallel execution, shared timing

**Synchronization**:

- Both shrink and `RespawnFadeOverlay` use `RespawnSchedule.timer.duration()`
- No explicit coordination; both systems tick independently
- Visual consistency achieved via identical duration

---

## Versioning

**Contract Version**: 1.0.0 **Compatibility**: Bevy 0.17.3, bevy_rapier3d 0.32.0

**Breaking Changes**:

- Changing `PaddleGrowing` component schema (add/remove fields)
- Changing shrink duration calculation
- Modifying execution order vs. respawn systems

**Non-Breaking Changes**:

- Adjusting easing curve (visual only)
- Adding debug logging
- Performance optimizations (same behavior)

---

## Testing Contracts

### Component State Tests

```rust
// After LifeLostEvent, paddle has PaddleGrowing with shrink target
assert!(paddle.contains::<PaddleGrowing>());
let growing = paddle.get::<PaddleGrowing>();
assert_eq!(growing.target_scale, Vec3::splat(0.01));
```

### Timing Tests

```rust
// Shrink duration matches respawn delay
let shrink_duration = paddle.get::<PaddleGrowing>().timer.duration();
let respawn_delay = respawn_schedule.timer.duration();
assert_eq!(shrink_duration, respawn_delay);
```

### Animation Tests

```rust
// Scale interpolates smoothly
advance_time(&mut app, 0.5); // Halfway through
let scale = paddle.get::<Transform>().scale;
assert!(scale.x > 0.01 && scale.x < 1.0); // Between min and max
```

---

## Notes

Since this is a game feature (not a web API), "contracts" refer to internal interfaces and behavioral guarantees within the Bevy ECS architecture.
External integrations would be limited to:

- Save/load systems (if paddle state persisted)
- Telemetry/analytics (if animation events logged)
- Modding APIs (if exposing component access)

None of these are currently in scope for this feature.
