//! Merkaba hazard scaffolding (foundational)
//!
//! Provides marker components and mesh builder stubs for the merkaba hazard.
//! Full behavior (spawn, physics, audio) is implemented in later phases.

use bevy::asset::RenderAssetUsages;
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_rapier3d::prelude::{
    ActiveEvents, Ccd, Collider, CollisionEvent, CollisionGroups, GravityScale, Group, LockedAxes,
    Restitution, RigidBody, SolverGroups, Velocity,
};

use crate::signals::{
    MerkabaBrickCollision, MerkabaPaddleCollision, MerkabaWallCollision, SpawnMerkabaMessage,
};
use crate::systems::respawn::{
    LifeLossCause, LifeLostEvent, LivesState, RespawnHandle, SpawnPoints,
};
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

fn setup_merkaba_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut merkaba_meshes: ResMut<MerkabaMeshes>,
) {
    if merkaba_meshes.primary.is_some() {
        return;
    }

    // Standard Tetrahedron (Point Up)
    // Vertices for a regular tetrahedron centered at origin, but offset so visual center is nice.
    // We use a regular tetrahedron inscribed in the unit sphere for size.
    // V0 (Top): (0, 1, 0)
    // Base vertices at y = -1/3
    let v0 = Vec3::new(0.0, 1.0, 0.0);
    let v1 = Vec3::new(0.0, -0.33333334, 0.94280905);
    let v2 = Vec3::new(-0.8164966, -0.33333334, -0.47140452);
    let v3 = Vec3::new(0.8164966, -0.33333334, -0.47140452);

    // Faces (CCW winding for outward normals):
    // 1: 0-1-3 (Front Right)
    // 2: 0-3-2 (Back)
    // 3: 0-2-1 (Front Left)
    // 4: 1-2-3 (Base, looking from bottom)

    // Split vertices for flat shading (unique normals per face)
    let positions = vec![
        // Face 1
        v0, v1, v3, // Face 2
        v0, v3, v2, // Face 3
        v0, v2, v1, // Face 4
        v1, v2, v3,
    ];

    // Generic UV mapping for each triangular face
    let uv_top = Vec2::new(0.5, 1.0);
    let uv_left = Vec2::new(0.0, 0.0);
    let uv_right = Vec2::new(1.0, 0.0);

    let uvs = vec![
        // Face 1
        uv_top, uv_left, uv_right, // Face 2
        uv_top, uv_left, uv_right, // Face 3
        uv_top, uv_left, uv_right, // Face 4
        uv_top, uv_left, uv_right,
    ];

    let mut mesh1 = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh1.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
    mesh1.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs.clone());
    // Indices not used to allow compute_flat_normals to work
    mesh1.compute_flat_normals();

    merkaba_meshes.primary = Some(meshes.add(mesh1));

    // Secondary Mesh (Inverted)
    // 1. Invert positions
    let inverted_positions: Vec<Vec3> = positions.iter().map(|p| -*p).collect();

    // 2. Reorder for CCW winding (swap 2nd and 3rd of each triangle)
    // This ensures outward normals for the inverted shape
    let mut positions2 = Vec::new();
    let mut uvs2 = Vec::new();

    for (pos_chunk, uv_chunk) in inverted_positions.chunks(3).zip(uvs.chunks(3)) {
        if pos_chunk.len() == 3 {
            positions2.push(pos_chunk[0]);
            positions2.push(pos_chunk[2]); // Swap
            positions2.push(pos_chunk[1]); // Swap

            // Also swap UVs to match vertex reordering
            uvs2.push(uv_chunk[0]);
            uvs2.push(uv_chunk[2]);
            uvs2.push(uv_chunk[1]);
        }
    }

    let mut mesh2 = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh2.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions2);
    mesh2.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs2);
    mesh2.compute_flat_normals();

    merkaba_meshes.secondary = Some(meshes.add(mesh2));
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
            .add_systems(Startup, setup_merkaba_meshes)
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
    merkaba_meshes: Res<MerkabaMeshes>,
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

        // Prioritize horizontal (Z-axis, screen horizontal) motion with a small X (screen vertical) variance.
        let base_speed = spawn.min_speed_y.max(0.1);
        let vz = base_speed;
        let vx = angle_rad.tan() * base_speed;
        let velocity = Vec3::new(vx, 0.0, vz);

        let pos = spawn.position;

        // Get pre-generated meshes
        let primary_mesh = merkaba_meshes
            .primary
            .clone()
            .expect("Merkaba meshes not initialized");
        let secondary_mesh = merkaba_meshes
            .secondary
            .clone()
            .expect("Merkaba meshes not initialized");

        // Get materials from texture registry (type_id 0 = blue, 1 = gold)
        let mat_blue_handle = type_registry
            .get(ObjectClass::Merkaba, 0)
            .expect("Merkaba blue material should be registered");
        let mat_gold_handle = type_registry
            .get(ObjectClass::Merkaba, 1)
            .expect("Merkaba gold material should be registered");

        commands
            .spawn((
                Merkaba,
                Transform::from_translation(pos),
                GlobalTransform::from_translation(pos),
                Visibility::Visible,
                // Physics components for bouncing and collision
                RigidBody::Dynamic,
                Collider::cylinder(1.0, 0.4),
                Velocity::linear(velocity),
                GravityScale(0.0), // No gravity - horizontal movement only
                LockedAxes::TRANSLATION_LOCKED_Y, // Constrain to XZ plane
                SolverGroups::new(Group::GROUP_2, Group::ALL ^ Group::GROUP_1), // Don't collide with Paddle (Group 1)
                CollisionGroups::new(Group::GROUP_2, Group::ALL), // Define explicit membership for KCC filtering
                Ccd::enabled(),                                   // Continuous collision detection
                Restitution::coefficient(0.8),                    // Bouncy
                ActiveEvents::COLLISION_EVENTS, // T032: required for collision detection
            ))
            .with_children(|parent| {
                let scale = 0.75;

                // 1. UPRIGHT TETRAHEDRON (Blue)
                parent.spawn((
                    Mesh3d(primary_mesh),
                    MeshMaterial3d(mat_blue_handle.clone()),
                    Transform::from_scale(Vec3::splat(scale)),
                ));

                // 2. INVERTED TETRAHEDRON (Gold)
                // Use the secondary mesh which is geometrically inverted with corrected winding
                parent.spawn((
                    Mesh3d(secondary_mesh),
                    MeshMaterial3d(mat_gold_handle.clone()),
                    Transform::from_scale(Vec3::splat(scale)),
                ));
            });
    }
}

fn rotate_merkabas(mut query: Query<&mut Transform, With<Merkaba>>, time: Res<Time>) {
    let angle = time.delta_secs() * 2.5;
    for mut transform in query.iter_mut() {
        transform.rotate_local_y(angle);
    }
}

/// T025: Enforce minimum horizontal (z-axis) speed of 3.0 u/s to prevent stalling
fn enforce_min_horizontal_speed(mut query: Query<&mut Velocity, With<Merkaba>>) {
    const MIN_Z_SPEED: f32 = 3.0;

    for mut velocity in query.iter_mut() {
        let z = velocity.linvel.z;
        if z.abs() < MIN_Z_SPEED {
            let sign = if z >= 0.0 { 1.0 } else { -1.0 };
            velocity.linvel.z = sign as f32 * MIN_Z_SPEED;
        }

        // Keep vertical drift in check so motion stays primarily horizontal.
        if velocity.linvel.x.abs() > velocity.linvel.z.abs() {
            velocity.linvel.x = velocity.linvel.x.signum() * velocity.linvel.z.abs() * 0.5;
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
/// Triggers standard life loss flow via LifeLostEvent to ensure visual feedback/respawn sequence runs.
fn on_merkaba_paddle_collision_life_loss(
    _trigger: On<MerkabaPaddleCollision>,
    mut life_lost_writer: MessageWriter<LifeLostEvent>,
    balls: Query<Entity, With<Ball>>,
    ball_handles: Query<&RespawnHandle, With<Ball>>,
    spawn_points: Res<SpawnPoints>,
) {
    // Pick a ball to attribute the loss to (needed for LifeLostEvent contract).
    // In multiball, any ball will do as they all get cleared on life loss.
    // If no ball exists, we can't trigger the standard respawn flow easily,
    // but that state shouldn't happen during active gameplay.
    if let Some(ball_entity) = balls.iter().next() {
        let ball_spawn = ball_handles
            .get(ball_entity)
            .map(|h| h.spawn)
            .unwrap_or_else(|_| spawn_points.ball_spawn());

        life_lost_writer.write(LifeLostEvent {
            ball: ball_entity,
            cause: LifeLossCause::MerkabaCollision,
            ball_spawn,
        });
    } else {
        warn!("Merkaba collision with paddle, but no balls found to trigger LifeLostEvent");
    }
}
