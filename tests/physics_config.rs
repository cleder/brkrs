#[test]
fn ball_physics_config_default_is_valid() {
    assert!(BallPhysicsConfig::default().validate().is_ok());
}

#[test]
fn paddle_physics_config_default_is_valid() {
    assert!(PaddlePhysicsConfig::default().validate().is_ok());
}

#[test]
fn brick_physics_config_default_is_valid() {
    assert!(BrickPhysicsConfig::default().validate().is_ok());
}
/// Unit tests for BallPhysicsConfig, PaddlePhysicsConfig, and BrickPhysicsConfig validation
use brkrs::physics_config::{BallPhysicsConfig, BrickPhysicsConfig, PaddlePhysicsConfig};

#[test]
fn ball_physics_config_valid() {
    let config = BallPhysicsConfig {
        restitution: 1.0,
        friction: 0.5,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(config.validate().is_ok());
}

#[test]
fn ball_physics_config_invalid() {
    let config = BallPhysicsConfig {
        restitution: -1.0,
        friction: 0.5,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(config.validate().is_err());
}

#[test]
fn paddle_physics_config_valid() {
    let config = PaddlePhysicsConfig {
        restitution: 1.0,
        friction: 0.5,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(config.validate().is_ok());
}

#[test]
fn paddle_physics_config_invalid() {
    let config = PaddlePhysicsConfig {
        restitution: 1.0,
        friction: -0.5,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(config.validate().is_err());
}

#[test]
fn brick_physics_config_valid() {
    let config = BrickPhysicsConfig {
        restitution: 1.0,
        friction: 0.5,
    };
    assert!(config.validate().is_ok());
}

#[test]
fn brick_physics_config_invalid() {
    let config = BrickPhysicsConfig {
        restitution: 1.0,
        friction: -0.5,
    };
    assert!(config.validate().is_err());
}
