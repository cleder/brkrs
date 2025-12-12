use bevy::app::App;
use bevy::prelude::*;
use bevy::MinimalPlugins;

use brkrs::{Paddle, PaddleSizeEffect, SizeEffectType};

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}

#[test]
fn paddle_size_effect_component_creation() {
    let mut app = test_app();

    // Spawn paddle with shrink effect
    let paddle = app
        .world_mut()
        .spawn((
            Paddle,
            Transform::from_scale(Vec3::ONE),
            PaddleSizeEffect {
                effect_type: SizeEffectType::Shrink,
                remaining_duration: 10.0,
                target_width: 14.0,
            },
        ))
        .id();

    // Verify component values
    let effect = app
        .world()
        .entity(paddle)
        .get::<PaddleSizeEffect>()
        .expect("Paddle should have size effect");

    assert_eq!(effect.effect_type, SizeEffectType::Shrink);
    assert_eq!(effect.target_width, 14.0);
    assert_eq!(effect.remaining_duration, 10.0);
}

#[test]
fn paddle_size_effect_shrink_target_width() {
    let mut app = test_app();

    // Spawn paddle with shrink effect
    let paddle = app
        .world_mut()
        .spawn((
            Paddle,
            PaddleSizeEffect {
                effect_type: SizeEffectType::Shrink,
                remaining_duration: 10.0,
                target_width: 14.0, // 70% of 20
            },
        ))
        .id();

    let effect = app.world().entity(paddle).get::<PaddleSizeEffect>().unwrap();
    assert_eq!(
        effect.target_width, 14.0,
        "Shrink should result in 14.0 width (70% of 20)"
    );
}

#[test]
fn paddle_size_effect_enlarge_target_width() {
    let mut app = test_app();

    // Spawn paddle with enlarge effect
    let paddle = app
        .world_mut()
        .spawn((
            Paddle,
            PaddleSizeEffect {
                effect_type: SizeEffectType::Enlarge,
                remaining_duration: 10.0,
                target_width: 30.0, // 150% of 20
            },
        ))
        .id();

    let effect = app.world().entity(paddle).get::<PaddleSizeEffect>().unwrap();
    assert_eq!(
        effect.target_width, 30.0,
        "Enlarge should result in 30.0 width (150% of 20)"
    );
}

#[test]
fn size_effect_type_comparison() {
    assert_eq!(SizeEffectType::Shrink, SizeEffectType::Shrink);
    assert_eq!(SizeEffectType::Enlarge, SizeEffectType::Enlarge);
    assert_ne!(SizeEffectType::Shrink, SizeEffectType::Enlarge);
}

#[test]
fn paddle_can_have_effect_removed() {
    let mut app = test_app();

    // Spawn paddle with effect
    let paddle = app
        .world_mut()
        .spawn((
            Paddle,
            PaddleSizeEffect {
                effect_type: SizeEffectType::Shrink,
                remaining_duration: 10.0,
                target_width: 14.0,
            },
        ))
        .id();

    // Verify effect exists
    assert!(app.world().entity(paddle).get::<PaddleSizeEffect>().is_some());

    // Remove effect
    app.world_mut().entity_mut(paddle).remove::<PaddleSizeEffect>();

    // Verify effect removed
    assert!(app.world().entity(paddle).get::<PaddleSizeEffect>().is_none());
}

#[test]
fn paddle_can_replace_effect() {
    let mut app = test_app();

    // Spawn paddle with shrink effect
    let paddle = app
        .world_mut()
        .spawn((
            Paddle,
            PaddleSizeEffect {
                effect_type: SizeEffectType::Shrink,
                remaining_duration: 5.0,
                target_width: 14.0,
            },
        ))
        .id();

    // Verify shrink effect
    let effect = app.world().entity(paddle).get::<PaddleSizeEffect>().unwrap();
    assert_eq!(effect.effect_type, SizeEffectType::Shrink);

    // Replace with enlarge effect
    app.world_mut()
        .entity_mut(paddle)
        .remove::<PaddleSizeEffect>()
        .insert(PaddleSizeEffect {
            effect_type: SizeEffectType::Enlarge,
            remaining_duration: 10.0,
            target_width: 30.0,
        });

    // Verify enlarge effect
    let effect = app.world().entity(paddle).get::<PaddleSizeEffect>().unwrap();
    assert_eq!(effect.effect_type, SizeEffectType::Enlarge);
    assert_eq!(effect.target_width, 30.0);
    assert_eq!(effect.remaining_duration, 10.0);
}
