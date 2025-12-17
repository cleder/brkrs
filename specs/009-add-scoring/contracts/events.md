# Event Contracts: Add Scoring System

**Date**: 16 December 2025 **Feature**: Add Scoring System **Format**: Rust Event Definitions (Bevy Messages)

## Overview

This document defines the message contracts (events) used by the scoring system to communicate with other game systems.

---

## Events

### BrickDestroyed

**Purpose**: Signals that a brick was destroyed and points should be awarded.

**Trigger Condition**: Ball collision causes brick health to reach zero OR single-hit brick is struck.

**Event Definition**:

```rust
use bevy::prelude::*;
use bevy::ecs::message::Message;

#[derive(Message, Debug, Clone, Copy)]
pub struct BrickDestroyed {
    /// The brick entity that was destroyed
    pub brick_entity: Entity,

    /// The type/index of the brick (determines point value)
    pub brick_type: BrickType,

    /// The ball that destroyed the brick (optional, for future analytics)
    pub destroyed_by: Option<Entity>,
}
```

**Emitters**:

- Brick collision handling systems in `src/systems/bricks/destruction.rs`
- Multi-hit brick degradation system when final hit occurs

**Consumers**:

- `award_points_system` in `src/systems/scoring.rs` - Awards points based on brick type

**Usage Pattern**:

```rust
// In brick collision/destruction system
fn handle_brick_destruction(
    // ... query params
    mut brick_destroyed_events: MessageWriter<BrickDestroyed>,
    mut commands: Commands,
) {
    // ... collision detection logic

    if brick_should_be_destroyed {
        // Emit event BEFORE despawning
        brick_destroyed_events.write(BrickDestroyed {
            brick_entity: brick,
            brick_type: BrickType::SimpleStone,  // or detected type
            destroyed_by: Some(ball_entity),
        });

        // Then despawn brick
        commands.entity(brick).despawn_recursive();
    }
}

// In scoring system
fn award_points(
    mut brick_destroyed_events: MessageReader<BrickDestroyed>,
    mut score_state: ResMut<ScoreState>,
    mut rng: ResMut<GlobalRng>,
) {
    for event in brick_destroyed_events.read() {
        let points = brick_points(event.brick_type, &mut rng);
        score_state.current_score += points;
        info!("Awarded {} points for {:?}", points, event.brick_type);
    }
}
```

---

### MilestoneReached

**Purpose**: Signals that the player's score crossed a 5000-point milestone threshold, triggering an extra ball (life) award.

**Trigger Condition**: `ScoreState.current_score / 5000 > ScoreState.last_milestone_reached`

**Event Definition**:

```rust
use bevy::prelude::*;
use bevy::ecs::message::Message;

#[derive(Message, Debug, Clone, Copy)]
pub struct MilestoneReached {
    /// Which milestone tier was reached (1 for 5000, 2 for 10000, etc.)
    pub milestone_tier: u32,

    /// The total score when milestone was triggered
    pub total_score: u32,
}
```

**Emitters**:

- `detect_milestone_system` in `src/systems/scoring.rs` - Checks after every score update

**Consumers**:

- `award_milestone_ball_system` in `src/systems/respawn.rs` - Increments `LivesState` to grant extra ball/life

**Usage Pattern**:

```rust
// In scoring system (milestone detection)
fn detect_milestone(
    score_state: Res<ScoreState>,
    mut milestone_events: MessageWriter<MilestoneReached>,
) {
    // Only run when score changes
    if score_state.is_changed() {
        let current_milestone = score_state.current_score / 5000;

        if current_milestone > score_state.last_milestone_reached {
            milestone_events.write(MilestoneReached {
                milestone_tier: current_milestone,
                total_score: score_state.current_score,
            });

            // Note: ScoreState.last_milestone_reached is updated
            // in the award_points system to maintain correct state
        }
    }
}

// In respawn system (award extra ball/life)
fn award_milestone_ball(
    mut milestone_events: MessageReader<MilestoneReached>,
    mut lives_state: ResMut<LivesState>,
) {
    for event in milestone_events.read() {
        info!("Milestone reached: tier {} at {} points - awarding extra ball",
              event.milestone_tier, event.total_score);

        // Grant extra ball/life by incrementing lives counter
        lives_state.lives_remaining += 1;
    }
}
```

---

## Event Ordering & Timing

### BrickDestroyed Event Flow

1. **Frame N**: Ball-brick collision detected by physics engine
2. **Frame N**: `CollisionEvent` emitted by Rapier
3. **Frame N**: Brick destruction system processes collision, emits `BrickDestroyed`
4. **Frame N**: Brick entity despawned
5. **Frame N+0**: Scoring system reads `BrickDestroyed`, updates `ScoreState`
6. **Frame N+0**: Milestone detection checks `ScoreState` (if changed)
7. **Frame N+0**: UI update system re-renders score display (if changed)

### MilestoneReached Event Flow

1. **Frame N**: `ScoreState.current_score` crosses 5000 threshold (e.g., 4980 → 5005)
2. **Frame N**: `detect_milestone_system` emits `MilestoneReached { tier: 1, score: 5005 }`
3. **Frame N**: `ScoreState.last_milestone_reached` updated to 1
4. **Frame N+0**: Ball award system reads `MilestoneReached`, increments `LivesState.lives_remaining`
5. **Frame N+0**: Lives counter UI updates to show new ball/life count

### System Execution Order

```rust
// Simplified system set ordering
Update.add_systems((
    // Phase 1: Physics & Collision
    collision_detection,  // Rapier emits CollisionEvents

    // Phase 2: Game Logic
    brick_destruction.after(collision_detection),  // Emits BrickDestroyed
    award_points.after(brick_destruction),         // Updates ScoreState
    detect_milestone.after(award_points),          // Emits MilestoneReached

    // Phase 3: Award Bonuses
    award_milestone_ball.after(detect_milestone),  // Increments LivesState

    // Phase 4: UI Updates (change detection)
    update_score_display.after(award_points),      // Renders score
    update_lives_display.after(award_milestone_ball),  // Renders lives/balls
));
```

---

## Error Handling

### Invalid Brick Type

**Scenario**: `BrickDestroyed` event with unrecognized `brick_type`.

**Handling**:

- Log warning with brick entity and type
- Award 0 points (safe fallback)
- Continue processing (don't panic)

```rust
let points = match brick_points(event.brick_type, &mut rng) {
    Ok(points) => points,
    Err(e) => {
        warn!("Invalid brick type: {:?} (entity: {:?})",
              event.brick_type, event.brick_entity);
        0  // Safe fallback
    }
};
```

### Milestone Overflow

**Scenario**: Score reaches u32::MAX, milestone calculation overflows.

**Handling**:

- Clamp score at u32::MAX (score stops increasing)
- No new milestones triggered past maximum
- Log warning if this occurs (unlikely in normal gameplay)

---

## Testing Contracts

### BrickDestroyed Contract Tests

```rust
#[test]
fn brick_destroyed_event_awards_correct_points() {
    let mut app = test_app();
    let mut events = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();

    events.write(BrickDestroyed {
        brick_entity: Entity::PLACEHOLDER,
        brick_type: BrickType::SimpleStone,  // 25 points
        destroyed_by: None,
    });

    app.update();

    let score = app.world().resource::<ScoreState>();
    assert_eq!(score.current_score, 25);
}
```

### MilestoneReached Contract Tests

```rust
#[test]
fn milestone_reached_awards_extra_ball() {
    let mut app = test_app();

    // Set score to 4999
    app.world_mut().resource_mut::<ScoreState>().current_score = 4999;
    let initial_lives = app.world().resource::<LivesState>().lives_remaining;

    // Award 25 points (crosses 5000 threshold)
    let mut events = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
    events.write(BrickDestroyed {
        brick_entity: Entity::PLACEHOLDER,
        brick_type: BrickType::SimpleStone,  // 25 points → 5024 total
        destroyed_by: None,
    });

    app.update();

    // Verify milestone event was emitted
    let milestone_events = app.world().resource::<Messages<MilestoneReached>>();
    assert_eq!(milestone_events.len(), 1);

    let event = milestone_events.iter().next().unwrap();
    assert_eq!(event.milestone_tier, 1);
    assert_eq!(event.total_score, 5024);

    // Verify lives were incremented (ball awarded)
    let lives = app.world().resource::<LivesState>();
    assert_eq!(lives.lives_remaining, initial_lives + 1, "Milestone should award extra ball/life");
}
```

---

## Version History

**v1.0** (2025-12-16): Initial event contracts for scoring system MVP.

---

## Future Considerations

### Potential New Events (Out of Scope for MVP)

- `ScoreMultiplierChanged`: When multiplier bricks (26-29) are implemented
- `ComboScoreAwarded`: For rapid successive brick destruction bonuses
- `HighScoreAchieved`: For persistent high score tracking (requires storage)
