# System Contracts: Paddle Size Powerups

**Phase**: 1 - Design & Contracts **Date**: 2025-12-12

This document defines the interface contracts between systems implementing paddle size powerups and the rest of the game.

## Event Contracts

### Input Events (From Physics Engine)

**Event**: `CollisionEvent::Started` (from Bevy Rapier3D)

```rust
pub enum CollisionEvent {
    Started(Entity, Entity, CollisionEventFlags),
    Stopped(Entity, Entity, CollisionEventFlags),
}
```

**Contract Usage**:

- Paddle size system listens to `Started` events
- Filters pairs by: one entity has `Paddle` component, other has `BrickType30` or `BrickType32`
- Triggers size effect creation on match

**Expected Frequency**: ~0-5 collisions per frame (typical gameplay)

---

### Output Events (From Paddle Size System)

**Event**: `PaddleSizeEffectApplied` (new, internal)

```rust
#[derive(Event)]
pub struct PaddleSizeEffectApplied {
    pub paddle_entity: Entity,
    pub effect_type: SizeEffectType,
    pub new_width: f32,
}
```

**Purpose**: Notify other systems (audio, VFX, UI) of size changes

**Subscribers**:

- `paddle_size_audio`: Play sound effect
- `paddle_size_visual`: Update material colors
- (Optional) UI system to display effect status

---

### Lifecycle Events (External)

**Event**: `LevelChangeEvent` (existing)

**Contract**:

- On any `LevelChangeEvent`, paddle size system removes `PaddleSizeEffect` component
- Paddle returns to normal width (20 units)
- No visual artifacts or state leakage to next level

**Event**: `PlayerLossEvent` (existing)

**Contract**:

- On any player loss (life count decrements), paddle size system removes `PaddleSizeEffect` component
- Paddle returns to normal width for next attempt
- No carryover of effect to respawned paddle

---

## Component Contracts

### Input Components

**Paddle** (existing entity)

Required siblings:

- `Transform`: Position, scale
- `Collider`: Physics collider for collision detection
- `Material`: Rendering properties (color, emission)

The paddle size system will:

- Query for `Paddle` component
- Insert/remove `PaddleSizeEffect` component
- Mutate `Material` for visual feedback

---

**BrickType30** (marker component on brick entity)

```rust
#[derive(Component)]
pub struct BrickType30;
```

Required siblings:

- `Transform`
- `Collider`
- `Handle<Mesh>`, `Handle<StandardMaterial>`, or equivalent

Invariant: When this component exists, collision detection includes this brick in size effect logic.

---

**BrickType32** (marker component on brick entity)

```rust
#[derive(Component)]
pub struct BrickType32;
```

Same siblings and invariants as `BrickType30`.

---

### Output Components

**PaddleSizeEffect** (created by paddle size system)

```rust
#[derive(Component, Clone)]
pub struct PaddleSizeEffect {
    pub effect_type: SizeEffectType,
    pub remaining_duration: f32,
    pub base_width: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SizeEffectType {
    Shrink,
    Enlarge,
}
```

Lifetime:

- Created: On collision between ball and brick 30/32
- Updated: Every frame (duration countdown)
- Deleted: When duration ≤ 0, or external event triggers (level change, loss)

Invariants:

- `remaining_duration ≥ 0.0` always
- `base_width` always equals paddle's base width (20 units)
- Only one per paddle entity

---

## System Scheduling Contract

The paddle size feature consists of four systems operating in order:

```text
Collision Detection
    ↓ (CollisionEvent::Started)
Paddle Size Effect Creation
    ↓ (PaddleSizeEffect inserted)
Paddle Size Timer Updates
    ↓ (duration countdown)
Paddle Size Visual Feedback
    ↓ (Material color updates)
```

**Scheduling Constraints**:

- All systems must run each frame
- Collision detection must complete before size effect creation
- Timer updates must complete before visual feedback
- No parallel conflicts with existing Rapier, Transform, or Material systems

---

## Query Contracts

### paddle_size system

```rust
Query<&Paddle, With<PaddleSizeEffect>>
```

Returns: All paddle entities with an active size effect

---

### paddle_size_effects system

```rust
Query<&mut PaddleSizeEffect>
```

Mutates: Decrement `remaining_duration` each frame

---

### paddle_size_visual system

```rust
Query<(&PaddleSizeEffect, &mut Handle<StandardMaterial>), With<Paddle>>
```

Mutates: Update material color and emission based on effect type

---

### paddle_size_audio system

Listens to: `PaddleSizeEffectApplied` events

Behavior: Load and play corresponding audio asset on event receipt

---

## Asset Contracts

### Audio Files

Required files in `assets/audio/`:

- `paddle_shrink.ogg` (or `.mp3`, `.wav`) — Triggered on brick 30 hit
- `paddle_enlarge.ogg` (or `.mp3`, `.wav`) — Triggered on brick 32 hit

Audio manifest (`assets/audio/manifest.ron`):

```ron
(
    sounds: {
        "paddle_shrink": Sound(path: "paddle_shrink.ogg"),
        "paddle_enlarge": Sound(path: "paddle_enlarge.ogg"),
    }
)
```

---

## Performance Contracts

| Operation | Target | Rationale |
|-----------|--------|-----------|
| Collision detection | < 1ms per frame | Rapier3D optimized |
| Effect creation | < 0.1ms | Single component insert |
| Timer update | < 0.5ms | 1-2 float subtractions |
| Visual update | < 0.5ms | Material mutation (cached) |
| Audio playback | < 1ms | Asset already loaded |
| **Total per frame** | **< 3ms** | Keep under 16.7ms budget (60 FPS) |

---

## Error Handling Contracts

| Error Condition | Handling |
|-----------------|----------|
| Paddle entity missing `Material` | Skip visual feedback, log warning |
| Audio asset not found | Log error, skip audio (game playable without sound) |
| `remaining_duration` overflows | Clamp to reasonable max (~1000s), log warning |
| Collision filter mismatch | Silently skip (no effect triggered) |

**Principle**: Graceful degradation.
Missing optional resources (audio, visual) don't break core gameplay.

---

## Testing Contracts

Each system must satisfy these test categories:

### Unit Tests

- Size calculation (multiplier, clamping)
- Timer countdown
- Effect type matching

### Integration Tests

- Collision trigger → component creation
- Timer expiry → component removal
- Effect replacement (shrink → enlarge)
- Level change → effect removal
- Visual feedback accuracy (colors match spec)
- Audio playback on trigger

### Acceptance Tests (From Spec)

- Paddle shrinks to 14u on brick 30
- Paddle enlarges to 30u on brick 32
- Timer is 10 seconds
- Effects clear on level change
- Visual and audio feedback present
