use bevy::prelude::*;

use brkrs::systems::textures::loader::{ObjectClass, TextureManifest, TypeVariantDefinition};
use brkrs::systems::textures::{
    FallbackRegistry, ProfileMaterialBank, TextureMaterialsPlugin, TypeVariantRegistry,
};

use brkrs::systems::textures::loader::RawTextureManifest as DiskTextureManifest;
use std::fs;

fn app_with_variant_registry() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(Assets::<Image>::default());
    app.add_plugins(TextureMaterialsPlugin);
    app.update();
    app
}

#[test]
fn disk_manifest_populates_registry_for_new_bricks() {
    let mut app = app_with_variant_registry();

    // Prepare two distinct materials that will serve as profile outputs
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

    // Populate profile material bank with the profile ids used by the on-disk manifest
    {
        let mut bank = app.world_mut().resource_mut::<ProfileMaterialBank>();
        bank.insert_for_tests("brick/type20", handle_a.clone());
        bank.insert_for_tests("brick/indestructible", handle_b.clone());
    }

    // Read manifest.ron from disk and build the runtime manifest
    let contents = fs::read_to_string("assets/textures/manifest.ron").expect("read manifest");
    // Deserialize into the disk manifest type used across the systems
    let raw_manifest: DiskTextureManifest =
        ron::de::from_str(&contents).expect("parse manifest ron");
    let manifest = brkrs::systems::textures::loader::TextureManifest::from_raw(raw_manifest);

    // Rebuild registry using the real manifest
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

    // Ensure registry now contains mappings for the new brick types
    let reg = app.world().resource::<TypeVariantRegistry>();
    assert_eq!(
        reg.get(brkrs::systems::textures::loader::ObjectClass::Brick, 20)
            .unwrap(),
        handle_a
    );
    assert_eq!(
        reg.get(brkrs::systems::textures::loader::ObjectClass::Brick, 90)
            .unwrap(),
        handle_b
    );
}

#[test]
fn immediate_spawn_registry_populated_from_manifest() {
    let mut app = app_with_variant_registry();

    // Prepare materials handle(s)
    let handle_1 = {
        let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
        materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.0, 0.0, 1.0),
            unlit: true,
            ..default()
        })
    };

    // Populate profile material bank with a profile id used by the manifest variant
    {
        let mut bank = app.world_mut().resource_mut::<ProfileMaterialBank>();
        bank.insert_for_tests("ball/default", handle_1.clone());
    }

    // Create a runtime manifest with a variant mapping ball type 7 -> ball/default
    let manifest = TextureManifest {
        profiles: Default::default(),
        type_variants: vec![TypeVariantDefinition {
            object_class: ObjectClass::Ball,
            type_id: 7,
            profile_id: "ball/default".to_string(),
            emissive_color: None,
            animation: None,
        }],
        level_overrides: Default::default(),
        level_switch: None,
    };

    // Rebuild registry from manifest - need to scope borrows properly
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

    // Now the registry should return the handle for (Ball, 7)
    let registry = app.world().resource::<TypeVariantRegistry>();
    let got = registry.get(ObjectClass::Ball, 7);
    assert!(got.is_some());
    assert_eq!(got.unwrap(), handle_1);
}

#[test]
fn runtime_mutation_updates_registry_mapping() {
    let mut app = app_with_variant_registry();

    // Create two distinct handles
    let (handle_a, handle_b) = {
        let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
        let a = materials.add(StandardMaterial {
            base_color: Color::srgba(0.1, 0.2, 0.3, 1.0),
            unlit: true,
            ..default()
        });
        let b = materials.add(StandardMaterial {
            base_color: Color::srgba(0.9, 0.8, 0.7, 1.0),
            unlit: true,
            ..default()
        });
        (a, b)
    };

    // Insert initial profile mapping to handle_a
    {
        let mut bank = app.world_mut().resource_mut::<ProfileMaterialBank>();
        bank.insert_for_tests("ball/default", handle_a.clone());
    }

    // Start manifest with mapping to handle_a
    let manifest = TextureManifest {
        profiles: Default::default(),
        type_variants: vec![TypeVariantDefinition {
            object_class: ObjectClass::Ball,
            type_id: 2,
            profile_id: "ball/default".to_string(),
            emissive_color: None,
            animation: None,
        }],
        level_overrides: Default::default(),
        level_switch: None,
    };

    // Rebuild registry from manifest
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

    // Confirm initial mapping
    {
        let registry = app.world().resource::<TypeVariantRegistry>();
        assert_eq!(registry.get(ObjectClass::Ball, 2).unwrap(), handle_a);
    }

    // Mutate bank to point same profile id to a different handle, representing new baked material
    {
        let mut bank = app.world_mut().resource_mut::<ProfileMaterialBank>();
        bank.insert_for_tests("ball/default", handle_b.clone());
    }

    // Rebuild registry (as would happen on manifest/asset change)
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

    // Confirm mapping updated to the new handle
    let registry = app.world().resource::<TypeVariantRegistry>();
    assert_eq!(registry.get(ObjectClass::Ball, 2).unwrap(), handle_b);
}
