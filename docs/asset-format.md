# Asset Format

This guide documents the file formats used for game assets in brkrs.

## Level Files

Levels are defined in RON (Rusty Object Notation) files located in `assets/levels/`.

### File Naming

- Format: `level_NNN.ron` where NNN is a zero-padded number
- Examples: `level_001.ron`, `level_002.ron`, `level_999.ron`

### Level Definition Structure

```rust
LevelDefinition(
  number: 1,                         // Level number (must match filename)
  gravity: Some((2.0, 0.0, 0.0)),    // Optional: custom gravity vector (x, y, z)
  matrix: [
    // 20 rows of 20 columns each
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    // ... 18 more rows ...
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
  ],
)
```

### Matrix Cell Values

| Value | Entity | Notes |
|-------|--------|-------|
| `0` | Empty | No entity spawned |
| `1` | Paddle | First occurrence only; additional 1s are ignored |
| `2` | Ball | First occurrence only; additional 2s are ignored |
| `3` | Brick | Standard destructible brick |
| `4` | Indestructible Brick | Cannot be destroyed by the ball |
| `5` | Brick Type 5 | (Reserved for future use) |

### Grid Coordinates

The game uses a 20×20 grid:

- **Origin**: Top-left corner is `[0][0]`
- **X-axis**: Columns (left to right, 0-19)
- **Z-axis**: Rows (top to bottom, 0-19)
- **Y-axis**: Fixed at Y=2.0 (gameplay plane)

```text
     0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19  (columns)
   ┌────────────────────────────────────────────────────────────┐
 0 │                                                            │
 1 │              ┌─────────────────────┐                       │
 2 │              │   BRICKS (3s)       │                       │
 3 │              └─────────────────────┘                       │
 4 │                                                            │
   │                                                            │
12 │                    ○ Ball (2)                              │
   │                                                            │
18 │                    ═ Paddle (1)                            │
19 │                                                            │
   └────────────────────────────────────────────────────────────┘
(rows)
```

### Gravity Override

The `gravity` field is optional:

```rust
// No gravity override (uses default)
gravity: None,

// Custom gravity vector
gravity: Some((2.0, 0.0, 0.0)),  // Pulls toward +X
gravity: Some((-1.0, 0.0, 1.0)), // Diagonal pull
```

During ball respawn, gravity is temporarily set to zero while the paddle grows back to normal size.

### Example: Complete Level

```rust
LevelDefinition(
  number: 1,
  gravity: Some((2.0, 0.0, 0.0)),
  matrix: [
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,3,3,3,3,3,3,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,3,3,3,3,3,3,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,3,3,3,3,3,3,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,3,3,3,3,3,3,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
  ],
)
```

## Texture Assets

Textures are stored in `assets/textures/` with a manifest file.

### Texture Manifest

The `manifest.ron` file maps texture identifiers to file paths:

```rust
// assets/textures/manifest.ron
TextureManifest(
  textures: {
    "brick_base": "fallback/brick_base.png",
    "paddle_base": "fallback/paddle_base.png",
    "ball_base": "fallback/ball_base.png",
    // ...
  }
)
```

### Fallback Textures

The `fallback/` directory contains default textures used when custom textures are not available:

- `brick_base.png` — Default brick texture
- `paddle_base.png` — Default paddle texture
- `ball_base.png` — Default ball texture
- `ground_base.png` — Floor texture
- `sidewall_base.png` — Wall textures
- `background_base.png` — Background texture

## Validation

### Level Validation Rules

1. **Matrix size**: Must be exactly 20 rows × 20 columns
2. **Paddle**: At least one cell with value `1` (or fallback spawn is used)
3. **Ball**: At least one cell with value `2` (or fallback spawn is used)
4. **Number**: Must match the filename (e.g., `number: 1` in `level_001.ron`)

### Common Errors

**"Matrix must be 20x20"**: Check that all rows have exactly 20 elements and there are exactly 20 rows.

**"Invalid cell value"**: Only values 0-5 are valid. Check for typos.

**"Missing paddle/ball"**: The level will still load but will use fallback spawn positions.
