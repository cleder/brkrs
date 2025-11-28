# Brkrs

`Brkrs` is a Breakout/Arkanoid style game implemented in Rust with the Bevy engine. It extends the classic formula with richer physics, paddle rotation, and per-level configuration.

The game area is divided into a 20×20 grid. Bricks occupy individual grid cells. Gameplay is rendered in 3D but constrained to a plane at `Y = 2.0`.

## Demo

You can play a web version on [GitHub Pages](https://cleder.github.io/brkrs/)

## Core Systems

1. **Physics (Rapier3D)** – 3D physics constrained to a flat play plane.
2. **Game State** – (planned) menu, playing, paused, game over, transitions.
3. **Level Loader** – RON file parsing, entity spawning, per-level gravity.
4. **Brick System** – Extensible brick behaviors via components & events.
5. **Pause System** – ESC to pause, click to resume, with window mode switching (native).

## Controls

- **Mouse Movement** – Move paddle (during gameplay)
- **Mouse Scroll** – Rotate paddle (during gameplay)
- **ESC** – Pause game (freezes physics, shows overlay, hides during level transitions)
- **Left Mouse Click** – Resume game (when paused)

**Note**: Controls are keyboard and mouse only. Gamepad and touch support are not implemented.

## Technical Considerations

### Plane Constraint

All gameplay bodies lock Y translation (`LockedAxes::TRANSLATION_LOCKED_Y`). Camera sits above looking down, allowing lighting & shadows for 3D feel.

### Collisions

Rapier handles base reflection via restitution. Paddle imparts directional "english" using recent mouse movement. Bricks may later apply custom post-collision effects.

### Pause Behavior

- **Physics**: Completely frozen when paused (velocities, positions preserved)
- **Paddle**: Ignores input when paused
- **Cursor**: Hidden during gameplay, visible when paused
- **Window Mode** (Native only): Switches fullscreen→windowed on pause, restores on resume
- **WASM**: Window mode unchanged (WASM doesn't support fullscreen switching)

## Level File Format

Levels live in `assets/levels/` and are RON files parsed into `LevelDefinition`:

```ron
LevelDefinition(
  number: 1,
  gravity: (2.0, 0.0, 0.0), // Optional per-level gravity (x,y,z)
  matrix: [ /* 20 x 20 grid of u8 values */ ]
)
```

### Gravity Override

If `gravity` is present it sets `GravityConfig.normal` and `RapierConfiguration.gravity` on load. During paddle growth after respawn gravity is temporarily set to zero and restored afterward.

### Matrix Cell Values

- `0` empty
- `1` paddle (first occurrence only)
- `2` ball (first occurrence only)
- `3` brick

Matrix must be 20×20. Missing paddle or ball results in fallback spawns.
