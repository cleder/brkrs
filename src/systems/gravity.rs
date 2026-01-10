/// Gravity switching bricks system for dynamic gravity mechanics.
///
/// This module handles gravity changes triggered by gravity brick destruction
/// (indices 21-25). When a gravity brick is destroyed, the ball's gravity is
/// immediately updated via the GravityChanged message system.
///
/// **Core Components**:
/// - `GravityChanged` message: Communicates gravity updates from brick destruction to physics
/// - `GravityConfiguration` resource: Tracks current and level-default gravity
/// - `GravityBrick` component: Marks brick entities and stores gravity output
///
/// **System Flow**:
/// 1. `gravity_configuration_loader_system` (Startup): Load level default gravity
/// 2. `brick_destruction_gravity_handler` (Update): Detect gravity brick destruction, send messages
/// 3. `gravity_application_system` (PhysicsUpdate): Read messages, update GravityConfiguration
/// 4. Physics system applies gravity to ball's rigid body
/// 5. `gravity_reset_on_life_loss_system` (PostUpdate): Reset gravity to level default on ball loss
use bevy::prelude::*;

/// Message indicating that the world's gravity has changed due to brick destruction.
///
/// This message is buffered and read by the gravity application system to update
/// the current gravity affecting the ball's physics. Multiple destructions in the
/// same frame result in multiple messages; the last message processed takes effect.
///
/// **Examples**:
/// - Zero Gravity (brick 21): `GravityChanged { gravity: Vec3::ZERO }`
/// - Earth Gravity (brick 23): `GravityChanged { gravity: Vec3::new(0.0, 10.0, 0.0) }`
/// - Queer Gravity (brick 25): Random value generated at destruction time
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct GravityChanged {
    /// New gravity vector to apply to the ball's physics.
    ///
    /// Format: Vec3 with components:
    /// - **X**: Horizontal gravity (-2.0 to +15.0 for Queer Gravity, 0.0 for others)
    /// - **Y**: Vertical gravity (0.0 to 20.0, positive = upward pull)
    /// - **Z**: Horizontal gravity (-5.0 to +5.0 for Queer Gravity, 0.0 for others)
    ///
    /// Coordinate system: Bevy standard (Y = up, X = right, Z = back)
    pub gravity: Vec3,
}

impl GravityChanged {
    /// Create a gravity change message with the specified gravity vector.
    ///
    /// # Arguments
    ///
    /// * `gravity` - The new gravity vector to apply (must be finite and in range [-30, +30])
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let msg = GravityChanged::new(Vec3::new(0.0, 10.0, 0.0));  // Earth gravity
    /// ```
    pub fn new(gravity: Vec3) -> Self {
        Self { gravity }
    }

    /// Validate that the gravity vector is within specification ranges.
    ///
    /// Returns `Ok(())` if valid, `Err(String)` with description if invalid.
    ///
    /// **Validation Rules**:
    /// - All components must be finite (no NaN or Inf)
    /// - All components must be in range [-30.0, +30.0] (physics realism bounds)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Any component is NaN or Inf
    /// - Any component is outside [-30.0, +30.0]
    pub fn validate(&self) -> Result<(), String> {
        if !self.gravity.is_finite() {
            return Err("Gravity vector contains NaN or Inf".to_string());
        }

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
