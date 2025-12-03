# Event Contracts: Brkrs Complete Game

**Feature**: 001-complete-game **Created**: 2025-10-31 **Purpose**: Define all custom events and observers for game communication

## Overview

This document specifies the event-driven communication patterns in the Brkrs game.
Events enable loose coupling between systems and provide clear contracts for feature interactions.

## Collision Events

### BallHit

Triggered when the ball collides with the paddle, allowing steering impulse application.

**Trigger Condition**: Paddle's `KinematicCharacterControllerOutput` detects collision with ball entity

**Event Data**:

```rust
#[derive(Event)]
pub struct BallHit {
    pub ball: Entity,
    pub impulse: Vec3,  // Steering impulse from paddle velocity
}
```

**Observers**:

- `on_paddle_ball_hit`: Applies steering impulse to ball's
  `ExternalImpulse` component

**Usage Pattern**:

```rust
// In collision detection system
for collision in paddle_output.collisions.iter() {
    if let Ok(ball) = balls.get(collision.entity) {
        commands.trigger(BallHit {
            ball: collision.entity,
            impulse: calculate_steering_impulse(paddle_velocity),
        });
    }
}

// Observer function
fn on_paddle_ball_hit(
    trigger: Trigger<BallHit>,
    mut balls: Query<&mut ExternalImpulse, With<Ball>>,
) {
    if let Ok(mut impulse) = balls.get_mut(trigger.event().ball) {
        impulse.impulse = trigger.event().impulse;
    }
}
```

### WallHit

Triggered when the paddle collides with a wall, providing bounce-back effect and optional screen shake.

**Trigger Condition**: Paddle's `KinematicCharacterControllerOutput` detects collision with border entity

**Event Data**:

```rust
#[derive(Event)]
pub struct WallHit {
    pub impulse: Vec3,  // Bounce-back force
}
```

**Observers**:

- `on_wall_hit`: Applies counter-impulse to paddle and triggers screen shake

**Usage Pattern**:

```rust
// In collision detection system
for collision in paddle_output.collisions.iter() {
    if walls.contains(collision.entity) {
        commands.trigger(WallHit {
            impulse: calculate_bounce_impulse(collision),
        });
    }
}

// Observer function
fn on_wall_hit(
    trigger: Trigger<WallHit>,
    mut paddles: Query<&mut KinematicCharacterController, With<Paddle>>,
    mut balls: Query<&mut ExternalImpulse, With<Ball>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
) {
    // Paddle bounce-back
    for mut controller in paddles.iter_mut() {
        controller.translation = Some(-trigger.event().impulse * 0.03);
    }

    // Optional: Give balls a small impulse
    for mut ball_impulse in balls.iter_mut() {
        ball_impulse.impulse = trigger.event().impulse * 0.001;
    }

    // Optional: Screen shake (move camera)
    if let Ok(mut cam_transform) = camera.get_single_mut() {
        cam_transform.translation += trigger.event().impulse * 0.01;
    }
}
```

### BrickHit

Triggered when the ball collides with a brick, allowing brick-specific behavior.

**Trigger Condition**: Rapier `CollisionEvent::Started` between ball and brick entities

**Event Data**:

```rust
#[derive(Event)]
pub struct BrickHit {
    pub brick: Entity,
    pub ball: Entity,
    pub collision_point: Vec3,
    pub collision_normal: Vec3,
}
```

**Observers**:

- `on_brick_hit`: Handles brick damage, destruction, and special behaviors
  based on `BrickType`

**Usage Pattern**:

```rust
// In collision event system
fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    bricks: Query<(Entity, &BrickType), With<Brick>>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, flags) = event {
            // Determine which is ball and which is brick
            if let Ok(ball) = balls.get(*e1) {
                if let Ok((brick, _)) = bricks.get(*e2) {
                    commands.trigger(BrickHit {
                        brick: *e2,
                        ball: *e1,
                        collision_point: /* from flags */,
                        collision_normal: /* from flags */,
                    });
                }
            }
        }
    }
}

// Observer function
fn on_brick_hit(
    trigger: Trigger<BrickHit>,
    mut commands: Commands,
    mut bricks: Query<(&mut Durability, &BrickType)>,
    mut balls: Query<&mut ExternalImpulse, With<Ball>>,
    mut level: ResMut<CurrentLevel>,
) {
    let event = trigger.event();

    if let Ok((mut durability, brick_type)) = bricks.get_mut(event.brick) {
        match brick_type {
            BrickType::Standard => {
                // Destroy immediately
                commands.entity(event.brick).despawn_recursive();
                level.bricks_remaining -= 1;
            }
            BrickType::MultiHit { .. } => {
                durability.0 -= 1;
                if durability.0 == 0 {
                    commands.entity(event.brick).despawn_recursive();
                    level.bricks_remaining -= 1;
                }
            }
            BrickType::SpeedUp => {
                // Apply velocity boost to ball
                if let Ok(mut impulse) = balls.get_mut(event.ball) {
                    impulse.impulse += event.collision_normal * 5.0;
                }
                commands.entity(event.brick).despawn_recursive();
                level.bricks_remaining -= 1;
            }
            // ... handle other brick types
            _ => {}
        }
    }
}
```

## Game State Events

### LevelComplete

Triggered when all destructible bricks are destroyed.

**Trigger Condition**: `CurrentLevel.bricks_remaining` reaches 0

**Event Data**:

```rust
#[derive(Event)]
pub struct LevelComplete {
    pub level_number: u8,
}
```

**Observers**:

- `on_level_complete`: Transitions to `GameState::LevelTransition`, prepares
  next level

**Usage Pattern**:

```rust
// In brick destruction system
fn check_level_complete(
    level: Res<CurrentLevel>,
    mut commands: Commands,
) {
    if level.bricks_remaining == 0 {
        commands.trigger(LevelComplete {
            level_number: level.number,
        });
    }
}

// Observer function
fn on_level_complete(
    trigger: Trigger<LevelComplete>,
    mut next_state: ResMut<NextState<GameState>>,
    mut level: ResMut<CurrentLevel>,
) {
    level.number += 1;
    next_state.set(GameState::LevelTransition);
}
```

### LifeLost

Triggered when the ball exits the lower boundary.

**Trigger Condition**: Ball's Transform.translation.z < lower_boundary

**Event Data**:

```rust
#[derive(Event)]
pub struct LifeLost {
    pub ball: Entity,
}
```

**Observers**:

- `on_life_lost`: Decrements `Lives`, despawns ball, checks for game over

**Usage Pattern**:

```rust
// In boundary checking system
fn check_ball_boundaries(
    balls: Query<(Entity, &Transform), With<Ball>>,
    config: Res<GameConfig>,
    mut commands: Commands,
) {
    for (entity, transform) in balls.iter() {
        if transform.translation.z < config.lower_boundary {
            commands.trigger(LifeLost { ball: entity });
        }
    }
}

// Observer function
fn on_life_lost(
    trigger: Trigger<LifeLost>,
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.entity(trigger.event().ball).despawn_recursive();
    lives.count -= 1;

    if lives.count == 0 {
        next_state.set(GameState::GameOver);
    } else {
        // Respawn ball after delay
        // (spawn_ball_delayed system in Playing state)
    }
}
```

### GameOver

Triggered when lives reach 0.

**Trigger Condition**: `Lives.count` == 0 after ball lost

**Event Data**:

```rust
#[derive(Event)]
pub struct GameOver {
    pub final_score: u32,
    pub final_level: u8,
}
```

**Observers**:

- `on_game_over`: Transitions to `GameState::GameOver`, displays stats

**Usage Pattern**:

```rust
// Triggered from LifeLost observer or directly
commands.trigger(GameOver {
    final_score: score.0,
    final_level: level.number,
});

// Observer function
fn on_game_over(
    trigger: Trigger<GameOver>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::GameOver);
    // Stats displayed by UI system reading the event
}
```

## Input Events

### PauseRequested

Triggered when player presses pause key.

**Trigger Condition**: Escape key pressed while in `GameState::Playing`

**Event Data**:

```rust
#[derive(Event)]
pub struct PauseRequested;
```

**Observers**:

- `on_pause_requested`: Transitions to `GameState::Paused`

### ResumeRequested

Triggered when player resumes from pause menu.

**Trigger Condition**: Resume button clicked or key pressed in `GameState::Paused`

**Event Data**:

```rust
#[derive(Event)]
pub struct ResumeRequested;
```

**Observers**:

- `on_resume_requested`: Transitions back to `GameState::Playing`

## Observer Registration

All observers registered in their respective plugins:

```rust
// In main.rs or plugin
app.add_observer(on_paddle_ball_hit)
   .add_observer(on_wall_hit)
   .add_observer(on_brick_hit)
   .add_observer(on_level_complete)
   .add_observer(on_life_lost)
   .add_observer(on_game_over)
   .add_observer(on_pause_requested)
   .add_observer(on_resume_requested);
```

## Event Flow Diagram

```text
[Ball-Paddle Collision] → BallHit → Apply Steering Impulse
[Paddle-Wall Collision] → WallHit → Bounce + Screen Shake
[Ball-Brick Collision] → BrickHit → Damage/Destroy Brick
                                           ↓
                          (bricks_remaining == 0?)
                                           ↓
                                    LevelComplete → Next Level

[Ball Below Boundary] → LifeLost → Despawn Ball
                                      ↓
                                (lives == 0?)
                                      ↓
                                  GameOver → Show Stats

[Escape Key] → PauseRequested → Change to Paused State
[Resume Input] → ResumeRequested → Change to Playing State
```

## Next Steps

With events defined, proceed to:

1. Write quickstart.md for build/run/test workflows
2. Update agent context with event patterns
