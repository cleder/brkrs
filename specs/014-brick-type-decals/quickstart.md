# Quickstart: Brick Type Decals (014)

## Prerequisites

- Rust 1.81 (edition 2021)
- Bevy 0.17.3
- All project dependencies installed (see Cargo.toml)

## Steps

1. Pull the feature branch:

   ```sh
   git checkout 014-brick-type-decals
   ```

2. Build and run the game:

   ```sh
   cargo run
   ```

3. To test decal rendering:
   - Load a level with multiple brick types.
   - Verify each brick displays a visible, type-specific decal centered on its top side.
   - Inspect lighting and camera angles to confirm normal/bump mapping effects.

## Testing

- Run all tests:

  ```sh
  cargo test
  ```

- Run Bevy linter:

  ```sh
  bevy lint
  ```

- Run Clippy:

  ```sh
  cargo clippy --all-targets --all-features
  ```

## Adding New Decals

- Place new decal textures and normal maps in `assets/textures/decals/`.
- Reference them in the appropriate RON level files or brick type definitions.
- Update tests if new brick types are added.
