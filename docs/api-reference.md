# API Reference

The brkrs API documentation is generated from source code using rustdoc.

## Rust API (rustdoc)

The API documentation is generated locally using `cargo doc`.
See [Building Documentation Locally](#building-documentation-locally) below.

:::{note} The embedded rustdoc is available when viewing documentation built by CI (see the [GitHub Actions artifacts](https://github.com/cleder/brkrs/actions/workflows/docs-main.yml)).
On Read the Docs, generate the docs locally using the instructions below.

**Browse the embedded rustdoc → [https://cleder.github.io/brkrs/docs/brkrs/index.html](https://cleder.github.io/brkrs/docs/brkrs/index.html)** :::

The rustdoc includes:

- All public modules, structs, enums, and traits
- Component definitions used by the ECS
- System function signatures
- Resource types for game state

## Module Overview

## Physics Config Resources

The following resources centralize physics configuration for balls, paddles, and bricks:

### BallPhysicsConfig

- `restitution: f32` — Bounciness coefficient (0.0–2.0 recommended)
- `friction: f32` — Friction coefficient (0.0–2.0 recommended)
- `linear_damping: f32` — Linear velocity damping (0.0–10.0 recommended)
- `angular_damping: f32` — Angular velocity damping (0.0–10.0 recommended)

### PaddlePhysicsConfig

- `restitution: f32` — Bounciness coefficient
- `friction: f32` — Friction coefficient
- `linear_damping: f32` — Linear velocity damping
- `angular_damping: f32` — Angular velocity damping

### BrickPhysicsConfig

- `restitution: f32` — Bounciness coefficient
- `friction: f32` — Friction coefficient

All configs provide a `validate()` method to check for finite, non-negative, and reasonable values.
Use these resources in spawn systems to ensure consistent physics parameters and prevent hardcoded values.

**Usage Example:**

```rust
use bevy::prelude::*;
use brkrs::physics_config::BallPhysicsConfig;

fn spawn_ball(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    ball_config: Res<BallPhysicsConfig>, // Inject config
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.3).mesh())),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        RigidBody::Dynamic,
        Collider::ball(0.3),
        Restitution::coefficient(ball_config.restitution),
        Friction::coefficient(ball_config.friction),
        Damping {
            linear_damping: ball_config.linear_damping,
            angular_damping: ball_config.angular_damping,
        },
    ));
}
```

See `src/physics_config.rs` for implementation and extension notes.

The crate is organized into the following modules:

| Module | Description |
|--------|-------------|
| `brkrs` | Main crate with game initialization and Bevy app setup |
| `level_format` | Level file parsing and RON deserialization |
| `level_loader` | Level loading, entity spawning, and grid management |
| `pause` | Pause system state machine and UI overlay |
| `systems` | Game systems (respawn, spawning, textures, level switching, debug) |
| `ui` | User interface components and palette definitions |

## Building Documentation Locally

To generate the rustdoc locally:

```bash
# Generate rustdoc
cargo doc --no-deps --all-features

# Open in browser
cargo doc --no-deps --open
```

To include rustdoc in the Sphinx documentation build:

```bash
# Stage rustdoc to docs/_static/rustdoc
./scripts/stage-rustdoc.sh

# Build Sphinx docs
cd docs && make html
```

## Version Compatibility

This API documentation corresponds to the version of brkrs you are viewing.
Use the version selector in the bottom-left corner to switch between versions.
