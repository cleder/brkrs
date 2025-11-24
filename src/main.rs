//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.

mod level_loader;
mod systems;

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{
    color::palettes::{basic::SILVER, css::RED},
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;

const BALL_RADIUS: f32 = 0.3;
const PADDLE_RADIUS: f32 = 0.3;
const PADDLE_HEIGHT: f32 = 3.0;
const PLANE_H: f32 = 30.0;
const PLANE_W: f32 = 40.0;

// Bounce/impulse tuning
// How strongly the wall collision pushes the ball (ExternalImpulse on balls)
const BALL_WALL_IMPULSE_FACTOR: f32 = 0.001;
// How strongly the paddle bounces back when hitting a wall
const PADDLE_BOUNCE_WALL_FACTOR: f32 = 0.03;
// How strongly the paddle bounces back when hitting a brick (separate from walls)
const PADDLE_BOUNCE_BRICK_FACTOR: f32 = 0.02;

// Grid debug overlay constants (22x22 grid covering PLANE_H × PLANE_W)
const GRID_WIDTH: usize = 22; // Columns (Z-axis)
const GRID_HEIGHT: usize = 22; // Rows (X-axis)
const CELL_WIDTH: f32 = PLANE_W / GRID_WIDTH as f32; // ~1.818 (Z dimension)
const CELL_HEIGHT: f32 = PLANE_H / GRID_HEIGHT as f32; // ~1.364 (X dimension)
                                                       // Cell aspect ratio: CELL_HEIGHT / CELL_WIDTH = 30/40 * 22/22 = 3/4 = 0.75
/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Paddle;
#[derive(Component)]
struct Ball;
#[derive(Component)]
struct Border;
#[derive(Component)]
struct LowerGoal;
#[derive(Component)]
struct GridOverlay;
#[derive(Component)]
struct Brick;
#[derive(Component)]
struct MarkedForDespawn;

#[derive(Event)]
struct WallHit {
    pub impulse: Vec3,
}

#[derive(Event)]
struct BrickHit {
    pub impulse: Vec3,
}

#[derive(Event)]
struct BallHit {
    pub impulse: Vec3,
    pub ball: Entity,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            #[cfg(not(target_arch = "wasm32"))]
            WireframePlugin::default(),
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(crate::level_loader::LevelLoaderPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (setup, spawn_border, systems::grid_debug::spawn_grid_overlay),
        )
        .add_systems(
            Update,
            (
                move_paddle,
                #[cfg(not(target_arch = "wasm32"))]
                toggle_wireframe,
                #[cfg(not(target_arch = "wasm32"))]
                systems::grid_debug::toggle_grid_visibility,
                grab_mouse,
                read_character_controller_collisions,
                despawn_ball_on_lower_goal_collision,
                mark_brick_on_ball_collision,
                despawn_marked_entities, // Runs after marking, allowing physics to resolve
                                         // display_events,
            ),
        )
        .add_observer(on_wall_hit)
        .add_observer(on_paddle_ball_hit)
        .add_observer(on_brick_hit)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    let rapier_config = rapier_config.single_mut();
    // Set gravity to 0.0.
    rapier_config.unwrap().gravity = Vec3::new(15.0, 0.0, 0.0);

    let _debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    // Level entities (paddle, ball, bricks) are spawned by LevelLoaderPlugin after level parsing.

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
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(PLANE_H, PLANE_W)
                    .subdivisions(4),
            ),
        ),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 37., 0.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        Text::new("Press space to toggle wire frames"),
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
    mut controllers: Query<&mut KinematicCharacterController>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if !window.focused {
        return;
    }
    let _sensitivity = 100.0 / window.height().min(window.width());
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(
            Vec3::new(
                accumulated_mouse_motion.delta.y,
                0.0,
                -accumulated_mouse_motion.delta.x,
            ) * 0.000_4
                / time.delta_secs(),
        );
    }
    // The built-in character controller does not support rotational movement.
    for mut transform in &mut query {
        transform.rotate_y(accumulated_mouse_scroll.delta.y * time.delta_secs() * 3.0);
        transform.translation.y = 2.0; // force the paddle to stay at the same height

        // Constrain paddle to play area bounds (with some padding for paddle size)
        let padding = PADDLE_HEIGHT / 2.0;
        let x_min = -PLANE_H / 2.0 + padding;
        let x_max = PLANE_H / 2.0 - padding;
        let z_min = -PLANE_W / 2.0 + padding;
        let z_max = PLANE_W / 2.0 - padding;

        transform.translation.x = transform.translation.x.clamp(x_min, x_max);
        transform.translation.z = transform.translation.z.clamp(z_min, z_max);
    }
}

fn spawn_border(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    _images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let border_material = materials.add(StandardMaterial {
        base_color: Color::from(RED),
        ..default()
    });

    // upper border
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 5.0, PLANE_W).mesh())),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz(-15.5, 0.0, 0.0),
        Collider::cuboid(1.0, 2.5, PLANE_W / 2.0),
        Border,
    ));
    //
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(PLANE_H, 5.0, 1.0).mesh())),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz(-0.0, 0.0, -20.5),
        Collider::cuboid(PLANE_H / 2.0, 2.5, 0.5),
        Border,
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(PLANE_H, 5.0, 1.0).mesh())),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz(-0.0, 0.0, 20.5),
        Collider::cuboid(PLANE_H / 2.0, 2.5, 0.5),
        Border,
    ));
    //  lower border
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.0, 5.0, PLANE_W).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.0, 0.0, 1.0),
            //alpha_mode: AlphaMode::Mask(0.0),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(15.5, 0.0, 0.0),
        Collider::cuboid(0.0, 2.5, PLANE_W / 2.0),
        //Sensor::default(),
        Border,
        LowerGoal,
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

/// Despawn the ball when it collides with the lower goal border
fn despawn_ball_on_lower_goal_collision(
    mut collision_events: EventReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    lower_goals: Query<Entity, With<LowerGoal>>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let e1_is_ball = balls.get(*e1).is_ok();
            let e2_is_ball = balls.get(*e2).is_ok();
            let e1_is_lower = lower_goals.get(*e1).is_ok();
            let e2_is_lower = lower_goals.get(*e2).is_ok();

            // If collision is between a Ball and a LowerGoal, despawn the ball
            if e1_is_ball && e2_is_lower {
                commands.entity(*e1).despawn();
            } else if e2_is_ball && e1_is_lower {
                commands.entity(*e2).despawn();
            }
        }
    }
}

/// Mark bricks for despawn when hit by the ball
/// This allows the physics collision response to complete before removal
fn mark_brick_on_ball_collision(
    mut collision_events: EventReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    bricks: Query<Entity, (With<Brick>, Without<MarkedForDespawn>)>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let e1_is_ball = balls.get(*e1).is_ok();
            let e2_is_ball = balls.get(*e2).is_ok();
            let e1_is_brick = bricks.get(*e1).is_ok();
            let e2_is_brick = bricks.get(*e2).is_ok();

            // If collision is between a Ball and a Brick, mark the brick for despawn
            // This ensures the physics collision response happens first
            if e1_is_ball && e2_is_brick {
                commands.entity(*e2).insert(MarkedForDespawn);
            } else if e2_is_ball && e1_is_brick {
                commands.entity(*e1).insert(MarkedForDespawn);
            }
        }
    }
}

/// Despawn entities marked for removal (runs after physics step)
fn despawn_marked_entities(marked: Query<Entity, With<MarkedForDespawn>>, mut commands: Commands) {
    for entity in marked.iter() {
        commands.entity(entity).despawn();
    }
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
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if !window.focused {
        return;
    }
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}

/* Read the character controller collisions stored in the character controller’s output. */
fn read_character_controller_collisions(
    paddle_outputs: Query<&KinematicCharacterControllerOutput, With<Paddle>>,
    walls: Query<Entity, With<Border>>,
    bricks: Query<Entity, With<Brick>>,
    balls: Query<Entity, With<Ball>>,
    time: Res<Time>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut commands: Commands,
) {
    let output = match paddle_outputs.single() {
        Ok(controller) => controller,
        Err(_) => return,
    };
    for collision in output.collisions.iter() {
        // paddle collides with the walls
        for wall in walls.iter() {
            if collision.entity == wall {
                commands.trigger(WallHit {
                    impulse: (collision.translation_applied + collision.translation_remaining)
                        / time.delta_secs(),
                });
            }
        }
    }
    for collision in output.collisions.iter() {
        // paddle collides with the bricks: emit BrickHit (separate from walls)
        for brick in bricks.iter() {
            if collision.entity == brick {
                commands.trigger(BrickHit {
                    impulse: (collision.translation_applied + collision.translation_remaining)
                        / time.delta_secs(),
                });
            }
        }
    }
    for collision in output.collisions.iter() {
        // paddle collides with the balls
        for ball in balls.iter() {
            if collision.entity == ball {
                // println!("hit ball {:?}", ball);
                println!("collision {:?}", collision);
                commands.trigger(BallHit {
                    impulse: Vec3::new(
                        accumulated_mouse_motion.delta.y,
                        0.0,
                        -accumulated_mouse_motion.delta.x,
                    ) / time.delta_secs(),
                    ball,
                });
            }
        }
    }
}

fn on_wall_hit(
    trigger: Trigger<WallHit>,
    mut balls: Query<&mut ExternalImpulse, With<Ball>>,
    mut controllers: Query<&mut KinematicCharacterController, With<Paddle>>,
) {
    let event = trigger.event();

    // give the balls an impulse
    for mut impulse in balls.iter_mut() {
        impulse.impulse = event.impulse * BALL_WALL_IMPULSE_FACTOR;
    }

    // let the paddle bounce back on wall collisions as well
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(-event.impulse * PADDLE_BOUNCE_WALL_FACTOR);
    }
}

fn on_brick_hit(
    trigger: Trigger<BrickHit>,
    mut controllers: Query<&mut KinematicCharacterController, With<Paddle>>,
) {
    let event = trigger.event();

    // let the paddle bounce back on brick collisions only
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(-event.impulse * PADDLE_BOUNCE_BRICK_FACTOR);
    }
}

fn on_paddle_ball_hit(
    trigger: Trigger<BallHit>,
    mut balls: Query<(Entity, &mut ExternalImpulse), With<Ball>>,
) {
    let event = trigger.event();
    println!("Received ball hit event: {:?}", event.impulse);

    // give the balls an impulse with "english" - paddle rotation affects ball trajectory
    // Tuned multiplier for noticeable but controlled steering effect
    for (ball, mut impulse) in balls.iter_mut() {
        if ball == event.ball {
            impulse.impulse = event.impulse * 0.001; // Increased from 0.000_2 for more noticeable effect
        }
    }
}
