# Data Model: Audio System

**Feature Branch**: `006-audio-system`
**Date**: 2025-11-29

## Entities

### SoundType (Enum)

Identifies the category of sound effect for mapping and concurrent tracking.

| Variant | Description | Triggered By |
|---------|-------------|--------------|
| `BrickDestroy` | Standard brick destruction | Ball destroys any brick (index 20+) |
| `MultiHitImpact` | Multi-hit brick damage | Ball hits multi-hit brick (index 10-13) |
| `WallBounce` | Ball bounces off wall | Ball collides with border |
| `PaddleHit` | Ball bounces off paddle | Paddle-ball collision |
| `PaddleWallHit` | Paddle bumps into wall | Paddle collides with border |
| `PaddleBrickHit` | Paddle bumps into brick | Paddle collides with brick |
| `LevelStart` | Level begins | Level loaded and gameplay starts |
| `LevelComplete` | Level finished | All destructible bricks destroyed |

### AudioConfig (Resource)

User-adjustable audio settings, persisted across sessions.

| Field | Type | Default | Validation | Description |
|-------|------|---------|------------|-------------|
| `master_volume` | `f32` | `1.0` | 0.0 - 1.0 | Global volume multiplier |
| `muted` | `bool` | `false` | - | Whether audio is muted |

**Persistence**: Serialized to `config/audio.ron` (native) or `localStorage` (WASM)

### AudioAssets (Resource)

Loaded sound asset handles, keyed by SoundType.

| Field | Type | Description |
|-------|------|-------------|
| `sounds` | `HashMap<SoundType, Handle<AudioSource>>` | Loaded audio asset handles |

**Loading**: Assets loaded during startup from `assets/audio/` directory.

### ActiveSounds (Resource)

Tracks concurrent playback count per sound type.

| Field | Type | Description |
|-------|------|-------------|
| `counts` | `HashMap<SoundType, u8>` | Active instances per sound type |

**Constraint**: Maximum 4 concurrent sounds per type (excess dropped).

---

## Events

### Existing Events (to observe)

| Event | Source | Audio Response |
|-------|--------|----------------|
| `MultiHitBrickHit` | `systems/multi_hit.rs` | Play `MultiHitImpact` |
| `WallHit` | `lib.rs` | Play `PaddleWallHit` |
| `BrickHit` | `lib.rs` | Play `PaddleBrickHit` |
| `BallHit` | `lib.rs` | Play `PaddleHit` |
| `LevelSwitchRequested` | `systems/level_switch.rs` | Play `LevelComplete` (on success) |

### New Events

| Event | Fields | Purpose |
|-------|--------|---------|
| `BrickDestroyed` | `entity: Entity` | Emitted when any destructible brick is despawned |
| `LevelStarted` | `level_index: u32` | Emitted when level gameplay begins |
| `BallWallHit` | `entity: Entity, impulse: Vec3` | Emitted when ball bounces off wall boundary |

---

## State Transitions

### AudioConfig Lifecycle

```text
[Initial Load]
    │
    ▼
┌─────────────────┐
│ Load from file  │──(file missing)──▶ [Use defaults]
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Active Config   │◀──────────────────┐
└────────┬────────┘                   │
         │                            │
    (user adjusts)                (game exit)
         │                            │
         ▼                            │
┌─────────────────┐                   │
│ Modified Config │───────────────────┘
└─────────────────┘      (persist)
```

### Sound Playback Flow

```text
[Game Event] ──▶ [Audio Observer]
                      │
                      ▼
                ┌─────────────┐
                │ Check muted │──(yes)──▶ [No-op]
                └──────┬──────┘
                       │ (no)
                       ▼
                ┌─────────────────┐
                │ Check asset     │──(missing)──▶ [Log warning]
                │ loaded          │
                └──────┬──────────┘
                       │ (present)
                       ▼
                ┌─────────────────┐
                │ Check concurrent│──(≥4)──▶ [Drop sound]
                │ count           │
                └──────┬──────────┘
                       │ (<4)
                       ▼
                ┌─────────────────┐
                │ Spawn AudioPlayer│
                │ with volume      │
                └──────┬──────────┘
                       │
                       ▼
                ┌─────────────────┐
                │ Increment count │
                └─────────────────┘
```

---

## Relationships

```text
┌─────────────────┐         ┌─────────────────┐
│   AudioConfig   │◀────────│   AudioPlugin   │
│   (Resource)    │ reads   │   (Plugin)      │
└─────────────────┘         └────────┬────────┘
                                     │ registers
                                     ▼
┌─────────────────┐         ┌─────────────────┐
│   AudioAssets   │◀────────│ Audio Observers │
│   (Resource)    │ uses    │   (Systems)     │
└─────────────────┘         └────────┬────────┘
                                     │ observes
                                     ▼
┌─────────────────┐         ┌─────────────────┐
│  ActiveSounds   │◀────────│  Game Events    │
│   (Resource)    │ tracks  │ (Multi-hit, etc)│
└─────────────────┘         └─────────────────┘
```

---

## Validation Rules

| Rule | Entity | Constraint |
|------|--------|------------|
| VR-001 | `AudioConfig.master_volume` | Must be in range [0.0, 1.0] |
| VR-002 | `ActiveSounds.counts[*]` | Must not exceed 4 |
| VR-003 | `AudioAssets.sounds` | Missing entries are logged, not errors |
