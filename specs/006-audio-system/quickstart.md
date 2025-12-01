# Quickstart: Audio System

**Feature Branch**: `006-audio-system`
**Date**: 2025-11-29

## Prerequisites

- Rust 1.81+ with `rustup`
- Bevy 0.17 (already in project)
- Audio assets in OGG format (placeholder or real)

## Quick Setup

### 1. Create Audio Assets Directory

```bash
mkdir -p assets/audio
```

### 2. Add Placeholder Audio Files

For development/testing without real audio, create empty OGG files or use any short sound effects:

```bash
# If you have ffmpeg, create silent placeholders:
for sound in brick_destroy multi_hit_impact wall_bounce paddle_hit paddle_wall_hit paddle_brick_hit level_start level_complete; do
  ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 0.1 -q:a 9 assets/audio/${sound}.ogg 2>/dev/null
done
```

Or download free sound effects from sources like:

- [Freesound.org](https://freesound.org)
- [OpenGameArt.org](https://opengameart.org)

### 3. Create Audio Manifest

Create `assets/audio/manifest.ron`:

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

### 4. Run the Game

```bash
cargo run
```

Audio should play on:

- Brick destruction
- Multi-hit brick impacts
- Wall bounces
- Paddle hits
- Level start/complete

## Testing Without Audio

The audio system gracefully degrades. To test without assets:

```bash
# Remove audio directory
rm -rf assets/audio

# Run game - should work without crashes
cargo run
# Warnings will be logged for missing assets
```

## Configuration

Audio settings persist in `config/audio.ron`:

```ron
AudioConfig(
    master_volume: 1.0,  // 0.0 to 1.0
    muted: false,
)
```

To test muted mode, edit the file or use in-game controls (when implemented).

### WASM Persistence Note

On WASM builds the audio configuration is persisted to the browser's
`localStorage` under the key `brkrs_audio`. The value is the RON-serialized
`AudioConfig` (the same structure used by the native `config/audio.ron`). To
reset the audio configuration in a browser session, run the following in the
DevTools console:

```js
localStorage.removeItem('brkrs_audio');
```

This ensures parity between native and WASM persistence and makes it easy to
inspect or reset settings during development.

## Verification Checklist

- [ ] Game starts without audio-related errors
- [ ] Brick destruction produces sound
- [ ] Multi-hit bricks play distinct impact sound
- [ ] Wall bounces are audible (ball)
- [ ] Paddle hits are audible (ball-paddle)
- [ ] Paddle-wall collisions are audible
- [ ] Paddle-brick collisions are audible
- [ ] Level transitions have audio cues
- [ ] Volume changes take effect immediately
- [ ] Mute toggle works
- [ ] Game runs without crashes when audio assets are missing

## Troubleshooting

### No Sound on WASM

Click anywhere in the game window first. Browser autoplay restrictions require user interaction before audio can play. The `wasm/restart-audio-context.js` script handles this automatically.

### Audio Clipping During Chain Reactions

Expected behavior: System limits to 4 concurrent sounds of the same type. Excess sounds are dropped to prevent distortion.

### Missing Asset Warnings

Check the console for warnings like:

```text
WARN brkrs::systems::audio: Audio asset missing for BrickDestroy
```

Ensure files exist in `assets/audio/` with correct names matching the manifest.
