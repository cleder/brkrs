# Event Contracts: Multi-Hit Bricks

**Feature**: 005-multi-hit-bricks
**Date**: 2025-11-29

## Overview

This document defines the event contracts for multi-hit brick behavior. Events enable decoupled communication between collision handling, visual updates, audio, and scoring systems.

## Events

### MultiHitBrickHit

Triggered when a ball collides with a multi-hit brick (indices 10-13), causing a state transition.

```rust
/// Event emitted when a multi-hit brick is damaged by ball collision.
///
/// This event is fired BEFORE the brick is despawned (for indices 10-13).
/// Subscribers can use this for audio feedback (Sound 29) and scoring (+50 pts).
#[derive(Event, Debug, Clone)]
pub struct MultiHitBrickHit {
    /// The brick entity that was hit
    pub entity: Entity,
    /// The BrickTypeId before the collision (10, 11, 12, or 13)
    pub previous_type: u8,
    /// The BrickTypeId after the collision (10, 11, 12, or 20)
    pub new_type: u8,
}
```

**Emitter**: `mark_brick_on_ball_collision` system (modified)

**Subscribers**:

- Audio system: Play Sound 29
- Scoring system: Award 50 points
- Visual system: Triggered by `Changed<BrickTypeId>` detection (not this event)

### BrickTypeChanged

Implicit "event" via Bevy's change detection. When `BrickTypeId` component is mutated, systems with `Changed<BrickTypeId>` query filter will detect it.

```rust
// No explicit event struct needed - use Bevy's change detection:
fn update_brick_material(
    changed_bricks: Query<
        (Entity, &BrickTypeId, &mut MeshMaterial3d<StandardMaterial>),
        (With<Brick>, Changed<BrickTypeId>)
    >,
    type_registry: Res<TypeVariantRegistry>,
) {
    for (entity, type_id, mut material) in changed_bricks.iter_mut() {
        if let Some(new_handle) = type_registry.get(ObjectClass::Brick, type_id.0) {
            *material = MeshMaterial3d(new_handle);
        }
    }
}
```

## System Ordering

```text
┌─────────────────────────────────────────────────────────────┐
│                    Frame Update Order                        │
└─────────────────────────────────────────────────────────────┘

1. Physics Step (Rapier)
   └─ Generates CollisionEvent::Started for ball-brick collisions

2. mark_brick_on_ball_collision (Modified)
   ├─ Reads CollisionEvent
   ├─ If multi-hit brick (10-13):
   │   ├─ Mutate BrickTypeId (13→12, 12→11, 11→10, 10→20)
   │   └─ Emit MultiHitBrickHit event
   └─ If simple brick (20) or legacy (3):
       └─ Insert MarkedForDespawn

3. update_brick_material (New - runs after collision)
   └─ Detects Changed<BrickTypeId>, updates MeshMaterial3d

4. despawn_marked_entities (Existing)
   └─ Removes entities with MarkedForDespawn

5. advance_level_when_cleared (Existing)
   └─ Checks if CountsTowardsCompletion query is empty
```

## Contract Guarantees

1. **Atomicity**: `BrickTypeId` mutation and event emission happen in same system call
2. **Ordering**: Material update happens AFTER type mutation (same frame)
3. **Consistency**: Entity retains `CountsTowardsCompletion` through all transitions
4. **Idempotency**: Each collision generates exactly one transition event

## Integration Points

### For Audio System

```rust
fn play_multi_hit_sound(
    mut events: EventReader<MultiHitBrickHit>,
    // audio resources...
) {
    for event in events.read() {
        // Play Sound 29 for multi-hit brick damage
        play_sound(SoundId::MULTI_HIT_BRICK);
    }
}
```

### For Scoring System

```rust
fn award_multi_hit_points(
    mut events: EventReader<MultiHitBrickHit>,
    mut score: ResMut<Score>,
) {
    for event in events.read() {
        score.add(50); // 50 points per multi-hit brick damage
    }
}
```

## Backward Compatibility

- Existing bricks (indices 3, 20, 90+) continue to work unchanged
- No changes to level file format required
- `CountsTowardsCompletion` logic unchanged (already excludes index 90)
