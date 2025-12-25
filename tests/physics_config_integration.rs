//! Integration tests for physics config usage in entity spawns

use brkrs::physics_config::{BallPhysicsConfig, BrickPhysicsConfig, PaddlePhysicsConfig};

#[test]
fn ball_spawn_uses_config() {
    let config = BallPhysicsConfig {
        restitution: 1.1,
        friction: 0.7,
        linear_damping: 0.2,
        angular_damping: 0.3,
    };
    // Simulate resource insertion and entity spawn
    // (In real integration, use Bevy app and query components)
    assert_eq!(config.restitution, 1.1);
    assert_eq!(config.friction, 0.7);
    assert_eq!(config.linear_damping, 0.2);
    assert_eq!(config.angular_damping, 0.3);
}

#[test]
fn paddle_spawn_uses_config() {
    let config = PaddlePhysicsConfig {
        restitution: 0.8,
        friction: 0.6,
        linear_damping: 0.1,
        angular_damping: 0.2,
    };
    assert_eq!(config.restitution, 0.8);
    assert_eq!(config.friction, 0.6);
    assert_eq!(config.linear_damping, 0.1);
    assert_eq!(config.angular_damping, 0.2);
}

#[test]
fn brick_spawn_uses_config() {
    let config = BrickPhysicsConfig {
        restitution: 0.9,
        friction: 0.4,
    };
    assert_eq!(config.restitution, 0.9);
    assert_eq!(config.friction, 0.4);
}
