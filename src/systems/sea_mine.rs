use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::math::primitives::Cone;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    ActiveEvents, Ccd, Collider, CollisionEvent, GravityScale, LockedAxes, RigidBody, Velocity,
};
use rand::RngExt;

use crate::level_format::{is_sea_mine_trigger_brick, SEA_MINE_BRICK};
use crate::signals::{
    SeaMineDetonationMessage, SeaMineExplosionTriggered, SeaMineTriggerCause, SpawnSeaMineMessage,
};
use crate::systems::particle_fx::SEA_MINE_EXPLOSION_DAMAGE_RADIUS;
use crate::systems::respawn::{BallLostEvent, LifeLossCause, SpawnPoints, SpawnTransform};
use crate::{Ball, Border, Brick, BrickTypeId, Paddle};

const SEA_MINE_RADIUS: f32 = 0.55;
const SEA_MINE_MIN_LINEAR_SPEED: f32 = 3.0;
const SEA_MINE_MIN_ANGULAR_SPEED: f32 = std::f32::consts::PI;
const SEA_MINE_BLAST_RADIUS: f32 = SEA_MINE_EXPLOSION_DAMAGE_RADIUS;
const SEA_MINE_SPIKE_COUNT: usize = 24;
const SEA_MINE_SPIKE_RADIUS: f32 = 0.12;
const SEA_MINE_SPIKE_HEIGHT: f32 = 0.72;
const SEA_MINE_SPIKE_SURFACE_OFFSET: f32 = SEA_MINE_RADIUS * 0.88;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct SeaMine;

pub struct SeaMinePlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SeaMineSystems {
    ResolveDetonation,
}

impl Plugin for SeaMinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_sea_mines_from_messages,
                maintain_sea_mine_motion,
                detect_sea_mine_detonation,
                resolve_sea_mine_detonations
                    .after(detect_sea_mine_detonation)
                    .in_set(SeaMineSystems::ResolveDetonation),
            ),
        );
    }
}

fn spawn_sea_mines_from_messages(
    mut commands: Commands,
    mut msgs: MessageReader<SpawnSeaMineMessage>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for msg in msgs.read() {
        if msg.source_brick_type != SEA_MINE_BRICK {
            continue;
        }

        let mut rng = rand::rng();
        let angle: f32 = rng.random_range(0.0..std::f32::consts::TAU);
        let launch = Vec3::new(angle.cos(), 0.0, angle.sin()) * SEA_MINE_MIN_LINEAR_SPEED;
        let spin_sign = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

        let mine_entity = commands
            .spawn((
                SeaMine,
                Transform::from_translation(msg.position),
                GlobalTransform::default(),
                RigidBody::Dynamic,
                Collider::ball(SEA_MINE_RADIUS),
                Velocity {
                    linvel: launch,
                    angvel: Vec3::new(0.0, spin_sign * SEA_MINE_MIN_ANGULAR_SPEED, 0.0),
                },
                GravityScale(0.0),
                LockedAxes::TRANSLATION_LOCKED_Y,
                ActiveEvents::COLLISION_EVENTS,
                Ccd::enabled(),
            ))
            .id();

        let body_mesh = meshes.add(Sphere::new(SEA_MINE_RADIUS).mesh());
        let body_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.7, 0.15),
            metallic: 0.15,
            perceptual_roughness: 0.5,
            ..default()
        });
        let spike_mesh = meshes.add(
            Cone {
                radius: SEA_MINE_SPIKE_RADIUS,
                height: SEA_MINE_SPIKE_HEIGHT,
            }
            .mesh(),
        );
        let spike_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.2),
            metallic: 0.55,
            perceptual_roughness: 0.42,
            ..default()
        });

        commands.entity(mine_entity).with_children(|parent| {
            parent.spawn((
                Mesh3d(body_mesh),
                MeshMaterial3d(body_material),
                Transform::default(),
            ));

            // Evenly distribute spikes around the sphere surface.
            let golden_ratio = (1.0 + 5.0_f32.sqrt()) / 2.0;
            for i in 0..SEA_MINE_SPIKE_COUNT {
                let theta = 2.0 * std::f32::consts::PI * i as f32 / golden_ratio;
                let phi = ((2 * i + 1) as f32 / SEA_MINE_SPIKE_COUNT as f32 - 1.0).acos();

                let direction =
                    Vec3::new(phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos());
                let position = direction * SEA_MINE_SPIKE_SURFACE_OFFSET;
                let rotation = Quat::from_rotation_arc(Vec3::Y, direction);

                parent.spawn((
                    Mesh3d(spike_mesh.clone()),
                    MeshMaterial3d(spike_material.clone()),
                    Transform::from_translation(position).with_rotation(rotation),
                ));
            }
        });
    }
}

fn maintain_sea_mine_motion(mut mines: Query<(&mut Velocity, &mut Transform), With<SeaMine>>) {
    for (mut velocity, mut transform) in &mut mines {
        let horizontal = Vec3::new(velocity.linvel.x, 0.0, velocity.linvel.z);
        let speed = horizontal.length();

        if speed < SEA_MINE_MIN_LINEAR_SPEED {
            let dir = if speed > f32::EPSILON {
                horizontal.normalize()
            } else {
                Vec3::X
            };
            let boosted = dir * SEA_MINE_MIN_LINEAR_SPEED;
            velocity.linvel = Vec3::new(boosted.x, 0.0, boosted.z);
        }

        let angular = velocity.angvel.y;
        if angular.abs() < SEA_MINE_MIN_ANGULAR_SPEED {
            let sign = if angular >= 0.0 { 1.0 } else { -1.0 };
            velocity.angvel.y = sign * SEA_MINE_MIN_ANGULAR_SPEED;
        }

        transform.translation.y = 2.0;
    }
}

fn detect_sea_mine_detonation(
    collision_events: Option<MessageReader<CollisionEvent>>,
    mines: Query<Entity, With<SeaMine>>,
    borders: Query<Entity, With<Border>>,
    paddles: Query<Entity, With<Paddle>>,
    bricks: Query<&BrickTypeId, With<Brick>>,
    mine_positions: Query<&Transform, With<SeaMine>>,
    mut detonation_writer: Option<MessageWriter<SeaMineDetonationMessage>>,
    mut seen_this_frame: Local<std::collections::HashSet<Entity>>,
) {
    seen_this_frame.clear();

    let Some(mut collision_events) = collision_events else {
        return;
    };
    let Some(writer) = detonation_writer.as_mut() else {
        return;
    };

    for event in collision_events.read() {
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };

        let (mine, other) = if mines.get(*e1).is_ok() {
            (*e1, *e2)
        } else if mines.get(*e2).is_ok() {
            (*e2, *e1)
        } else {
            continue;
        };

        if seen_this_frame.contains(&mine) {
            continue;
        }

        let cause = if borders.get(other).is_ok() {
            Some(SeaMineTriggerCause::Wall)
        } else if paddles.get(other).is_ok() {
            Some(SeaMineTriggerCause::Paddle)
        } else if let Ok(brick_type) = bricks.get(other) {
            if is_sea_mine_trigger_brick(brick_type.0) {
                Some(SeaMineTriggerCause::BrickGt90)
            } else {
                None
            }
        } else {
            None
        };

        let Some(cause) = cause else {
            continue;
        };

        let position = mine_positions
            .get(mine)
            .map(|t| t.translation)
            .unwrap_or(Vec3::ZERO);

        writer.write(SeaMineDetonationMessage {
            entity: mine,
            position,
            cause,
            radius: SEA_MINE_BLAST_RADIUS,
        });
        seen_this_frame.insert(mine);
    }
}

fn resolve_sea_mine_detonations(
    mut commands: Commands,
    mut messages: MessageReader<SeaMineDetonationMessage>,
    mut explosion_writer: Option<MessageWriter<SeaMineExplosionTriggered>>,
    balls: Query<(Entity, &Transform), With<Ball>>,
    paddles: Query<(Entity, &Transform), With<Paddle>>,
    spawn_points: Option<Res<SpawnPoints>>,
    mut ball_lost_writer: Option<MessageWriter<BallLostEvent>>,
) {
    for detonation in messages.read() {
        commands.entity(detonation.entity).try_despawn();
        if let Some(writer) = explosion_writer.as_mut() {
            writer.write(SeaMineExplosionTriggered {
                position: detonation.position,
                radius: detonation.radius,
            });
        }

        let mut first_ball_in_radius = None;
        let mut ball_destroyed_count = 0usize;
        let mut ball_survivor_count = 0usize;

        for (ball, transform) in &balls {
            if transform.translation.distance(detonation.position) <= detonation.radius {
                if first_ball_in_radius.is_none() {
                    first_ball_in_radius = Some(ball);
                }
                ball_destroyed_count += 1;
                commands.entity(ball).try_despawn();
            } else {
                ball_survivor_count += 1;
            }
        }

        let mut paddle_was_destroyed = false;
        for (paddle, transform) in &paddles {
            if transform.translation.distance(detonation.position) <= detonation.radius {
                commands.entity(paddle).try_despawn();
                paddle_was_destroyed = true;
            }
        }

        let last_ball_destroyed = ball_destroyed_count > 0 && ball_survivor_count == 0;

        if paddle_was_destroyed || last_ball_destroyed {
            let representative_ball =
                first_ball_in_radius.or_else(|| balls.iter().next().map(|(e, _)| e));
            if let (Some(ball), Some(writer)) = (representative_ball, ball_lost_writer.as_mut()) {
                let ball_spawn = spawn_points
                    .as_ref()
                    .map(|sp| sp.ball_spawn())
                    .unwrap_or_else(|| SpawnTransform::new(Vec3::ZERO, Quat::IDENTITY));
                writer.write(BallLostEvent {
                    ball,
                    cause: LifeLossCause::PaddleHazard,
                    ball_spawn,
                });
            }
        }
    }
}
