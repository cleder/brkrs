use bevy::ecs::message::{Message, MessageWriter};
use bevy::prelude::*;
use std::path::{Path, PathBuf};
use tracing::info;
#[cfg(not(target_arch = "wasm32"))]
use tracing::warn;

/// Message emitted when any source requests a level switch.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelSwitchRequested {
    pub source: LevelSwitchSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelSwitchSource {
    Keyboard,
    Automation,
}

/// Ordered list of playable level files plus bookkeeping for pending transitions.
#[derive(Resource, Debug)]
pub struct LevelSwitchState {
    ordered_levels: Vec<LevelSlot>,
    trigger_file: PathBuf,
    pending_transition: bool,
}

impl Default for LevelSwitchState {
    fn default() -> Self {
        Self {
            ordered_levels: discover_level_slots(),
            trigger_file: PathBuf::from(".level-switch-next"),
            pending_transition: false,
        }
    }
}

impl LevelSwitchState {
    pub fn ordered_levels(&self) -> &[LevelSlot] {
        &self.ordered_levels
    }

    pub fn next_level_after(&self, current: u32) -> Option<&LevelSlot> {
        if self.ordered_levels.is_empty() {
            return None;
        }
        self.ordered_levels
            .iter()
            .find(|slot| slot.number > current)
            .or_else(|| self.ordered_levels.first())
    }

    pub fn mark_transition_start(&mut self) {
        self.pending_transition = true;
    }

    pub fn mark_transition_end(&mut self) {
        self.pending_transition = false;
    }

    pub fn is_transition_pending(&self) -> bool {
        self.pending_transition
    }

    pub fn trigger_file(&self) -> &Path {
        &self.trigger_file
    }

    pub fn set_trigger_file(&mut self, path: PathBuf) {
        self.trigger_file = path;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LevelSlot {
    pub number: u32,
    pub path: String,
}

fn discover_level_slots() -> Vec<LevelSlot> {
    let mut slots: Vec<LevelSlot> = Vec::new();
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(entries) = std::fs::read_dir("assets/levels") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if let Some(num) = parse_level_number(name) {
                        slots.push(LevelSlot {
                            number: num,
                            path: format!("assets/levels/{name}"),
                        });
                    }
                }
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        // On WASM, hardcode the level list since there's no filesystem access
        for i in 1..=74 {
            slots.push(LevelSlot {
                number: i,
                path: format!("assets/levels/level_{:03}.ron", i),
            });
        }
        // Add special debug levels
        slots.push(LevelSlot {
            number: 997,
            path: "assets/levels/level_997.ron".to_string(),
        });
        slots.push(LevelSlot {
            number: 998,
            path: "assets/levels/level_998.ron".to_string(),
        });
        slots.push(LevelSlot {
            number: 999,
            path: "assets/levels/level_999.ron".to_string(),
        });
    }
    if slots.is_empty() {
        slots.push(LevelSlot {
            number: 1,
            path: "assets/levels/level_001.ron".to_string(),
        });
        info!(target: "level_switch", "No level files discovered; defaulting to level_001 only");
    }
    slots.sort_by_key(|slot| slot.number);
    slots
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_level_number(file_name: &str) -> Option<u32> {
    let prefix = "level_";
    let suffix = ".ron";
    if !file_name.starts_with(prefix) || !file_name.ends_with(suffix) {
        return None;
    }
    let number_part = &file_name[prefix.len()..file_name.len() - suffix.len()];
    number_part.parse::<u32>().ok()
}

pub struct LevelSwitchPlugin;

impl Plugin for LevelSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LevelSwitchRequested>()
            .init_resource::<LevelSwitchState>()
            .add_systems(Update, (queue_keyboard_requests, poll_contract_trigger));
    }
}

fn queue_keyboard_requests(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut events: MessageWriter<LevelSwitchRequested>,
) {
    if keyboard.just_pressed(KeyCode::KeyL) {
        events.write(LevelSwitchRequested {
            source: LevelSwitchSource::Keyboard,
        });
    }
}

fn poll_contract_trigger(
    #[cfg(not(target_arch = "wasm32"))] state: Res<LevelSwitchState>,
    #[cfg(not(target_arch = "wasm32"))] mut events: MessageWriter<LevelSwitchRequested>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = state.trigger_file().to_path_buf();
        if path.exists() {
            if let Err(err) = std::fs::remove_file(&path) {
                warn!(
                    target: "level_switch",
                    ?path,
                    ?err,
                    "Failed to remove automation trigger file"
                );
            }
            events.write(LevelSwitchRequested {
                source: LevelSwitchSource::Automation,
            });
        }
    }
}
