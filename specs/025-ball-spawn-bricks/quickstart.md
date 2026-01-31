# Quick Start Guide: Ball Spawn Bricks

**Feature**: 025-ball-spawn-bricks  
**Date**: 2026-01-31  
**Audience**: Developers implementing and testing this feature

## Overview

This guide walks through the development workflow for implementing ball spawn bricks (Red 1, Red 2, Red 3 at indices 37, 38, 39).
Follow the TDD-first approach: write tests, get approval, then implement.

## Prerequisites

- Rust 1.81+ (check: `rustc --version`)
- Bevy 0.17.3 (in project Cargo.toml)
- Git access to repository
- Linux/Windows/macOS for native development (WASM support optional)

## Setup

### 1. Checkout Feature Branch

```bash
git checkout 025-ball-spawn-bricks
git pull origin 025-ball-spawn-bricks
```

### 2. Verify Build

```bash
cargo build
cargo test --lib
```

Both should compile without errors.

## Development Workflow

### Phase 1: Write Tests (Red Phase)

**Step 1**: Create `tests/ball_spawn_bricks.rs` with test module structure

```bash
# File: tests/ball_spawn_bricks.rs
touch tests/ball_spawn_bricks.rs
```

**Step 2**: Write test specifications from acceptance scenarios in spec.md

```rust
// tests/ball_spawn_bricks.rs
use bevy::prelude::*;
use brkrs::signals::BrickDestroyed;

mod red_2_tests {
    use super::*;

    #[test]
    fn red_2_spawns_one_additional_ball() {
        // Given one ball and a Red 2 brick
        // When the ball hits the brick
        // Then a new ball spawns with inverse velocity

        let mut app = setup_test_app();
        let ball = spawn_test_ball(&mut app, Vec3::new(0.0, 0.0, 0.0));
        let brick = spawn_test_brick(&mut app, 38, Vec3::new(5.0, 0.0, 10.0));

        // Trigger collision
        app.world_mut()
            .resource_mut::<Messages<BrickDestroyed>>()
            .write(BrickDestroyed {
                brick_entity: brick,
                brick_type: 38,
                brick_position: Vec3::new(5.0, 0.0, 10.0),
                destroyed_by: Some(ball),
            });

        app.update();  // Process message + spawn system

        // Assert: 2 balls now in play
        let ball_count = app.world().query::<With<Ball>>().iter().count();
        assert_eq!(ball_count, 2, "Expected 2 balls (original + spawned)");
    }

    #[test]
    fn red_2_spawned_ball_has_inverse_velocity() {
        // Spawned ball velocity = -triggering_ball velocity
    }
}

mod red_3_tests {
    use super::*;

    #[test]
    fn red_3_spawns_two_additional_balls() {
        // Similar structure to red_2 test
    }

    #[test]
    fn red_3_spawns_y_shaped_pattern() {
        // Verify velocities form Y-shape
    }
}

mod red_1_tests {
    use super::*;

    #[test]
    fn red_1_despawns_all_except_triggering() {
        // Given 5 balls in play
        // When Red 1 brick is hit
        // Then only triggering ball remains
    }

    #[test]
    fn red_1_maintains_triggering_ball_velocity() {
        // Triggering ball continues with same velocity
    }
}

mod scoring_tests {
    use super::*;

    #[test]
    fn all_three_bricks_award_100_points() {
        // Red 1, Red 2, Red 3 each award 100 points
    }
}

mod persistence_tests {
    use super::*;

    #[test]
    fn spawned_balls_persist_10_frames() {
        // Spawned ball exists across 10 app.update() calls
        // Verify velocity changes frame-to-frame (physics applies)
    }
}

// Helper functions
fn setup_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(PhysicsPlugin)  // Rapier3D
        .add_message::<BrickDestroyed>()
        .add_systems(Update, ball_spawn_system)  // Will be implemented
        .init_resource::<BrickSpawnConfig>();
    app
}

fn spawn_test_ball(app: &mut App, pos: Vec3) -> Entity {
    // Create ball with physics components
    todo!()
}

fn spawn_test_brick(app: &mut App, index: u32, pos: Vec3) -> Entity {
    // Create brick entity
    todo!()
}
```

**Step 3**: Run tests to confirm they fail (Red Phase)

```bash
cargo test --test ball_spawn_bricks
```

Expected result: **ALL TESTS FAIL** ✅ (This is the red phase)

**Step 4**: Commit failing tests

```bash
git add tests/ball_spawn_bricks.rs
git commit -m "test: add ball spawn brick acceptance tests (red phase)"
```

**Step 5**: Request approval

Share branch with feature owner/requestor.
They should:

- [ ] Review test specifications
- [ ] Confirm tests match spec acceptance scenarios
- [ ] Approve before moving to implementation

### Phase 2: Implement (Green Phase)

**Step 1**: Create system module `src/systems/ball_spawn_bricks.rs`

```bash
touch src/systems/ball_spawn_bricks.rs
```

**Step 2**: Implement ball spawning logic

```rust
// src/systems/ball_spawn_bricks.rs
use bevy::prelude::*;
use bevy::ecs::message::MessageReader;
use crate::signals::BrickDestroyed;

#[derive(Resource, Debug, Clone)]
pub struct BrickSpawnConfig {
    pub brick_spawn_rules: HashMap<u8, BrickSpawnRule>,
}

#[derive(Debug, Clone)]
pub struct BrickSpawnRule {
    pub spawn_count: u32,
    pub velocity_modifier: VelocityModifier,
    pub name: &'static str,
}

#[derive(Debug, Clone)]
pub enum VelocityModifier {
    DespawnAll,
    Inverse,
    YShaped { angle_degrees: f32 },
}

pub fn ball_spawn_system(
    mut commands: Commands,
    config: Res<BrickSpawnConfig>,
    mut reader: MessageReader<BrickDestroyed>,
    ball_entities: Query<Entity, With<Ball>>,
    ball_sources: Query<&Velocity, With<Ball>>,
) {
    for message in reader.read() {
        let Some(triggering_ball) = message.destroyed_by else { continue; };
        let Some(rule) = config.brick_spawn_rules.get(&message.brick_type) else { continue; };

        match &rule.velocity_modifier {
            VelocityModifier::DespawnAll => {
                // Red 1: Despawn all except triggering
                for entity in ball_entities.iter() {
                    if entity != triggering_ball {
                        commands.entity(entity).despawn();
                    }
                }
            }
            VelocityModifier::Inverse => {
                // Red 2: Spawn 1 with inverse velocity
                let Ok(trigger_vel) = ball_sources.get(triggering_ball) else { continue; };
                spawn_ball(&mut commands, message.brick_position, -trigger_vel.linvel);
            }
            VelocityModifier::YShaped { angle_degrees } => {
                // Red 3: Spawn 2 with Y-shaped spread
                let Ok(trigger_vel) = ball_sources.get(triggering_ball) else { continue; };
                let (vel1, vel2) = y_shaped_velocity(trigger_vel.linvel, *angle_degrees);
                spawn_ball(&mut commands, message.brick_position, vel1);
                spawn_ball(&mut commands, message.brick_position, vel2);
            }
        }
    }
}

fn spawn_ball(commands: &mut Commands, position: Vec3, velocity: Vec3) {
    // Spawn ball entity with physics components
    // Match existing ball spawning pattern in src/systems/spawning.rs
    todo!()
}

fn y_shaped_velocity(base_vel: Vec3, angle_deg: f32) -> (Vec3, Vec3) {
    // Split velocity into Y-shaped pattern
    // Left and right at angle_deg degrees from original
    todo!()
}
```

**Step 3**: Register system in plugin

```rust
// src/systems/mod.rs - Add at module level
pub mod ball_spawn_bricks;

// In your plugin registration (likely src/lib.rs):
pub struct BallSpawnBricksPlugin;

impl Plugin for BallSpawnBricksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BrickSpawnConfig>()
            .add_systems(Update, ball_spawn_bricks::ball_spawn_system);
    }
}
```

**Step 4**: Run tests to confirm they pass (Green Phase)

```bash
cargo test --test ball_spawn_bricks
```

Expected result: **ALL TESTS PASS** ✅

**Step 5**: Commit implementation

```bash
git add src/systems/ball_spawn_bricks.rs
git add src/systems/mod.rs
git add src/lib.rs
git commit -m "feat: implement ball spawn brick logic (green phase)"
```

### Phase 3: Test Coverage & Verification

**Step 1**: Run full test suite

```bash
cargo test
```

Verify no regressions in existing tests.

**Step 2**: Test WASM build (if supported)

```bash
cargo build --target wasm32-unknown-unknown
```

Confirm compilation succeeds.

**Step 3**: Manual gameplay testing

1. Load a test level with Red 1, Red 2, Red 3 bricks
2. Hit each brick type and verify:
   - **Red 2**: One ball spawns with opposite trajectory
   - **Red 3**: Two balls spawn in Y-shaped pattern
   - **Red 1**: All other balls disappear
   - **Scoring**: 100 points awarded each time
   - **Persistence**: Spawned balls continue moving for 10+ frames

## Common Issues

### Issue: Tests don't compile

**Solution**: Ensure test helpers match your `Ball` component definition

```rust
// Check src/lib.rs for Ball struct
#[derive(Component)]
pub struct Ball {
    // ... check actual fields
}
```

### Issue: Spawned balls don't move

**Solution**: Verify Rapier3D physics components are added

```rust
// Spawned ball must have:
// - Transform (position)
// - Velocity (Rapier3D)
// - RigidBody (Rapier3D)
// - Collider (Rapier3D)
```

### Issue: Red 1 doesn't despawn other balls

**Solution**: Verify query correctly filters all balls

```rust
// Should iterate ALL balls except triggering
for ball in ball_query.iter() {
    if ball.entity() != message.triggering_ball {
        // Despawn this one
    }
}
```

## Code Review Checklist

Before submitting PR, verify:

- [ ] Failing test commit visible in history (red phase)
- [ ] Tests approved by feature owner
- [ ] All tests pass (`cargo test --test ball_spawn_bricks`)
- [ ] No panicking queries (use `.ok()`, not `.unwrap()`)
- [ ] No repeated asset loading
- [ ] WASM builds without errors
- [ ] Rustdoc comments on public functions
- [ ] No `allow` attributes without justification

## Performance Testing

**Spawn 10 balls and measure frame time**:

```bash
cargo run --release
# In-game: Hit Red 3 brick multiple times to reach ~10 balls
# Monitor FPS (should stay at 60)
```

## Next Steps

After implementation is approved and merged:

1. Add more levels with Red 1/2/3 bricks
2. Tune velocity spread angle for Red 3 (currently 45°, may adjust based on playtesting)
3. Consider audio/visual feedback for ball spawning
4. Performance profiling with many balls (edge case testing)

## Resources

- [Bevy 0.17 Book](https://docs.bevyengine.org/bevy/index.html)
- [Physics (Rapier3D) Docs](https://www.rapier.org/)
- [Project Constitution](../../.specify/memory/constitution.md)
- [Feature Spec](spec.md)
- [Data Model](data-model.md)
- [Message Contracts](contracts/ball_spawn_bricks_messages.md)

## Support

For issues during implementation:

1. Check the Troubleshooting section above
2. Review existing brick systems (paddle_size.rs, gravity.rs) for patterns
3. Ask in project discussion or contact feature owner
