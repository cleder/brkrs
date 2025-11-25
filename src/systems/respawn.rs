use bevy::prelude::*;

/// Plugin wiring the respawn system sets so future systems have stable ordering.
pub struct RespawnPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RespawnSystemSet {
    Detect,
    Schedule,
    Execute,
    Control,
}

impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                RespawnSystemSet::Detect,
                RespawnSystemSet::Schedule,
                RespawnSystemSet::Execute,
                RespawnSystemSet::Control,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                detect_ball_loss_placeholder.in_set(RespawnSystemSet::Detect),
                schedule_respawn_placeholder.in_set(RespawnSystemSet::Schedule),
                execute_respawn_placeholder.in_set(RespawnSystemSet::Execute),
                restore_control_placeholder.in_set(RespawnSystemSet::Control),
            ),
        );
    }
}

fn detect_ball_loss_placeholder() {}

fn schedule_respawn_placeholder() {}

fn execute_respawn_placeholder() {}

fn restore_control_placeholder() {}
