# Level files (LevelDefinition)

This directory contains game levels represented as RON `LevelDefinition` files that the runtime reads at startup and when advancing levels. The loader expects a 20×20 matrix by default and will normalize (pad or truncate) incoming data to that shape.

## File structure

A level file is a RON value with the `LevelDefinition` structure the runtime expects. Minimal example:

```ron
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
- `gravity: Option<(f32,f32,f32)>` — optional gravity override for the level (X, Y, Z). If omitted, the runtime uses the global gravity.
- `matrix: Vec<Vec<u8>>` — the tile grid, encoded as rows of byte values. The runtime normalizes input to 20×20 using `src/level_loader.rs::normalize_matrix_simple` (padding/truncating rows or columns as needed).

## Tile values and semantics

The runtime uses numeric tile values to determine what to spawn at each grid cell. Common tokens:

- `0` — empty / no entity.
- `1` — paddle spawn cell (the first `1` found sets the paddle spawn point).
- `2` — ball spawn cell (the first `2` found sets the ball spawn point).
- `3` — legacy simple (destructible) brick index. This value is supported for a compatibility window but should be migrated to `20` in repository assets.
- `20` — canonical simple (destructible) brick index; recommended for newly authored levels.
- `>= 3` — any value 3..=255 is treated as a brick `BrickTypeId` (appearance and variants determined by texture manifest if enabled).
- `90` — indestructible brick — collides and renders like a brick but does NOT count toward level completion.

Notes for designers

- Prefer `20` for standard destructible bricks. `3` is legacy and will be migrated automatically for repository assets (see below).
- `90` is reserved for indestructible bricks — they cannot be destroyed but still collide and participate in gameplay.
- Only the first `1` (paddle) and `2` (ball) in the matrix are used; add at most one of each. If they are absent the runtime spawns reasonable defaults.
- The loader will convert input matrices to the expected 20×20 shape; but editing a properly sized matrix makes human editing and visual reasoning easier.

## Editing and testing workflow

- Edit files directly using your text editor - the RON format is human-readable and Git-friendly.
- To test a level locally, set environment variable `BK_LEVEL` to the level number when running the desktop build, e.g.:

```bash
BK_LEVEL=997 cargo run --release
```

- Unit and integration tests exercise level loading and migration tooling. See `tests/` for examples.

## Migration & repository landing

This repository provides a small migration CLI that can update your level files across the codebase:

- Tool: `tools/migrate-level-indices` — replaces tile values in RON LevelDefinition matrices (e.g., `3` -> `20`).
- Wrapper: `scripts/migrate-assets.sh` — convenience wrapper that builds the migration tool, runs it and prints helpful guidance. Use `--backup` to keep `*.ron.bak` backups.

Example (recommended):

```bash
cd tools/migrate-level-indices && cargo build
./scripts/migrate-assets.sh --backup --from 3 --to 20 assets/levels/*.ron
```

After running the migration script you should review changed files and commit them. CI also runs a migration parity test for PRs that modify files in `assets/levels/`.

## Visual / texture mapping

If the `texture_manifest` feature is enabled, a texture registry maps `BrickTypeId`s (tile values) to visual assets. See `assets/textures/README.md` for texture profile names and how to add type variant mappings for new brick types.

## Common pitfalls

- Avoid trailing commas inside the numeric arrays that make `ron` parsing ambiguous; follow the style of the existing files in this folder.
- Don't rely on the runtime to always find a paddle/ball — give explicit spawn points to avoid surprising fallback behavior.
- When adding custom brick types (>90), confirm textures are available (when `texture_manifest` is enabled) or the default debug material will be used.

If anything is unclear or you want an interactive level editor added (GUI), open an issue or follow-up PR — we can add a small in-editor toolchain or live editor later.
