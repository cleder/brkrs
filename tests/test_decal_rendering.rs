//! Integration test for brick-type-decals: verifies decals are visible and centered on the top side of each brick.

use bevy::prelude::*;
use brkrs::level_format::brick_types::{BrickType, Decal};

#[test]
fn decals_are_visible_and_centered() {
    // Setup a minimal Bevy app for testing
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    // Spawn bricks with decals (mocked for test)
    let brick_entity = app
        .world_mut()
        .spawn((BrickType::Standard, Decal::default()))
        .id();
    // Query for decal and check position
    let decal = app
        .world()
        .get::<Decal>(brick_entity)
        .expect("Decal missing");
    // Replace with actual position/visibility logic
    assert!(
        decal.is_centered_on_top(),
        "Decal is not centered on top side"
    );
    assert!(decal.is_visible(), "Decal is not visible");
}
