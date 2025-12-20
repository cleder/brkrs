use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;

use brkrs::signals::UiBeep;
use brkrs::systems::audio::AudioPlugin;

#[test]
fn audio_plugin_registers_uibeep_message() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AudioPlugin);

    // After adding the AudioPlugin, the buffered UiBeep message queue should exist.
    let exists = app.world().get_resource::<Messages<UiBeep>>().is_some();
    assert!(
        exists,
        "AudioPlugin must register Messages<UiBeep> (buffered message)"
    );
}
