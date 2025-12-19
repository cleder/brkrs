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

/// System sets for cheat mode organization.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheatModeSystems {
    /// Input handling for cheat mode toggle
    Input,
}

/// Plugin for cheat mode systems
pub struct CheatModePlugin;

impl Plugin for CheatModePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PreUpdate, CheatModeSystems::Input);
        
        app.init_resource::<CheatModeState>()
            .add_message::<CheatModeToggled>()
            .add_systems(
                PreUpdate,
                toggle_cheat_mode_input
                    .run_if(crate::pause::not_paused)
                    .in_set(CheatModeSystems::Input),
            );
    }
}

/// Toggle cheat mode when G is pressed during gameplay.
///
/// # Purpose
///
/// Provides debug mode toggle for testing and development. When activated,
/// cheat mode resets the score and lives, allowing unrestricted gameplay.
///
/// # When to Use
///
/// Automatically registered by CheatModePlugin. Runs in PreUpdate schedule
/// to observe `just_pressed` reliably.
///
/// # Behavior
///
/// - Press G to toggle cheat mode on/off
/// - On activation: Reset score to 0, reset lives to 3, remove game-over overlay
/// - On deactivation: Reset score to 0 (score during cheat mode doesn't count)
/// - Emits CheatModeToggled message for UI/audio feedback
fn toggle_cheat_mode_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cheat: ResMut<CheatModeState>,
    mut toggle_events: MessageWriter<CheatModeToggled>,
    mut score_state: ResMut<crate::systems::scoring::ScoreState>,
    mut commands: Commands,
    overlays: Query<Entity, With<crate::ui::game_over_overlay::GameOverOverlay>>,
    mut lives_state: Option<ResMut<crate::systems::respawn::LivesState>>,
) -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}
