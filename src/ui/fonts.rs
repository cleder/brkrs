use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct UiFonts {
    pub orbitron: Handle<Font>,
}

/// Load UI fonts at startup (desktop only).
/// On WASM, font loading is skipped at startup to avoid blocking texture loading.
pub fn load_ui_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let orbitron = asset_server.load("fonts/Orbitron/Orbitron-VariableFont_wght.ttf");
        commands.insert_resource(UiFonts { orbitron });
    }
}

/// Ensure UI fonts are loaded (runs in Update schedule, WASM only).
/// This defers font loading on WASM to avoid blocking textures at startup.
pub fn ensure_ui_fonts_loaded(
    #[cfg(target_arch = "wasm32")] mut commands: Commands,
    #[cfg(target_arch = "wasm32")] asset_server: Res<AssetServer>,
    #[cfg(target_arch = "wasm32")] fonts: Option<Res<UiFonts>>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        if fonts.is_none() {
            let orbitron = asset_server.load("fonts/Orbitron/Orbitron-VariableFont_wght.ttf");
            commands.insert_resource(UiFonts { orbitron });
        }
    }
}
