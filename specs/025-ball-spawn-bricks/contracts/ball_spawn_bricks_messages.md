# Message Contracts: Ball Spawn Bricks

**Feature**: 025-ball-spawn-bricks  
**Date**: 2026-01-31  
**Standard**: Bevy 0.17 Messages (buffered, frame-agnostic communication)

## Overview

This feature uses the existing `BrickDestroyed` message from `crate::signals` to coordinate ball spawning/despawning across systems.
No new message types are required.

## Message Specifications

### 1. BrickDestroyed (Existing - Reused)

**Location**: `crate::signals::BrickDestroyed`

**Purpose**: Notify all systems that a brick has been destroyed.
Contains context needed for scoring, audio, and ball spawning.

**Message Type**: `#[derive(Message)]` — buffered, double-buffered queue

**Definition**:

```rust
#[derive(Message, Debug, Clone)]
pub struct BrickDestroyed {
    /// Entity ID of the destroyed brick
    pub brick_entity: Entity,

    /// Brick type/index from level data (range: 10-57, 90-97)
    pub brick_type: u8,

    /// 3D world position of brick center
    /// Used for: ball spawning position, audio source location
    pub brick_position: Vec3,

    /// Entity ID of the ball that triggered the destruction (if any)
    /// Used for: velocity inheritance, Red 1 despawning logic
    pub destroyed_by: Option<Entity>,
}
```

**Constraint Verification** (from spec):

- ✅ Contains `brick_type` to identify Red 1 (37), Red 2 (38), Red 3 (39)
- ✅ Contains `brick_position` for spawning at brick center
- ✅ Contains `destroyed_by` (Option<Entity>) to access triggering ball's velocity
- ✅ No additional message types needed

**Producer**:

```rust
// Existing system in collision/destruction module
fn brick_destruction_system(
    mut commands: Commands,
    mut writer: MessageWriter<BrickDestroyed>,
    collision_events: EventReader<CollisionEvent>,
    // ... query for brick/ball components
) {
    for event in collision_events.read() {
        // Detect ball-brick collision
        if is_brick_collision(&event) {
            let brick_pos = brick_query.get(brick_entity).unwrap().translation;
            writer.write(BrickDestroyed {
                brick_entity,
                brick_index,
                brick_position: brick_pos,
                triggering_ball: ball_entity,
            });
        }
    }
}
```

**Consumers** (with this feature):

1. **Scoring System** (`src/systems/scoring.rs` - existing):
   - Reads message
   - Awards points if `brick_index in [10..=57, 90..=97]` (all destructible types)
   - For Red 1/2/3: awards exactly 100 points

2. **Ball Spawn System** (`src/systems/ball_spawn_bricks.rs` - NEW):
   - Reads message
   - Checks if `brick_index in [37, 38, 39]`
   - If Red 2 (38): spawn 1 ball with inverse velocity
   - If Red 3 (39): spawn 2 balls with Y-shaped velocity spread
   - If Red 1 (37): despawn all balls except `triggering_ball`

3. **Audio System** (`src/systems/audio.rs` - existing):
   - Reads message
   - Plays brick destruction sound at `brick_position`
   - No changes needed for ball spawn bricks

---

## System Integration Points

### Ball Spawn System Implementation Pattern

```rust
use bevy::ecs::message::MessageReader;
use crate::signals::BrickDestroyed;

fn ball_spawn_system(
    mut commands: Commands,
    config: Res<BrickSpawnConfig>,
    mut reader: MessageReader<BrickDestroyed>,
    ball_sources: Query<&Velocity, With<Ball>>,
    // other params
) {
    for message in reader.read() {
        let Some(triggering_ball) = message.destroyed_by else { continue; };
        match message.brick_type {
            37 => {
                // Red 1: Despawn all balls except triggering_ball
                despawn_except_triggering(&mut commands, &ball_sources, triggering_ball);
            }
            38 => {
                // Red 2: Spawn 1 ball with inverse velocity
                let Ok(trigger_vel) = ball_sources.get(triggering_ball) else { continue; };
                spawn_ball(
                    &mut commands,
                    message.brick_position,
                    -trigger_vel.linvel,  // Inverse direction
                );
            }
            39 => {
                // Red 3: Spawn 2 balls with Y-shaped spread
                let Ok(trigger_vel) = ball_sources.get(triggering_ball) else { continue; };
                let (vel1, vel2) = spread_velocity_y_shaped(trigger_vel.linvel, 37.5);
                spawn_ball(&mut commands, message.brick_position, vel1);
                spawn_ball(&mut commands, message.brick_position, vel2);
            }
            _ => {
                // Other brick indices: no action
            }
        }
    }
}
```

---

## Message Lifecycle

### Timeline: Single Red 2 Brick Destruction

```text
Frame N:
  10:30 μs  - Ball physics collision detected (physics step)
  10:31 μs  - BrickDestroyed message written to buffer
  10:32 μs  - Brick entity despawned by destruction system

Frame N (later in schedule):
  11:00 μs  - Ball spawn system reads BrickDestroyed message
  11:01 μs  - Checks brick_index == 38
  11:02 μs  - Spawns new Ball entity with inverted velocity
  11:03 μs  - Scoring system reads same message
  11:04 μs  - Awards 100 points to score resource

Frame N+1:
  08:00 μs  - Physics system updates both balls independently
  08:01 μs  - New ball moves per physics simulation
  08:02 μs  - Collision detection for both balls
```

### Multi-Ball Scenario: Red 3 Brick Hit

```text
Frame N:
  - Ball A collides with Red 3 brick (index 39)
  - BrickDestroyed { triggering_ball: A, brick_index: 39, ... } written

Frame N (schedule order):
  1. Ball spawn system reads message
  2. Queries Ball A's velocity
  3. Spawns Ball B (Y-shape left) and Ball C (Y-shape right)
  4. Scoring system reads message, awards 100 points

Frame N+1:
  - Physics updates Ball A, B, C independently
  - All three balls subject to collisions
  - If Ball B hits another brick, new BrickDestroyed message sent
  - Process repeats for that brick
```

---

## Error Handling

### Missing Data Cases

**Case**: `triggering_ball` entity not found in `ball_query`

- **Cause**: Ball despawned before message processed (edge case, shouldn't happen)
- **Handling**: Log warning, skip ball spawning for this message
- **Impact**: Minimal; ball either wasn't present or was destroyed before spawn logic

**Case**: Brick position invalid (out of bounds)

- **Cause**: Level data corruption or system bug
- **Handling**: Clamp position to playfield bounds, log warning
- **Impact**: Ball spawned at clamped position; may immediately hit wall

### Constraints Checked

- ✅ `brick_index` must be in [37, 38, 39] for ball spawn behavior
- ✅ `brick_position` must be valid Vec3 (finite, not NaN)
- ✅ `triggering_ball` must be valid Entity (in world)
- ✅ No constraint on ball count (unlimited balls allowed)

---

## Testing Contracts

### Unit Test: Message Parsing

```rust
#[test]
fn test_brick_destroyed_message_for_red_2() {
    let brick_index = 38;
    let brick_pos = Vec3::new(5.0, 0.0, 10.0);

    let msg = BrickDestroyed {
        brick_entity: Entity::PLACEHOLDER,
        brick_index,
        brick_position: brick_pos,
        triggering_ball: Entity::PLACEHOLDER,
    };

    assert_eq!(msg.brick_index, 38);
    assert_eq!(msg.brick_position.z, 10.0);
}
```

### Integration Test: Message Flow

```rust
#[test]
fn test_message_flow_red_2_spawning() {
    let mut app = App::new();
    // Setup plugins, systems, entities

    // Simulate ball collision → message write
    let ball_entity = spawn_test_ball(&mut app);
    let brick_entity = spawn_test_brick(&mut app, 38);

    // Manually write message (or trigger via collision)
    app.world_mut()
        .resource_mut::<MessageWriter<BrickDestroyed>>()
        .write(BrickDestroyed {
            brick_entity,
            brick_index: 38,
            brick_position: Vec3::new(5.0, 0.0, 10.0),
            triggering_ball: ball_entity,
        });

    app.update();  // Process message

    // Verify spawned ball exists
    assert_eq!(app.world().query::<With<Ball>>().iter().count(), 2);
}
```

---

## Backward Compatibility

**Existing Systems**: No breaking changes.

- Scoring system continues to read `BrickDestroyed` as before
- Audio system continues to read `BrickDestroyed` as before
- New ball spawn system is **additive** (reads same message, new logic path)
- Message structure **extended** with `brick_position` and optional `destroyed_by` (additive fields)

**Migration Path**: None required.
Feature is backward compatible.

---

## Summary Table

| Aspect | Detail |
| ------ | ------ |
| **Message Type** | `BrickDestroyed` (existing, reused) |
| **Bevy Pattern** | `MessageWriter<T>` / `MessageReader<T>` (buffered) |
| **Producers** | Brick destruction system (1 producer path) |
| **Consumers** | Scoring, Audio, Ball Spawn systems (3 consumer paths) |
| **Buffering** | Double-buffered (automatic via Bevy) |
| **Ordering** | All consumers read same message in arbitrary order (safe) |
| **Error Handling** | Missing entity → log warning, skip action |
| **Cardinality** | 1 message per brick destroyed, 0+ systems consuming |
| **Latency** | 1 frame buffer (message written frame N, read frame N+1) |

No new message types needed.
Spec requirements fully satisfied by existing `BrickDestroyed` message structure.
