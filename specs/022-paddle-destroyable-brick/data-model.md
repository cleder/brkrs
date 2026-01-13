# Data Model: Paddle-Destroyable Brick (Type 57)

**Feature**: 022-paddle-destroyable-brick | **Date**: 2026-01-13

## Overview

This feature adds a new brick type (57) with inverse destruction mechanics: destroyed only by paddle contact (not ball), awards 250 points, counts toward level completion.
The data model reuses existing components and messages to minimize complexity.

---

## Core Entities

### Paddle-Destroyable Brick (Type 57)

**Purpose**: A brick entity that is destroyed when the paddle touches it, awards 250 points, and causes the ball to bounce off without destruction.

**Components**:

- `Brick` - Marker component identifying entity as a brick
- `BrickTypeId(57)` - Type identifier stored as u8 (value 57)
- `CountsTowardsCompletion` - Marker indicating brick must be destroyed for level completion
- `Transform` - Spatial position and rotation (required for rendering and physics)
- `Collider` - Physics collider for paddle and ball contact detection (bevy_rapier3d)
- `MeshMaterial3d<StandardMaterial>` - Rendering material (textured visual from manifest)
- `Mesh3d` - 3D mesh handle (typically cuboid for bricks)

**Invariants**:

- `BrickTypeId.0 == 57` uniquely identifies paddle-destroyable bricks
- `CountsTowardsCompletion` MUST be present (spec FR-008)
- Collider MUST allow both paddle and ball physics interactions
- Entity is despawned (destroyed) on paddle contact, NOT on ball contact

**Lifecycle**:

1. Spawned by level loader from RON file entry `{ brick_type: 57, ... }`
2. Exists in world until paddle collides with it
3. On paddle contact: Marked with `MarkedForDespawn` component
4. Despawned by `despawn_marked_entities` system (emits `BrickDestroyed` message)
5. Score system reads `BrickDestroyed` and awards 250 points

---

## Components

### Existing Components (Reused)

**`Brick`** (marker)

- **Purpose**: Identifies entity as a brick for queries
- **Fields**: None (marker component)
- **Usage**: `Query<Entity, With<Brick>>` to find all bricks

**`BrickTypeId(u8)`** (data)

- **Purpose**: Stores brick type ID (57 for paddle-destroyable)
- **Fields**: `0: u8` - Type identifier
- **Usage**: `brick_type.0 == 57` to identify paddle-destroyable bricks

**`CountsTowardsCompletion`** (marker)

- **Purpose**: Flags brick as required for level completion
- **Fields**: None (marker component)
- **Usage**: Completion tracking system counts bricks with this component

**`MarkedForDespawn`** (marker)

- **Purpose**: Tags entities for removal in despawn system
- **Fields**: None (marker component)
- **Usage**: Added by paddle collision handler; read by `despawn_marked_entities`

**No New Components Required** - Feature entirely uses existing component types.

---

## Messages (Events)

### Existing Messages (Reused)

**`BrickDestroyed`**

- **Purpose**: Signals that a brick was destroyed (by ball OR paddle)
- **Definition** (from `src/signals.rs`):

  ```rust
  #[derive(Message, Debug, Clone, Copy)]
  pub struct BrickDestroyed {
      pub brick_entity: Entity,  // The destroyed brick
      pub brick_type: u8,         // Brick type ID (57 for paddle-destroyable)
      pub destroyed_by: Option<Entity>,  // None for paddle destruction
  }
  ```

- **Emitted By**: `despawn_marked_entities` system (when `MarkedForDespawn` entity has `BrickTypeId`)
- **Consumed By**: `award_points_system` in `src/systems/scoring.rs`
- **Contract**: Emitted once per brick destruction before entity despawn

**No New Messages Required** - Existing `BrickDestroyed` message handles both ball-triggered and paddle-triggered destruction.

---

## Functions / Utilities

### `is_paddle_destroyable_brick(brick_type: u8) -> bool`

**Purpose**: Identify paddle-destroyable bricks by type ID.

**Signature**:

```rust
pub fn is_paddle_destroyable_brick(brick_type: u8) -> bool {
    brick_type == 57
}
```

**Usage**: Guard in ball-brick collision handler to prevent ball-triggered destruction.

**Location**: `src/lib.rs` (alongside `is_multi_hit_brick()`)

---

### `brick_points(brick_type: u8, rng: &mut impl Rng) -> u32`

**Purpose**: Map brick type to point value (already implemented).

**Existing Mapping** (in `src/systems/scoring.rs` line 123):

```rust
57 => 250,  // Paddle-destroyable brick
```

**No Changes Needed** - Scoring function already configured for type 57.

---

## Systems

### New System: Paddle-Brick Collision Handler Extension

**Location**: `src/lib.rs:read_character_controller_collisions` (extend existing system)

**Purpose**: Detect paddle-brick type 57 collisions and mark brick for despawn.

**Query Additions**:

- Add `brick_types: Query<&BrickTypeId, With<Brick>>` parameter
- Add `marked_despawn: Query<&MarkedForDespawn>` parameter

**Logic**:

```rust
// In existing paddle-brick collision loop:
for brick in bricks.iter() {
    if collision.entity == brick {
        // NEW: Check if this is a paddle-destroyable brick
        if let Ok(brick_type) = brick_types.get(brick) {
            if brick_type.0 == 57 {
                debug!(
                    target: "paddle_destroyable",
                    "Paddle-brick type 57 collision: paddle={:?}, brick={:?}",
                    paddle_entity, brick
                );
                // Mark for despawn (triggers score award via BrickDestroyed message)
                commands.entity(brick).insert(MarkedForDespawn);
            }
        }
        // Emit BrickHit event for audio (existing logic continues)
    }
}
```

---

### Modified System: Ball-Brick Collision Handler

**Location**: `src/lib.rs:handle_collision_events` (add guard)

**Purpose**: Prevent ball from destroying paddle-destroyable bricks.

**Logic Addition**:

```rust
// In ball-brick collision processing:
if let Some((entity, brick_type_ro, gt_opt, t_opt)) = brick_info {
    let current_type = brick_type_ro.0;

    // NEW: Skip destruction if paddle-destroyable brick
    if is_paddle_destroyable_brick(current_type) {
        continue;  // Ball bounces off (physics automatic), no destruction
    }

    // Existing destruction logic continues for other brick types...
}
```

---

## Relationships

```text
Level File (RON)
    ↓ (brick_type: 57)
Level Loader (spawn_level_entities_impl)
    ↓ (spawns entity with components)
Paddle-Destroyable Brick Entity
    ├─ BrickTypeId(57)
    ├─ Brick (marker)
    ├─ CountsTowardsCompletion (marker)
    ├─ Transform, Collider, Mesh3d, MeshMaterial3d

Paddle Contact
    ↓ (kinematic controller collision)
read_character_controller_collisions
    ↓ (inserts component)
MarkedForDespawn
    ↓ (read by despawn system)
despawn_marked_entities
    ↓ (emits message, despawns entity)
BrickDestroyed (message)
    ↓ (read by scoring)
award_points_system
    ↓ (mutates resource)
ScoreState (+250 points)

Ball Contact
    ↓ (rapier collision event)
handle_collision_events
    ↓ (checks brick type)
is_paddle_destroyable_brick() == true
    → Early return (no destruction)
    → Ball bounces (bevy_rapier3d automatic)
```

---

## Validation Rules

1. **Type Uniqueness**: `BrickTypeId.0 == 57` is the sole identifier for paddle-destroyable bricks
2. **Completion Requirement**: All type 57 bricks MUST have `CountsTowardsCompletion` component
3. **Point Value**: Type 57 MUST map to exactly 250 points in `brick_points()` function
4. **Destruction Source**: Only paddle contact can trigger `MarkedForDespawn` for type 57 bricks
5. **Ball Interaction**: Ball MUST bounce off type 57 bricks without triggering destruction
6. **Despawn Method**: Type 57 bricks MUST use `commands.entity(brick).despawn_recursive()` to handle potential hierarchy
7. **Message Contract**: `BrickDestroyed` message MUST be emitted before entity despawn (handled by `despawn_marked_entities`)

---

## Performance Characteristics

- **Collision Detection**: O(n) where n = paddle collisions per frame (typically 0-3)
- **Type Check**: O(1) hash lookup for `BrickTypeId` component
- **Memory Overhead**: Zero new component types; reuses existing ECS infrastructure
- **Message Throughput**: 1 `BrickDestroyed` message per paddle-brick destruction (typically 0-1 per frame)

---

## Testing Implications

### Unit Tests

- `is_paddle_destroyable_brick()` returns true for 57, false for all other types
- `brick_points(57, &mut rng)` returns exactly 250

### Integration Tests (see `tests/paddle_destroyable_brick.rs`)

- Paddle contact destroys brick within 1 frame
- Ball contact does NOT destroy brick (brick persists 10+ frames)
- Score increases by exactly 250 on paddle destruction
- Brick counts toward level completion
- `BrickDestroyed` message emitted with correct fields
- Multi-frame persistence: score and brick state persist 10 frames
