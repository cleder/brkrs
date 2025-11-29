# Data Model: Multi-Hit Bricks

**Feature**: 005-multi-hit-bricks
**Date**: 2025-11-29

## Entities

### BrickTypeId Component (Existing)

The existing `BrickTypeId(u8)` component serves as the state representation for multi-hit bricks.

| Field | Type | Description |
|-------|------|-------------|
| `0` | `u8` | Brick type index (10-13 for multi-hit, 20 for simple stone) |

**State Values**:

| Index | Name | Hits Remaining | Next State |
|-------|------|----------------|------------|
| 13 | Hit 4 | 4 | 12 |
| 12 | Hit 3 | 3 | 11 |
| 11 | Hit 2 | 2 | 10 |
| 10 | Hit 1 | 1 | 20 |
| 20 | Simple Stone | 1 | Destroyed |

### Multi-Hit Brick Constants (New)

Located in `src/level_format/mod.rs`:

```rust
/// First multi-hit brick index (needs 1 more hit to become simple stone)
pub const MULTI_HIT_BRICK_1: u8 = 10;

/// Multi-hit brick needing 2 more hits
pub const MULTI_HIT_BRICK_2: u8 = 11;

/// Multi-hit brick needing 3 more hits
pub const MULTI_HIT_BRICK_3: u8 = 12;

/// Multi-hit brick needing 4 more hits (maximum durability)
pub const MULTI_HIT_BRICK_4: u8 = 13;

/// Range check helper
pub fn is_multi_hit_brick(type_id: u8) -> bool {
    (MULTI_HIT_BRICK_1..=MULTI_HIT_BRICK_4).contains(&type_id)
}
```

## State Transitions

```text
┌─────────────────────────────────────────────────────────────┐
│                    Multi-Hit Brick Lifecycle                │
└─────────────────────────────────────────────────────────────┘

  ┌──────────┐    Hit    ┌──────────┐    Hit    ┌──────────┐
  │ Index 13 │ ────────► │ Index 12 │ ────────► │ Index 11 │
  │  Hit 4   │  +50 pts  │  Hit 3   │  +50 pts  │  Hit 2   │
  └──────────┘           └──────────┘           └──────────┘
                                                      │
                                                      │ Hit
                                                      │ +50 pts
                                                      ▼
  ┌──────────┐    Hit    ┌──────────┐    Hit    ┌──────────┐
  │ Despawn  │ ◄──────── │ Index 20 │ ◄──────── │ Index 10 │
  │          │  +25 pts  │  Stone   │  +50 pts  │  Hit 1   │
  └──────────┘           └──────────┘           └──────────┘
```

## Validation Rules

1. **Index Range**: Multi-hit brick indices MUST be in range 10-13
2. **Transition Order**: Decrements by 1 on each hit until reaching 10
3. **Terminal Transition**: Index 10 transitions to index 20 (simple stone)
4. **Destruction**: Index 20 follows normal brick despawn behavior
5. **Level Completion**: All entities with `CountsTowardsCompletion` must be despawned

## Relationships

```text
┌─────────────────┐
│     Entity      │
│  (Brick)        │
├─────────────────┤
│ - Brick         │ ← Marker component
│ - BrickTypeId   │ ← State (10-13, 20, etc.)
│ - CountsTowards │ ← Level completion tracking
│   Completion    │
│ - Transform     │ ← Position
│ - Collider      │ ← Physics
│ - MeshMaterial  │ ← Visual (updated on type change)
└─────────────────┘
         │
         │ Changed(BrickTypeId)
         ▼
┌─────────────────┐
│ TypeVariant     │
│ Registry        │
├─────────────────┤
│ Brick variants: │
│  10 → material  │
│  11 → material  │
│  12 → material  │
│  13 → material  │
└─────────────────┘
```

## Level File Format

Multi-hit bricks are placed in level matrices using their index value (10-13):

```ron
LevelDefinition(
    number: 1,
    matrix: [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 13, 13, 12, 12, 11, 11, 10, 10, 20, 20, 10, 10, 11, 11, 12, 12, 13, 13, 0],
        // ... 13 = 4 hits, 12 = 3 hits, 11 = 2 hits, 10 = 1 hit, 20 = simple
    ]
)
```

## Events (For Future Audio/Scoring Integration)

### MultiHitBrickHit (Proposed)

Emitted when a multi-hit brick (10-13) is hit by the ball.

| Field | Type | Description |
|-------|------|-------------|
| `entity` | `Entity` | The brick entity that was hit |
| `previous_type` | `u8` | Type ID before hit (10-13) |
| `new_type` | `u8` | Type ID after hit (9-12 or 20) |
| `points` | `u32` | Points to award (50) |

### BrickDestroyed (Proposed)

Emitted when any brick (including former multi-hit) is destroyed.

| Field | Type | Description |
|-------|------|-------------|
| `entity` | `Entity` | The brick entity being destroyed |
| `type_id` | `u8` | Final type ID at destruction |
| `points` | `u32` | Points to award |

**Note**: Event implementation is optional for MVP. Core functionality works without events; they enable future audio/scoring integration.
