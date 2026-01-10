# Audio Assets Guide

This directory contains all sound effects and audio configuration for the game.

<!-- INCLUSION-MARKER-DO-NOT-REMOVE -->

## Overview

The game uses an **audio manifest system** (`manifest.ron`) to map gameplay events to audio files.
Sound effects are triggered automatically by the audio system in response to game events.

## File Structure

```text
assets/audio/
├── README.md           # This guide
├── manifest.ron        # Audio configuration mapping events to files
└── [your audio files]  # OGG audio files
```

## Supported Audio Format

**OGG Vorbis** is the only supported audio format:

- **Format**: `.ogg` (Vorbis codec)
- **Why OGG**:
  - Excellent compression (smaller than WAV, similar to MP3)
  - Open-source and royalty-free
  - Well-supported by Bevy audio system
  - Works in WASM builds
- **Recommended Settings**:
  - Sample rate: 44.1 kHz or 48 kHz
  - Bit rate: 128-192 kbps for sound effects
  - Channels: Mono for most sound effects (stereo for music/ambience)
- **Conversion**: Use tools like Audacity or ffmpeg to convert from WAV/MP3 to OGG

## Adding Audio Files

### Step 1: Prepare Your Audio File

1. **Create or obtain** your audio file in a supported format (WAV, MP3, FLAC, etc.)
2. **Convert to OGG Vorbis**:

   ```bash
   # Using ffmpeg
   ffmpeg -i input.wav -c:a libvorbis -q:a 5 output.ogg

   # Using Audacity: File → Export → Export as OGG Vorbis
   ```

3. **Use descriptive filenames**: `brick_destroy.ogg`, `level_complete.ogg`, `paddle_hit_metal.ogg`
4. **Place the file** in `assets/audio/` directory

### Step 2: Update the Audio Manifest

Edit `manifest.ron` to map the sound effect to its filename:

```rust
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
        UiBeep: "cheat_mode_toggle.ogg",
        // Add your new sound here:
        NewSoundType: "new_sound.ogg",
    }
)
```

**Key Requirements**:

- The **key** (e.g., `BrickDestroy`) must match a valid `SoundType` enum variant in the code
- The **value** is the filename of your audio file (relative to `assets/audio/`)
- No duplicate keys allowed
- Filenames are case-sensitive

### Step 3: Test Your Audio

1. **Run the game**: `cargo run`
2. **Trigger the event** that should play your sound (e.g., destroy a brick for `BrickDestroy`)
3. **Check logs**: If the sound doesn't play, check console for warnings about missing files

## Supported Sound Types

The following sound types are currently supported (defined in `src/systems/audio.rs`):

| Sound Type | Event Trigger | Notes |
|------------|---------------|-------|
| `BrickDestroy` | Brick destroyed by ball | Standard brick destruction |
| `MultiHitImpact` | Multi-hit brick damaged | Brick hit but not destroyed |
| `WallBounce` | Ball bounces off wall | Border collision |
| `PaddleHit` | Ball bounces off paddle | Paddle collision |
| `PaddleWallHit` | Paddle collides with wall | Paddle movement limit |
| `PaddleBrickHit` | Paddle collides with brick | Rare edge case |
| `LevelStart` | New level begins | Level initialization |
| `LevelComplete` | All bricks cleared | Level completion |
| `UiBeep` | UI interaction blocked | Error/feedback sound |

**Adding New Sound Types**: To add a new sound type, you must update the `SoundType` enum in `src/systems/audio.rs` and trigger the corresponding audio signal in the relevant game system.

## Audio Manifest Reference

### Structure

```rust
AudioManifest(
    sounds: {
        <SoundType>: "<filename.ogg>",
        // ... more mappings
    }
)
```

### Fields

- **`sounds`**: A map (dictionary) from `SoundType` enum variants to audio filenames
- **Keys**: Must be valid Rust identifiers matching `SoundType` enum variants
- **Values**: Filenames (strings) relative to `assets/audio/` directory

### Example: Complete Manifest

```rust
AudioManifest(
    sounds: {
        // Collision sounds
        BrickDestroy: "impact_brick_shatter.ogg",
        MultiHitImpact: "impact_brick_thud.ogg",
        WallBounce: "impact_wall_bounce.ogg",
        PaddleHit: "impact_paddle_hit.ogg",

        // Paddle edge cases
        PaddleWallHit: "paddle_wall_collision.ogg",
        PaddleBrickHit: "paddle_brick_collision.ogg",

        // Level events
        LevelStart: "level_start_chime.ogg",
        LevelComplete: "level_complete_fanfare.ogg",

        // UI feedback
        UiBeep: "ui_error_beep.ogg",
    }
)
```

## Best Practices

### Audio File Creation

- **Keep files short**: Sound effects should be 0.5-2 seconds for most impacts
- **Normalize volume**: Aim for consistent loudness across all sound effects
- **Remove silence**: Trim leading/trailing silence to ensure immediate playback
- **Avoid clipping**: Keep peak levels below 0 dB to prevent distortion
- **Test in-game**: Audio that sounds good in isolation may not fit gameplay context

### File Organization

- **Use descriptive names**: `brick_heavy_impact.ogg` is better than `sound1.ogg`
- **Group related sounds**: Consider prefixes like `impact_`, `ui_`, `level_`
- **Keep file sizes small**: Compress to ~128 kbps for most effects (good quality, small size)
- **Optimize for WASM**: Total audio assets affect web build loading time

### Manifest Maintenance

- **Keep alphabetical**: Sort sound type keys for easy reference
- **Comment variations**: Add notes if using different sounds for similar events
- **Version control**: Track manifest changes to understand audio design evolution

## Troubleshooting

### Sound Not Playing

1. **Check filename**: Ensure filename in manifest matches actual file (case-sensitive)
2. **Verify format**: Confirm file is valid OGG Vorbis (use `file` command or media player)
3. **Check logs**: Look for asset loading warnings in console output
4. **Test file**: Play the OGG file directly in a media player to confirm it works
5. **Verify trigger**: Ensure the game event that should trigger the sound is actually occurring

### Sound Plays But Is Wrong

1. **Check mapping**: Verify correct sound is mapped to the event in manifest
2. **Test isolation**: Temporarily map all sounds to one file to isolate the issue
3. **Volume check**: Ensure audio file isn't silent or too quiet

### Manifest Parse Errors

1. **Check RON syntax**: Ensure proper RON format (commas between entries, colon between key/value)
2. **Verify sound types**: All keys must match exact `SoundType` enum variant names
3. **Check for typos**: Sound type names are case-sensitive (`BrickDestroy` not `brickdestroy`)
4. **Look for duplicates**: Each sound type can only appear once in the map

### Common Errors

**"File not found"**: Filename in manifest doesn't match actual file.
Check spelling and case.

**"Unsupported format"**: File is not OGG Vorbis.
Convert using ffmpeg or Audacity.

**"Unknown sound type"**: Key in manifest doesn't match any `SoundType` enum variant.
Check spelling and capitalization.

## Audio Configuration

The game provides global audio configuration in `config/audio.ron`:

```rust
AudioConfig(
    master_volume: 1.0,      // Overall volume (0.0-1.0)
    effects_volume: 0.8,     // Sound effects volume multiplier
    music_volume: 0.6,       // Music volume multiplier (when implemented)
)
```

See `config/audio.ron` for current settings and documentation.

<!-- INCLUSION-MARKER-END-DO-NOT-REMOVE -->

---

For more technical details, see the audio system implementation in `src/systems/audio.rs`.
