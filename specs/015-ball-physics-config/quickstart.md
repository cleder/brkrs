# Quickstart: Ball, Paddle, Brick Physics Config

## Overview

This feature centralizes all physics configuration (restitution, friction, damping, etc.) for balls, paddles, and bricks in dedicated Bevy resources, defined in source code.
No config is hot-reloadable or loaded from files.

## How to Update Physics Config

1. Edit the relevant struct in source (e.g., `BallPhysicsConfig` in `src/`)
2. Change the values as needed for tuning
3. Rebuild and run the game to apply changes

## How to Add/Update Usage

- When spawning a ball, paddle, or brick, always query the relevant config resource and apply its values to the collider/rigidbody.
- Never hardcode restitution, friction, or damping values in spawn logic.

## How to Test

- Run `cargo test` to verify all entities use the config and no hardcoded values remain
- Run static analysis/lint to check for hardcoded physics values

## Example (Rust)

```rust
// Register config as a resource
app.insert_resource(BallPhysicsConfig {
    restitution: 0.9,
    friction: 0.1,
    linear_damping: 0.05,
    angular_damping: 0.01,
});

// In spawn system
fn spawn_ball(
    config: Res<BallPhysicsConfig>,
    mut commands: Commands,
    // ...
) {
    commands.spawn((
        // ...
        Collider::ball(radius).with_restitution(config.restitution).with_friction(config.friction),
        RigidBody::dynamic().with_linear_damping(config.linear_damping).with_angular_damping(config.angular_damping),
        // ...
    ));
}
```

## No Hot-Reload

- All config is source-only.
  Do not attempt to load from files or support runtime changes.
