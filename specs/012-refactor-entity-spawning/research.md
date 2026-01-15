# Research: Refactor Entity Spawning

**Feature**: `012-refactor-entity-spawning` **Date**: 2025-12-20

## Unknowns & Clarifications

### 1. Bevy 0.17 Spawning Best Practices

- **Question**: Are there any specific mandates for spawning standard entities like Camera and Light in Bevy 0.17?
- **Finding**: Bevy 0.17 uses `Commands` to spawn entities.
  Components like `Camera3d`, `PointLight`, `Mesh3d`, and `MeshMaterial3d` are standard.
  The constitution mandates using `ChildOf` for hierarchy, but these are top-level entities.
- **Decision**: Use standard `commands.spawn((...))` pattern.

### 2. Module Structure

- **Question**: Where should `MainCamera` be defined?
- **Finding**: `MainCamera` is currently defined inside `setup`.
  It needs to be accessible to other systems (e.g., for queries).
- **Decision**: Move `MainCamera` to `src/systems/spawning.rs` and make it `pub`.
  Re-export it in `src/systems/mod.rs` or `src/lib.rs` if needed, but `use crate::systems::spawning::MainCamera` is preferred.

### 3. System Registration

- **Question**: How to replace `setup`?
- **Finding**: `setup` currently does multiple things: configures physics (gravity) and spawns entities.
- **Decision**:
  - Rename `setup` to `configure_physics` (or keep as `setup` but remove spawning logic).
  - Register `spawn_camera`, `spawn_ground_plane`, `spawn_light` as separate startup systems.
  - Order: `add_systems(Startup, (configure_physics, spawn_camera, spawn_ground_plane, spawn_light))`.
    Order doesn't strictly matter for these independent entities.

## Technology Choices

- **Bevy ECS**: Standard system registration.
- **Rapier3D**: `RapierConfiguration` access remains in the physics configuration system.

## Implementation Strategy

1. Create `src/systems/spawning.rs`.
2. Move `MainCamera` struct to `spawning.rs`.
3. Implement `spawn_camera`, `spawn_ground_plane`, `spawn_light` in `spawning.rs`, copying logic from `src/lib.rs`.
4. Update `src/lib.rs`:
   - Add `mod spawning;` to `src/systems/mod.rs` (or `lib.rs` if `systems` module structure allows).
   - Remove spawning logic from `setup`.
   - Register new systems in `App` builder.
   - Update imports for `MainCamera`.

## Alternatives Considered

- **Keep in `lib.rs`**: Rejected to improve modularity and reduce file size.
- **Single `spawn_scene` system**: Rejected because splitting allows for more granular testing and potential future separation (e.g., different levels might need different lights but same camera).
