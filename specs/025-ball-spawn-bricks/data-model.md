# Data Model: Ball Spawn Bricks

**Feature**: 025-ball-spawn-bricks  
**Date**: 2026-01-31  
**Scope**: Entity definitions, message schemas, component relationships

## Overview

This feature extends the existing ball physics and brick destruction systems with three new destructible brick types that manipulate ball quantity during gameplay.
The data model reuses existing `Ball` and `BrickDestroyed` entities/messages, requiring only configuration and spawning logic additions.

## Entities

### 1. Ball (Existing Component - No Changes Required)

**Purpose**: Represents a single ball in play on the game board.

**Current Definition** (assumed from codebase):

```rust
pub struct Ball {
    // Physics properties managed by Bevy Transform + RigidBody + Collider
    // Velocity via Rapier3D Velocity component
    // Position via Transform component
}
```

**No changes needed**: Spawned balls will use the same component bundle as existing balls.

---

### 2. BrickSpawnConfig (New Resource)

**Purpose**: Configuration mapping brick indices to spawn behaviors and point values.

**Definition**:

```rust
#[derive(Resource, Debug, Clone)]
pub struct BrickSpawnConfig {
    /// Map brick index → (spawn_count, velocity_modifier)
    pub brick_spawn_rules: HashMap<u32, BrickSpawnRule>,

    /// Point value for all ball spawn bricks (37, 38, 39)
    pub spawn_brick_score: u32,
}

#[derive(Debug, Clone)]
pub struct BrickSpawnRule {
    /// Brick index (37, 38, or 39)
    pub brick_index: u32,

    /// How many balls to spawn (0 for Red 1/despawn, 1 for Red 2, 2 for Red 3)
    pub spawn_count: u32,

    /// Type of velocity modification to apply
    pub velocity_modifier: VelocityModifier,

    /// Human-readable name for logging/documentation
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VelocityModifier {
    /// No spawning (Red 1: despawn all except triggering ball)
    DespawnAll,
    /// Negate velocity vector (Red 2: inverse direction)
    Inverse,
    /// Spread in Y-shaped pattern (Red 3: ±30-45 degrees left/right)
    YShaped {
        /// Spread angle in degrees (default 45)
        angle_degrees: f32,
    },
}
```

**Initialization** (in plugin Startup):

```rust
fn init_brick_spawn_config(mut commands: Commands) {
    let config = BrickSpawnConfig {
        brick_spawn_rules: [
            (37, BrickSpawnRule {
                brick_index: 37,
                spawn_count: 0,
                velocity_modifier: VelocityModifier::DespawnAll,
                name: "Red 1 (Despawn)",
            }),
            (38, BrickSpawnRule {
                brick_index: 38,
                spawn_count: 1,
                velocity_modifier: VelocityModifier::Inverse,
                name: "Red 2 (Spawn 1)",
            }),
            (39, BrickSpawnRule {
                brick_index: 39,
                spawn_count: 2,
                velocity_modifier: VelocityModifier::YShaped { angle_degrees: 45.0 },
                name: "Red 3 (Spawn 2)",
            }),
        ].into_iter().collect(),
        spawn_brick_score: 100,
    };
    commands.insert_resource(config);
}
```

---

### 3. BrickDestroyed (Existing Message - Used As-Is)

**Purpose**: Signal that a brick has been destroyed, including context for ball spawning.

**Current Definition** (from `crate::signals`):

```rust
#[derive(Message, Debug, Clone)]
pub struct BrickDestroyed {
    pub brick_entity: Entity,
    pub brick_index: u32,
    pub brick_position: Vec3,  // Needed for spawning at brick center
    pub triggering_ball: Entity,  // Needed to identify which ball triggered
}
```

**Required Fields for Ball Spawning**:

- `brick_index`: Determines spawn behavior (37/38/39)
- `brick_position`: Where to spawn new balls (XZ + Y)
- `triggering_ball`: Whose velocity to use for spawned balls

**No changes needed**: Existing message already contains all required data.
If not, enhancement required before implementation.

---

## Component Relationships

### Entity Hierarchies

**Ball Entities**:

- No parent-child relationships
- Independent physics entities
- Components:
  - `Ball` (marker component)
  - `Transform` (position, inherited from brick at spawn time)
  - `GlobalTransform` (auto-updated by Bevy)
  - `Velocity` (Rapier3D: inherits from triggering ball, modified per brick type)
  - `RigidBody` (Rapier3D physics)
  - `Collider` (Rapier3D collision detection)
  - `Restitution`, `Friction` (physics tuning, inherited from ball template)

**Brick Entities** (unchanged):

- Brick components already in place (existing feature)
- No interaction with spawned balls at component level
- Communication via `BrickDestroyed` message only

---

## State Transitions

### Ball Lifecycle

**Single Ball**:

```text
Spawned (original) → Active (moving) → Destroyed (paddle miss or despawned)
```

**Multi-Ball (Red 2)**:

```text
Initial Ball: Spawned → Active → Destroyed
On Red 2 brick hit:
  ├─ Triggering Ball: continues as Active
  └─ New Ball: Spawned (at brick center) → Active → Destroyed
```

**Multi-Ball (Red 3)**:

```text
Initial Ball: Spawned → Active → Destroyed
On Red 3 brick hit:
  ├─ Triggering Ball: continues as Active
  ├─ New Ball 1: Spawned (Y-shape left) → Active → Destroyed
  └─ New Ball 2: Spawned (Y-shape right) → Active → Destroyed
```

**Reset (Red 1)**:

```text
Multiple Balls: Active → Active (many)
On Red 1 brick hit:
  ├─ Triggering Ball: continues as Active
  └─ Other Balls: Active → Despawned (destroyed immediately)
```

---

## Message Flow

### Sequence: Red 2 Brick Destruction

```text
1. Ball Physics System
   - Ball collides with brick 38
   - Physics collision detected

2. Brick Destruction System (existing)
   - Brick entity despawned
   - BrickDestroyed message written:
     {
       brick_entity: [brick],
       brick_index: 38,
       brick_position: Vec3 { x, y, z },
       triggering_ball: [ball entity]
     }

3. Ball Spawn System (NEW)
   - MessageReader<BrickDestroyed> reads message
   - Checks brick_index == 38
   - Queries triggering_ball for Velocity component
   - Calculates new ball velocity: -velocity (inverse)
   - Spawns new Ball entity at brick_position with inverted velocity
   - NEW Ball immediately subject to physics (next frame)

4. Scoring System (existing)
   - MessageReader<BrickDestroyed> reads same message
   - Awards 100 points (brick_index in [37, 38, 39])
   - May trigger milestone logic (+1 life at 5000-point intervals)

5. Next Frame
   - Physics system updates both balls independently
   - Both balls may collide with other bricks/walls
   - Cycle continues
```

### Sequence: Red 1 Brick Destruction

```text
1. Ball Physics System
   - Any ball collides with brick 37
   - Physics collision detected

2. Brick Destruction System (existing)
   - Brick entity despawned
   - BrickDestroyed message written:
     {
       brick_entity: [brick],
       brick_index: 37,
       brick_position: Vec3 { x, y, z },
       triggering_ball: [ball entity A]
     }

3. Ball Spawn System (NEW)
   - MessageReader<BrickDestroyed> reads message
   - Checks brick_index == 37
   - Queries all Ball entities EXCEPT triggering_ball
   - Despawns all non-triggering balls (removes from world)
   - Triggering ball continues unchanged

4. Scoring System (existing)
   - Awards 100 points

5. Next Frame
   - Only triggering ball remains in play
   - Physics continues for single ball
```

---

## Validation Rules

### Ball Spawning Constraints

1. **Position Validity**: Spawned ball position must be at brick's XZ center + brick's Y (not out-of-bounds)
2. **Velocity Validity**: Spawned ball velocity must be non-zero and inheritable from triggering ball
3. **Entity Limit**: No hard cap; system handles unlimited balls (no error condition)

### Ball Despawning Constraints

1. **Triggering Ball Preservation**: Red 1 brick MUST NOT despawn the ball that hit it
2. **All-or-Nothing**: Red 1 despawning is immediate; no partial despawning
3. **No Resurrection**: Despawned balls remain gone (no respawn checks for 10+ frames)

### Score Constraints

1. **Exactness**: All three brick types MUST award exactly 100 points (no random values)
2. **Consistency**: Score awarded once per brick destruction (no double-counting)

---

## Performance Considerations

### Runtime Complexity

- **Spawning**: O(1) per Red 2/3 brick destruction (constant-time entity creation)
- **Despawning**: O(n) per Red 1 brick destruction (query all balls, despawn n-1; typical n=3-5)
- **Message Processing**: O(1) amortized (buffered message reader)

### Memory Overhead

- **Per Spawned Ball**: ~1-2 KB (Transform, Velocity, Collider components)
- **Config Resource**: <1 KB (3 brick entries + metadata)
- **Total WASM Impact**: ~2-3 KB compiled size (minimal)

---

## Testing Considerations

### Unit Test Entities

```rust
// Test ball spawning at brick position
let ball = spawn_test_ball(pos);
let brick = spawn_test_brick(38, brick_pos);
trigger_collision(ball, brick);
assert_eq!(ball_query.iter().len(), 2);  // One original + one spawned

// Test velocity inheritance
let spawned = ball_query.iter().find(|b| /* not original */);
assert_eq!(spawned.velocity, -original.velocity);

// Test Red 1 despawning
spawn_multiple_balls(5);
trigger_brick_37_destruction();
assert_eq!(ball_query.iter().len(), 1);
```

### Multi-Frame Persistence Test

```rust
// Spawn 10 frames, verify physics applies
for frame in 0..10 {
    app.update();
    let spawned = ball_query.iter().find(|b| is_spawned(b));
    assert!(spawned.is_some());
    assert_ne!(spawned.transform.translation, previous_position);
}
```

---

## Related Entities (Existing)

- **Paddle** (existing): Can collide with spawned balls normally
- **Bricks** (existing): Spawned balls can hit other bricks and trigger their effects
- **Walls** (existing): Spawned balls bounce off walls normally
- **Camera** (existing): No changes needed; spawned balls visible like originals
- **Physics World** (existing): Handles spawned ball gravity, collision, velocity

---

## Summary

The data model is minimal and extends existing infrastructure:

| Item | Type | New/Reuse | Notes |
|------|------|-----------|-------|
| Ball | Component | Reuse | No changes; spawned balls use existing template |
| BrickSpawnConfig | Resource | New | Maps indices 37/38/39 to spawn behaviors |
| BrickDestroyed | Message | Reuse | Already contains position & triggering ball |
| VelocityModifier | Enum | New | Configuration for velocity transformation |
| State Transitions | Logic | New | Tracked implicitly (balls exist or don't) |

No database changes, no new resource serialization, no additional persistence layers.
All state ephemeral (in-memory ECS), consistent with existing game architecture.
