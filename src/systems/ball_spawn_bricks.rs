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
            ball_spawn_system.after(crate::despawn_marked_entities),
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
    for event in reader.read() {
        let Some(rule) = config.brick_spawn_rules.get(&event.brick_type) else {
            continue;
        };
        let Some(triggering_ball) = event.destroyed_by else {
            continue;
        };

        match rule.velocity_modifier {
            VelocityModifier::DespawnAll => {
                for ball in ball_entities.iter() {
                    if ball != triggering_ball {
                        commands.entity(ball).despawn();
                    }
                }
            }
            VelocityModifier::Inverse => {
                let Ok((velocity, mesh, material, ball_type)) = ball_sources.get(triggering_ball)
                else {
                    continue;
                };
                let visuals = BallVisuals::from_source(mesh, material, ball_type);
                spawn_ball(
                    &mut commands,
                    &physics,
                    event.brick_position,
                    -velocity.linvel,
                    visuals,
                );
            }
            VelocityModifier::YShaped { angle_degrees } => {
                let Ok((velocity, mesh, material, ball_type)) = ball_sources.get(triggering_ball)
                else {
                    continue;
                };
                let visuals = BallVisuals::from_source(mesh, material, ball_type);
                let (left, right) = y_shaped_velocity(velocity.linvel, angle_degrees);
                spawn_ball(
                    &mut commands,
                    &physics,
                    event.brick_position,
                    left,
                    visuals.clone(),
                );
                spawn_ball(
                    &mut commands,
                    &physics,
                    event.brick_position,
                    right,
                    visuals,
                );
            }
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
    let mut entity = commands.spawn((
        Ball,
        Transform::from_translation(position),
        RigidBody::Dynamic,
        Velocity::linear(velocity),
        CollidingEntities::default(),
        ActiveEvents::COLLISION_EVENTS,
        Collider::ball(BALL_RADIUS),
        Restitution {
            coefficient: physics.restitution,
            combine_rule: CoefficientCombineRule::Max,
        },
        Friction {
            coefficient: physics.friction,
            combine_rule: CoefficientCombineRule::Max,
        },
        Damping {
            linear_damping: physics.linear_damping,
            angular_damping: physics.angular_damping,
        },
        LockedAxes::TRANSLATION_LOCKED_Y,
        Ccd::enabled(),
        ExternalImpulse::default(),
        GravityScale(1.0),
        visuals.ball_type,
    ));

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
        return (Vec3::ZERO, Vec3::ZERO);
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
