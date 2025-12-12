# Research & Technical Decisions: Paddle Size Powerups

**Phase**: 0 - Research & Analysis **Date**: 2025-12-12 **Status**: Complete

## 1. Paddle Component State Management

**Decision**: Extend existing paddle entity with `PaddleSizeEffect` component

**Rationale**:

- Bevy ECS encourages component-based state representation
- Avoids mutable shared state outside entities
- Enables efficient change detection for visual updates
- Follows established pattern in codebase (see `paddle_shrink.rs`)

**Alternatives Considered**:

- Global resource for effect state → Rejected: Breaks ECS pattern, limits multi-paddle scenarios
- Nested state machine → Rejected: Overly complex for single binary choice (shrink/enlarge)
- Events only (no state component) → Rejected: Loss of effect duration context for rendering

**Implementation Approach**:

```rust
#[derive(Component)]
pub struct PaddleSizeEffect {
    pub effect_type: SizeEffectType,      // Shrink | Enlarge
    pub remaining_duration: f32,           // Seconds
    pub base_width: f32,                   // Store original width for reset
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SizeEffectType {
    Shrink,  // 0.7x multiplier
    Enlarge, // 1.5x multiplier
}
```

**Related Requirements**: FR-001 through FR-008, FR-011, FR-011b

---

## 2. Collision Detection & Brick Type Identification

**Decision**: Use Rapier3D collision events filtered by brick collider entity marker components

**Rationale**:

- Rapier3D already processes collisions efficiently (parallel safe)
- Existing collision event system used by other features (ball respawn, etc.)
- Avoids raycasting/querying in hot loops
- Deterministic, testable physics integration

**Alternatives Considered**:

- Manual transform distance checks → Rejected: Redundant with physics engine, error-prone
- Custom collision shapes → Rejected: Over-engineers; use existing collision system
- Raycast on paddle position → Rejected: Less accurate, additional query cost

**Implementation Approach**:

- Add `BrickType30` and `BrickType32` marker components to respective bricks during level loading
- Listen to `CollisionEvent::Started` from Rapier3D
- Filter events by these components
- Trigger size effect on collision

**Related Requirements**: FR-001, FR-002, FR-005

---

## 3. Effect Timer & Lifecycle Management

**Decision**: Implement timer countdown in dedicated system using `Res<Time>` and component mutation

**Rationale**:

- Bevy's `Time` resource provides frame-rate-independent delta
- Component mutation in systems aligns with ECS mutation patterns
- Enables non-blocking cleanup (removal triggered by condition)
- Supports independent timer restart on duplicate hits

**Alternatives Considered**:

- Entity despawn after 10s → Rejected: Destroys paddle entity (wrong target)
- One-shot timer in events → Rejected: Requires external event storage
- Plugin timer channels → Rejected: Unnecessary complexity

**Implementation Approach**:

```rust
// System: paddle_size_effects
fn update_paddle_size_effects(
    mut paddles: Query<&mut PaddleSizeEffect>,
    time: Res<Time>,
) {
    for mut effect in &mut paddles {
        effect.remaining_duration -= time.delta_seconds();
        if effect.remaining_duration <= 0.0 {
            // Mark for removal (handled by removal system)
        }
    }
}
```

**Related Requirements**: FR-006, FR-007, FR-007, FR-008

---

## 4. Size Clamping & Boundary Enforcement

**Decision**: Clamp calculated size to [10, 30] units after applying multiplier, store base width separately

**Rationale**:

- Prevents unplayable extremes while maintaining effect activation
- Preserves intent (player knows effect triggered) even when clamped
- Clear separation between "desired size" and "actual clamped size"
- Supports timer reset without size shift

**Alternatives Considered**:

- Prevent effect activation at limits → Rejected: Violates acceptance criteria (effect should activate)
- Dynamic multiplier adjustment → Rejected: Adds confusion, unclear behavior
- Skip clamp, allow unbounded growth → Rejected: Breaks gameplay

**Implementation Approach**:

```rust
fn apply_size_effect(width: f32, effect: &PaddleSizeEffect) -> f32 {
    let multiplier = match effect.effect_type {
        SizeEffectType::Shrink => 0.7,
        SizeEffectType::Enlarge => 1.5,
    };
    let desired = width * multiplier;
    desired.clamp(10.0, 30.0)
}
```

**Related Requirements**: FR-009, FR-010

---

## 5. Visual Feedback Implementation

**Decision**: Use Bevy material color + outline shader (glow effect via emission)

**Rationale**:

- Color tinting is straightforward (material color override)
- Outline/glow implemented via emission channel in PBR material
- No custom shaders required (Bevy standard materials support emission)
- Works across native + WASM platforms
- Reuses material system already in use for paddle

**Alternatives Considered**:

- Custom shader → Rejected: Unnecessary; emission achieves glow in standard material
- Particle effects → Rejected: Violates "subtle" spec, higher memory footprint
- Post-processing bloom → Rejected: Global overhead, may affect other entities

**Implementation Approach**:

```rust
// In paddle_size_visual system:
for (entity, effect, mut material) in &mut materials_query {
    match effect.effect_type {
        SizeEffectType::Shrink => {
            material.base_color = Color::srgb(1.0, 0.3, 0.3);    // Red tint
            material.emissive = LinearRgba::rgb(0.3, 0.0, 0.0);  // Glow
        }
        SizeEffectType::Enlarge => {
            material.base_color = Color::srgb(0.3, 1.0, 0.3);    // Green tint
            material.emissive = LinearRgba::rgb(0.0, 0.3, 0.0);  // Glow
        }
    }
}
```

**Related Requirements**: FR-012, FR-013

---

## 6. Audio Feedback System

**Decision**: Trigger audio playback on collision using asset loading + Bevy audio plugin

**Rationale**:

- Bevy has built-in audio plugin (not disabled in project)
- Audio assets loaded from `assets/audio/` as RON manifest (existing pattern)
- One-shot audio on event (no looping during effect)
- Minimal performance impact (fire-and-forget)

**Alternatives Considered**:

- WAV/OGG file direct load → Rejected: Use existing manifest system for consistency
- Procedural tone generation → Rejected: Adds complexity, audio already designed
- Looping during effect → Rejected: Spec says "sound effect on brick hit" (one-time)

**Implementation Approach**:

- Add `shrink_sound.ogg` and `enlarge_sound.ogg` to `assets/audio/`
- Extend `audio.ron` manifest with new entries
- On collision, load and play appropriate sound via `audio.play(asset_handle)`

**Related Requirements**: FR-014, FR-015

---

## 7. Level Transitions & State Clearing

**Decision**: Clear `PaddleSizeEffect` component when level changes or player loses life

**Rationale**:

- Simple event-driven removal tied to existing level/life events
- Respects clean-slate principle (each level starts fresh)
- Aligns with requirement FR-011b (clear on level advance)
- No special cleanup needed; just component removal triggers visual reset

**Alternatives Considered**:

- Persist across levels with decay → Rejected: Spec explicitly says "clear"
- Conditional carry-over → Rejected: Added complexity, violates spec

**Implementation Approach**:

```rust
fn clear_effects_on_level_change(
    mut commands: Commands,
    mut level_events: EventReader<LevelChangeEvent>,
    paddles: Query<Entity, With<Paddle>>,
) {
    for _ in level_events.read() {
        for paddle_entity in &paddles {
            commands.entity(paddle_entity).remove::<PaddleSizeEffect>();
        }
    }
}
```

**Related Requirements**: FR-011, FR-011b

---

## 8. Effect Replacement Logic

**Decision**: When new effect triggered, remove old component and add new one (full replacement)

**Rationale**:

- Simplest, most predictable behavior
- One effect active at a time (matches clarification Q3: A)
- Timer naturally resets when new component created
- Avoids state machine complexity

**Alternatives Considered**:

- Merge effects multiplicatively → Rejected: Clarification A specifies "one active at a time"
- Queue effects → Rejected: Unnecessary complexity not in spec

**Implementation Approach**:

```rust
fn on_brick_collision(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut paddles: Query<Entity, With<Paddle>>,
) {
    for collision in collisions.read() {
        // Detect brick type, determine effect
        let new_effect = PaddleSizeEffect { /* ... */ };

        for paddle_entity in &mut paddles {
            commands.entity(paddle_entity)
                .remove::<PaddleSizeEffect>()  // Remove old if exists
                .insert(new_effect.clone());    // Insert new
        }
    }
}
```

**Related Requirements**: FR-007, FR-007, Q3

---

## Summary of Key Decisions

| Decision | Key Rationale | Spec Coverage |
|----------|---------------|---------------|
| Component-based state | ECS alignment, change detection | FR-001-008 |
| Rapier collision events | Physics-driven, efficient, existing pattern | FR-001-002 |
| Timer countdown system | Frame-rate independent, non-blocking | FR-006-007 |
| Size clamping with activation | Honors effect trigger even at limits | FR-009-010 |
| Material color + emission | No custom shaders, WASM compatible | FR-012-013 |
| Audio via manifest + plugin | Consistent with existing asset system | FR-014-015 |
| Component removal on transition | Simple, clean-slate semantics | FR-011-011b |
| Full effect replacement | Single active effect, predictable | Q3: A |

**All clarifications resolved without additional NEEDS CLARIFICATION markers.**
