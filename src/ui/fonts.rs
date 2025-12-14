use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct UiFonts {
    pub orbitron: Handle<Font>,
}

pub fn load_ui_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let orbitron = asset_server.load("fonts/Orbitron/Orbitron-VariableFont_wght.ttf");
    commands.insert_resource(UiFonts { orbitron });
}
