# Event Contracts: Level Navigation Bricks (Bricks 50 & 54)

**Date**: 2026-01-31 **Feature**: Level Navigation Bricks **Format**: Rust Message Definitions (Bevy Messages)

## Overview

This document defines the message contracts (events) used by level navigation bricks to communicate level transitions, audio triggers, and scoring integration with other game systems.

---

## Messages

### LevelSwitchRequested

**Purpose**: Signals that a level transition should occur (next or previous level).

**Trigger Condition**: Ball collision destroys brick 50 (Level Up) or brick 54 (Level Down).

**Message Definition**:

```rust
use bevy::prelude::*;
use bevy::ecs::message::Message;

#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelSwitchRequested {
    /// The source that requested the level switch
    pub source: LevelSwitchSource,
    /// The direction of the level transition
    pub direction: LevelSwitchDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelSwitchSource {
    /// Keyboard input (e.g., 'N' for next, 'P' for previous)
    Keyboard,
    /// Navigation brick destroyed (brick 50 or 54)
    Brick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelSwitchDirection {
    /// Advance to the next level (brick 50 behavior)
    Next,
    /// Return to the previous level (brick 54 behavior)
    Previous,
}
```

**Emitters**:

- **Brick 50 (Level Up)**: `mark_brick_on_ball_collision` system emits `LevelSwitchRequested { source: Brick, direction: Next }` when brick 50 is hit
- **Brick 54 (Level Down)**: `mark_brick_on_ball_collision` system emits `LevelSwitchRequested { source: Brick, direction: Previous }` when brick 54 is hit
- Keyboard input system (existing, not modified by this feature)

**Consumers**:

- `process_level_switch_requests` system in `src/level_loader.rs` - handles level transitions and boundary conditions

**Usage Pattern**:

```rust
// In brick collision system (src/lib.rs:mark_brick_on_ball_collision)
fn handle_navigation_brick_collision(
    brick_type: u8,
    mut level_switch_writer: MessageWriter<LevelSwitchRequested>,
) {
    match brick_type {
        BRICK_50 => {
            // Level Up brick: advance to next level
            level_switch_writer.write(LevelSwitchRequested {
                source: LevelSwitchSource::Brick,
                direction: LevelSwitchDirection::Next,
            });
        }
        BRICK_54 => {
            // Level Down brick: return to previous level
            level_switch_writer.write(LevelSwitchRequested {
                source: LevelSwitchSource::Brick,
                direction: LevelSwitchDirection::Previous,
            });
        }
        _ => {}
    }
}

// In level switch consumer (src/level_loader.rs:process_level_switch_requests)
fn process_level_switch_requests(
    mut requests: MessageReader<LevelSwitchRequested>,
    mut switch_state: ResMut<LevelSwitchState>,
    current_level: Option<Res<CurrentLevel>>,
    // ... other parameters
) {
    for request in requests.read() {
        let current_number = current_level.map(|c| c.0.number).unwrap_or(0);

        match request.direction {
            LevelSwitchDirection::Next => {
                if let Some(next_level) = switch_state.next_level_after(current_number) {
                    // Load next level
                    force_load_level_from_path(&next_level.path, /* ... */);
                } else {
                    // On final level: show victory screen
                    if request.source == LevelSwitchSource::Brick {
                        spawn_victory_screen(&mut commands);
                        game_progress.finished = true;
                    }
                }
            }
            LevelSwitchDirection::Previous => {
                if let Some(prev_level) = switch_state.previous_level_before(current_number) {
                    // Load previous level
                    force_load_level_from_path(&prev_level.path, /* ... */);
                }
                // If on level 1: no-op (already handled by None check)
            }
        }
    }
}
```

**Boundary Behavior**:

- **Brick 50 on final level**: `next_level_after()` returns `None` → spawn victory screen, set `GameProgress.finished = true`
- **Brick 54 on level 1**: `previous_level_before()` returns `None` → no transition occurs (brick still destroyed and removed)

---

### BrickDestroyed

**Purpose**: Signals that a brick was destroyed and audio/scoring systems should respond.

**Trigger Condition**: Ball collision causes brick to be marked for despawn.

**Message Definition**:

```rust
use bevy::prelude::*;
use bevy::ecs::message::Message;

#[derive(Message, Debug, Clone, Copy)]
pub struct BrickDestroyed {
    /// The brick entity that was destroyed
    pub brick_entity: Entity,
    /// The type/index of the brick (determines point value and audio)
    pub brick_type: u8,
    /// The ball that destroyed the brick (optional, for future analytics)
    pub destroyed_by: Option<Entity>,
}
```

**Emitters**:

- `despawn_marked_entities` system in `src/lib.rs` - emits before despawning bricks marked with `MarkedForDespawn`

**Consumers**:

- `award_points_system` in `src/systems/scoring.rs` - awards 0 points for brick types 50 and 54
- Audio system (location TBD) - maps brick type 50 → `SoundType::Brick50LevelUp`, brick type 54 → `SoundType::Brick54LevelDown`

**Usage Pattern**:

```rust
// In despawn system (existing, not modified)
fn despawn_marked_entities(
    marked: Query<(Entity, Option<&BrickTypeId>), With<MarkedForDespawn>>,
    mut brick_events: Option<MessageWriter<BrickDestroyed>>,
    mut commands: Commands,
) {
    for (entity, brick_type) in marked.iter() {
        if let Some(brick_type) = brick_type {
            if let Some(writer) = brick_events.as_mut() {
                writer.write(BrickDestroyed {
                    brick_entity: entity,
                    brick_type: brick_type.0,
                    destroyed_by: None,
                });
            }
        }
        commands.entity(entity).despawn();
    }
}

// In scoring system (src/systems/scoring.rs:award_points_system)
fn award_points(
    mut brick_destroyed_events: MessageReader<BrickDestroyed>,
    mut score_state: ResMut<ScoreState>,
) {
    for event in brick_destroyed_events.read() {
        let points = brick_points(event.brick_type);
        // brick_points(50) => 0, brick_points(54) => 0
        score_state.current_score = score_state.current_score.saturating_add(points);
    }
}

// In audio system (location TBD)
fn play_brick_destruction_sound(
    mut brick_destroyed_events: MessageReader<BrickDestroyed>,
    // ... audio resources
) {
    for event in brick_destroyed_events.read() {
        let sound = match event.brick_type {
            41 => SoundType::Brick41ExtraLife,
            50 => SoundType::Brick50LevelUp,
            54 => SoundType::Brick54LevelDown,
            _ => SoundType::BrickDestroy,
        };
        // Play sound (implementation details omitted)
    }
}
```

**Scoring Integration**:

- Brick type 50 → 0 points (utility brick)
- Brick type 54 → 0 points (utility brick)

**Audio Integration**:

- Brick type 50 → `SoundType::Brick50LevelUp` (unique sound)
- Brick type 54 → `SoundType::Brick54LevelDown` (unique sound)
- Fallback to `SoundType::BrickDestroy` if assets missing

---

## Message Flow Diagram

```text
Ball Collision with Brick 50 or 54
    ↓
mark_brick_on_ball_collision() system
    ├─ Insert MarkedForDespawn component
    └─ Write LevelSwitchRequested message
        └─ { source: Brick, direction: Next (50) or Previous (54) }
    ↓
despawn_marked_entities() system (next frame)
    ├─ Write BrickDestroyed message
    │   └─ { brick_entity, brick_type: 50 or 54, destroyed_by }
    └─ Despawn brick entity
    ↓
process_level_switch_requests() system
    ├─ Read LevelSwitchRequested messages
    ├─ Query LevelSwitchState for target level
    │   ├─ direction: Next → next_level_after(current)
    │   └─ direction: Previous → previous_level_before(current)
    ├─ If target exists → force_load_level_from_path()
    └─ If target None and source Brick and direction Next
        └─ Spawn victory screen, set GameProgress.finished = true
    ↓
award_points_system() and audio system
    ├─ Read BrickDestroyed messages
    ├─ Award 0 points (brick types 50 and 54)
    └─ Play unique destruction sound (Brick50LevelUp or Brick54LevelDown)
```

---

## Event Ordering Constraints

### System Execution Order

1. **Collision Detection** (`mark_brick_on_ball_collision`)
   - Runs in `Update` schedule
   - Writes `LevelSwitchRequested` and inserts `MarkedForDespawn`

2. **Entity Despawn** (`despawn_marked_entities`)
   - Runs in `Update` schedule (same frame or next frame)
   - Writes `BrickDestroyed` before despawning
   - Must run after collision detection to process `MarkedForDespawn`

3. **Level Transition** (`process_level_switch_requests`)
   - Runs in `Update` schedule
   - Reads `LevelSwitchRequested` messages
   - Must run after despawn to ensure messages are emitted

4. **Audio/Scoring** (`award_points_system`, audio system)
   - Runs in `Update` schedule
   - Reads `BrickDestroyed` messages
   - Can run in parallel (no ordering dependency between them)

**Ordering Guarantee**:

- `despawn_marked_entities` must run **after** `mark_brick_on_ball_collision`
- `process_level_switch_requests` has no strict ordering requirement (messages buffered across frames)

---

## Message Persistence

### Multi-Frame Buffering

- **LevelSwitchRequested**: Buffered via `MessageWriter`/`MessageReader` (Messages pattern)
  - Written in frame N
  - Read in frame N or later (buffered, not immediate)
  - Cleared after `process_level_switch_requests` processes them

- **BrickDestroyed**: Buffered via `MessageWriter`/`MessageReader` (Messages pattern)
  - Written in frame N
  - Read in frame N or later by audio/scoring systems
  - Multiple systems can read the same message (parallel consumption)

### State Persistence

- **CurrentLevel**: Resource updated during level transition
  - Must persist across minimum 10 frames (multi-frame persistence requirement)
  - Tests verify no unconditional overwrites in `Update` schedule

- **GameProgress.finished**: Set to `true` when brick 50 hit on final level
  - Persists until game restart or new game initiated

---

## Fallback & Error Handling

### Missing Level Files

**Scenario**: `next_level_after()` or `previous_level_before()` returns `None` due to missing level files

**Handling**:

- Log warning: "No level entries available for switching"
- Clear message queue
- No transition occurs
- Game continues in current level

**Special Case (Brick 50 on Final Level)**:

- If `next_level_after()` returns `None` and `source == Brick`:
  - Treat as victory condition
  - Spawn victory screen
  - Set `GameProgress.finished = true`

### Missing Audio Assets

**Scenario**: Dedicated sound files for brick 50/54 not loaded

**Handling** (per FR-006):

- Fallback to `SoundType::BrickDestroy` (generic brick sound)
- Gameplay proceeds normally
- No blocking or panic

### Concurrent Transitions

**Scenario**: Multiple `LevelSwitchRequested` messages emitted in same frame

**Handling**:

- `LevelSwitchState.pending_transition` flag prevents concurrent transitions
- First message processed, subsequent messages ignored until transition completes
- Log info: "Level transition already active; ignoring switch request"

---

## Testing Contracts

### Test Assertions

**For LevelSwitchRequested**:

1. Brick 50 collision → message emitted with `direction: Next`
2. Brick 54 collision → message emitted with `direction: Previous`
3. Message consumed by `process_level_switch_requests` within 1 frame
4. Boundary conditions:
   - Brick 50 on final level → victory screen spawned, no transition
   - Brick 54 on level 1 → no transition, no error

**For BrickDestroyed**:

1. Brick 50 despawn → message emitted with `brick_type: 50`
2. Brick 54 despawn → message emitted with `brick_type: 54`
3. Scoring system awards 0 points
4. Audio system plays unique sound (or fallback)

**Multi-Frame Persistence**:

1. After level transition, verify `CurrentLevel.number` persists across 10+ frames
2. No systems unconditionally overwrite `CurrentLevel` in `Update` schedule

---

## API Compatibility

### Breaking Changes

**None**.
All changes are additive:

- New `LevelSwitchSource::Brick` variant (extends existing enum)
- Existing `LevelSwitchRequested` and `BrickDestroyed` messages unchanged
- No modifications to existing message consumers

### Backward Compatibility

- Existing keyboard-based level switching continues to work
- Existing brick destruction audio/scoring continues to work
- New brick types (50, 54) integrate seamlessly with existing systems

---

## References

- **Message System**: Bevy 0.17 Message-Event Separation (`.specify/memory/constitution.md`)
- **Existing Patterns**:
  - Brick 41 `BrickDestroyed` handling: `specs/019-extra-ball-brick/contracts/events.md`
  - Level switching: `src/systems/level_switch.rs`
- **Multi-Frame Persistence**: `specs/020-gravity-bricks/retrospective.md` (constitutional amendment)
