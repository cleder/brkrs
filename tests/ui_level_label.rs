//! Tests for level label update behavior.
//!
//! Purpose: Verify level label updates via both observer-driven (LevelStarted event)
//! and resource sync (CurrentLevel change detection) paths. Ensures safe error handling
//! when resources or entities are missing.
//!
//! Constitution VII: Red test commit required before implementation.

use bevy::prelude::*;

use brkrs::level_loader::{CurrentLevel, LevelDefinition};
use brkrs::systems::LevelStarted;
use brkrs::ui::fonts::UiFonts;
use brkrs::ui::level_label::{
    on_level_started, spawn_level_label, sync_with_current_level, AccessibilityAnnouncement,
    LevelLabelText,
};

// Helper to create minimal LevelDefinition for testing
fn test_level(number: u32) -> LevelDefinition {
    LevelDefinition {
        number,
        gravity: None,
        matrix: vec![],
        #[cfg(feature = "texture_manifest")]
        presentation: None,
        description: None,
        author: None,
    }
}

/// Test that level label updates via LevelStarted observer (event-driven path).
///
/// Scenario:
/// 1. Spawn level label with initial level "Level 1"
/// 2. Trigger LevelStarted event with level_index=2
/// 3. Expected: Label updates to "Level 2", accessibility announcement recorded
#[test]
fn level_label_updates_on_level_started_event() {
    let mut app = App::new();

    // Add required resources
    app.insert_resource(UiFonts {
        orbitron: Handle::default(),
    });
    app.insert_resource(AccessibilityAnnouncement::default());
    app.insert_resource(CurrentLevel(test_level(1)));

    // Register systems
    app.add_systems(Update, spawn_level_label);
    app.add_observer(on_level_started);

    // Step 1: Spawn label with initial level
    app.update();

    // Verify label spawned with "Level 1"
    let text = app
        .world_mut()
        .query_filtered::<&Text, With<LevelLabelText>>()
        .iter(app.world())
        .next()
        .expect("Label should exist");
    assert_eq!(**text, "Level 1", "Initial label should be 'Level 1'");

    // Step 2: Trigger LevelStarted event with level_index=2
    app.world_mut().trigger(LevelStarted { level_index: 2 });
    app.update();

    // Verify label updated to "Level 2"
    let text_after = app
        .world_mut()
        .query_filtered::<&Text, With<LevelLabelText>>()
        .iter(app.world())
        .next()
        .expect("Label should exist");
    assert_eq!(
        **text_after, "Level 2",
        "Label should update to 'Level 2' after LevelStarted event"
    );

    // Verify accessibility announcement recorded
    let announcement = app.world().resource::<AccessibilityAnnouncement>();
    assert_eq!(
        announcement.last,
        Some("Level 2".to_string()),
        "Accessibility announcement should be 'Level 2'"
    );
}

/// Test that level label updates via CurrentLevel resource sync (change detection path).
///
/// Scenario:
/// 1. Spawn level label with initial level "Level 1"
/// 2. Change CurrentLevel resource to level 3
/// 3. Expected: Label updates to "Level 3", accessibility announcement recorded
#[test]
fn level_label_syncs_on_current_level_change() {
    let mut app = App::new();

    // Add required resources
    app.insert_resource(UiFonts {
        orbitron: Handle::default(),
    });
    app.insert_resource(AccessibilityAnnouncement::default());
    app.insert_resource(CurrentLevel(test_level(1)));

    // Register systems
    app.add_systems(Update, (spawn_level_label, sync_with_current_level));

    // Step 1: Spawn label with initial level
    app.update();

    // Verify label spawned with "Level 1"
    let text = app
        .world_mut()
        .query_filtered::<&Text, With<LevelLabelText>>()
        .iter(app.world())
        .next()
        .expect("Label should exist");
    assert_eq!(**text, "Level 1", "Initial label should be 'Level 1'");

    // Step 2: Change CurrentLevel to level 3
    app.insert_resource(CurrentLevel(test_level(3)));
    app.update();

    // Verify label updated to "Level 3"
    let text_after = app
        .world_mut()
        .query_filtered::<&Text, With<LevelLabelText>>()
        .iter(app.world())
        .next()
        .expect("Label should exist");
    assert_eq!(
        **text_after, "Level 3",
        "Label should update to 'Level 3' after CurrentLevel change"
    );

    // Verify accessibility announcement recorded
    let announcement = app.world().resource::<AccessibilityAnnouncement>();
    assert_eq!(
        announcement.last,
        Some("Level 3".to_string()),
        "Accessibility announcement should be 'Level 3'"
    );
}

/// Test that level label systems handle missing label entity gracefully (no crash).
///
/// Scenario:
/// 1. Do NOT spawn level label entity
/// 2. Trigger LevelStarted event
/// 3. Expected: No crash, system continues (Constitution VIII: Error Recovery)
#[test]
fn level_label_handles_missing_entity_gracefully() {
    let mut app = App::new();

    // Add required resources (but no label entity)
    app.insert_resource(AccessibilityAnnouncement::default());
    app.insert_resource(CurrentLevel(test_level(1)));

    // Register observer (but NOT spawn system)
    app.add_observer(on_level_started);

    // Trigger event with missing label entity
    app.world_mut().trigger(LevelStarted { level_index: 2 });
    app.update();

    // Verify no crash (system handles missing entity gracefully)
    // Check that accessibility announcement was still recorded
    let announcement = app.world().resource::<AccessibilityAnnouncement>();
    assert_eq!(
        announcement.last,
        Some("Level 2".to_string()),
        "Accessibility announcement should be recorded even when label entity missing"
    );
}

/// Test that sync system handles missing AccessibilityAnnouncement resource gracefully.
///
/// Scenario:
/// 1. Spawn level label
/// 2. Remove AccessibilityAnnouncement resource
/// 3. Change CurrentLevel
/// 4. Expected: Label updates, no crash (Constitution VIII: Error Recovery)
#[test]
fn level_label_handles_missing_announcement_resource_gracefully() {
    let mut app = App::new();

    // Add required resources
    app.insert_resource(UiFonts {
        orbitron: Handle::default(),
    });
    app.insert_resource(AccessibilityAnnouncement::default());
    app.insert_resource(CurrentLevel(test_level(1)));

    // Register systems
    app.add_systems(Update, (spawn_level_label, sync_with_current_level));

    // Spawn label
    app.update();

    // Remove AccessibilityAnnouncement resource
    app.world_mut()
        .remove_resource::<AccessibilityAnnouncement>();

    // Change CurrentLevel (should still update label without crashing)
    app.insert_resource(CurrentLevel(test_level(5)));
    app.update();

    // Verify label updated to "Level 5"
    let text = app
        .world_mut()
        .query_filtered::<&Text, With<LevelLabelText>>()
        .iter(app.world())
        .next()
        .expect("Label should exist");
    assert_eq!(
        **text, "Level 5",
        "Label should update even when AccessibilityAnnouncement missing"
    );
}
