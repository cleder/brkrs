use brkrs::physics_config::{BallPhysicsConfig, BrickPhysicsConfig, PaddlePhysicsConfig};

#[test]
fn ball_physics_config_edge_cases() {
    // Zero values
    let zero = BallPhysicsConfig {
        restitution: 0.0,
        friction: 0.0,
        linear_damping: 0.0,
        angular_damping: 0.0,
    };
    assert!(zero.validate().is_ok());

    // Maximum recommended values
    let max = BallPhysicsConfig {
        restitution: 2.0,
        friction: 2.0,
        linear_damping: 10.0,
        angular_damping: 10.0,
    };
    assert!(max.validate().is_ok());

    // Out-of-bounds
    let invalid = BallPhysicsConfig {
        restitution: -1.0,
        friction: 3.0,
        linear_damping: -0.1,
        angular_damping: 20.0,
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn paddle_physics_config_edge_cases() {
    let zero = PaddlePhysicsConfig {
        restitution: 0.0,
        friction: 0.0,
        linear_damping: 0.0,
        angular_damping: 0.0,
    };
    assert!(zero.validate().is_ok());

    let max = PaddlePhysicsConfig {
        restitution: 2.0,
        friction: 2.0,
        linear_damping: 10.0,
        angular_damping: 10.0,
    };
    assert!(max.validate().is_ok());

    let invalid = PaddlePhysicsConfig {
        restitution: -1.0,
        friction: 3.0,
        linear_damping: -0.1,
        angular_damping: 20.0,
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn brick_physics_config_edge_cases() {
    let zero = BrickPhysicsConfig {
        restitution: 0.0,
        friction: 0.0,
    };
    assert!(zero.validate().is_ok());

    let max = BrickPhysicsConfig {
        restitution: 2.0,
        friction: 2.0,
    };
    assert!(max.validate().is_ok());

    let invalid = BrickPhysicsConfig {
        restitution: -1.0,
        friction: 3.0,
    };
    assert!(invalid.validate().is_err());
}
