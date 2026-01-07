//! Merkaba hazard scaffolding (foundational)
//!
//! Provides marker components and mesh builder stubs for the merkaba hazard.
//! Full behavior (spawn, physics, audio) is implemented in later phases.

use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    ActiveEvents, Ccd, Collider, CollisionEvent, GravityScale, LockedAxes, Restitution, RigidBody,
    Velocity,
};

use crate::signals::{
    MerkabaBrickCollision, MerkabaPaddleCollision, MerkabaWallCollision, SpawnMerkabaMessage,
};
use crate::systems::respawn::LivesState;
use crate::systems::textures::{ObjectClass, TypeVariantRegistry};
use crate::{Ball, Border, Brick, LowerGoal, Paddle};

/// Marker component for merkaba hazard entities.
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Merkaba;

/// System set placeholders for organizing merkaba systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum MerkabaSystems {
    Startup,
    Update,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum MerkabaSpawnFlow {
    Queue,
    Process,
}

/// Resource placeholder for any merkaba mesh handles.
/// Currently unused; will be populated when dual-tetrahedron meshes are built.
#[derive(Resource, Default)]
pub struct MerkabaMeshes {
    pub primary: Option<Handle<Mesh>>,
    pub secondary: Option<Handle<Mesh>>,
}

#[derive(Debug)]
pub struct PendingMerkabaSpawn {
    pub timer: Timer,
    pub position: Vec3,
    pub angle_variance_deg: f32,
    pub min_speed_y: f32,
}

#[derive(Resource, Default, Debug)]
pub struct PendingMerkabaSpawns {
    pub entries: Vec<PendingMerkabaSpawn>,
}

#[derive(Resource, Default, Debug)]
struct MerkabaAngleState {
    flip: bool,
}

/// Placeholder mesh builder for dual tetrahedron children.
/// This stub intentionally does nothing yet; it will be filled in during US1 implementation.
pub fn build_dual_tetrahedron_children(
    _commands: &mut Commands,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<StandardMaterial>,
    _parent: Entity,
) {
    // Implementation will construct dual tetrahedron children and attach to parent.
}

/// Merkaba plugin registers marker components and placeholder resources.
pub struct MerkabaPlugin;

impl Plugin for MerkabaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MerkabaMeshes>()
            .init_resource::<PendingMerkabaSpawns>()
            .init_resource::<MerkabaAngleState>()
            .configure_sets(
                Update,
                (MerkabaSpawnFlow::Queue, MerkabaSpawnFlow::Process).chain(),
            )
            .add_systems(Update, queue_merkaba_spawns.in_set(MerkabaSpawnFlow::Queue))
            .add_systems(
                Update,
                process_pending_merkaba_spawns.in_set(MerkabaSpawnFlow::Process),
            );
        app.add_observer(on_merkaba_paddle_collision_life_loss);
        app.add_systems(PostUpdate, rotate_merkabas);
        app.add_systems(
            Update,
            (
                enforce_min_horizontal_speed,
                detect_goal_collision,
                detect_merkaba_wall_collision,
                detect_merkaba_brick_collision,
                detect_merkaba_paddle_collision,
                despawn_balls_and_merkabas_on_life_loss,
            ),
        );
    }
}

pub fn queue_merkaba_spawns(
    mut reader: MessageReader<SpawnMerkabaMessage>,
    mut pending: ResMut<PendingMerkabaSpawns>,
) {
    for msg in reader.read() {
        pending.entries.push(PendingMerkabaSpawn {
            timer: Timer::from_seconds(msg.delay_seconds, TimerMode::Once),
            position: msg.position,
            angle_variance_deg: msg.angle_variance_deg,
            min_speed_y: msg.min_speed_y,
        });
    }
}

fn process_pending_merkaba_spawns(
    time: Res<Time>,
    mut pending: ResMut<PendingMerkabaSpawns>,
    mut commands: Commands,
    mut angle_state: ResMut<MerkabaAngleState>,
    mut meshes: ResMut<Assets<Mesh>>,
    type_registry: Res<TypeVariantRegistry>,
) {
    let delta = time.delta();
    let mut to_spawn = Vec::new();

    for entry in pending.entries.iter_mut() {
        entry.timer.tick(delta);
        if entry.timer.is_finished() {
            to_spawn.push(PendingMerkabaSpawn {
                timer: entry.timer.clone(),
                position: entry.position,
                angle_variance_deg: entry.angle_variance_deg,
                min_speed_y: entry.min_speed_y,
            });
        }
    }

    pending.entries.retain(|entry| !entry.timer.is_finished());

    for spawn in to_spawn {
        let angle_max = spawn.angle_variance_deg.abs().clamp(5.0, 20.0);
        let sign = if angle_state.flip { -1.0 } else { 1.0 };
        angle_state.flip = !angle_state.flip;
        let angle_deg = angle_max * 0.5 * sign;
        let angle_rad = angle_deg.to_radians();

        // Prioritize horizontal (x-axis) motion with a small z variance.
        let base_speed = spawn.min_speed_y.max(0.1);
        let vx = base_speed;
        let vz = angle_rad.tan() * base_speed;
        let velocity = Vec3::new(vx, 0.0, vz);

        let pos = spawn.position;

        // Create merkaba mesh: two tetrahedrons (upright and inverted)
        let tetra_mesh = meshes.add(Tetrahedron::default());

        // Get materials from texture registry (type_id 0 = blue, 1 = gold)
        let mat_blue_handle = type_registry
            .get(ObjectClass::Merkaba, 0)
            .expect("Merkaba blue material should be registered");
        let mat_gold_handle = type_registry
            .get(ObjectClass::Merkaba, 1)
            .expect("Merkaba gold material should be registered");

        let merkaba_entity = commands
            .spawn((
                Merkaba,
                Transform::from_translation(pos),
                GlobalTransform::from_translation(pos),
                Visibility::Visible,
                // Physics components for bouncing and collision
                RigidBody::Dynamic,
                Collider::ball(0.8),
                Velocity::linear(velocity),
                GravityScale(0.0), // No gravity - horizontal movement only
                LockedAxes::TRANSLATION_LOCKED_Y, // Constrain to XZ plane
                Ccd::enabled(),    // Continuous collision detection
                Restitution::coefficient(0.8), // Bouncy
                ActiveEvents::COLLISION_EVENTS, // T032: required for collision detection
            ))
            .with_children(|parent| {
                let scale = 1.5;

                // 1. UPRIGHT TETRAHEDRON (Blue)
                parent.spawn((
                    Mesh3d(tetra_mesh.clone()),
                    MeshMaterial3d(mat_blue_handle.clone()),
                    Transform::from_scale(Vec3::splat(scale)),
                ));

                // 2. INVERTED TETRAHEDRON (Gold)
                // We use NEGATIVE scale. This performs a "Point Inversion",
                // creating the exact dual shape needed for the star.
                parent.spawn((
                    Mesh3d(tetra_mesh),
                    MeshMaterial3d(mat_gold_handle.clone()),
                    Transform::from_scale(Vec3::splat(-scale)), // Negative Scale!
                ));
            })
            .id();

        commands.entity(merkaba_entity).with_children(|parent| {
            parent.spawn((Transform::default(), GlobalTransform::default()));
            parent.spawn((Transform::default(), GlobalTransform::default()));
        });
    }
}

fn rotate_merkabas(mut query: Query<&mut Transform, With<Merkaba>>, time: Res<Time>) {
    let angle = time.delta_secs() * 2.5;
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(angle);
    }
}

/// T025: Enforce minimum horizontal (x-axis) speed of 3.0 u/s to prevent stalling
fn enforce_min_horizontal_speed(mut query: Query<&mut Velocity, With<Merkaba>>) {
    const MIN_X_SPEED: f32 = 3.0;

    for mut velocity in query.iter_mut() {
        let x = velocity.linvel.x;
        if x.abs() < MIN_X_SPEED {
            let sign = if x >= 0.0 { 1.0 } else { -1.0 };
            velocity.linvel.x = sign as f32 * MIN_X_SPEED;
        }

        // Keep vertical drift in check so motion stays primarily horizontal.
        if velocity.linvel.z.abs() > velocity.linvel.x.abs() {
            velocity.linvel.z = velocity.linvel.z.signum() * velocity.linvel.x.abs() * 0.5;
        }
    }
}

/// T027: Detect goal collision and despawn merkaba
fn detect_goal_collision(
    collision_events: Option<MessageReader<CollisionEvent>>,
    merkabas: Query<Entity, With<Merkaba>>,
    goals: Query<Entity, With<LowerGoal>>,
    mut commands: Commands,
) {
    if let Some(mut collision_events) = collision_events {
        for event in collision_events.read() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                // Check if one entity is merkaba and the other is goal
                let merkaba = if merkabas.get(*e1).is_ok() && goals.get(*e2).is_ok() {
                    Some(*e1)
                } else if merkabas.get(*e2).is_ok() && goals.get(*e1).is_ok() {
                    Some(*e2)
                } else {
                    None
                };

                if let Some(merkaba_entity) = merkaba {
                    commands.entity(merkaba_entity).despawn();
                }
            }
        }
    }
}
/// T028: Detect and emit merkaba wall collision sounds
fn detect_merkaba_wall_collision(
    collision_events: Option<MessageReader<CollisionEvent>>,
    merkabas: Query<Entity, With<Merkaba>>,
    walls: Query<Entity, With<Border>>,
    mut writer: Option<bevy::ecs::message::MessageWriter<MerkabaWallCollision>>,
) {
    if let Some(mut collision_events) = collision_events {
        for event in collision_events.read() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                let (merkaba, wall) = if merkabas.get(*e1).is_ok() && walls.get(*e2).is_ok() {
                    (Some(*e1), Some(*e2))
                } else if merkabas.get(*e2).is_ok() && walls.get(*e1).is_ok() {
                    (Some(*e2), Some(*e1))
                } else {
                    (None, None)
                };

                if let (Some(merkaba_entity), Some(wall_entity)) = (merkaba, wall) {
                    if let Some(writer) = writer.as_mut() {
                        writer.write(MerkabaWallCollision {
                            merkaba_entity,
                            wall_entity,
                        });
                    }
                }
            }
        }
    }
}

/// T028: Detect and emit merkaba brick collision sounds
fn detect_merkaba_brick_collision(
    collision_events: Option<MessageReader<CollisionEvent>>,
    merkabas: Query<Entity, With<Merkaba>>,
    bricks: Query<Entity, With<Brick>>,
    mut writer: Option<bevy::ecs::message::MessageWriter<MerkabaBrickCollision>>,
) {
    if let Some(mut collision_events) = collision_events {
        for event in collision_events.read() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                let (merkaba, brick) = if merkabas.get(*e1).is_ok() && bricks.get(*e2).is_ok() {
                    (Some(*e1), Some(*e2))
                } else if merkabas.get(*e2).is_ok() && bricks.get(*e1).is_ok() {
                    (Some(*e2), Some(*e1))
                } else {
                    (None, None)
                };

                if let (Some(merkaba_entity), Some(brick_entity)) = (merkaba, brick) {
                    if let Some(writer) = writer.as_mut() {
                        writer.write(MerkabaBrickCollision {
                            merkaba_entity,
                            brick_entity,
                        });
                    }
                }
            }
        }
    }
}
/// T032: Detect merkaba-paddle collision and trigger penalty
fn detect_merkaba_paddle_collision(
    mut collision_events: MessageReader<CollisionEvent>,
    merkabas: Query<Entity, With<Merkaba>>,
    paddles: Query<Entity, With<Paddle>>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        // debug!("Collision event: {:?}", event);
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (merkaba, paddle) = if merkabas.contains(*e1) && paddles.contains(*e2) {
                (Some(*e1), Some(*e2))
            } else if merkabas.contains(*e2) && paddles.contains(*e1) {
                (Some(*e2), Some(*e1))
            } else {
                (None, None)
            };

            if let (Some(merkaba_entity), Some(paddle_entity)) = (merkaba, paddle) {
                info!(
                    "Merkaba-Paddle collision detected: {:?} <-> {:?}",
                    merkaba_entity, paddle_entity
                );
                // Trigger observer for life loss and audio
                commands.trigger(MerkabaPaddleCollision {
                    merkaba_entity,
                    paddle_entity,
                });
            }
        }
    }
}

/// T033: Despawn all balls and merkabas when a life is lost
fn despawn_balls_and_merkabas_on_life_loss(
    lives_state: Res<LivesState>,
    balls: Query<Entity, With<Ball>>,
    merkabas: Query<Entity, With<Merkaba>>,
    mut commands: Commands,
    mut local_state: Local<Option<u8>>,
) {
    let current_lives = lives_state.lives_remaining;

    // Initialize local state on first run
    if local_state.is_none() {
        *local_state = Some(current_lives);
        return;
    }

    // Check if lives decreased
    if local_state.unwrap() > current_lives {
        // Life was lost - despawn all balls and merkabas
        for ball_entity in balls.iter() {
            commands.entity(ball_entity).despawn();
        }

        for merkaba_entity in merkabas.iter() {
            commands.entity(merkaba_entity).despawn();
        }
    }

    // Update local state to current lives count
    *local_state = Some(current_lives);
}
/// T032: Handle life loss on merkaba-paddle collision (Observer)
fn on_merkaba_paddle_collision_life_loss(
    _trigger: On<MerkabaPaddleCollision>,
    mut lives_state: ResMut<LivesState>,
) {
    if lives_state.lives_remaining > 0 {
        lives_state.lives_remaining -= 1;
        lives_state.on_last_life = lives_state.lives_remaining == 1;
    }
}
