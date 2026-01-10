# Level Files (LevelDefinition)

This directory contains game levels represented as RON `LevelDefinition` files that the runtime reads at startup and when advancing levels.
The loader expects a 20×20 matrix by default and will normalize (pad or truncate) incoming data to that shape.

<!-- INCLUSION-MARKER-DO-NOT-REMOVE -->

## File Naming

- **Format**: `level_NNN.ron` where NNN is a zero-padded number
- **Examples**: `level_001.ron`, `level_002.ron`, `level_999.ron`

## Level Definition Structure

A level file is a RON value with the `LevelDefinition` structure the runtime expects.
Minimal example:

```rust
LevelDefinition(
  number: 1,                         // Level number (must match filename)
  gravity: Some((2.0, 0.0, 0.0)),    // Optional: custom gravity vector (x, y, z)
  description: Some("Level design notes and gameplay hints"), // Optional: level documentation
  author: Some("[Jane Smith](mailto:jane@example.com)"),      // Optional: contributor attribution
  matrix: [
    // 20 rows of 20 columns each
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    // ... 18 more rows ...
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
  ],
  presentation: Some((              // Optional: per-level texture overrides
    level_number: 1,
    ground_profile: Some("ground/custom"),
    background_profile: None,
    sidewall_profile: None,
    tint: None,
    notes: Some("Custom ground texture"),
  )),
)
```

### Fields Reference

- **`number: u32`** — Level index/identifier; used by the loader to find `level_{:03}.ron` next-level files.
  Must match the filename (e.g., `number: 1` in `level_001.ron`).
- **`gravity: Option<(f32, f32, f32)>`** — Optional gravity override for the level (X, Y, Z).
  If omitted, the runtime uses the global gravity configuration.
  During ball respawn, gravity is temporarily set to zero while the paddle grows back to normal size.
- **`matrix: Vec<Vec<u8>>`** — The tile grid, encoded as rows of byte values.
  The runtime normalizes input to 20×20 using `src/level_loader.rs::normalize_matrix_simple` (padding/truncating rows or columns as needed).
- **`description: Option<String>`** — Optional level design documentation.
  Use for design notes, gameplay hints, technical implementation details, or any other information helpful to other developers.
  Supports multiline strings and special characters.
- **`author: Option<String>`** — Optional contributor attribution.
  Use plain text names or Markdown link format `[Name](url)` for email/website attribution.
  The runtime provides helper methods to extract display names from Markdown links.
- **`presentation: Option<LevelTextureSet>`** — Optional per-level texture overrides for ground, background, and sidewall materials.
  See "Assigning Per-Level Ground Textures" below.

### Grid Coordinates

The game uses a 20×20 grid with the following coordinate system:

- **Origin**: Top-left corner is `[0][0]`
- **X-axis**: Columns (left to right, 0-19)
- **Z-axis**: Rows (top to bottom, 0-19)
- **Y-axis**: Fixed at Y=2.0 (gameplay plane)

```text
     0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19  (columns)
   ┌────────────────────────────────────────────────────────────┐
 0 │                                                            │
 1 │              ┌─────────────────────┐                       │
 2 │              │   BRICKS (20s)      │                       │
 3 │              └─────────────────────┘                       │
 4 │                                                            │
   │                                                            │
12 │                    ═ Paddle (2)                            │
   │                                                            │
18 │                    ○ Ball (1)                              │
19 │                                                            │
   └────────────────────────────────────────────────────────────┘
(rows)
```

### Gravity Override Examples

The `gravity` field is optional and allows per-level physics customization:

```rust
// No gravity override (uses default global gravity)
gravity: None,

// Custom gravity vector
gravity: Some((2.0, 0.0, 0.0)),  // Pulls toward +X (right)
gravity: Some((-1.0, 0.0, 1.0)), // Diagonal pull
gravity: Some((0.0, -9.81, 0.0)), // Standard downward gravity
```

## Metadata Fields (Description and Author)

Level files can include optional metadata fields for better organization and attribution:

### Description Field

The `description` field allows level designers to document their design intent, gameplay mechanics, or technical notes:

```rust
LevelDefinition(
  number: 42,
  description: Some(r#"
    Expert challenge level featuring moving obstacles.

    Design goals:
    - Test player precision timing
    - Introduce complex brick patterns
    - Maintain 60 FPS performance

    Technical notes:
    - Uses custom brick type 100
    - Requires texture_manifest feature
  "#),
  matrix: [
    // ... level matrix
  ],
)
```

**Features**:

- Multiline strings using raw string literals (`r#"..."#`)
- Special characters and formatting
- Detailed design documentation
- Technical implementation notes

### Author Field

The `author` field credits contributors and supports both plain text and Markdown link formats:

```rust
// Plain text attribution
author: Some("Jane Smith")

// Markdown email link
author: Some("[Jane Smith](mailto:jane@example.com)")

// Markdown website link
author: Some("[Game Team](https://github.com/org/repo)")
```

The runtime provides `extract_author_name()` function and `LevelDefinition::author_name()` method to extract display names from Markdown links, returning "Jane Smith" or "Game Team" respectively.

### Backward Compatibility

Both metadata fields are optional and default to `None`.
Existing level files without these fields continue to work unchanged.
The runtime treats empty/whitespace-only values as `None` for helper methods like `has_description()` and `has_author()`.

## Matrix Cell Values (Tile Semantics)

The runtime uses numeric tile values to determine what to spawn at each grid cell:

## Matrix Cell Values (Tile Semantics)

The runtime uses numeric tile values to determine what to spawn at each grid cell:

| Value | Entity | Notes |
|-------|--------|-------|
| `0` | Empty | No entity spawned |
| `1` | Ball | First occurrence only; additional 1s are ignored. At least one recommended. |
| `2` | Paddle | First occurrence only; additional 2s are ignored. At least one recommended. |
| `20` | Standard Brick | Canonical destructible brick type (recommended for new levels) |
| `3` | Legacy Brick | Standard destructible brick (legacy; prefer `20` for new levels) |
| `90` | Indestructible Brick | Collides like a brick but does NOT count toward level completion |
| `4-89, 91-255` | Custom Brick Types | Appearance and behavior determined by texture manifest (if enabled) |

### Notes for Designers

- **Prefer `20` for standard destructible bricks**.
  Value `3` is legacy and will be migrated automatically for repository assets.
- **`90` is reserved for indestructible bricks** — they cannot be destroyed but still collide and participate in gameplay.
- **Only the first `2` (paddle) and `1` (ball) in the matrix are used**; add at most one of each.
  If they are absent, the runtime spawns reasonable defaults.
- **The loader will convert input matrices to the expected 20×20 shape**; but editing a properly sized matrix makes human editing and visual reasoning easier.
- When adding custom brick types (4-89, 91-255), confirm textures are available (when `texture_manifest` is enabled) or the default debug material will be used.

- When adding custom brick types (4-89, 91-255), confirm textures are available (when `texture_manifest` is enabled) or the default debug material will be used.

## Example: Complete Level

```rust
LevelDefinition(
  number: 1,
  gravity: Some((2.0, 0.0, 0.0)),
  description: Some("Tutorial level: Learn the basics"),
  author: Some("[Tutorial Team](https://github.com/brkrs/levels)"),
  matrix: [
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,20,20,20,20,20,20,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,20,20,20,20,20,20,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,20,20,20,20,20,20,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,20,20,20,20,20,20,0,0,0,0,0,0,0],
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
  presentation: Some((
    level_number: 1,
    ground_profile: Some("ground/tutorial"),
    background_profile: None,
    sidewall_profile: None,
    tint: None,
    notes: Some("Tutorial level ground texture"),
  )),
)
```

## Editing and Testing Workflow

## Editing and Testing Workflow

- **Edit files directly** using your text editor - the RON format is human-readable and Git-friendly.
- **Test a specific level** by setting environment variable `BK_LEVEL` to the level number when running the desktop build:

  ```bash
  BK_LEVEL=997 cargo run --release
  ```

- **Hot-reload**: In-game, press **L** to cycle through levels quickly for visual verification.
- **Unit and integration tests** exercise level loading and migration tooling.
  See `tests/` for examples.

## Assigning Per-Level Ground Textures

You can assign unique ground textures to each level automatically or manually:

### Automatic Assignment (Recommended)

Use the helper script:

```bash
python scripts/assign_and_add_ground_profiles.py --mode=all      # Reassigns all levels
python scripts/assign_and_add_ground_profiles.py --mode=missing  # Only assigns to levels without a ground_profile
```

This script:

- Randomly assigns a unique texture from `assets/textures/background/` to each level's ground plane (as a `ground_profile` in the `presentation` field)
- Updates `assets/textures/manifest.ron` to add any missing ground profiles for the assigned textures
- Can be run multiple times to reshuffle textures or fill in missing assignments

### Manual Assignment

1. **Choose a texture** from `assets/textures/background/` (e.g., `nsTile1044.png`).
2. **Add a profile** to `assets/textures/manifest.ron` if not already present:

    ```rust
    (
        id: "ground/nsTile1044",
        albedo_path: "background/nsTile1044.png",
        normal_path: None,
        roughness: 0.9,
        metallic: 0.0,
        uv_scale: (4.0, 3.0),
        uv_offset: (0.0, 0.0),
        fallback_chain: ["ground/default"],
    ),
    ```

3. **Edit the level file** (e.g., `assets/levels/level_001.ron`) and add or update the `presentation` field:

    ```rust
    LevelDefinition(
      number: 1,
      ...
      presentation: Some((
        level_number: 1,
        ground_profile: Some("ground/nsTile1044"),
        background_profile: None,
        sidewall_profile: None,
        tint: None,
        notes: Some("Custom ground texture"),
      )),
    )
    ```

4. **Save and reload the game** (or use hot-reload if supported) to see the new ground texture in-game.

See also: `assets/textures/README.md` for more on texture profiles and manifest editing.

## Visual / Texture Mapping

If the `texture_manifest` feature is enabled, a texture registry maps `BrickTypeId`s (tile values) to visual assets.
See `assets/textures/README.md` for texture profile names and how to add type variant mappings for new brick types.

## Validation

### Level Validation Rules

1. **Matrix size**: Must be exactly 20 rows × 20 columns (normalized automatically by the loader)
2. **Level number**: Must match the filename (e.g., `number: 1` in `level_001.ron`)
3. **Paddle spawn**: At least one cell with value `2` recommended (fallback spawn position used if absent)
4. **Ball spawn**: At least one cell with value `1` recommended (fallback spawn position used if absent)
5. **Cell values**: All values must be 0-255 (u8 range)

### Common Errors

**"Matrix must be 20x20"**: Check that all rows have exactly 20 elements and there are exactly 20 rows.
The loader will normalize mismatched matrices, but editing with the correct size makes debugging easier.

**"Invalid cell value"**: Only values 0-255 are valid (u8 range).
Check for typos or negative values.

**"Missing paddle/ball"**: The level will still load but will use fallback spawn positions.
For clarity, always include at least one `2` (paddle) and one `1` (ball) in the matrix.

**"Failed to parse level"**: Check RON syntax - common issues include trailing commas inside arrays, unmatched brackets, or missing commas between array elements.

## Common Pitfalls

## Common Pitfalls

- **Trailing commas**: Avoid trailing commas inside numeric arrays as they can make RON parsing ambiguous.
  Follow the style of existing files in this folder.
- **Missing spawn points**: Don't rely on the runtime to always find a paddle/ball - give explicit spawn points to avoid surprising fallback behavior.
- **Custom brick types without textures**: When adding custom brick types (>90), confirm textures are available (when `texture_manifest` is enabled) or the default debug material will be used.
- **Mismatched level number**: Ensure the `number` field matches the filename number to avoid confusion during level transitions.

<!-- INCLUSION-MARKER-END-DO-NOT-REMOVE -->

If anything is unclear or you want an interactive level editor added (GUI), open an issue or follow-up PR — we can add a small in-editor toolchain or live editor later.
