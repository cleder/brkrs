# Research: Audio System

**Feature Branch**: `006-audio-system` **Date**: 2025-11-29

## Research Summary

All technical questions resolved.
Ready for Phase 1 design.

---

## 1. Bevy Audio API (0.17)

**Decision**: Use Bevy's built-in `AudioPlugin` with `AudioPlayer` and `PlaybackSettings`

**Rationale**:

- Bevy 0.17 provides native audio support via `bevy_audio` crate (included in `DefaultPlugins`)
- Uses `rodio` backend for native, `cpal` for cross-platform support
- WASM audio context handling already exists in `wasm/restart-audio-context.js`
- No external audio crate needed

**Alternatives Considered**:

- `bevy_kira_audio`: More features but adds dependency complexity
- `bevy_oddio`: Lower-level, more control but more boilerplate
- Custom audio: Unnecessary complexity for simple sound effects

**Key APIs**:

```rust
// Spawn audio with settings
commands.spawn((
    AudioPlayer::new(asset_server.load("audio/sound.ogg")),
    PlaybackSettings::DESPAWN,  // Auto-cleanup after playback
));

// Volume control via PlaybackSettings
PlaybackSettings {
    volume: Volume::Linear(0.5),
    ..default()
}
```

---

## 2. Event-to-Sound Mapping Pattern

**Decision**: Observer-based architecture using existing Bevy events

**Rationale**:

- Project already uses observers for game events (`on_wall_hit`, `on_brick_hit`, `on_multi_hit_brick_sound`)
- Follows Constitution Principle III (Modular Design) with event-driven communication
- Existing `MultiHitBrickHit` event infrastructure ready for audio integration
- Decouples audio from game logic

**Pattern**:

```rust
// Existing events to observe:
// - WallHit (already defined in lib.rs)
// - BrickHit (already defined in lib.rs, triggered on paddle-brick collision)
// - MultiHitBrickHit (systems/multi_hit.rs)
// - LevelSwitchRequested (systems/level_switch.rs, for level transitions)

// New observer:
fn on_brick_destroyed(trigger: On<BrickDestroyed>, audio: Res<AudioAssets>, ...) {
    // Play brick destruction sound
}
```

**Alternatives Considered**:

- Direct system calls: Tighter coupling, harder to test
- Message channels: Overkill for simple audio triggers
- Query-based polling: Less responsive, higher latency

---

## 3. Concurrent Sound Limiting

**Decision**: Track active instances per sound type, drop excess (max 4)

**Rationale**:

- Prevents audio clipping during rapid brick destruction chains
- Simple counter-based tracking per sound type
- Matches clarified requirement from spec (max 3-4 concurrent)

**Implementation Approach**:

```rust
#[derive(Resource, Default)]
struct ActiveSounds {
    counts: HashMap<SoundType, u8>,
}

const MAX_CONCURRENT_PER_TYPE: u8 = 4;
```

**Alternatives Considered**:

- Priority queue with interruption: More complex, not needed for short SFX
- Global limit across all types: Could starve important sounds
- No limit: Risk of audio distortion

---

## 4. Graceful Degradation

**Decision**: Optional asset loading with warning logs on missing files

**Rationale**:

- Allows headless testing without audio assets
- Follows existing texture fallback pattern in project
- Matches FR-008 requirement

**Implementation Approach**:

```rust
// Check if audio assets loaded before playing
if let Some(handle) = audio_assets.get(SoundType::BrickDestroy) {
    commands.spawn((AudioPlayer::new(handle.clone()), ...));
} else {
    warn!("Audio asset missing for {:?}", SoundType::BrickDestroy);
}
```

---

## 5. Audio Configuration Persistence

**Decision**: RON file at `config/audio.ron` with serde serialization

**Rationale**:

- Project already uses RON format for level files and texture manifests
- Simple, human-readable format
- Native Rust serialization support

**Schema**:

```rust
#[derive(Serialize, Deserialize, Resource)]
struct AudioConfig {
    master_volume: f32,  // 0.0-1.0
    muted: bool,
}
```

**Storage Location**: `config/audio.ron` (native) or `localStorage` (WASM)

---

## 6. Web Audio Context

**Decision**: Leverage existing `wasm/restart-audio-context.js` solution

**Rationale**:

- Already implemented and tested in project
- Handles browser autoplay restrictions
- No additional work needed

**Verification**: The existing script intercepts `AudioContext` creation and resumes suspended contexts on user interaction.

---

## 7. Sound Asset Format

**Decision**: OGG Vorbis format for all sound effects

**Rationale**:

- Compressed, smaller file sizes for web delivery
- Wide browser support
- Bevy's audio plugin supports OGG natively

**Alternatives Considered**:

- WAV: Larger files, unnecessary for short SFX
- MP3: Patent concerns, less quality at similar file size
- FLAC: Overkill for sound effects, larger files

---

## Resolved Questions

| Question | Resolution |
|----------|------------|
| Which Bevy audio API? | Built-in `AudioPlayer` + `PlaybackSettings` |
| How to map events to sounds? | Observer pattern on existing events |
| How to limit concurrent sounds? | Counter per sound type, drop excess at max 4 |
| How to handle missing assets? | Optional loading with warning log |
| How to persist settings? | RON file (native) / localStorage (WASM) |
| Web audio restrictions? | Existing restart-audio-context.js handles it |
| Audio file format? | OGG Vorbis |
