use crate::signals::DirectionBrickEffect;
/// Direction brick effects system for applying velocity impulses.
///
/// This module implements the observer pattern for direction bricks (types 43-48, 52),
/// which apply instantaneous impulses to the ball when destroyed.
///
/// # Architecture
///
/// Direction brick effects use Bevy 0.17's observer pattern via `On<DirectionBrickEffect>`,
/// providing immediate (within-frame) impulses via rapier3d's `ExternalImpulse` component.
/// This contrasts with buffered message-based systems like scoring, which batch updates across
/// frame boundaries.
///
/// **Event Flow**:
/// 1. Ball collides with direction brick (collision system)
/// 2. Collision system emits `On<DirectionBrickEffect>` via `commands.trigger()`
/// 3. Observer system `apply_direction_brick_effects` reads trigger synchronously
/// 4. System modifies ball's `ExternalImpulse` component (rapier3d)
/// 5. Physics integration applies impulse and picks up velocity change in same frame
///
/// **Multi-Frame Persistence**:
/// Velocity changes persist until modified by:
/// - Subsequent direction brick collisions
/// - Gravity brick effects
/// - Physics forces (air resistance, gravity)
/// - Wall bounces (handled by rapier3d)
///
/// # Coordinate System
///
/// **Horizontal Plane**: XZ (gameplay surface)
/// - X-axis: Left/Right from ball's perspective (negative left, positive right)
/// - Z-axis: Forward/Backward (positive toward bricks, negative toward paddle)
/// - **Note**: Gameplay convention differs from Bevy's `Transform::forward()` (-Z direction).
///   Direction bricks use XZ directly (Constitution Principle VIII).
///
/// **Vertical**: Y-axis
/// - Positive Y: Up (away from gravity)
/// - Negative Y: Gravity pull (down)
///
/// **Z-Axis Invariant**: No direction brick modifies Z-component of velocity.
///
/// # Impulse Magnitudes
///
/// **Cardinal Directions (Bricks 43-48)**: 5.0 units/sec additive impulse per brick type
/// - Brick 43 (Left): `velocity.x -= 5.0`
/// - Brick 44 (Right): `velocity.x += 5.0`
/// - Brick 45 (Up): `velocity.y += 5.0`
/// - Brick 46 (Down): `velocity.y -= 5.0`
/// - Brick 47 (Forward): `velocity.z += 5.0`
/// - Brick 48 (Backward): `velocity.z -= 5.0`
///
/// **Random Direction (Brick 52)**: Magnitude 5.0-15.0 units/sec, direction 0.0..2π radians
/// - Replaces both X and Y velocities (NOT additive)
/// - Preserves Z velocity
/// - Uses seeded RNG for deterministic behavior in tests
///
/// # Rapid Succession Handling
///
/// When multiple direction bricks are hit in the same `app.update()` cycle
/// (e.g., bricks 45 and 46 both trigger during same collision frame):
/// - All `Trigger<DirectionBrickEffect>` events fire in same cycle
/// - Observer system processes them sequentially
/// - Velocities compound additive (except brick 52 which replaces)
/// - Final velocity is result of all impulses in order of firing
///
/// # Physics Integration
///
/// The `ExternalImpulse` component from bevy_rapier3d is modified by the observer.
/// Physics integration occurs in the `PhysicsSet::Integration` schedule:
/// 1. Collision detection (this system reads collision results)
/// 2. **Direction brick effects applied HERE** (observer fires, impulse set on component)
/// 3. Physics integration (rapier picks up `ExternalImpulse`, applies to velocity, clears impulse)
///
/// This scheduling ensures direction brick impulses are integrated before
/// the physics simulation step, preventing one-frame lag artifacts.
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// =============================================================================
// DIRECTION BRICK CONSTANTS
// =============================================================================

/// Brick type ID for Forward direction brick (toward far wall, +X).
pub const BRICK_TYPE_DIRECTION_FORWARD: u8 = 43;
/// Brick type ID for Left direction brick (+Z).
pub const BRICK_TYPE_DIRECTION_LEFT: u8 = 44;
/// Brick type ID for Right direction brick (-Z).
pub const BRICK_TYPE_DIRECTION_RIGHT: u8 = 45;
/// Brick type ID for Backward direction brick (toward paddle, -X).
pub const BRICK_TYPE_DIRECTION_BACKWARD: u8 = 46;
/// Brick type ID for Backward-Right diagonal direction brick (-X, -Z).
pub const BRICK_TYPE_DIRECTION_BACKWARD_RIGHT: u8 = 47;
/// Brick type ID for Backward-Left diagonal direction brick (-X, +Z).
pub const BRICK_TYPE_DIRECTION_BACKWARD_LEFT: u8 = 48;
/// Brick type ID for Random direction brick.
pub const BRICK_TYPE_DIRECTION_RANDOM: u8 = 52;

/// Velocity impulse magnitude for cardinal direction bricks (units/sec).
/// Applied additively to the ball's velocity.
pub const IMPULSE_MAGNITUDE_CARDINAL: f32 = 5.0;

/// Minimum velocity magnitude for random direction brick (units/sec).
pub const IMPULSE_MAGNITUDE_RANDOM_MIN: f32 = 5.0;
/// Maximum velocity magnitude for random direction brick (units/sec).
pub const IMPULSE_MAGNITUDE_RANDOM_MAX: f32 = 15.0;

// =============================================================================
// OBSERVER SYSTEM
/// Observer function: Apply direction brick effects to ball via impulse.
///
/// **Trigger**: `On<DirectionBrickEffect>` from collision system
/// **Schedule**: PhysicsSet::Integration (after collision detection, before physics step)
///
/// **Behavior**:
/// - For cardinal direction bricks (43-48): Apply impulse in cardinal direction
/// - For random direction brick (52): Apply impulse with random magnitude/direction
/// - All bricks: Preserve Z impulse component (no Z-axis impulses)
/// - Impulses compound with existing velocity and forces
///
/// # Arguments
///
/// * `trigger` - Trigger event containing ball entity, brick type, position, and impulse vector
/// * `mut query` - Mutable query over ball's ExternalImpulse component
///
/// # Error Handling
///
/// If the ball entity is not found or missing ExternalImpulse component, the error is logged
/// and the observer continues without applying the impulse. This is acceptable because:
/// - The ball may have despawned between collision detection and observer firing (rare)
/// - The impulse is non-critical for gameplay (physics will handle without it)
///
/// # Examples
///
/// ```ignore
/// // Registered as observer:
/// app.add_observer(apply_direction_brick_effects);
/// ```
pub fn apply_direction_brick_effects(
    trigger: On<DirectionBrickEffect>,
    mut query: Query<&mut ExternalImpulse>,
) {
    let effect = trigger.event();

    // Create a tracing span for this direction brick effect
    let span = debug_span!(
        "apply_direction_brick_effect",
        brick_type = effect.brick_type,
        brick_position = ?effect.brick_position,
        velocity_before = ?effect.velocity_before,
        impulse = ?effect.impulse,
    );
    let _guard = span.enter();

    match query.get_mut(effect.ball_entity) {
        Ok(mut external_impulse) => {
            // Accumulate impulses instead of overwriting to preserve collision response
            external_impulse.impulse += effect.impulse;

            debug!(
                ball_entity = ?effect.ball_entity,
                impulse_applied = ?effect.impulse,
                "Direction brick effect applied to ball"
            );
        }
        Err(_) => {
            // Ball entity not found or missing ExternalImpulse
            // This can occur if the ball despawned between trigger and observer execution
            warn!(
                ball = ?effect.ball_entity,
                brick_type = effect.brick_type,
                "Ball entity not found or missing ExternalImpulse component when applying direction brick effect"
            );
        }
    }
}
