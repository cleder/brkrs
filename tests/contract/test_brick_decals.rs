//! Contract test for brick-type-decals: verifies all brick types in the test level have correct decals assigned.

use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use brkrs::level_format::brick_types::{BrickType, Decal};

#[test]
fn all_brick_types_have_correct_decals() {
    // Setup a minimal Bevy app for testing
    let mut app = App::new();
    // Add required plugins and resources
    app.add_plugins(MinimalPlugins);
    // Add brick types and decals (mocked for test)
    // This should be replaced with actual asset loading and level setup
    app.world.spawn((BrickType::Standard, Decal::default()));
    app.world.spawn((BrickType::Indestructible, Decal::default()));
    app.world.spawn((BrickType::MultiHit, Decal::default()));
    // Query all bricks and check for decal assignment
    let mut state: SystemState<Query<(&BrickType, &Decal)>> = SystemState::new(&mut app.world);
    let query = state.get(&app.world);
    for (brick_type, decal) in query.iter() {
        // Replace with actual decal validation logic
        assert!(decal.is_valid_for_type(brick_type), "Decal not valid for brick type: {:?}", brick_type);
    }
}
