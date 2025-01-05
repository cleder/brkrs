//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.

use std::f32::consts::PI;

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{
    color::palettes::{basic::SILVER, css::RED}, input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll}, prelude::*, render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    }, window::CursorGrabMode
};
use bevy_rapier3d::prelude::*;

const SHAPES_X_EXTENT: f32 = 14.0;
const Z_EXTENT: f32 = 5.0;
const BALL_RADIUS: f32 = 0.3;
const PADDLE_RADIUS: f32 = 0.3;
const PADDLE_HEIGHT: f32 = 2.0;
const PLANE_H: f32 = 30.0;
const PLANE_W: f32 = 40.0;
/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Paddle;
#[derive(Component)]
struct Ball;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            #[cfg(not(target_arch = "wasm32"))]
            WireframePlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, spawn_border))
        .add_systems(
            Update,
            (
                move_paddle,
                #[cfg(not(target_arch = "wasm32"))]
                toggle_wireframe,
                grab_mouse,
            ),
        )
        .run();
}





fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    let mut rapier_config = rapier_config.single_mut();
    // Set gravity to 0.0.
    rapier_config.gravity = Vec3::ZERO;

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });


    // ball
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh()),),
        MeshMaterial3d(debug_material.clone()),
        Transform::from_xyz(
            SHAPES_X_EXTENT / 2.  /  SHAPES_X_EXTENT,
            2.0,
            Z_EXTENT / 2.,
        ),
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
            coefficient: 0.8,
            combine_rule: CoefficientCombineRule::Max,
        },
        LockedAxes::TRANSLATION_LOCKED_Y,
    )).insert(Ccd::enabled());
    // paddle
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(PADDLE_RADIUS, PADDLE_HEIGHT).mesh()),),
        MeshMaterial3d(debug_material.clone()),
        Transform::from_xyz(
            -SHAPES_X_EXTENT / 2.  /  SHAPES_X_EXTENT,
            2.0,
            Z_EXTENT / 2.,
        )
        .with_rotation(Quat::from_rotation_x(-PI / 2.)),
        Paddle,
        RigidBody::KinematicPositionBased,
        CollidingEntities::default(),
        Collider::capsule_y(PADDLE_HEIGHT / 2.0, PADDLE_RADIUS),
        LockedAxes::TRANSLATION_LOCKED_Y,

    )).insert(Ccd::enabled());

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(-4.0, 20.0, 2.0),
    ));

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(PLANE_H, PLANE_W).subdivisions(4))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 37., 0.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        Text::new("Press space to toggle wireframes"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn move_paddle(
    mut query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    ) {
    for mut transform in &mut query {
        transform.rotate_y(accumulated_mouse_scroll.delta.y * time.delta_secs() * 3.0);
        transform.translation.x += accumulated_mouse_motion.delta.y * time.delta_secs();
        transform.translation.z -= accumulated_mouse_motion.delta.x * time. delta_secs();

    }
}


fn spawn_border(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let border_material = materials.add(StandardMaterial {
        base_color: Color::from(RED),
        ..default()
    });

    // upper border
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new( 1.0, 5.0, PLANE_W).mesh()),),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz( -15.5, 0.0, 0.0),
        Collider::cuboid(1.0, 2.5, PLANE_W / 2.0,),
    ));
    //
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new( PLANE_H, 5.0, 1.0).mesh()),),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz( -0.0, 0.0, -20.5),
        Collider::cuboid( PLANE_H / 2.0 , 2.5, 0.5),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new( PLANE_H, 5.0, 1.0).mesh()),),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz( -0.0, 0.0, 20.5),
        Collider::cuboid(PLANE_H / 2.0, 2.5, 0.5),
    ));
    //  lower border
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new( 0.0, 5.0, PLANE_W).mesh()),),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            //alpha_mode: AlphaMode::Mask(0.0),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz( 15.5, 0.0, 0.0),
        Collider::cuboid(0.0, 2.5, PLANE_W / 2.0,),
        //Sensor::default(),
    ));
}


/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn grab_mouse(
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}
