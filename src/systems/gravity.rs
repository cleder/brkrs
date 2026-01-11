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

/// Load gravity configuration from the current level definition.
///
/// Initializes `GravityConfiguration.level_default` and `GravityConfiguration.current` from
/// `LevelDefinition.default_gravity` if present; otherwise falls back to `Vec3::ZERO`.
/// Safe to run multiple times; it simply re-syncs the resource with the level metadata.
pub fn gravity_configuration_loader_system(
    current_level: Option<Res<crate::level_loader::CurrentLevel>>,
    mut gravity_cfg: ResMut<crate::GravityConfiguration>,
) {
    let Some(level) = current_level else {
        return;
    };

    let default = level.0.default_gravity.unwrap_or(Vec3::ZERO);

    gravity_cfg.level_default = default;
    gravity_cfg.current = default;
}

/// Detect gravity brick destruction and send GravityChanged messages.
///
/// When a brick with the `GravityBrick` component is despawned, this system
/// detects the removal and sends a `GravityChanged` message with the gravity
/// value from the brick's component.
///
/// **Approach**: Tracks GravityBrick components in a resource before they're
/// despawned, allowing us to send the correct gravity message when destruction occurs.
pub fn brick_destruction_gravity_handler(
    // Query for bricks with GravityBrick component
    gravity_bricks: Query<(Entity, &crate::GravityBrick), With<crate::Brick>>,
    // Query for bricks marked for despawn
    marked_for_despawn: Query<Entity, With<crate::MarkedForDespawn>>,
    mut gravity_writer: MessageWriter<GravityChanged>,
) {
    // Check if any gravity bricks are marked for despawn
    for (entity, gravity_brick) in gravity_bricks.iter() {
        if marked_for_despawn.contains(entity) {
            // This gravity brick is about to be destroyed
            let msg = GravityChanged::new(gravity_brick.gravity);

            // Validate before sending (defensive programming)
            match msg.validate() {
                Ok(()) => {
                    gravity_writer.write(msg);
                    debug!(
                        "Gravity brick detected for destruction (entity: {:?}, index: {}, gravity: {:?})",
                        entity, gravity_brick.index, gravity_brick.gravity
                    );
                }
                Err(e) => {
                    warn!(
                        "Invalid gravity in brick {:?} (index {}): {}",
                        entity, gravity_brick.index, e
                    );
                }
            }
        }
    }
}

/// Apply gravity changes from messages to the GravityConfiguration resource.
///
/// This system reads `GravityChanged` messages and updates the current gravity
/// in the `GravityConfiguration` resource. Multiple messages in the same frame
/// will be processed in order, with the last message taking effect.
///
/// **Scheduling**: Should run in `PhysicsUpdate` schedule after messages are
/// sent but before physics step applies gravity to entities.
pub fn gravity_application_system(
    mut gravity_reader: MessageReader<GravityChanged>,
    mut gravity_cfg: ResMut<crate::GravityConfiguration>,
) {
    for msg in gravity_reader.read() {
        // Validate gravity before applying (defensive programming)
        if msg.validate().is_ok() {
            gravity_cfg.current = msg.gravity;
            debug!("Gravity updated to: {:?}", msg.gravity);
        } else {
            warn!("Invalid gravity message received: {:?}", msg);
        }
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
