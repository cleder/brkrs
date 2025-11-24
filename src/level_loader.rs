use bevy::prelude::*;
use ron::de::from_str;
use serde::Deserialize;

use crate::{
    Ball, Brick, Paddle, BALL_RADIUS, CELL_HEIGHT, CELL_WIDTH, PADDLE_HEIGHT, PADDLE_RADIUS,
    PLANE_H, PLANE_W,
};
use bevy_rapier3d::prelude::*;

#[derive(Deserialize, Debug)]
pub struct LevelDefinition {
    pub number: u32,
    pub matrix: Vec<Vec<u8>>, // expect 22 x 22
}

#[derive(Resource, Debug)]
pub struct CurrentLevel(pub LevelDefinition);

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_level, spawn_level_entities).chain());
    }
}

fn load_level(mut commands: Commands) {
    // For now, load a single embedded level; later can use AssetServer
    #[cfg(target_arch = "wasm32")]
    let level_str: &str = include_str!("../assets/levels/level_001.ron");
    #[cfg(not(target_arch = "wasm32"))]
    let level_str: &str = match std::fs::read_to_string("assets/levels/level_001.ron") {
        Ok(s) => Box::leak(s.into_boxed_str()),
        Err(e) => {
            warn!("Failed to read level file: {e}. Falling back to empty level");
            "LevelDefinition(number:0,matrix:[])"
        }
    };

    match from_str::<LevelDefinition>(level_str) {
        Ok(def) => {
            // basic validation
            if def.matrix.len() != 22 || def.matrix.iter().any(|r| r.len() != 22) {
                warn!("Level matrix wrong dimensions; expected 22x22");
            }
            info!("Loaded level {}", def.number);
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
    level: Option<Res<CurrentLevel>>,
) {
    let Some(level) = level else {
        return;
    };
    debug!("Spawning entities for level {}", level.0.number);
    // Shared material
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        unlit: false,
        ..default()
    });

    let mut paddle_spawned = false;
    let mut ball_spawned = false;

    for (row, row_data) in level.0.matrix.iter().enumerate() {
        for (col, value) in row_data.iter().enumerate() {
            let x = -PLANE_H / 2.0 + (row as f32 + 0.5) * CELL_HEIGHT;
            let z = -PLANE_W / 2.0 + (col as f32 + 0.5) * CELL_WIDTH;
            match value {
                0 => {}
                1 => {
                    // Paddle
                    if !paddle_spawned {
                        paddle_spawned = true;
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
                        commands.spawn((
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
        commands.spawn((
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
            LockedAxes::TRANSLATION_LOCKED_Y,
            Ccd::enabled(),
            ExternalImpulse::default(),
            GravityScale(1.0),
        ));
    }
}
