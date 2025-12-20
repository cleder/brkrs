# Quickstart: Refactor Entity Spawning

**Feature**: `012-refactor-entity-spawning`

## Running the Game

Standard execution:

```bash
cargo run
```

## Verifying the Refactor

1. **Visual Check**: Run the game and ensure the camera angle, lighting, and ground plane look identical to the `develop` branch.
2. **Code Check**: Verify that `src/lib.rs` no longer contains `spawn` calls for camera, light, or ground in the `setup` function.
3. **Module Check**: Verify `src/systems/spawning.rs` exists and contains the spawning logic.

## Testing

Run unit tests:

```bash
cargo test
```

(Note: New tests will be added to verify system existence/registration).
