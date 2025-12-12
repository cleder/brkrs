# Quick Start: Paddle Size Powerups Implementation

**Phase**: 1 - Design & Contracts **Date**: 2025-12-12

A high-level guide to implementing the paddle size powerups feature.
Use this to orient yourself to the codebase structure and understand the integration points.

## Feature Overview

When a breakout ball collides with special bricks:

- **Brick Type 30** → Paddle shrinks to 70% (14 units) for 10 seconds
- **Brick Type 32** → Paddle enlarges to 150% (30 units) for 10 seconds

Effects are temporary, replaced if a different effect is triggered, and cleared on level changes.

## Architecture at a Glance

```text
Physics Engine (Rapier3D)
    ↓
Collision Detection
    ↓
[paddle_size] System
    ├→ Create/replace PaddleSizeEffect component on paddle
    ├→ Emit PaddleSizeEffectApplied event
    └→ [Other systems listen to this event]

Parallel systems:
├→ [paddle_size_effects] Timer countdown
├→ [paddle_size_visual] Update material (color + glow)
└→ [paddle_size_audio] Play sound effect
```

All systems use **ECS components & events**.
No shared mutable state outside entities.

## Key Files to Create

### 1. Component Definition

**File**: `src/systems/paddle_size_components.rs`

```rust
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct PaddleSizeEffect {
    pub effect_type: SizeEffectType,
    pub remaining_duration: f32,
    pub base_width: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SizeEffectType {
    Shrink,  // 0.7x
    Enlarge, // 1.5x
}

// Marker components for bricks
#[derive(Component)]
pub struct BrickType30;

#[derive(Component)]
pub struct BrickType32;

// Event emitted when effect is applied
#[derive(Event)]
pub struct PaddleSizeEffectApplied {
    pub paddle_entity: Entity,
    pub effect_type: SizeEffectType,
    pub new_width: f32,
}
```

### 2. Core Systems

**File**: `src/systems/paddle_size.rs`

Four systems (described below) in order of execution:

#### System 1: Effect Creation (on collision)

```rust
pub fn create_size_effect(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    paddles: Query<(Entity, &Paddle)>,
    brick30: Query<&BrickType30>,
    brick32: Query<&BrickType32>,
    mut effect_applied_events: EventWriter<PaddleSizeEffectApplied>,
) {
    // Listen to collisions
    // If brick 30 hit: Create PaddleSizeEffect(Shrink)
    // If brick 32 hit: Create PaddleSizeEffect(Enlarge)
    // Emit PaddleSizeEffectApplied event
}
```

#### System 2: Timer Countdown

```rust
pub fn update_paddle_size_effects(
    mut paddles: Query<&mut PaddleSizeEffect>,
    time: Res<Time>,
) {
    // Decrement remaining_duration each frame
    // [Removal happens in separate system when duration ≤ 0]
}
```

#### System 3: Cleanup on Expiry

```rust
pub fn remove_expired_effects(
    mut commands: Commands,
    paddles: Query<(Entity, &PaddleSizeEffect)>,
) {
    // Find effects with remaining_duration ≤ 0
    // Remove component
}
```

#### System 4: Visual Feedback

```rust
pub fn update_visual_feedback(
    effects: Query<(&PaddleSizeEffect, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // For each active effect:
    // - Shrink: Red tint + glow
    // - Enlarge: Green tint + glow
}
```

### 3. Audio Feedback

**File**: `src/systems/paddle_size_audio.rs`

```rust
pub fn play_size_effect_audio(
    mut effect_applied: EventReader<PaddleSizeEffectApplied>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,  // Loaded from manifest
) {
    // On PaddleSizeEffectApplied event:
    // - If Shrink: play audio_assets.paddle_shrink
    // - If Enlarge: play audio_assets.paddle_enlarge
}
```

### 4. Lifecycle Cleanup

**File**: `src/systems/paddle_size_lifecycle.rs`

```rust
pub fn clear_effects_on_level_change(
    mut commands: Commands,
    mut level_events: EventReader<LevelChangeEvent>,
    paddles: Query<Entity, With<Paddle>>,
) {
    // On LevelChangeEvent: Remove PaddleSizeEffect from all paddles
}

pub fn clear_effects_on_loss(
    mut commands: Commands,
    mut loss_events: EventReader<PlayerLossEvent>,
    paddles: Query<Entity, With<Paddle>>,
) {
    // On PlayerLossEvent: Remove PaddleSizeEffect from all paddles
}
```

## Integration Checklist

### 1. Level Loader Changes

**File**: `src/level_loader.rs`

When loading bricks from RON level files:

- Detect brick type 30 → Add `BrickType30` component
- Detect brick type 32 → Add `BrickType32` component

```rust
// In brick loading logic:
if brick_type == 30 {
    commands.entity(brick_entity).insert(BrickType30);
}
if brick_type == 32 {
    commands.entity(brick_entity).insert(BrickType32);
}
```

### 2. App Setup

**File**: `src/lib.rs` or `src/main.rs`

Register systems and events:

```rust
app
    .add_event::<PaddleSizeEffectApplied>()
    .add_systems(Update, (
        create_size_effect,
        update_paddle_size_effects,
        remove_expired_effects,
        update_visual_feedback,
        play_size_effect_audio,
        clear_effects_on_level_change,
        clear_effects_on_loss,
    ).chain())  // Chain ensures order
    ;
```

### 3. Audio Assets

**File**: `assets/audio/manifest.ron`

Add entries:

```ron
sounds: {
    // ... existing ...
    "paddle_shrink": Sound(path: "paddle_shrink.ogg"),
    "paddle_enlarge": Sound(path: "paddle_enlarge.ogg"),
}
```

Add files to `assets/audio/`:

- `paddle_shrink.ogg`
- `paddle_enlarge.ogg`

### 4. Testing

Create `tests/paddle_size_integration.rs`:

```rust
#[test]
fn test_paddle_shrinks_on_brick_30() {
    // Setup: world with paddle + brick 30
    // Trigger collision
    // Assert: paddle has PaddleSizeEffect(Shrink), width is 14
}

#[test]
fn test_paddle_enlarges_on_brick_32() {
    // Setup: world with paddle + brick 32
    // Trigger collision
    // Assert: paddle has PaddleSizeEffect(Enlarge), width is 30
}

#[test]
fn test_effect_expires_after_10_seconds() {
    // Setup: world with active effect
    // Advance time 10 seconds
    // Assert: PaddleSizeEffect component removed
}

#[test]
fn test_effect_clears_on_level_change() {
    // Setup: world with active effect
    // Emit LevelChangeEvent
    // Assert: PaddleSizeEffect removed
}
```

## Code Patterns to Follow

### Using the ECS

```rust
// Query entities with a component
let paddles = query.iter();

// Insert a component
commands.entity(paddle_entity).insert(component);

// Remove a component
commands.entity(paddle_entity).remove::<ComponentType>();

// Listen to events
for event in reader.read() { }
```

### Size Calculation

```rust
fn calculate_padded_width(base_width: f32, effect: &PaddleSizeEffect) -> f32 {
    let multiplier = match effect.effect_type {
        SizeEffectType::Shrink => 0.7,
        SizeEffectType::Enlarge => 1.5,
    };
    (base_width * multiplier).clamp(10.0, 30.0)
}
```

### Material Updates

```rust
fn update_material_for_effect(material: &mut StandardMaterial, effect_type: SizeEffectType) {
    match effect_type {
        SizeEffectType::Shrink => {
            material.base_color = Color::srgb(1.0, 0.3, 0.3);  // Red
            material.emissive = LinearRgba::rgb(0.3, 0.0, 0.0);
        }
        SizeEffectType::Enlarge => {
            material.base_color = Color::srgb(0.3, 1.0, 0.3);  // Green
            material.emissive = LinearRgba::rgb(0.0, 0.3, 0.0);
        }
    }
}
```

## Validation Commands

Before submitting, run:

```bash
cargo test                          # Unit + integration tests
cargo fmt --all                     # Code formatting
cargo clippy --all-targets --all-features  # Linting
bevy lint                           # Bevy-specific checks
```

## Expected Behavior

1. **Level loads** → No effect (paddle normal)
2. **Ball hits brick 30** → Paddle shrinks, red glow, shrink sound plays
3. **Ball hits brick 32** → Paddle enlarges, green glow, enlarge sound plays
4. **10 seconds pass** → Effect expires, paddle returns to normal
5. **Level changes** → All effects cleared, clean slate
6. **Player loses life** → All effects cleared, respawn with normal paddle

## Next Steps

1. Create the component definition file
2. Implement the four core systems
3. Add audio system
4. Add lifecycle cleanup systems
5. Update level loader to add brick markers
6. Register in app setup
7. Write and run tests
8. Validate against spec acceptance criteria
