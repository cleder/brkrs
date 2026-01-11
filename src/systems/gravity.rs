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
    /// let msg = GravityChanged::new(Vec3::new(10.0, 0.0, 0.0));  // Earth gravity
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
/// `LevelDefinition.gravity` if present; otherwise falls back to `Vec3::ZERO`.
/// Safe to run multiple times; it simply re-syncs the resource with the level metadata.
pub fn gravity_configuration_loader_system(
    current_level: Option<Res<crate::level_loader::CurrentLevel>>,
    mut gravity_cfg: ResMut<crate::GravityConfiguration>,
) {
    let Some(level) = current_level else {
        return;
    };

    let base = level
        .0
        .gravity
        .map(|(x, y, z)| Vec3::new(x, y, z))
        .unwrap_or(Vec3::ZERO);

    gravity_cfg.level_default = base;
    gravity_cfg.current = base;
}

/// Detect gravity brick destruction by listening to BrickDestroyed messages.
///
/// Reads `BrickDestroyed` messages and checks if the destroyed brick was a gravity brick
/// (indices 21-25). If so, sends a `GravityChanged` message with the appropriate gravity value.
///
/// For brick index 25 (Queer Gravity), generates random gravity within specified ranges:
/// - X ∈ [-2.0, +15.0]
/// - Y = 0.0 (always, no randomization)
/// - Z ∈ [-5.0, +5.0]
///
/// **Approach**: Uses the `brick_type` field from the `BrickDestroyed` message (sent by
/// `mark_brick_on_ball_collision`) to determine gravity. This avoids querying the brick entity,
/// which may already be despawned when the message is processed.
pub fn brick_destruction_gravity_handler(
    // Read BrickDestroyed messages to detect when gravity bricks are destroyed
    mut destroyed_bricks: MessageReader<crate::signals::BrickDestroyed>,
    mut gravity_writer: MessageWriter<GravityChanged>,
) {
    use rand::Rng;

    for destroyed in destroyed_bricks.read() {
        // Map brick type to gravity; we avoid querying the entity because it may already be despawned
        let gravity = match destroyed.brick_type {
            21 => Some(Vec3::ZERO),
            22 => Some(Vec3::new(2.0, 0.0, 0.0)),
            23 => Some(Vec3::new(10.0, 0.0, 0.0)),
            24 => Some(Vec3::new(20.0, 0.0, 0.0)),
            25 => {
                // Queer Gravity: Generate random gravity
                let mut rng = rand::rng();
                let x = rng.random_range(-2.0..=15.0);
                let z = rng.random_range(-5.0..=5.0);
                Some(Vec3::new(x, 0.0, z))
            }
            _ => None,
        };

        let Some(gravity) = gravity else {
            continue;
        };

        let msg = GravityChanged::new(gravity);

        // Validate before sending (defensive programming)
        match msg.validate() {
            Ok(()) => {
                gravity_writer.write(msg);
                debug!(
                    "Gravity brick destroyed (entity: {:?}, brick_type: {}, gravity: {:?})",
                    destroyed.brick_entity, destroyed.brick_type, gravity
                );
            }
            Err(e) => {
                warn!(
                    "Invalid gravity for brick {:?} (brick_type {}): {}",
                    destroyed.brick_entity, destroyed.brick_type, e
                );
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
            let old_gravity = gravity_cfg.current;
            gravity_cfg.current = msg.gravity;
            info!(
                "Gravity configuration updated: {:?} -> {:?}",
                old_gravity, msg.gravity
            );
        } else {
            warn!("Invalid gravity message received: {:?}", msg);
        }
    }
}

/// Apply gravity configuration to Rapier physics engine.
///
/// This system applies the current gravity from `GravityConfiguration` to the
/// Rapier physics engine's RapierConfiguration. This must run after gravity
/// is updated (from both level loading and brick destruction) and before
/// the physics simulation step.
///
/// **Effect**: The gravity setting affects all physics bodies, but gravity bricks
/// are designed to be used in ways that effectively apply gravity to the ball only
/// (since the paddle and other entities don't have gravity applied in typical usage).
pub fn apply_gravity_to_physics(
    gravity_cfg: Res<crate::GravityConfiguration>,
    mut rapier_config: Query<&mut bevy_rapier3d::prelude::RapierConfiguration>,
) {
    // Apply the current gravity configuration to the physics engine
    // This runs every frame, updating the physics engine's gravity setting
    if let Ok(mut config) = rapier_config.single_mut() {
        // Only update if gravity has changed to avoid unnecessary writes
        if config.gravity != gravity_cfg.current {
            info!(
                "Applying gravity change: {:?} -> {:?}",
                config.gravity, gravity_cfg.current
            );
            config.gravity = gravity_cfg.current;
        }
    } else {
        warn!("Failed to query RapierConfiguration - gravity not applied!");
    }
}

/// System that resets gravity to level default when a ball is lost.
///
/// **Execution**: Update (RespawnSystems::Detect)
/// **Dependencies**: Listens to `LifeLostEvent` from respawn system
/// **Behavior**: On ball loss, resets `GravityConfiguration::current` to `level_default`
///
/// This ensures each new ball spawns with the level's original gravity,
/// providing a consistent gameplay reset mechanic.
///
/// # Bevy 0.17 Compliance
/// - Uses `MessageReader` for ball loss events (not Observers)
/// - No panicking queries or unwraps
pub fn gravity_reset_on_life_loss_system(
    mut gravity_cfg: ResMut<crate::GravityConfiguration>,
    life_lost_events: Option<MessageReader<crate::systems::respawn::LifeLostEvent>>,
) {
    let Some(mut events) = life_lost_events else {
        // Message system not available (startup/shutdown)
        return;
    };

    for event in events.read() {
        // Reset gravity to level default
        gravity_cfg.current = gravity_cfg.level_default;

        info!(
            ball = ?event.ball,
            cause = ?event.cause,
            reset_gravity = ?gravity_cfg.level_default,
            "Gravity reset to level default after ball loss"
        );
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
