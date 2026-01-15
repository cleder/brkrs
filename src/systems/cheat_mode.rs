use bevy::ecs::message::{Message, MessageWriter};
use bevy::prelude::*;

#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct CheatModeState {
    pub active: bool,
    pub activated_at: Option<f64>,
}

impl CheatModeState {
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn activate(&mut self, now: f64) {
        self.active = true;
        self.activated_at = Some(now);
    }

    pub fn deactivate(&mut self, now: f64) {
        self.active = false;
        self.activated_at = Some(now);
    }

    pub fn toggle(&mut self, now: f64) {
        if self.active {
            self.deactivate(now);
        } else {
            self.activate(now);
        }
    }
}

#[derive(Message, Debug, Clone, Copy)]
pub struct CheatModeToggled {
    pub active: bool,
}

/// Plugin skeleton for cheat mode systems
pub struct CheatModePlugin;

impl Plugin for CheatModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CheatModeState>()
            .add_message::<CheatModeToggled>()
            // Run the toggle producer in PreUpdate so it observes `just_pressed` reliably
            .add_systems(
                PreUpdate,
                toggle_cheat_mode_input.run_if(crate::pause::not_paused),
            );
    }
}

/// Toggle cheat mode when G is pressed during gameplay
fn toggle_cheat_mode_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cheat: ResMut<CheatModeState>,
    mut toggle_events: MessageWriter<CheatModeToggled>,
    mut score_state: ResMut<crate::systems::scoring::ScoreState>,
    mut commands: Commands,
    overlays: Query<Entity, With<crate::ui::game_over_overlay::GameOverOverlay>>,
    mut lives_state: Option<ResMut<crate::systems::respawn::LivesState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        let now = time.elapsed_secs_f64();
        cheat.toggle(now);
        toggle_events.write(CheatModeToggled {
            active: cheat.is_active(),
        });
        // Reset score on both enter and exit per spec
        crate::systems::scoring::reset_score(&mut score_state);
        tracing::info!("Cheat mode toggled: {}", cheat.is_active());

        // On activation, reset lives and remove any game-over overlay so player can resume
        if cheat.is_active() {
            if let Some(lives) = lives_state.as_mut() {
                lives.lives_remaining = 3;
                lives.on_last_life = false;
                tracing::info!("Lives reset to 3 due to cheat activation");
            }
            for ent in overlays.iter() {
                commands.entity(ent).despawn();
                tracing::info!("Removed GameOverOverlay due to cheat activation");
            }
        }
    }
}
