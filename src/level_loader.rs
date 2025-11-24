use bevy::prelude::*;
use ron::de::from_str;
use serde::Deserialize;

use crate::{
    Ball, Brick, GameProgress, GravityConfig, InitialPositions, Paddle, BALL_RADIUS, CELL_HEIGHT,
    CELL_WIDTH, PADDLE_HEIGHT, PADDLE_RADIUS, PLANE_H, PLANE_W,
};
use bevy_rapier3d::prelude::*;

#[derive(Deserialize, Debug)]
pub struct LevelDefinition {
    pub number: u32,
    /// Optional gravity override for this level (x,y,z). If omitted the existing GravityConfig value is used.
    pub gravity: Option<(f32, f32, f32)>,
    pub matrix: Vec<Vec<u8>>, // expect 22 x 22
}

#[derive(Resource, Debug)]
pub struct CurrentLevel(pub LevelDefinition);

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_level, spawn_level_entities).chain());
        app.add_systems(Update, (advance_level_when_cleared, restart_level_on_key));
    }
}

fn load_level(
    mut commands: Commands,
    mut gravity_cfg: ResMut<GravityConfig>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    let chosen_path = {
        use std::env;
        if let Ok(num) = env::var("BK_LEVEL") {
            let trimmed = num.trim();
            if let Ok(n) = trimmed.parse::<u32>() {
                format!("assets/levels/level_{:03}.ron", n)
            } else {
                warn!("BK_LEVEL='{}' not a number; defaulting to level_001", num);
                "assets/levels/level_001.ron".to_string()
            }
        } else {
            "assets/levels/level_001.ron".to_string()
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    let level_str: &str = match std::fs::read_to_string(&chosen_path) {
        Ok(s) => {
            info!("Loading level file: {}", chosen_path);
            Box::leak(s.into_boxed_str())
        }
        Err(e) => {
            warn!(
                "Failed to read level file '{}': {e}. Falling back to empty level",
                chosen_path
            );
            "LevelDefinition(number:0,matrix:[])"
        }
    };
    #[cfg(target_arch = "wasm32")]
    let level_str: &str = include_str!("../assets/levels/level_001.ron");

    match from_str::<LevelDefinition>(level_str) {
        Ok(def) => {
            // basic validation
            if def.matrix.len() != 22 || def.matrix.iter().any(|r| r.len() != 22) {
                warn!("Level matrix wrong dimensions; expected 22x22");
            }
            info!("Loaded level {}", def.number);
            // Apply per-level gravity if present
            if let Some((x, y, z)) = def.gravity {
                gravity_cfg.normal = Vec3::new(x, y, z);
                if let Ok(mut config) = rapier_config.single_mut() {
                    config.gravity = gravity_cfg.normal;
                }
                info!("Level gravity set to {:?}", gravity_cfg.normal);
            } else {
                debug!(
                    "Level has no gravity override; using existing {:?}",
                    gravity_cfg.normal
                );
            }
            commands.insert_resource(CurrentLevel(def));
        }
        Err(e) => {
            warn!("Failed to parse level: {e}");
        }
    }
}

fn spawn_level_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut initial_positions: ResMut<InitialPositions>,
    level: Option<Res<CurrentLevel>>,
) {
    let Some(level) = level else {
        return;
    };
    spawn_level_entities_impl(
        &level.0,
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut initial_positions,
    );
}

fn spawn_level_entities_impl(
    def: &LevelDefinition,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    initial_positions: &mut ResMut<InitialPositions>,
) {
    debug!("Spawning entities for level {}", def.number);
    // Shared material
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        unlit: false,
        ..default()
    });

    let mut paddle_spawned = false;
    let mut ball_spawned = false;

    for (row, row_data) in def.matrix.iter().enumerate() {
        for (col, value) in row_data.iter().enumerate() {
            let x = -PLANE_H / 2.0 + (row as f32 + 0.5) * CELL_HEIGHT;
            let z = -PLANE_W / 2.0 + (col as f32 + 0.5) * CELL_WIDTH;
            match value {
                0 => {}
                1 => {
                    // Paddle
                    if !paddle_spawned {
                        paddle_spawned = true;
                        initial_positions.paddle_pos = Some(Vec3::new(x, 2.0, z));
                        commands.spawn((
                            Mesh3d(meshes.add(Capsule3d::new(PADDLE_RADIUS, PADDLE_HEIGHT).mesh())),
                            MeshMaterial3d(debug_material.clone()),
                            Transform::from_xyz(x, 2.0, z)
                                .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
                            Paddle,
                            RigidBody::KinematicPositionBased,
                            GravityScale(0.0),
                            CollidingEntities::default(),
                            Collider::capsule_y(PADDLE_HEIGHT / 2.0, PADDLE_RADIUS),
                            LockedAxes::TRANSLATION_LOCKED_Y,
                            KinematicCharacterController::default(),
                            Ccd::enabled(),
                            Friction {
                                coefficient: 2.0,
                                combine_rule: CoefficientCombineRule::Max,
                            },
                        ));
                    }
                }
                2 => {
                    // Ball
                    if !ball_spawned {
                        ball_spawned = true;
                        initial_positions.ball_pos = Some(Vec3::new(x, 2.0, z));
                        commands
                            .spawn((
                                Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh())),
                                MeshMaterial3d(debug_material.clone()),
                                Transform::from_xyz(x, 2.0, z),
                                Ball,
                                RigidBody::Dynamic,
                                CollidingEntities::default(),
                                ActiveEvents::COLLISION_EVENTS,
                                Collider::ball(BALL_RADIUS),
                                Restitution {
                                    coefficient: 0.9,
                                    combine_rule: CoefficientCombineRule::Max,
                                },
                                Friction {
                                    coefficient: 2.0,
                                    combine_rule: CoefficientCombineRule::Max,
                                },
                                Damping {
                                    linear_damping: 0.5,
                                    angular_damping: 0.5,
                                },
                            ))
                            .insert((
                                LockedAxes::TRANSLATION_LOCKED_Y,
                                Ccd::enabled(),
                                ExternalImpulse::default(),
                                GravityScale(1.0),
                            ));
                    }
                }
                3 => {
                    // Brick
                    commands.spawn((
                        Mesh3d(meshes.add(Cuboid::new(CELL_HEIGHT * 0.9, 0.5, CELL_WIDTH * 0.9))),
                        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.1, 0.1))),
                        Transform::from_xyz(x, 2.0, z),
                        Brick,
                        RigidBody::Fixed,
                        Collider::cuboid(CELL_HEIGHT * 0.9 / 2.0, 0.25, CELL_WIDTH * 0.9 / 2.0),
                        Restitution {
                            coefficient: 1.0,
                            combine_rule: CoefficientCombineRule::Max,
                        },
                        CollidingEntities::default(),
                        ActiveEvents::COLLISION_EVENTS,
                    ));
                }
                _ => {
                    warn!("Unsupported cell value {value} at ({row},{col})");
                }
            }
        }
    }

    if !paddle_spawned {
        warn!("No paddle found in level matrix; spawning fallback paddle.");
        let x = 0.0;
        let z = 0.0;
        initial_positions.paddle_pos = Some(Vec3::new(x, 2.0, z));
        commands.spawn((
            Mesh3d(meshes.add(Capsule3d::new(PADDLE_RADIUS, PADDLE_HEIGHT).mesh())),
            MeshMaterial3d(debug_material.clone()),
            Transform::from_xyz(x, 2.0, z)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
            Paddle,
            RigidBody::KinematicPositionBased,
            GravityScale(0.0),
            CollidingEntities::default(),
            Collider::capsule_y(PADDLE_HEIGHT / 2.0, PADDLE_RADIUS),
            LockedAxes::TRANSLATION_LOCKED_Y,
            KinematicCharacterController::default(),
            Ccd::enabled(),
            Friction {
                coefficient: 2.0,
                combine_rule: CoefficientCombineRule::Max,
            },
        ));
    }
    if !ball_spawned {
        warn!("No ball found in level matrix; spawning fallback ball.");
        initial_positions.ball_pos = Some(Vec3::new(0.0, 2.0, 0.0));
        commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh())),
                MeshMaterial3d(debug_material.clone()),
                Transform::from_xyz(0.0, 2.0, 0.0),
                Ball,
                RigidBody::Dynamic,
                CollidingEntities::default(),
                ActiveEvents::COLLISION_EVENTS,
                Collider::ball(BALL_RADIUS),
                Restitution {
                    coefficient: 0.9,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Friction {
                    coefficient: 2.0,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Damping {
                    linear_damping: 0.5,
                    angular_damping: 0.5,
                },
            ))
            .insert((
                LockedAxes::TRANSLATION_LOCKED_Y,
                Ccd::enabled(),
                ExternalImpulse::default(),
                GravityScale(1.0),
            ));
    }
}

/// Advance to the next level when all bricks have been cleared.
fn advance_level_when_cleared(
    bricks: Query<Entity, With<Brick>>,
    paddle_q: Query<Entity, With<Paddle>>,
    ball_q: Query<Entity, With<Ball>>,
    current_level: Option<Res<CurrentLevel>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut initial_positions: ResMut<InitialPositions>,
    mut gravity_cfg: ResMut<GravityConfig>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    mut game_progress: ResMut<GameProgress>,
) {
    let Some(curr) = current_level else {
        return;
    };
    if !bricks.is_empty() {
        return; // still bricks remaining
    }
    let next_number = curr.0.number + 1;
    let path = format!("assets/levels/level_{:03}.ron", next_number);
    if !std::path::Path::new(&path).exists() {
        if !game_progress.finished {
            info!(
                "All bricks cleared; no next level file {}. Game complete.",
                path
            );
            game_progress.finished = true;
            // Despawn remaining paddle and ball to freeze gameplay
            for p in paddle_q.iter() {
                commands.entity(p).despawn();
            }
            for b in ball_q.iter() {
                commands.entity(b).despawn();
            }
            // Spawn completion text (desktop only UI style similar to wireframe text)
            #[cfg(not(target_arch = "wasm32"))]
            commands.spawn((
                Text::new("GAME COMPLETE - Press Q to Quit"),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(60.0),
                    left: Val::Px(60.0),
                    ..default()
                },
            ));
        }
        return;
    }

    // Despawn existing paddle and ball (bricks already gone or will be ignored)
    for p in paddle_q.iter() {
        commands.entity(p).despawn();
    }
    for b in ball_q.iter() {
        commands.entity(b).despawn();
    }

    match std::fs::read_to_string(&path) {
        Ok(content) => match from_str::<LevelDefinition>(&content) {
            Ok(def) => {
                info!("Advancing to level {}", def.number);
                // Apply gravity override if present
                if let Some((x, y, z)) = def.gravity {
                    gravity_cfg.normal = Vec3::new(x, y, z);
                    if let Ok(mut config) = rapier_config.single_mut() {
                        config.gravity = gravity_cfg.normal;
                    }
                    info!("Level gravity set to {:?}", gravity_cfg.normal);
                }
                // Spawn entities first, then publish resource (systems reading CurrentLevel next frame)
                spawn_level_entities_impl(
                    &def,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut initial_positions,
                );
                commands.insert_resource(CurrentLevel(def));
            }
            Err(e) => warn!("Failed to parse next level '{}': {e}", path),
        },
        Err(e) => warn!("Failed to read next level file '{}': {e}", path),
    }
}

/// Restart the current level when the user presses R.
fn restart_level_on_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_level: Option<Res<CurrentLevel>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut initial_positions: ResMut<InitialPositions>,
    mut gravity_cfg: ResMut<GravityConfig>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    bricks: Query<Entity, With<Brick>>,
    paddle_q: Query<Entity, With<Paddle>>,
    ball_q: Query<Entity, With<Ball>>,
    mut game_progress: ResMut<GameProgress>,
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }
    let level_number = current_level.map(|cl| cl.0.number).unwrap_or(1);
    let path = format!("assets/levels/level_{:03}.ron", level_number);
    info!("Restarting level {} from {}", level_number, path);

    // Despawn existing entities
    for e in bricks.iter() {
        commands.entity(e).despawn();
    }
    for e in paddle_q.iter() {
        commands.entity(e).despawn();
    }
    for e in ball_q.iter() {
        commands.entity(e).despawn();
    }

    // Reset progress if we were finished
    game_progress.finished = false;

    // Reset initial positions
    *initial_positions = InitialPositions::default();

    match std::fs::read_to_string(&path) {
        Ok(content) => match from_str::<LevelDefinition>(&content) {
            Ok(def) => {
                // Apply gravity override
                if let Some((x, y, z)) = def.gravity {
                    gravity_cfg.normal = Vec3::new(x, y, z);
                    if let Ok(mut config) = rapier_config.single_mut() {
                        config.gravity = gravity_cfg.normal;
                    }
                    info!("Level gravity set to {:?}", gravity_cfg.normal);
                } else {
                    // Ensure rapier gravity matches current stored config
                    if let Ok(mut config) = rapier_config.single_mut() {
                        config.gravity = gravity_cfg.normal;
                    }
                }
                // Spawn entities then update resource
                spawn_level_entities_impl(
                    &def,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut initial_positions,
                );
                commands.insert_resource(CurrentLevel(def));
            }
            Err(e) => warn!("Failed to parse level on restart '{}': {e}", path),
        },
        Err(e) => warn!("Failed to read level file on restart '{}': {e}", path),
    }
}
