//! UI module for the game
//!
//! Contains UI components including the pause overlay.
//!
//! # Architecture
//!
//! UI systems in this module follow Bevy 0.17 best practices:
//! - All systems return `Result<(), UiSystemError>` for fallible execution.
//! - Queries use `?` operator for early returns on expected failures.
//! - UI updates are change-driven (`Changed<T>`) rather than per-frame.
//! - Assets are loaded once in startup systems and stored in resources.
//! - Systems are organized into system sets for reusability and parallelism.

use std::fmt;

/// Error type for UI systems.
///
/// Encapsulates failures in UI setup, entity lookup, or state updates.
/// All UI systems return `Result<(), UiSystemError>` and propagate errors with `?`.
#[derive(Debug, Clone)]
pub enum UiSystemError {
    /// A required UI entity was not found (e.g., `single()` or `single_mut()` failed).
    EntityNotFound(String),
    /// A required resource was not available.
    ResourceNotFound(String),
    /// A UI asset failed to load or was unavailable.
    AssetNotAvailable(String),
    /// A configuration or setup error in UI initialization.
    SetupError(String),
    /// Other errors.
    Other(String),
}

impl fmt::Display for UiSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntityNotFound(msg) => write!(f, "UI entity not found: {}", msg),
            Self::ResourceNotFound(msg) => write!(f, "UI resource not found: {}", msg),
            Self::AssetNotAvailable(msg) => write!(f, "UI asset not available: {}", msg),
            Self::SetupError(msg) => write!(f, "UI setup error: {}", msg),
            Self::Other(msg) => write!(f, "UI error: {}", msg),
        }
    }
}

impl std::error::Error for UiSystemError {}

pub mod cheat_indicator;
pub mod fonts;
pub mod game_over_overlay;
pub mod level_label;
pub mod lives_counter;
pub mod palette;
pub mod pause_overlay;
pub mod score_display;

// ============================================================================
// Result-Returning System Wrapper Pattern (Constitution VIII: Fallible Systems)
// ============================================================================
//
// Future UI systems should return `Result<(), UiSystemError>` for fallible operations.
// However, Bevy 0.17's `.add_systems()` expects functions with `()` return type.
//
// **Solution: Use wrapper functions in lib.rs registration:**
//
// Example (future implementation):
// ```rust
// // In src/ui/some_system.rs (returns Result):
// pub fn my_system_result(/* queries/resources */) -> Result<(), UiSystemError> {
//     // Fallible work
//     Ok(())
// }
//
// // In src/lib.rs (wrapper to register):
// fn my_system_wrapper(/* queries/resources */) {
//     if let Err(e) = ui::some_system::my_system_result(/* params */) {
//         warn!("UI system error: {}", e);
//         // Optionally reschedule, log diagnostic, or gracefully degrade.
//     }
// }
//
// app.add_systems(Update, my_system_wrapper);
// ```
//
// This pattern allows Result-returning systems without breaking Bevy 0.17 compatibility
// while maintaining clear error boundaries and fallible semantics in the UI module.

// ============================================================================
// UI Plugin (Constitution VIII: Plugin-Based Architecture)
// ============================================================================

use bevy::prelude::*;

/// System sets for organizing UI systems.
/// Constitution VIII: System Organization — use system sets with `*Systems` suffix.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiSystems {
    /// UI spawning systems (run once or idempotent).
    Spawn,
    /// UI update systems (change-driven or per-frame as needed).
    Update,
    /// Input-driven UI systems (palette, cheats).
    Input,
}

/// Self-contained UI plugin that registers all UI resources and systems.
/// Constitution VIII: Plugin-Based Architecture — keep related code together.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        use crate::systems::RespawnSystems;

        // Configure system sets
        app.configure_sets(
            Update,
            (
                UiSystems::Spawn,
                UiSystems::Update.after(RespawnSystems::Schedule),
                UiSystems::Input,
            ),
        );

        // Initialize resources
        app.init_resource::<palette::PaletteState>();
        app.init_resource::<palette::SelectedBrick>();
        app.insert_resource(level_label::AccessibilityAnnouncement::default());

        // UI asset initialization
        app.add_systems(Startup, setup_ui_assets);

        // UI spawn systems
        app.add_systems(
            Update,
            (
                score_display::spawn_score_display_system,
                lives_counter::spawn_lives_counter,
                level_label::spawn_level_label,
            )
                .in_set(UiSystems::Spawn),
        );

        // UI update systems (change-driven)
        app.add_systems(
            Update,
            (
                lives_counter::update_lives_counter,
                game_over_overlay::spawn_game_over_overlay,
                cheat_indicator::handle_cheat_indicator,
                level_label::sync_with_current_level,
                score_display::update_score_display_system
                    .after(crate::systems::scoring::detect_milestone_system),
            )
                .in_set(UiSystems::Update),
        );

        // Palette input systems
        app.add_systems(
            Update,
            (
                palette::toggle_palette,
                palette::ensure_palette_ui,
                palette::handle_palette_selection,
                palette::update_palette_selection_feedback,
                palette::update_ghost_preview,
                palette::place_bricks_on_drag,
            )
                .in_set(UiSystems::Input),
        );

        // Observer for level started events
        app.add_observer(level_label::on_level_started);
    }
}

/// Initialize UI assets at startup.
///
/// Creates and caches materials/textures used by UI systems:
/// - Ghost preview material for palette system
/// - Cheat indicator texture for cheat mode display
///
/// Constitution VIII: Asset Handle Reuse — load once at startup, reuse in update systems.
fn setup_ui_assets(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    use bevy::prelude::*;

    // Initialize ghost preview material (cached for palette system)
    let ghost_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.5, 0.5, 0.5, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    commands.insert_resource(palette::GhostPreviewMaterial {
        handle: ghost_material,
    });

    // Initialize cheat indicator texture (cached for cheat mode system)
    let cheat_texture = asset_server.load("textures/default/cheat-mode-128.png");
    commands.insert_resource(cheat_indicator::CheatIndicatorTexture {
        handle: cheat_texture,
    });
}
