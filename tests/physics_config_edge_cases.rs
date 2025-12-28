use brkrs::physics_config::{BallPhysicsConfig, BrickPhysicsConfig, PaddlePhysicsConfig};

#[test]
fn ball_physics_config_edge_cases() {
    // NaN and infinity
    use std::f32;
    let nan = BallPhysicsConfig {
        restitution: f32::NAN,
        friction: 0.5,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(nan.validate().is_err());
    let inf = BallPhysicsConfig {
        restitution: f32::INFINITY,
        friction: 0.5,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(inf.validate().is_err());
    let neg_inf = BallPhysicsConfig {
        restitution: f32::NEG_INFINITY,
        friction: 0.5,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(neg_inf.validate().is_err());
    // Repeat for other fields
    let nan_friction = BallPhysicsConfig {
        restitution: 1.0,
        friction: f32::NAN,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(nan_friction.validate().is_err());
    let inf_damping = BallPhysicsConfig {
        restitution: 1.0,
        friction: 0.5,
        linear_damping: f32::INFINITY,
        angular_damping: 0.1,
    };
    assert!(inf_damping.validate().is_err());
    let nan_angular = BallPhysicsConfig {
        restitution: 1.0,
        friction: 0.5,
        linear_damping: 0.1,
        angular_damping: f32::NAN,
    };
    assert!(nan_angular.validate().is_err());
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
    use std::f32;
    let nan = PaddlePhysicsConfig {
        restitution: f32::NAN,
        friction: 0.5,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(nan.validate().is_err());
    let inf = PaddlePhysicsConfig {
        restitution: f32::INFINITY,
        friction: 0.5,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(inf.validate().is_err());
    let neg_inf = PaddlePhysicsConfig {
        restitution: f32::NEG_INFINITY,
        friction: 0.5,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(neg_inf.validate().is_err());
    let nan_friction = PaddlePhysicsConfig {
        restitution: 1.0,
        friction: f32::NAN,
        linear_damping: 0.1,
        angular_damping: 0.1,
    };
    assert!(nan_friction.validate().is_err());
    let inf_damping = PaddlePhysicsConfig {
        restitution: 1.0,
        friction: 0.5,
        linear_damping: f32::INFINITY,
        angular_damping: 0.1,
    };
    assert!(inf_damping.validate().is_err());
    let nan_angular = PaddlePhysicsConfig {
        restitution: 1.0,
        friction: 0.5,
        linear_damping: 0.1,
        angular_damping: f32::NAN,
    };
    assert!(nan_angular.validate().is_err());
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
    use std::f32;
    let nan = BrickPhysicsConfig {
        restitution: f32::NAN,
        friction: 0.5,
    };
    assert!(nan.validate().is_err());
    let inf = BrickPhysicsConfig {
        restitution: f32::INFINITY,
        friction: 0.5,
    };
    assert!(inf.validate().is_err());
    let neg_inf = BrickPhysicsConfig {
        restitution: f32::NEG_INFINITY,
        friction: 0.5,
    };
    assert!(neg_inf.validate().is_err());
    let nan_friction = BrickPhysicsConfig {
        restitution: 1.0,
        friction: f32::NAN,
    };
    assert!(nan_friction.validate().is_err());
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
