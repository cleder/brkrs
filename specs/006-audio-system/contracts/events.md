# Event Contracts: Audio System

**Feature Branch**: `006-audio-system` **Date**: 2025-11-29

## Overview

This document defines the event contracts for the audio system.
The audio system observes existing game events and introduces two new events.

---

## Existing Events (Observed)

### MultiHitBrickHit

**Source**: `src/systems/multi_hit.rs` **Trigger**: Ball collides with multi-hit brick (index 10-13)

```rust
#[derive(Event, Debug, Clone)]
pub struct MultiHitBrickHit {
    pub entity: Entity,
    pub previous_type: u8,  // 10-13
    pub new_type: u8,       // 10-12 or 20
}
```

**Audio Response**: Play `MultiHitImpact` sound

---

### WallHit

**Source**: `src/lib.rs` **Trigger**: Paddle collides with border wall

```rust
#[derive(Event)]
struct WallHit {
    pub impulse: Vec3,
}
```

**Audio Response**: Play `PaddleWallHit` sound

---

### BrickHit

**Source**: `src/lib.rs` **Trigger**: Paddle collides with brick

```rust
#[derive(Event)]
struct BrickHit {
    pub impulse: Vec3,
}
```

**Audio Response**: Play `PaddleBrickHit` sound

---

### BallHit

**Source**: `src/lib.rs` **Trigger**: Paddle collides with ball

```rust
#[derive(Event)]
struct BallHit {
    pub impulse: Vec3,
    pub ball: Entity,
}
```

**Audio Response**: Play `PaddleHit` sound

---

### LevelSwitchRequested

**Source**: `src/systems/level_switch.rs` **Trigger**: Level transition requested (keyboard or automation)

```rust
#[derive(Event)]
pub struct LevelSwitchRequested {
    pub source: LevelSwitchSource,
}
```

**Audio Response**: Play `LevelComplete` sound when source is level completion

---

## New Events

### BrickDestroyed

**Source**: `src/lib.rs` (new) **Trigger**: Any destructible brick is despawned

```rust
/// Emitted when a destructible brick is removed from the game.
/// Used by audio system to play brick destruction sound.
#[derive(Event, Debug, Clone)]
pub struct BrickDestroyed {
    /// The entity that was destroyed (for potential future use)
    pub entity: Entity,
    /// The brick type that was destroyed
    pub brick_type: u8,
}
```

**Audio Response**: Play `BrickDestroy` sound (unless brick_type was 10-13, which uses MultiHitImpact)

**Emit Location**: `despawn_marked_entities` system in `lib.rs`

---

### LevelStarted

**Source**: `src/level_loader.rs` (new) **Trigger**: Level has finished loading and gameplay can begin

```rust
/// Emitted when a level has finished loading and is ready for play.
/// Used by audio system to play level start sound.
#[derive(Event, Debug, Clone)]
pub struct LevelStarted {
    /// Index of the level that started
    pub level_index: u32,
}
```

**Audio Response**: Play `LevelStart` sound

**Emit Location**: `spawn_level_entities` or equivalent in `level_loader.rs`

---

### BallWallHit

**Source**: `src/lib.rs` (new) **Trigger**: Ball bounces off a wall boundary

```rust
/// Emitted when the ball bounces off a wall boundary.
/// Used by audio system to play wall bounce sound.
#[derive(Event, Debug, Clone)]
pub struct BallWallHit {
    /// The ball entity that hit the wall
    pub entity: Entity,
    /// The collision impulse
    pub impulse: Vec3,
}
```

**Audio Response**: Play `WallBounce` sound

**Emit Location**: Collision detection system in `lib.rs` (needs to be added for ball-border collisions)

---

## Audio Manifest Contract

**Location**: `assets/audio/manifest.ron`

```ron
AudioManifest(
    sounds: {
        BrickDestroy: "brick_destroy.ogg",
        MultiHitImpact: "multi_hit_impact.ogg",
        WallBounce: "wall_bounce.ogg",
        PaddleHit: "paddle_hit.ogg",
        PaddleWallHit: "paddle_wall_hit.ogg",
        PaddleBrickHit: "paddle_brick_hit.ogg",
        LevelStart: "level_start.ogg",
        LevelComplete: "level_complete.ogg",
    }
)
```

**Validation**:

- All files referenced must exist in `assets/audio/`
- Missing files result in warning log, not error
- Format: OGG Vorbis

---

## AudioConfig Persistence Contract

**Location**: `config/audio.ron` (native) or `localStorage:brkrs_audio` (WASM)

```ron
AudioConfig(
    master_volume: 1.0,
    muted: false,
)
```

**Validation**:

- `master_volume`: f32 in range [0.0, 1.0], clamped if out of range
- `muted`: bool
- Missing file: use defaults (volume: 1.0, muted: false)
- Parse error: log warning, use defaults
