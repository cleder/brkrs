use bevy::prelude::*;
use brkrs::level_loader::{CurrentLevel, LevelDefinition};
use brkrs::systems::gravity::gravity_configuration_loader_system;
use brkrs::GravityConfiguration;

#[test]
fn test_loader_does_not_overwrite_current_after_initial_load() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Insert a test level with gravity = (2,0,0)
    let level_def = LevelDefinition {
        number: 42,
        gravity: Some((2.0, 0.0, 0.0)),
        matrix: vec![vec![]],
        #[cfg(feature = "texture_manifest")]
        presentation: None,
        description: None,
        author: None,
    };

    app.insert_resource(CurrentLevel(level_def));
    app.init_resource::<GravityConfiguration>();

    // Register loader system and run one update to load defaults
    app.add_systems(Update, gravity_configuration_loader_system);
    app.update();

    // After initial load, both level_default and current should be set to (2,0,0)
    let cfg = app.world().resource::<GravityConfiguration>();
    assert_eq!(cfg.level_default, Vec3::new(2.0, 0.0, 0.0));
    assert_eq!(cfg.current, Vec3::new(2.0, 0.0, 0.0));
    assert_eq!(cfg.last_level_number, Some(42));

    // Simulate a runtime gravity change (e.g., gravity brick destroyed)
    {
        let mut cfg_mut = app.world_mut().resource_mut::<GravityConfiguration>();
        cfg_mut.current = Vec3::new(10.0, 0.0, 0.0);
    }

    // Run loader again (should NOT overwrite current because level number unchanged)
    app.update();

    let cfg2 = app.world().resource::<GravityConfiguration>();
    assert_eq!(cfg2.current, Vec3::new(10.0, 0.0, 0.0));
}
