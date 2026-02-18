//! Ball spawn bricks system (Red 1/2/3 at indices 37-39).
//!
//! Provides message-driven ball spawn/despawn behavior when brick destruction
//! messages arrive. Designed to be ECS-safe, fallible, and message-oriented.

use std::collections::HashMap;

use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::physics_config::BallPhysicsConfig;
use crate::signals::BrickDestroyed;
use crate::{Ball, BallTypeId, BALL_RADIUS};

/// Plugin registering ball spawn/despawn systems for Red 1/2/3 bricks.
#[derive(Default)]
pub struct BallSpawnBricksPlugin;

impl Plugin for BallSpawnBricksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BrickSpawnConfig>();
        app.add_systems(
            Update,
            ball_spawn_system
                .after(crate::despawn_marked_entities)
                .before(crate::systems::respawn::RespawnSystems::Detect),
        );
    }
}

/// Configuration mapping for ball spawn brick behavior.
#[derive(Resource, Debug, Clone)]
pub struct BrickSpawnConfig {
    /// Map brick index → spawn rule
    pub brick_spawn_rules: HashMap<u8, BrickSpawnRule>,
}

impl Default for BrickSpawnConfig {
    fn default() -> Self {
        let rules = [
            (
                37,
                BrickSpawnRule {
                    spawn_count: 0,
                    velocity_modifier: VelocityModifier::DespawnAll,
                    name: "Red 1 (Despawn)",
                },
            ),
            (
                38,
                BrickSpawnRule {
                    spawn_count: 1,
                    velocity_modifier: VelocityModifier::Inverse,
                    name: "Red 2 (Spawn 1)",
                },
            ),
            (
                39,
                BrickSpawnRule {
                    spawn_count: 2,
                    velocity_modifier: VelocityModifier::YShaped {
                        angle_degrees: 37.5,
                    },
                    name: "Red 3 (Spawn 2)",
                },
            ),
        ]
        .into_iter()
        .collect();

        Self {
            brick_spawn_rules: rules,
        }
    }
}

/// Spawn behavior for a single brick index.
#[derive(Debug, Clone)]
pub struct BrickSpawnRule {
    /// Number of balls to spawn (0 for Red 1/despawn)
    pub spawn_count: u8,
    /// Velocity modification rule
    pub velocity_modifier: VelocityModifier,
    /// Human-readable name for logging/debugging
    pub name: &'static str,
}

/// Velocity transformation rules for spawned balls.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VelocityModifier {
    /// No spawning (Red 1: despawn all except triggering ball)
    DespawnAll,
    /// Negate velocity vector (Red 2: inverse direction)
    Inverse,
    /// Spread in Y-shaped pattern (Red 3: ±angle degrees left/right)
    YShaped { angle_degrees: f32 },
}

#[derive(Debug, Clone)]
struct BallVisuals {
    mesh: Option<Mesh3d>,
    material: Option<MeshMaterial3d<StandardMaterial>>,
    ball_type: BallTypeId,
}

impl BallVisuals {
    fn from_source(
        mesh: Option<&Mesh3d>,
        material: Option<&MeshMaterial3d<StandardMaterial>>,
        ball_type: Option<&BallTypeId>,
    ) -> Self {
        Self {
            mesh: mesh.cloned(),
            material: material.cloned(),
            ball_type: ball_type.copied().unwrap_or(BallTypeId(0)),
        }
    }
}

/// Spawn/despawn balls based on BrickDestroyed messages for indices 37-39.
///
/// Reads buffered `BrickDestroyed` messages and applies spawn rules:
/// - Red 1 (37): Despawn all balls except triggering ball
/// - Red 2 (38): Spawn one ball with inverse velocity
/// - Red 3 (39): Spawn two balls in Y-shaped spread (±37.5°)
pub fn ball_spawn_system(
    mut commands: Commands,
    config: Res<BrickSpawnConfig>,
    physics: Res<BallPhysicsConfig>,
    mut reader: MessageReader<BrickDestroyed>,
    ball_entities: Query<Entity, With<Ball>>,
    ball_sources: Query<
        (
            &Velocity,
            Option<&Mesh3d>,
            Option<&MeshMaterial3d<StandardMaterial>>,
            Option<&BallTypeId>,
        ),
        With<Ball>,
    >,
) {
    let mut spawned_this_frame: Vec<Entity> = Vec::new();
    for event in reader.read() {
        let Some(rule) = config.brick_spawn_rules.get(&event.brick_type) else {
            continue;
        };
        let Some(triggering_ball) = event.destroyed_by else {
            warn!(
                "BrickDestroyed event missing triggering ball for brick type {}",
                event.brick_type
            );
            continue;
        };

        match rule.velocity_modifier {
            VelocityModifier::DespawnAll => {
                // Verify triggering ball exists before despawning others
                if ball_entities.get(triggering_ball).is_err() {
                    continue;
                }
                // Despawn all existing balls except triggering
                for ball in ball_entities.iter() {
                    if ball != triggering_ball {
                        commands.entity(ball).despawn();
                    }
                }
                // Despawn any balls spawned in this same frame, except triggering
                for spawned in spawned_this_frame.iter() {
                    if *spawned != triggering_ball {
                        commands.entity(*spawned).despawn();
                    }
                }
            }
            VelocityModifier::Inverse => {
                // Verify ball still exists before accessing it
                if ball_entities.get(triggering_ball).is_err() {
                    warn!(
                        "Triggering ball {:?} not found for Red 2 (inverse) spawn (entity despawned or doesn't exist)",
                        triggering_ball
                    );
                    continue;
                }
                let Ok((velocity, mesh, material, ball_type)) = ball_sources.get(triggering_ball)
                else {
                    warn!(
                        "Triggering ball {:?} not found for Red 2 (inverse) spawn (missing required components)",
                        triggering_ball
                    );
                    continue;
                };
                let visuals = BallVisuals::from_source(mesh, material, ball_type);
                let spawned = spawn_ball(
                    &mut commands,
                    &physics,
                    event.brick_position,
                    -velocity.linvel,
                    visuals,
                );
                spawned_this_frame.push(spawned);
            }
            VelocityModifier::YShaped { angle_degrees } => {
                // Verify ball still exists before accessing it
                if ball_entities.get(triggering_ball).is_err() {
                    warn!(
                        "Triggering ball {:?} not found for Red 3 (Y-shaped) spawn (entity despawned or doesn't exist)",
                        triggering_ball
                    );
                    continue;
                }
                let Ok((velocity, mesh, material, ball_type)) = ball_sources.get(triggering_ball)
                else {
                    warn!(
                        "Triggering ball {:?} not found for Red 3 (Y-shaped) spawn (missing required components)",
                        triggering_ball
                    );
                    continue;
                };
                let visuals = BallVisuals::from_source(mesh, material, ball_type);
                let (left, right) = y_shaped_velocity(velocity.linvel, angle_degrees);
                let left_id = spawn_ball(
                    &mut commands,
                    &physics,
                    event.brick_position,
                    left,
                    visuals.clone(),
                );
                let right_id = spawn_ball(
                    &mut commands,
                    &physics,
                    event.brick_position,
                    right,
                    visuals,
                );
                spawned_this_frame.push(left_id);
                spawned_this_frame.push(right_id);
            }
        }
    }
}

/// Bundle for spawning ball entities with full physics configuration.
///
/// Combines all required components for a physical, renderable ball that participates
/// in collision detection. Uses the provided physics config to ensure consistency with
/// existing balls in the world.
#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    transform: Transform,
    rigid_body: RigidBody,
    velocity: Velocity,
    colliding_entities: CollidingEntities,
    active_events: ActiveEvents,
    collider: Collider,
    restitution: Restitution,
    friction: Friction,
    damping: Damping,
    locked_axes: LockedAxes,
    ccd: Ccd,
    impulse: ExternalImpulse,
    gravity_scale: GravityScale,
    ball_type: BallTypeId,
}

impl BallBundle {
    /// Create a new ball bundle at the given position and velocity.
    fn new(
        position: Vec3,
        velocity: Vec3,
        physics: &BallPhysicsConfig,
        ball_type: BallTypeId,
    ) -> Self {
        Self {
            ball: Ball,
            transform: Transform::from_translation(position),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::linear(velocity),
            colliding_entities: CollidingEntities::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::ball(BALL_RADIUS),
            restitution: Restitution {
                coefficient: physics.restitution,
                combine_rule: CoefficientCombineRule::Max,
            },
            friction: Friction {
                coefficient: physics.friction,
                combine_rule: CoefficientCombineRule::Max,
            },
            damping: Damping {
                linear_damping: physics.linear_damping,
                angular_damping: physics.angular_damping,
            },
            locked_axes: LockedAxes::TRANSLATION_LOCKED_Y,
            ccd: Ccd::enabled(),
            impulse: ExternalImpulse::default(),
            gravity_scale: GravityScale(1.0),
            ball_type,
        }
    }
}

/// Spawn a ball entity at the given position and velocity, reusing physics config.
///
/// This helper ensures spawned balls use the same physics tuning as existing
/// balls and avoids repeated asset loading by cloning handles from the source.
fn spawn_ball(
    commands: &mut Commands,
    physics: &BallPhysicsConfig,
    position: Vec3,
    velocity: Vec3,
    visuals: BallVisuals,
) -> Entity {
    let bundle = BallBundle::new(position, velocity, physics, visuals.ball_type);
    let mut entity = commands.spawn(bundle);

    if let Some(mesh) = visuals.mesh {
        entity.insert(mesh);
    }
    if let Some(material) = visuals.material {
        entity.insert(material);
    }

    entity.id()
}

/// Calculate a Y-shaped velocity spread in the XZ plane.
///
/// For base velocity `v`, returns two vectors at `±angle_degrees` with the same
/// horizontal speed and original Y component preserved.
fn y_shaped_velocity(base: Vec3, angle_degrees: f32) -> (Vec3, Vec3) {
    let horizontal = Vec2::new(base.x, base.z);
    let speed = horizontal.length();
    if speed == 0.0 {
        // Zero horizontal speed: preserve Y component, return minimal velocities
        // to avoid spawning fully stationary balls
        let min_vel = 0.1; // Minimal movement to prevent stuck balls
        return (
            Vec3::new(min_vel, base.y, 0.0),
            Vec3::new(-min_vel, base.y, 0.0),
        );
    }
    let base_angle = horizontal.y.atan2(horizontal.x);
    let offset = angle_degrees.to_radians();

    let left_angle = base_angle + offset;
    let right_angle = base_angle - offset;

    let left = Vec3::new(speed * left_angle.cos(), base.y, speed * left_angle.sin());
    let right = Vec3::new(speed * right_angle.cos(), base.y, speed * right_angle.sin());

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y_shaped_velocity_returns_offsets() {
        let base = Vec3::new(1.0, 0.0, 0.0);
        let (left, right) = y_shaped_velocity(base, 37.5);
        assert!(left.length() > 0.0);
        assert!(right.length() > 0.0);
    }
}
