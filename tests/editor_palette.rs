use bevy::prelude::*;
use bevy::MinimalPlugins;

use brkrs::systems::textures::loader::{ObjectClass, TextureManifest, TypeVariantDefinition};
use brkrs::systems::textures::TextureMaterialsPlugin;
use brkrs::systems::textures::{FallbackRegistry, ProfileMaterialBank, TypeVariantRegistry};

fn palette_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    // Provide asset containers used by the texture system and initialize texture plugin.
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.add_plugins(TextureMaterialsPlugin);
    // run once so TextureMaterialsPlugin initializes fallback/bank/registry resources
    app.update();
    app.insert_resource(ButtonInput::<KeyCode>::default());
    // Register the PaletteState resource
    app.init_resource::<brkrs::ui::palette::PaletteState>();
    app.add_systems(
        Update,
        (
            brkrs::ui::palette::toggle_palette,
            brkrs::ui::palette::ensure_palette_ui,
        ),
    );

    // Populate ProfileMaterialBank and TypeVariantRegistry with simple test handles so
    // palette previews can resolve a material handle for brick types 20 and 90.
    let (handle_a, handle_b) = {
        let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
        (
            materials.add(StandardMaterial {
                base_color: Color::srgba(0.5, 0.5, 0.5, 1.0),
                unlit: true,
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color: Color::srgba(0.9, 0.2, 0.2, 1.0),
                unlit: true,
                ..default()
            }),
        )
    };

    {
        let mut bank = app.world_mut().resource_mut::<ProfileMaterialBank>();
        bank.insert_for_tests("brick/type20", handle_a.clone());
        bank.insert_for_tests("brick/indestructible", handle_b.clone());
    }

    // Build a small runtime manifest to map Brick 20 -> brick/type20 and Brick 90 -> brick/indestructible
    let manifest = TextureManifest {
        profiles: Default::default(),
        type_variants: vec![
            TypeVariantDefinition {
                object_class: ObjectClass::Brick,
                type_id: 20,
                profile_id: "brick/type20".to_string(),
                emissive_color: None,
                animation: None,
            },
            TypeVariantDefinition {
                object_class: ObjectClass::Brick,
                type_id: 90,
                profile_id: "brick/indestructible".to_string(),
                emissive_color: None,
                animation: None,
            },
        ],
        level_overrides: Default::default(),
        level_switch: None,
    };

    // Rebuild registry to pick up our mappings
    {
        let world = app.world_mut();
        world.resource_scope(|world, mut registry: Mut<TypeVariantRegistry>| {
            world.resource_scope(|world, bank: Mut<ProfileMaterialBank>| {
                world.resource_scope(|_world, mut fallback: Mut<FallbackRegistry>| {
                    registry.rebuild(&manifest, &bank, &mut fallback);
                });
            });
        });
    }
    app
}

#[test]
fn pressing_p_opens_and_closes_palette() {
    let mut app = palette_test_app();

    // Initially closed
    app.update();
    let world = app.world_mut();
    assert!(
        world
            .query::<&brkrs::ui::palette::PaletteRoot>()
            .iter(world)
            .count()
            == 0
    );

    // Press P to open
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyP);
    }
    app.update();
    app.update();
    let world = app.world_mut();
    let found = world
        .query::<&brkrs::ui::palette::PaletteRoot>()
        .iter(world)
        .count();
    assert!(found == 1, "Palette should be spawned when P is pressed");

    // Verify palette contains the expected labels (simple / indestructible).
    fn contains_text(world: &mut World, needle: &str) -> bool {
        let mut q = world.query::<&Text>();
        for text in q.iter(world) {
            if text.0.contains(needle) {
                return true;
            }
        }
        false
    }

    assert!(
        contains_text(world, "20 — Simple Brick"),
        "Palette should include simple brick label"
    );
    assert!(
        contains_text(world, "90 — Indestructible"),
        "Palette should include indestructible label"
    );

    // Check that a preview component exists for both entries and contains material handles
    let mut previews = world.query::<(&brkrs::ui::palette::PalettePreview, Entity)>();
    let mut found_20 = None;
    let mut found_90 = None;
    for (p, _e) in previews.iter(world) {
        if p.type_id == 20 {
            found_20 = Some(p.material.clone());
        } else if p.type_id == 90 {
            found_90 = Some(p.material.clone());
        }
    }
    assert!(found_20.is_some(), "expected preview for type 20");
    assert!(found_90.is_some(), "expected preview for type 90");
    assert!(
        found_20.unwrap().is_some(),
        "expected material handle for type 20"
    );
    assert!(
        found_90.unwrap().is_some(),
        "expected material handle for type 90"
    );

    // Also assert that the 3D preview viewports were spawned and reference the same material handles.
    let mut viewports = world.query::<&brkrs::ui::palette::PreviewViewport>();
    let mut vp_20 = None;
    let mut vp_90 = None;
    for pv in viewports.iter(world) {
        if pv.type_id == 20 {
            vp_20 = Some(pv.material.clone());
        } else if pv.type_id == 90 {
            vp_90 = Some(pv.material.clone());
        }
    }
    assert!(vp_20.is_some(), "expected 3D preview viewport for 20");
    assert!(vp_90.is_some(), "expected 3D preview viewport for 90");
    assert!(
        vp_20.as_ref().unwrap().is_some(),
        "expected material handle for viewport 20"
    );
    assert!(
        vp_90.as_ref().unwrap().is_some(),
        "expected material handle for viewport 90"
    );

    // Ensure that MeshMaterial3d components were spawned for the textured thumbnails
    let mut mat_handles: Vec<Handle<StandardMaterial>> = vec![];
    let mut materials_q = world.query::<&MeshMaterial3d<StandardMaterial>>();
    for m in materials_q.iter(world) {
        mat_handles.push(m.0.clone());
    }
    let expected_20 = vp_20.as_ref().unwrap().as_ref().unwrap().clone();
    let expected_90 = vp_90.as_ref().unwrap().as_ref().unwrap().clone();
    assert!(
        mat_handles.contains(&expected_20),
        "expected MeshMaterial3d for viewport 20"
    );
    assert!(
        mat_handles.contains(&expected_90),
        "expected MeshMaterial3d for viewport 90"
    );

    // Verify the preview UI nodes have BackgroundColor matching the material base_color.
    let mut colors = world.query::<(&brkrs::ui::palette::PalettePreview, &BackgroundColor)>();
    let mut color_20 = None;
    let mut color_90 = None;
    for (p, bg) in colors.iter(world) {
        if p.type_id == 20 {
            color_20 = Some(bg.0);
        } else if p.type_id == 90 {
            color_90 = Some(bg.0);
        }
    }
    assert_eq!(color_20.unwrap(), Color::srgba(0.5, 0.5, 0.5, 1.0));
    assert_eq!(color_90.unwrap(), Color::srgba(0.9, 0.2, 0.2, 1.0));

    // Press P again to close — ensure we release and then press in separate frames so
    // InputPlugin registers a new `just_pressed` event.
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.release(KeyCode::KeyP);
    }
    // allow the input state to update for the release
    app.update();

    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyP);
    }
    app.update();
    app.update();
    let world = app.world_mut();
    let found2 = world
        .query::<&brkrs::ui::palette::PaletteRoot>()
        .iter(world)
        .count();
    assert!(
        found2 == 0,
        "Palette should be removed when P is pressed again"
    );

    // After closing the palette the labels should no longer be present.
    let world_ref = app.world_mut();
    assert!(!contains_text(world_ref, "20 — Simple Brick"));
    assert!(!contains_text(world_ref, "90 — Indestructible"));
}

#[test]
fn click_selects_palette_item_and_updates_resource() {
    let mut app = palette_test_app();

    // Initialize SelectedBrick resource
    app.init_resource::<brkrs::ui::palette::SelectedBrick>();
    app.add_systems(
        Update,
        (
            brkrs::ui::palette::handle_palette_selection,
            brkrs::ui::palette::update_palette_selection_feedback
                .after(brkrs::ui::palette::handle_palette_selection),
        ),
    );

    // Open palette
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyP);
    }
    app.update();

    // Clear the input so just_pressed doesn't trigger again
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.clear();
    }

    app.update();

    // Initially, nothing should be selected
    {
        let selected = app.world().resource::<brkrs::ui::palette::SelectedBrick>();
        assert!(
            selected.type_id.is_none(),
            "Initially no brick should be selected"
        );
    }

    // Initially, nothing should be selected
    {
        let selected = app.world().resource::<brkrs::ui::palette::SelectedBrick>();
        assert!(
            selected.type_id.is_none(),
            "Initially no brick should be selected"
        );
    }

    // Simulate clicking on type 20 preview by setting Interaction to Pressed
    let preview_20_entity = {
        let world = app.world_mut();
        let mut query = world.query::<(Entity, &brkrs::ui::palette::PalettePreview)>();
        query
            .iter(&world)
            .find(|(_e, p)| p.type_id == 20)
            .map(|(e, _)| e)
            .expect("Should find type 20 preview")
    };

    {
        let world = app.world_mut();
        world
            .entity_mut(preview_20_entity)
            .insert(Interaction::Pressed);
    }
    app.update();
    app.update();

    // Verify selection was updated
    {
        let selected = app.world().resource::<brkrs::ui::palette::SelectedBrick>();
        assert_eq!(selected.type_id, Some(20), "Type 20 should be selected");
    }

    // Verify visual feedback - selected item should have yellow highlight
    {
        let world = app.world_mut();
        let mut query = world.query::<(&brkrs::ui::palette::PalettePreview, &BackgroundColor)>();
        let mut found_highlighted = false;
        for (preview, bg) in query.iter(world) {
            if preview.type_id == 20 {
                assert_eq!(
                    bg.0,
                    Color::srgba(1.0, 1.0, 0.0, 1.0),
                    "Selected item should be highlighted yellow"
                );
                found_highlighted = true;
            }
        }
        assert!(found_highlighted, "Should find highlighted preview");
    }

    // Now click type 90
    let preview_90_entity = {
        let world = app.world_mut();
        let mut query = world.query::<(Entity, &brkrs::ui::palette::PalettePreview)>();
        query
            .iter(&world)
            .find(|(_e, p)| p.type_id == 90)
            .map(|(e, _)| e)
            .expect("Should find type 90 preview")
    };

    {
        let world = app.world_mut();
        // Reset type 20 interaction
        world
            .entity_mut(preview_20_entity)
            .insert(Interaction::None);
        // Click type 90
        world
            .entity_mut(preview_90_entity)
            .insert(Interaction::Pressed);
    }
    app.update();
    app.update(); // Run one more frame for visual feedback system

    // Verify selection changed
    {
        let selected = app.world().resource::<brkrs::ui::palette::SelectedBrick>();
        assert_eq!(selected.type_id, Some(90), "Type 90 should now be selected");
    }

    // Verify visual feedback updated
    {
        let world = app.world_mut();
        let mut query = world.query::<(&brkrs::ui::palette::PalettePreview, &BackgroundColor)>();
        for (preview, bg) in query.iter(world) {
            if preview.type_id == 90 {
                assert_eq!(
                    bg.0,
                    Color::srgba(1.0, 1.0, 0.0, 1.0),
                    "Type 90 should be highlighted"
                );
            } else if preview.type_id == 20 {
                // Type 20 should revert to its original color
                assert_eq!(
                    bg.0,
                    Color::srgba(0.5, 0.5, 0.5, 1.0),
                    "Type 20 should revert to original color"
                );
            }
        }
    }
}
