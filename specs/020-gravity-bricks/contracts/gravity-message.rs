// contracts/gravity-message.rs
// GravityChanged Message Contract Definition
// Feature: 020-gravity-bricks
// Date: 2026-01-10

use bevy::prelude::*;

/// Message indicating that the world's gravity has changed due to brick destruction.
///
/// This message is sent when a gravity brick (index 21-25) is destroyed.
/// It communicates the new gravity vector to apply to the ball's physics.
///
/// **Message Flow**:
/// 1. `brick_destruction_system` detects gravity brick destruction
/// 2. Reads `GravityBrick` component to get output gravity
/// 3. Writes `GravityChanged` message via `MessageWriter`
/// 4. `gravity_application_system` reads message via `MessageReader`
/// 5. Updates `GravityConfiguration::current`
/// 6. Next physics frame applies new gravity to ball's rigid body
///
/// **Frequency**: One message per gravity brick destruction (0-multiple per frame)
/// **Queue Depth**: Buffered; processed in order
/// **Ordering**: Deterministic; last message "wins" if multiple received same frame
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct GravityChanged {
    /// New gravity vector to apply to the ball's physics
    ///
    /// Format: Vec3 with components:
    /// - X: Horizontal gravity (-2.0 to +15.0 for Queer Gravity, 0.0 for others)
    /// - Y: Vertical gravity (0.0 to 20.0, negative for downward)
    /// - Z: Horizontal gravity (-5.0 to +5.0 for Queer Gravity, 0.0 for others)
    ///
    /// Examples:
    /// - Zero Gravity (21): Vec3::ZERO = (0.0, 0.0, 0.0)
    /// - 2G Moon Gravity (22): (0.0, 2.0, 0.0)
    /// - 10G Earth Gravity (23): (0.0, 10.0, 0.0)
    /// - 20G High Gravity (24): (0.0, 20.0, 0.0)
    /// - Queer Gravity (25): Random (X: [-2.0, +15.0], Y: 0.0, Z: [-5.0, +5.0])
    pub gravity: Vec3,
}

impl GravityChanged {
    /// Create a gravity change message with the specified gravity vector.
    pub fn new(gravity: Vec3) -> Self {
        Self { gravity }
    }

    /// Validate that the gravity vector is within specification ranges.
    /// Returns `Ok(())` if valid, `Err(String)` with description if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if !self.gravity.is_finite() {
            return Err("Gravity vector contains NaN or Inf".to_string());
        }

        // Allow reasonable physics range
        let valid_range = -30.0..=30.0;
        if !valid_range.contains(&self.gravity.x)
            || !valid_range.contains(&self.gravity.y)
            || !valid_range.contains(&self.gravity.z)
        {
            return Err(format!(
                "Gravity vector {:?} outside valid range [-30, +30]",
                self.gravity
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_changed_creation() {
        let msg = GravityChanged::new(Vec3::new(0.0, 10.0, 0.0));
        assert_eq!(msg.gravity, Vec3::new(0.0, 10.0, 0.0));
    }

    #[test]
    fn test_gravity_changed_zero_gravity() {
        let msg = GravityChanged::new(Vec3::ZERO);
        assert_eq!(msg.gravity, Vec3::ZERO);
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn test_gravity_changed_earth_gravity() {
        let msg = GravityChanged::new(Vec3::new(0.0, 10.0, 0.0));
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn test_gravity_changed_queer_gravity() {
        let msg = GravityChanged::new(Vec3::new(5.0, 0.0, -2.0));
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn test_gravity_changed_invalid_nan() {
        let msg = GravityChanged::new(Vec3::new(f32::NAN, 10.0, 0.0));
        assert!(msg.validate().is_err());
    }

    #[test]
    fn test_gravity_changed_out_of_range() {
        let msg = GravityChanged::new(Vec3::new(50.0, 10.0, 0.0));
        assert!(msg.validate().is_err());
    }
}
