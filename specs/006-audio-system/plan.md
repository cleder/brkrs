# Implementation Plan: Audio System

**Branch**: `006-audio-system` | **Date**: 2025-11-29 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/006-audio-system/spec.md` **Related Issues**: [#10](https://github.com/cleder/brkrs/issues/10), [#23](https://github.com/cleder/brkrs/issues/23)

## Summary

Implement an event-driven audio system for the brick-breaker game that provides audio feedback for brick collisions (with distinct multi-hit impact sound for indices 10-13), wall/paddle bounces, and level transitions.
The system uses Bevy's built-in audio with observer pattern for event-to-sound mapping, graceful degradation when assets are missing, and configurable volume/mute settings.

## Technical Context

**Language/Version**: Rust 1.81 (Rust 2021 edition) **Primary Dependencies**: Bevy 0.17 (AudioPlugin, AudioSource, observers), bevy_rapier3d **Storage**: RON configuration file for audio settings persistence **Testing**: cargo test (unit tests for audio config, integration tests for event firing) **Target Platform**: Native (Linux/Windows/macOS) + WASM **Project Type**: Single Bevy game project **Performance Goals**: 60 FPS, audio playback within 50ms of collision event **Constraints**: Max 4 concurrent sounds of same type, graceful no-op on missing assets **Scale/Scope**: ~8 distinct sound events, single audio configuration resource

## Constitution Check

*GATE: Must pass before Phase 0 research.*
          *Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. ECS-First | ✅ PASS | Audio events as Bevy Events, config as Resource, playback via observer systems |
| II. Physics-Driven | ✅ PASS | Audio triggered by existing collision events (MultiHitBrickHit, WallHit, BrickHit) |
| III. Modular Design | ✅ PASS | AudioPlugin as independent module, event-driven, no tight coupling |
| IV. Performance-First | ✅ PASS | Concurrent sound limiting (max 4), 50ms latency target |
| V. Cross-Platform | ✅ PASS | Web audio context handling exists in wasm/restart-audio-context.js |
| VI. Rustdoc | ✅ PASS | All public API will have rustdoc with purpose-focused documentation |

## Project Structure

### Documentation (this feature)

```text
specs/006-audio-system/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (event contracts)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### WASM Persistence

The audio configuration uses `config/audio.ron` on native platforms and persists to `localStorage` on WASM builds for parity.
The WASM persistence stores the RON-serialized `AudioConfig` under the key `brkrs_audio` in the browser's `localStorage`.
This was implemented as task `T038` and is gate-compiled under `target_arch = "wasm32"` using `web-sys` for storage access.
Native file-based behavior remains unchanged.

### Source Code (repository root)

```text
src/
├── systems/
│   ├── mod.rs           # Export AudioPlugin
│   ├── audio.rs         # NEW: AudioPlugin, AudioConfig, event observers
│   └── multi_hit.rs     # Update: integrate with audio observer
├── lib.rs               # Register AudioPlugin
└── ...

assets/
├── audio/               # NEW: Sound assets directory
│   ├── manifest.ron     # Sound-to-file mapping
│   ├── brick_destroy.ogg
│   ├── multi_hit_impact.ogg
│   ├── wall_bounce.ogg
│   ├── paddle_hit.ogg
│   ├── paddle_wall_hit.ogg
│   ├── paddle_brick_hit.ogg
│   ├── level_start.ogg
│   └── level_complete.ogg
└── ...

tests/
├── audio_config.rs      # NEW: Unit tests for AudioConfig
└── audio_events.rs      # NEW: Integration tests for event firing
```

**Structure Decision**: Follows existing single-project Bevy pattern.
Audio system added as new module under `src/systems/audio.rs` with corresponding plugin, matching the pattern used by `TextureManifestPlugin` and `RespawnPlugin`.
