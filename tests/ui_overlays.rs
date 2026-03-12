//! Tests for UI overlay behavior after legacy game-over overlay removal.

use bevy::prelude::*;
use bevy::window::WindowMode;

use brkrs::pause::PauseState;
use brkrs::systems::respawn::LivesState;
use brkrs::ui::fonts::UiFonts;
use brkrs::ui::pause_overlay::{spawn_pause_overlay, PauseOverlay};

#[test]
fn pause_overlay_spawns_when_paused() {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());

    app.insert_resource(UiFonts {
        orbitron: Handle::default(),
    });
    app.insert_resource(LivesState {
        lives_remaining: 3,
        on_last_life: false,
    });
    #[cfg(not(target_arch = "wasm32"))]
    app.insert_resource(PauseState::Paused {
        window_mode_before_pause: WindowMode::Windowed,
    });
    #[cfg(target_arch = "wasm32")]
    app.insert_resource(PauseState::Paused {});

    app.add_systems(Update, spawn_pause_overlay);
    app.update();
    app.update();

    let pause_count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(pause_count, 1, "pause overlay should spawn exactly once");
}
