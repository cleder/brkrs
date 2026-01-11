use bevy::prelude::SceneSpawner;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use brkrs::signals::BrickDestroyed;
use brkrs::systems::gravity::*;
use brkrs::systems::GravityChanged;
use brkrs::GravityConfiguration;

#[test]
fn test_gravity_change_affects_ball_velocity() {
    let mut app = App::new();
    // Minimal + rapier
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    // Messages and resources
    app.add_message::<BrickDestroyed>();
    app.add_message::<GravityChanged>();
    app.init_resource::<GravityConfiguration>();

    // Add minimal assets resources required by Rapier/Bevy collider init
    app.world_mut().insert_resource(Assets::<Mesh>::default());
    app.world_mut()
        .insert_resource(Assets::<StandardMaterial>::default());
    app.world_mut().insert_resource(SceneSpawner::default());

    // Register gravity systems (same order as in lib)
    app.add_systems(Update, gravity_configuration_loader_system);
    app.add_systems(Update, brick_destruction_gravity_handler);
    app.add_systems(
        Update,
        gravity_application_system.after(brick_destruction_gravity_handler),
    );
    app.add_systems(
        Update,
        apply_gravity_to_physics.after(gravity_application_system),
    );

    // Spawn a dynamic ball at rest
    let ball_entity = app
        .world_mut()
        .spawn((
            RigidBody::Dynamic,
            Collider::ball(0.5),
            Velocity::zero(),
            ExternalImpulse::default(),
            GravityScale(1.0),
            // place it above plane so gravity can accelerate it
            Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
            GlobalTransform::from_translation(Vec3::ZERO),
        ))
        .id();

    // Ensure initial gravity is zero
    {
        let mut cfg = app.world_mut().resource_mut::<GravityConfiguration>();
        cfg.level_default = Vec3::ZERO;
        cfg.current = Vec3::ZERO;
    }

    // Directly send a GravityChanged message for a strong X-axis gravity
    {
        let mut msgs = app.world_mut().resource_mut::<Messages<GravityChanged>>();
        msgs.write(GravityChanged::new(Vec3::new(10.0, 0.0, 0.0)));
    }

    // Step several frames and observe ball velocity
    let mut velocities = Vec::new();
    for _ in 0..10 {
        app.update();
        // after physics step, query velocity
        let entity_ref = app.world().entity(ball_entity);
        let vel = entity_ref.get::<Velocity>().unwrap().linvel;
        velocities.push(vel);
    }

    // Expect that X velocity increases (becomes non-zero positive)
    let final_vx = velocities.last().unwrap().x;
    assert!(
        final_vx.abs() > 0.01,
        "Expected ball X velocity to change due to gravity, but got {}",
        final_vx
    );
}
