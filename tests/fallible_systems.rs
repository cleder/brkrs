//! Integration tests for plugin initialization
//!
//! These tests verify plugins initialize correctly with required resources.

use bevy::prelude::*;
use bevy_rapier3d::prelude::CollisionEvent;

#[test]
fn audio_plugin_requires_asset_server() {
    use brkrs::systems::audio::AudioPlugin;

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default()); // AssetPlugin adds AssetServer
    app.add_plugins(AudioPlugin);

    // Plugin should initialize without panic when AssetServer exists
    app.update();
}

#[test]
fn texture_plugin_requires_asset_server() {
    use brkrs::systems::textures::materials::TextureMaterialsPlugin;

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(TextureMaterialsPlugin);

    // Plugin should initialize without panic
    app.update();
}

#[test]
fn respawn_plugin_initializes_required_resources() {
    use brkrs::systems::respawn::{LivesState, RespawnPlugin};

    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins(MinimalPlugins);
    app.add_event::<CollisionEvent>();
    app.add_plugins(RespawnPlugin);

    // Plugin should initialize LivesState and other resources
    app.update();

    assert!(
        app.world().get_resource::<LivesState>().is_some(),
        "RespawnPlugin should initialize LivesState"
    );
}

#[test]
fn paddle_size_plugin_initializes() {
    use brkrs::systems::paddle_size::PaddleSizePlugin;

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_event::<CollisionEvent>();
    app.add_plugins(PaddleSizePlugin);

    // Plugin should initialize without panic
    app.update();
}

#[test]
fn physics_config_resources_are_inserted_by_production_plugins() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_event::<CollisionEvent>();
    app.add_plugins(brkrs::systems::respawn::RespawnPlugin);
    app.update();
    assert!(
        app.world()
            .get_resource::<brkrs::physics_config::BallPhysicsConfig>()
            .is_some(),
        "BallPhysicsConfig should be registered by RespawnPlugin"
    );
    assert!(
        app.world()
            .get_resource::<brkrs::physics_config::PaddlePhysicsConfig>()
            .is_some(),
        "PaddlePhysicsConfig should be registered by RespawnPlugin"
    );
    assert!(
        app.world()
            .get_resource::<brkrs::physics_config::BrickPhysicsConfig>()
            .is_some(),
        "BrickPhysicsConfig should be registered by RespawnPlugin"
    );
}
