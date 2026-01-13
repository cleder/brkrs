# Event Contracts: Paddle-Destroyable Brick (Type 57)

**Feature**: 022-paddle-destroyable-brick | **Date**: 2026-01-13

## Overview

This feature reuses the existing `BrickDestroyed` message contract for paddle-triggered brick destruction.
No new messages are introduced.
This document clarifies how paddle-destroyable bricks (type 57) integrate with the existing event system.

---

## Messages

### BrickDestroyed (Existing - Reused)

**Purpose**: Signals that a brick was destroyed, triggering point award and completion tracking.

**Trigger Conditions**:

- **Ball collision** causes brick health to reach zero OR single-hit brick is struck (existing)
- **Paddle collision** with paddle-destroyable brick (type 57) - **NEW**

**Event Definition** (from `src/signals.rs`):

```rust
use bevy::prelude::*;
use bevy::ecs::message::Message;

#[derive(Message, Debug, Clone, Copy)]
pub struct BrickDestroyed {
    /// The brick entity that was destroyed
    pub brick_entity: Entity,

    /// The type/index of the brick (determines point value)
    /// For paddle-destroyable bricks, this will be 57
    pub brick_type: u8,

    /// The entity that destroyed the brick
    /// - Some(ball_entity) for ball-triggered destruction
    /// - None for paddle-triggered destruction (type 57)
    pub destroyed_by: Option<Entity>,
}
```

**Emitters**:

- `despawn_marked_entities` in `src/lib.rs` - Emits for ALL brick types marked with `MarkedForDespawn`
  - Ball-triggered: Ball collision handler marks brick → despawn system emits message
  - Paddle-triggered: Paddle collision handler marks type 57 brick → despawn system emits message

**Consumers**:

- `award_points_system` in `src/systems/scoring.rs` - Awards points based on `brick_type` field
  - Reads `brick_type == 57` → awards 250 points via `brick_points()` lookup
- Completion tracking system - Decrements remaining brick count for bricks with `CountsTowardsCompletion`

---

## Usage Patterns

### Paddle-Triggered Destruction (Type 57)

**In Paddle Collision Handler** (`src/lib.rs:read_character_controller_collisions`):

```rust
// Detect paddle collision with paddle-destroyable brick
fn read_character_controller_collisions(
    paddle_outputs: Query<&KinematicCharacterControllerOutput, With<Paddle>>,
    bricks: Query<Entity, With<Brick>>,
    brick_types: Query<&BrickTypeId, With<Brick>>,  // NEW
    mut commands: Commands,
) {
    let output = match paddle_outputs.single() {
        Ok(controller) => controller,
        Err(_) => return,
    };

    for collision in output.collisions.iter() {
        for brick in bricks.iter() {
            if collision.entity == brick {
                // NEW: Check for paddle-destroyable brick
                if let Ok(brick_type) = brick_types.get(brick) {
                    if brick_type.0 == 57 {
                        debug!(
                            target: "paddle_destroyable",
                            "Paddle-brick type 57 collision detected: brick={:?}",
                            brick
                        );
                        // Mark for despawn (despawn system will emit BrickDestroyed)
                        commands.entity(brick).insert(MarkedForDespawn);
                    }
                }
                // Emit BrickHit event for audio (existing logic)
                commands.trigger(BrickHit { /* ... */ });
            }
        }
    }
}
```

**In Despawn System** (`src/lib.rs:despawn_marked_entities` - NO CHANGES):

```rust
// System automatically emits BrickDestroyed for ALL marked bricks
fn despawn_marked_entities(
    marked: Query<(Entity, Option<&BrickTypeId>), With<MarkedForDespawn>>,
    mut commands: Commands,
    mut brick_events: Option<MessageWriter<BrickDestroyed>>,
) {
    for (entity, brick_type) in marked.iter() {
        if let Some(type_id) = brick_type {
            if let Some(writer) = brick_events.as_mut() {
                writer.write(BrickDestroyed {
                    brick_entity: entity,
                    brick_type: type_id.0,  // Will be 57 for paddle-destroyable
                    destroyed_by: None,     // No destroyed_by for paddle destruction
                });
            }
        }
        commands.entity(entity).despawn_recursive();
    }
}
```

**In Scoring System** (`src/systems/scoring.rs` - NO CHANGES):

```rust
// System automatically awards 250 points for brick_type == 57
fn award_points_system(
    mut brick_destroyed_events: MessageReader<BrickDestroyed>,
    mut score_state: ResMut<ScoreState>,
) {
    let mut rng = rng();

    for event in brick_destroyed_events.read() {
        let points = brick_points(event.brick_type, &mut rng);
        // brick_points(57, _) returns 250 (already implemented)
        score_state.current_score = score_state.current_score.saturating_add(points);

        debug!(
            "Awarded {} points for brick type {}",
            points,
            event.brick_type
        );
    }
}
```

---

### Ball Interaction Prevention (Type 57)

**In Ball-Brick Collision Handler** (`src/lib.rs:handle_collision_events`):

```rust
// Prevent ball from destroying paddle-destroyable bricks
fn handle_collision_events(
    mut collision_events: MessageReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    bricks_info: Query<(Entity, &BrickTypeId, /* ... */), With<Brick>>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // Determine if ball-brick collision
            let brick_info = if balls.contains(*e1) {
                bricks_info.get(*e2).ok()
            } else if balls.contains(*e2) {
                bricks_info.get(*e1).ok()
            } else {
                None
            };

            if let Some((entity, brick_type, /* ... */)) = brick_info {
                let current_type = brick_type.0;

                // NEW: Skip destruction for paddle-destroyable bricks
                if is_paddle_destroyable_brick(current_type) {
                    continue;  // Ball bounces (automatic), no destruction
                }

                // Existing destruction logic for other brick types...
                // (multi-hit transition, regular brick despawn, etc.)
            }
        }
    }
}
```

---

## Contract Guarantees

### For Paddle-Destroyable Bricks (Type 57)

1. **Emission Timing**: `BrickDestroyed` message emitted BEFORE entity despawn (guaranteed by `despawn_marked_entities` system order)
2. **Field Values**:
   - `brick_entity`: Entity ID of the destroyed brick
   - `brick_type`: Always `57` for paddle-destroyable bricks
   - `destroyed_by`: Always `None` (paddle contact doesn't track paddle entity in message)
3. **Uniqueness**: Exactly ONE `BrickDestroyed` message per brick destruction (no duplicates)
4. **Ordering**: Message available for scoring system in same or next frame (buffered message queue)
5. **Persistence**: Score updates from message processing persist across all frames (multi-frame persistence)

### For Ball Interactions

1. **No Message**: Ball contact with type 57 bricks does NOT emit `BrickDestroyed` message
2. **No Despawn**: Ball contact does NOT mark type 57 bricks with `MarkedForDespawn`
3. **Physics Only**: Ball-brick collision generates standard physics response (bounce) via bevy_rapier3d
4. **No Score**: No points awarded when ball hits type 57 brick (0 points)

---

## Testing Contract

### Integration Test Expectations

**File**: `tests/paddle_destroyable_brick.rs`

```rust
// Test: Paddle contact emits BrickDestroyed with correct fields
#[test]
fn paddle_contact_emits_brick_destroyed_message() {
    let mut app = test_app();

    // Spawn paddle-destroyable brick (type 57)
    let brick = app.world_mut().spawn((
        Brick,
        BrickTypeId(57),
        CountsTowardsCompletion,
        Transform::from_xyz(0.0, 0.0, 10.0),
    )).id();

    // Simulate paddle collision (insert MarkedForDespawn)
    app.world_mut().entity_mut(brick).insert(MarkedForDespawn);

    // Run systems (despawn_marked_entities should emit message)
    app.update();

    // Verify BrickDestroyed message emitted
    let messages = app.world().resource::<Messages<BrickDestroyed>>();
    let mut reader = messages.reader();
    let events: Vec<_> = reader.read().collect();

    assert_eq!(events.len(), 1, "Exactly one BrickDestroyed message");
    assert_eq!(events[0].brick_type, 57, "Message brick_type is 57");
    assert_eq!(events[0].destroyed_by, None, "destroyed_by is None for paddle");
}

// Test: Ball contact does NOT emit BrickDestroyed message
#[test]
fn ball_contact_does_not_emit_message() {
    let mut app = test_app();

    // Spawn paddle-destroyable brick and ball
    let brick = app.world_mut().spawn((
        Brick,
        BrickTypeId(57),
        Transform::from_xyz(0.0, 0.0, 10.0),
    )).id();

    let ball = app.world_mut().spawn((
        Ball,
        Transform::from_xyz(0.0, 0.0, 9.5),
    )).id();

    // Simulate ball-brick collision event
    app.world_mut().write_message(CollisionEvent::Started(ball, brick, /* flags */));

    // Run collision handler
    app.update();

    // Verify brick still exists (not destroyed)
    assert!(app.world().get_entity(brick).is_some(), "Brick not destroyed by ball");

    // Verify NO BrickDestroyed message
    let messages = app.world().resource::<Messages<BrickDestroyed>>();
    let mut reader = messages.reader();
    assert_eq!(reader.read().count(), 0, "No BrickDestroyed message for ball contact");
}
```

---

## Migration Notes

**NO API CHANGES** - This feature uses the existing `BrickDestroyed` message contract without modifications.
Consumers of `BrickDestroyed` (scoring, completion tracking, audio) require zero changes to handle paddle-destroyable bricks.

**Backward Compatibility**: ✅ FULL - All existing brick types continue to work identically.
Type 57 adds new trigger condition (paddle contact) but does not alter existing ball-triggered destruction flow.

---

## Debug/Observability

**Logging**: Paddle-brick type 57 collisions logged at DEBUG level:

```rust
debug!(
    target: "paddle_destroyable",
    "Paddle-brick type 57 collision detected: brick={:?}",
    brick
);
```

**Message Inspection**: Use `MessageReader<BrickDestroyed>` in debug systems to observe message flow:

```rust
fn debug_brick_destroyed(mut reader: MessageReader<BrickDestroyed>) {
    for event in reader.read() {
        info!("BrickDestroyed: type={}, by={:?}", event.brick_type, event.destroyed_by);
    }
}
```
