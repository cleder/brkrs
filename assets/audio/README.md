# Audio Assets Guide

This directory contains all sound effects used by the game.
To add or update sound effects, follow these steps:

## 1. Add Your Audio Files

- Place new `.ogg` files in this directory.
- Use clear, descriptive filenames (e.g., `brick_destroy.ogg`, `level_complete.ogg`).

## 2. Update the Manifest

- Edit `manifest.ron` to map each sound effect to its filename.
- The manifest uses the following format:

```text
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
    }
)
```

- The key (e.g., `BrickDestroy`) must match a valid `SoundType` in the code.
- The value is the filename of your audio file.

## 3. Supported Sound Types

- Only sound types defined in the code (`SoundType` enum) are supported.
- Common types include:
  - BrickDestroy
  - MultiHitImpact
  - WallBounce
  - PaddleHit
  - PaddleWallHit
  - PaddleBrickHit
  - LevelStart
  - LevelComplete
  - UiBeep

## 4. Testing

- Run the game and trigger the relevant event to test your sound.
- If a sound is missing or misnamed, a warning will be logged but the game will not crash.

## 5. Troubleshooting

- Ensure filenames in `manifest.ron` match the actual files.
- Only `.ogg` format is supported.
- If you add a new sound type, update the code to support it.

---

For more details, see the audio system documentation in `src/systems/audio.rs`.
