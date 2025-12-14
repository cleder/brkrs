//! UI font loading utilities (desktop vs. WASM).
//! On desktop we load immediately at startup; on WASM we defer and wait for the asset to finish
//! loading to avoid using an unavailable font handle.

use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use bevy::asset::LoadState;

#[derive(Resource, Clone)]
pub struct UiFonts {
    pub orbitron: Handle<Font>,
}

/// Load UI fonts at startup (desktop only).
/// On WASM, font loading is skipped at startup to avoid blocking texture loading.
#[cfg(not(target_arch = "wasm32"))]
pub fn load_ui_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let orbitron = asset_server.load("fonts/Orbitron/Orbitron-VariableFont_wght.ttf");
    commands.insert_resource(UiFonts { orbitron });
}

/// No-op on WASM; the Update system will handle loading.
#[cfg(target_arch = "wasm32")]
pub fn load_ui_fonts(_commands: Commands, _asset_server: Res<AssetServer>) {}

/// Ensure UI fonts are loaded (runs in Update schedule, WASM only).
/// This defers font loading on WASM to avoid blocking textures at startup and only inserts the
/// resource once the asset is ready.
#[cfg(target_arch = "wasm32")]
pub fn ensure_ui_fonts_loaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: Option<Res<UiFonts>>,
    mut pending: Local<Option<Handle<Font>>>,
) {
    // Already available
    if fonts.is_some() {
        *pending = None;
        return;
    }

    let handle = pending
        .get_or_insert_with(|| asset_server.load("fonts/Orbitron/Orbitron-VariableFont_wght.ttf"))
        .clone();

    match asset_server.get_load_state(&handle) {
        LoadState::Loaded => {
            commands.insert_resource(UiFonts {
                orbitron: handle.clone(),
            });
            *pending = None;
        }
        LoadState::Failed => {
            warn!("Failed to load Orbitron font for UI; overlay text will be missing");
            *pending = None;
        }
        _ => {
            // Still loading; keep waiting.
        }
    }
}

/// No-op on desktop; fonts are loaded during Startup.
#[cfg(not(target_arch = "wasm32"))]
pub fn ensure_ui_fonts_loaded(
    _commands: Commands,
    _asset_server: Res<AssetServer>,
    _fonts: Option<Res<UiFonts>>,
    _pending: Local<Option<Handle<Font>>>,
) {
}
