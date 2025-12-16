# Data Model: Add Scoring System

**Date**: 16 December 2025 **Feature**: Add Scoring System **Branch**: 009-add-scoring

## Resources

### `ScoreState`

**Purpose**: Global game state tracking cumulative player score and milestone progress.

| Field | Type | Description |
|-------|------|-------------|
| `current_score` | `u32` | Total points accumulated in current game session. Range: 0 to u32::MAX. |
| `last_milestone_reached` | `u32` | Highest milestone tier achieved (e.g., 0, 1, 2 for 0, 5000, 10000 points). |

**Validation Rules**:

- `current_score` MUST be non-negative (enforced by type)
- `last_milestone_reached` MUST be ≤ `current_score / 5000`
- Score resets to 0 on new game start
- Score persists across level transitions within same game session

**State Transitions**:

- Game start → `current_score = 0`, `last_milestone_reached = 0`
- Brick destroyed → `current_score += brick_points(brick_type)`
- `current_score / 5000 > last_milestone_reached` → emit `MilestoneReached`, increment `last_milestone_reached`

---

## Messages (Events)

### `BrickDestroyed`

**Purpose**: Domain signal that a brick was destroyed by ball collision, triggering point award.

| Field | Type | Description |
|-------|------|-------------|
| `brick_entity` | `Entity` | The brick entity that was destroyed. |
| `brick_type` | `BrickType` (or equivalent) | Type/index of brick (determines point value). |
| `destroyed_by` | `Entity` | The ball entity that destroyed the brick (optional, for future features). |

**Emitted By**: Brick destruction/collision systems in `src/systems/bricks/`.

**Consumed By**: `award_points_system` in `src/systems/scoring.rs`.

---

### MilestoneReached (Message)

**Purpose**: Domain signal that score crossed a 5000-point threshold, triggering an extra ball/life award.

| Field | Type | Description |
|-------|------|-------------|
| `milestone_tier` | `u32` | Which milestone was reached (1 for 5000, 2 for 10000, etc.). |
| `total_score` | `u32` | Current score when milestone triggered. |

**Emitted By**: `detect_milestone_system` in `src/systems/scoring.rs`.

**Consumed By**: Ball award logic in `src/systems/respawn.rs` (increments `LivesState.lives_remaining`).

---

## Components

### `ScoreDisplayUi`

**Purpose**: Marker component tagging the UI entity that displays current score.

**Fields**: None (marker only).

**Usage**: Attached to Bevy `TextBundle` entity.
Systems query `With<ScoreDisplayUi>` to update text content.

---

## Functions / Utilities

### `brick_points(brick_type: BrickType) -> u32`

**Purpose**: Map brick type to point value per docs/bricks.md.

**Implementation**: Match expression covering all brick indices (10-57).

**Special Cases**:

- Question brick (53): Returns random value via `rng.gen_range(25..=300)`
- Extra Ball brick (41): Returns 0 (grants ball, not points)
- Magnet bricks (55-56): Returns 0 (no point value)
- Solid bricks (90-97): Returns 0 (indestructible, shouldn't trigger this function)

**Validation**: Compile-time exhaustiveness checking ensures all brick types handled.

---

## Relationships

```text
BrickDestroyed (event)
    ↓
award_points_system
    ↓ (mutates)
ScoreState (resource)
    ↓ (reads)
detect_milestone_system
    ↓ (conditionally emits)
MilestoneReached (event)
    ↓
award_milestone_ball_system
    ↓ (mutates)
LivesState (resource)
    ↓ (change detection)
lives_display_system (existing)

ScoreState (resource)
    ↓ (change detection)
update_score_display_system
    ↓ (mutates)
TextBundle (with ScoreDisplayUi marker)
```

---

## Integration with Existing Systems

### Brick Destruction Systems

**Current**: Brick collision/destruction logic despawns brick entities.

**Change Required**: Emit `BrickDestroyed` message with brick type before despawning.

```rust
// Example pseudo-code
fn handle_brick_collision(...) {
    if brick_should_be_destroyed {
        brick_destroyed_events.write(BrickDestroyed {
            brick_entity,
            brick_type,
            destroyed_by: ball_entity,
        });
        commands.entity(brick_entity).despawn_recursive();
    }
}
```

---

### Ball Spawning (Respawn System)

**Current**: Spawns balls on `LifeLostEvent` and level start.

**Change Required**: Add system listening to `MilestoneReached`, increment `LivesState` to grant extra ball/life.

```rust
// Example pseudo-code
fn award_milestone_ball(
    mut milestone_events: MessageReader<MilestoneReached>,
    mut lives_state: ResMut<LivesState>,
) {
    for event in milestone_events.read() {
        // Award extra ball/life by incrementing lives counter
        lives_state.lives_remaining += 1;
        info!("Extra ball awarded! Lives: {}", lives_state.lives_remaining);
    }
}
```

---

### Level Transitions

**Current**: Level loader initializes game state on level start.

**Change Requirement**: DO NOT reset `ScoreState` on level advance.
Only reset on explicit game restart.

---

## Data Flow Example

1. Player's ball hits Simple Stone brick (index 20, worth 25 points)
2. Brick collision system emits `BrickDestroyed { brick_entity, brick_type: SimpleStone, ... }`
3. `award_points_system` reads event, calls `brick_points(SimpleStone)` → 25
4. System mutates `ScoreState.current_score += 25` (e.g., 4980 → 5005)
5. `detect_milestone_system` sees `5005 / 5000 = 1 > last_milestone_reached (0)`
6. System emits `MilestoneReached { milestone_tier: 1, total_score: 5005 }`
7. System updates `ScoreState.last_milestone_reached = 1`
8. `award_milestone_ball_system` reads `MilestoneReached`, increments `LivesState.lives_remaining += 1`
9. Lives counter UI updates to show new ball/life count (e.g., "♥ ♥ ♥ ♥")
10. `update_score_display_system` detects `ScoreState` change, updates UI text to "5005"

---

## Testing Validation Points

- `ScoreState.current_score` matches sum of all destroyed brick values
- `last_milestone_reached` increments only when crossing 5000-point thresholds
- `MilestoneReached` emitted exactly once per milestone
- Random Question brick scores fall within [25, 300] range
- Score persists across level transitions
- Score resets to 0 on game restart
- UI text content matches `current_score` value
