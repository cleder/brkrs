//! Centralized physics configuration for balls, paddles, and bricks.
//!
//! # Usage
//! These configs are registered as Bevy resources and injected into spawn systems for balls, paddles, and bricks.
//! All gameplay-relevant fields must be explicitly listed. Extension fields are allowed but must be justified and documented.
//! Config is source-only and not hot-reloadable.
//!
//! # Validation
//! Use the `validate()` method on each config to ensure all values are finite, non-negative, and within reasonable bounds.
//! This prevents configuration drift and runtime errors due to invalid physics parameters.
//!
//! # Extension
//! If you add new fields, document the rationale and update validation logic accordingly.

use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct BallPhysicsConfig {
    pub restitution: f32,
    pub friction: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

impl BallPhysicsConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !self.restitution.is_finite() || self.restitution < 0.0 || self.restitution > 2.0 {
            return Err(format!(
                "Ball restitution out of bounds: {}",
                self.restitution
            ));
        }
        if !self.friction.is_finite() || self.friction < 0.0 || self.friction > 2.0 {
            return Err(format!("Ball friction out of bounds: {}", self.friction));
        }
        if !self.linear_damping.is_finite()
            || self.linear_damping < 0.0
            || self.linear_damping > 10.0
        {
            return Err(format!(
                "Ball linear_damping out of bounds: {}",
                self.linear_damping
            ));
        }
        if !self.angular_damping.is_finite()
            || self.angular_damping < 0.0
            || self.angular_damping > 10.0
        {
            return Err(format!(
                "Ball angular_damping out of bounds: {}",
                self.angular_damping
            ));
        }
        Ok(())
    }
}

impl Default for BallPhysicsConfig {
    fn default() -> Self {
        Self {
            restitution: 0.9,
            friction: 2.0,
            linear_damping: 0.5,
            angular_damping: 0.5,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct PaddlePhysicsConfig {
    pub restitution: f32,
    pub friction: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

impl PaddlePhysicsConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !self.restitution.is_finite() || self.restitution < 0.0 || self.restitution > 2.0 {
            return Err(format!(
                "Paddle restitution out of bounds: {}",
                self.restitution
            ));
        }
        if !self.friction.is_finite() || self.friction < 0.0 || self.friction > 2.0 {
            return Err(format!("Paddle friction out of bounds: {}", self.friction));
        }
        if !self.linear_damping.is_finite()
            || self.linear_damping < 0.0
            || self.linear_damping > 10.0
        {
            return Err(format!(
                "Paddle linear_damping out of bounds: {}",
                self.linear_damping
            ));
        }
        if !self.angular_damping.is_finite()
            || self.angular_damping < 0.0
            || self.angular_damping > 10.0
        {
            return Err(format!(
                "Paddle angular_damping out of bounds: {}",
                self.angular_damping
            ));
        }
        Ok(())
    }
}

impl Default for PaddlePhysicsConfig {
    fn default() -> Self {
        Self {
            restitution: 0.7,
            friction: 2.0,
            linear_damping: 0.5,
            angular_damping: 0.5,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct BrickPhysicsConfig {
    pub restitution: f32,
    pub friction: f32,
}

impl BrickPhysicsConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !self.restitution.is_finite() || self.restitution < 0.0 || self.restitution > 2.0 {
            return Err(format!(
                "Brick restitution out of bounds: {}",
                self.restitution
            ));
        }
        if !self.friction.is_finite() || self.friction < 0.0 || self.friction > 2.0 {
            return Err(format!("Brick friction out of bounds: {}", self.friction));
        }
        Ok(())
    }
}

impl Default for BrickPhysicsConfig {
    fn default() -> Self {
        Self {
            restitution: 1.0,
            friction: 1.0,
        }
    }
}
