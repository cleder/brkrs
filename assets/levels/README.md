# Level files (LevelDefinition)

This directory contains game levels represented as RON `LevelDefinition` files that the runtime reads at startup and when advancing levels.
The loader expects a 20×20 matrix by default and will normalize (pad or truncate) incoming data to that shape.

## File structure

A level file is a RON value with the `LevelDefinition` structure the runtime expects.
Minimal example:

```rust
LevelDefinition(
  number: 1,
  gravity: Some((0.0, -9.81, 0.0)), // optional override per-level
  matrix: [
    // 20 rows of 20 u8 values each (outer vector = rows)
    [0,0,0,...],
    ...
  ],
)
```

Fields

- `number: u32` — level index/identifier; used by the loader to find `level_{:03}.ron` next-level files.
- `gravity: Option<(f32,f32,f32)>` — optional gravity override for the level (X, Y, Z).
  If omitted, the runtime uses the global gravity.
- `matrix: Vec<Vec<u8>>` — the tile grid, encoded as rows of byte values.
  The runtime normalizes input to 20×20 using `src/level_loader.rs::normalize_matrix_simple` (padding/truncating rows or columns as needed).
- `description: Option<String>` — optional level design documentation.
  Use for design notes, gameplay hints, technical implementation details, or any other information helpful to other developers.
  Supports multiline strings and special characters.
- `author: Option<String>` — optional contributor attribution.
  Use plain text names or Markdown link format `[Name](url)` for email/website attribution.
  The runtime provides helper methods to extract display names from Markdown links.

## Metadata fields (description and author)

Level files can include optional metadata fields for better organization and attribution:

### Description field

The `description` field allows level designers to document their design intent, gameplay mechanics, or technical notes:

```rust
LevelDefinition(
  number: 42,
  description: Some(r#"
    Expert challenge level featuring moving obstacles.

    Design goals:
    - Test player precision timing
    - Introduce moving brick patterns
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

### Author field

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

### Backward compatibility

Both fields are optional and default to `None`.
Existing level files without these fields continue to work unchanged.
The runtime treats empty/whitespace-only values as `None` for helper methods like `has_description()` and `has_author()`.

## Tile values and semantics

The runtime uses numeric tile values to determine what to spawn at each grid cell.
Common tokens:

- `0` — empty / no entity.
- `2` — paddle spawn cell (the first `2` found sets the paddle spawn point).
- `1` — ball spawn cell (the first `1` found sets the ball spawn point).
- `20` — canonical simple (destructible) brick index; recommended for newly authored levels.
- `>= 3` — any value 3..=255 is treated as a brick `BrickTypeId` (appearance and variants determined by texture manifest if enabled).
- `90` — indestructible brick — collides and renders like a brick but does NOT count toward level completion.

Notes for designers

- Prefer `20` for standard destructible bricks. `3` is legacy and will be migrated automatically for repository assets (see below).
- `90` is reserved for indestructible bricks — they cannot be destroyed but still collide and participate in gameplay.
- Only the first `2` (paddle) and `1` (ball) in the matrix are used; add at most one of each.
  If they are absent the runtime spawns reasonable defaults.
- The loader will convert input matrices to the expected 20×20 shape; but editing a properly sized matrix makes human editing and visual reasoning easier.

## Editing and testing workflow

- Edit files directly using your text editor - the RON format is human-readable and Git-friendly.
- To test a level locally, set environment variable `BK_LEVEL` to the level number when running the desktop build, e.g.:

```bash
BK_LEVEL=997 cargo run --release
```

- Unit and integration tests exercise level loading and migration tooling.
  See `tests/` for examples.

## Visual / texture mapping

If the `texture_manifest` feature is enabled, a texture registry maps `BrickTypeId`s (tile values) to visual assets.
See `assets/textures/README.md` for texture profile names and how to add type variant mappings for new brick types.

## Common pitfalls

- Avoid trailing commas inside the numeric arrays that make `ron` parsing ambiguous; follow the style of the existing files in this folder.
- Don't rely on the runtime to always find a paddle/ball — give explicit spawn points to avoid surprising fallback behavior.
- When adding custom brick types (>90), confirm textures are available (when `texture_manifest` is enabled) or the default debug material will be used.

If anything is unclear or you want an interactive level editor added (GUI), open an issue or follow-up PR — we can add a small in-editor toolchain or live editor later.
