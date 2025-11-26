use bevy::prelude::*;

use brkrs::systems::textures::materials::{
    FallbackMaterial, FallbackRegistry, TextureMaterialsPlugin,
};

fn app_with_fallback_registry() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.add_plugins(TextureMaterialsPlugin);
    app.update();
    app
}

#[test]
fn log_once_warns_only_on_first_use() {
    let mut app = app_with_fallback_registry();
    let mut registry = app.world_mut().resource_mut::<FallbackRegistry>();

    assert!(registry.log_once("brick/glow"));
    assert!(!registry.log_once("brick/glow"));
    assert!(registry.log_once("ball/hologram"));
}

#[test]
fn fallback_handles_exist_for_all_material_classes() {
    let app = app_with_fallback_registry();
    let registry = app.world().resource::<FallbackRegistry>();
    let materials = app.world().resource::<Assets<StandardMaterial>>();

    for bucket in [
        FallbackMaterial::Ball,
        FallbackMaterial::Paddle,
        FallbackMaterial::Brick,
        FallbackMaterial::Sidewall,
        FallbackMaterial::Ground,
        FallbackMaterial::Background,
    ] {
        let handle = registry.handle(bucket);
        assert!(
            materials.get(handle).is_some(),
            "handle for {:?} should resolve to a baked material",
            bucket
        );
    }
}
