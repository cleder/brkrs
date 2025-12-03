# Research: Pause and Resume System

**Feature**: `004-pause-system` **Date**: 2025-11-28 **Status**: Complete

## Overview

This document consolidates research findings for implementing a pause/resume system in Bevy 0.16 with bevy_rapier3d 0.31.
All NEEDS CLARIFICATION items from Technical Context have been resolved.

---

## Key Research Questions

### 1. Window Mode Switching API

**Question**: How to programmatically switch between fullscreen and windowed modes at runtime?

**Decision**: Use `Window::mode` field mutation via `Single<&mut Window, With<PrimaryWindow>>` query

**Rationale**:

- Bevy 0.16 provides mutable access to Window resource through ECS query
- Window mode can be changed at runtime by mutating the `mode` field
- Platform-specific handling required for WASM (MonitorSelection unavailable)

**Implementation Pattern**:

```rust
use bevy::window::{Window, WindowMode, PrimaryWindow};

fn switch_to_windowed(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    window.mode = WindowMode::Windowed;
}

fn switch_to_fullscreen(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use bevy::window::MonitorSelection;
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
    }
    #[cfg(target_arch = "wasm32")]
    {
        // WASM: Stay in windowed mode; browser handles fullscreen via user gesture
        window.mode = WindowMode::Windowed;
    }
}
```

**Platform Constraints**:

- **Native**: Full support for `BorderlessFullscreen(MonitorSelection::Current)` and `Windowed` modes
- **WASM**: `MonitorSelection` enum not available (requires platform-specific imports).
  Browser fullscreen requires user gesture for security.
  Recommend staying in `Windowed` mode on WASM and documenting browser F11 as fullscreen method.

**Codebase Precedent**: Project already uses conditional window mode compilation in `src/lib.rs:162-164` and `grab_mouse` function demonstrates Window mutation pattern.

---

### 2. Physics Simulation Control

**Question**: How to freeze and resume bevy_rapier3d physics simulation while preserving state?

**Decision**: Use `RapierConfiguration::physics_pipeline_active` toggle

**Rationale**:

- Complete physics freeze (all bodies, velocities, forces preserved)
- State preservation guarantees exact resume from pause point
- Cleaner than timestep manipulation (no edge cases with zero timestep)
- Codebase already queries `RapierConfiguration` in multiple systems

**Implementation Pattern**:

```rust
use bevy_rapier3d::prelude::RapierConfiguration;

fn pause_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = false;
}

fn resume_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = true;
}
```

**Alternatives Considered**:

| Approach | Pros | Cons | Verdict |
|----------|------|------|---------|
| `physics_pipeline_active` | Complete freeze, state preserved, simple API | None identified | ✅ **Selected** |
| `TimestepMode` zero dt | Fine-grained control | Edge cases with dt=0, more complex state management | ❌ Rejected |
| Manual velocity zeroing | Simple | State not preserved (velocities lost), requires storage | ❌ Rejected |

**Performance Notes**: No performance overhead; physics systems simply skip execution when pipeline inactive.

**Codebase Precedent**: `GravityConfig` resource already used for runtime physics tuning (see `src/level_loader.rs` lines 359-391 for paddle growth gravity manipulation).

---

### 3. Input Debouncing

**Question**: How to prevent rapid ESC key spam from causing unintended pause state issues?

**Decision**: Use `ButtonInput::just_pressed()` (built-in frame-level debouncing) without additional timer

**Rationale**:

- `just_pressed()` returns `true` only on the frame where key transitions from unpressed to pressed
- Holding ESC does not repeatedly trigger pause (unlike `pressed()`)
- Codebase extensively uses `just_pressed()` pattern (20+ occurrences)
- Additional timer-based debouncing unnecessary unless explicit UX rate-limiting desired

**Implementation Pattern**:

```rust
use bevy::input::{ButtonInput, keyboard::KeyCode};

fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut pause_state: ResMut<PauseState>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        pause_state.pause();  // Only triggers once per key press
    }
}
```

**Optional Enhancement** (if UX requires rate-limiting):

```rust
#[derive(Resource)]
struct InputDebounce {
    esc_cooldown: Timer,  // e.g., 200ms
}

fn handle_pause_input_with_cooldown(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut debounce: ResMut<InputDebounce>,
    mut pause_state: ResMut<PauseState>,
) {
    debounce.esc_cooldown.tick(time.delta());

    if keyboard.just_pressed(KeyCode::Escape) && debounce.esc_cooldown.finished() {
        pause_state.pause();
        debounce.esc_cooldown.reset();
    }
}
```

**Recommendation**: Start with `just_pressed()` alone (simpler, meets FR-014).
Add cooldown timer only if testing reveals need for explicit rate-limiting.

**Codebase Precedent**: All keyboard input in project uses `just_pressed()` (see `grab_mouse`, `toggle_wireframe`, level switching at lines 586, 600, 605, 610, 732, 769).

---

### 4. UI Text Rendering

**Question**: Does Bevy 0.16 require bundled fonts for text overlays?

**Decision**: Use Bevy's embedded default font (no asset bundling needed)

**Rationale**:

- Bevy 0.16 includes default font in engine
- Zero asset loading delay for pause overlay
- Sufficient for pause message text (no custom typography required)
- Custom fonts optional for future enhancements

**Implementation Pattern**:

```rust
use bevy::prelude::*;

#[derive(Component)]
struct PauseOverlay;

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Text::new("PAUSED\nClick to Resume"),
        TextFont {
            font_size: 60.0,
            ..default()  // Uses embedded default font
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        PauseOverlay,  // Marker component for cleanup
    ));
}

fn despawn_pause_overlay(
    mut commands: Commands,
    overlay: Query<Entity, With<PauseOverlay>>,
) {
    for entity in overlay.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
```

**Platform Notes**: Text rendering identical on native and WASM.

**Codebase Precedent**: Project uses marker components (`GridOverlay`, `FadeOverlay`, `Ball`, `Paddle`) for entity identification in queries (lines 67-92, level_loader.rs).

---

### 5. WASM Window Mode Limitations

**Question**: Are there platform-specific constraints for window mode switching on WASM?

**Finding**: Yes, significant limitations exist

**Constraints**:

1. **MonitorSelection unavailable**: The `bevy::window::MonitorSelection` enum is not available on WASM target (requires native-only imports)
2. **Browser security model**: Fullscreen requires user gesture (cannot be triggered programmatically on page load or via arbitrary events)
3. **API differences**: Browser Fullscreen API differs from native window management

**Implementation Strategy**:

```rust
#[derive(Resource)]
struct WindowModeSnapshot {
    #[cfg(not(target_arch = "wasm32"))]
    mode_before_pause: WindowMode,

    #[cfg(target_arch = "wasm32")]
    _unused: (),  // WASM: No window mode switching
}

fn pause_with_window_switch(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut snapshot: ResMut<WindowModeSnapshot>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        snapshot.mode_before_pause = window.mode.clone();
        window.mode = WindowMode::Windowed;
    }

    #[cfg(target_arch = "wasm32")]
    {
        // No-op: WASM stays in current window mode
        // Document: Users can press F11 for browser fullscreen
    }
}
```

**User Documentation Required**:

- Native: Automatic fullscreen ↔ windowed switching on pause/resume
- WASM: Use browser's native fullscreen (F11 or fullscreen button); game pause does not affect window mode

**Rationale for WASM Limitation**:

- Browser security prevents arbitrary fullscreen triggering
- User gesture requirement cannot be satisfied by in-game events
- Attempting fullscreen without gesture results in browser denying the request
- Better UX to skip window mode switching on WASM than show failed attempts

**Codebase Precedent**: Project already uses `#[cfg(target_arch = "wasm32")]` for platform-specific behavior (window mode initialization, wireframe support, level loading).
See `src/lib.rs:162-164`, `src/systems/grid_debug.rs:82-85`.

---

## Architecture Decisions

### Pause State Management

**Decision**: Use ECS Resource for global pause state

**Structure**:

```rust
#[derive(Resource, Default)]
pub struct PauseState {
    pub is_paused: bool,

    #[cfg(not(target_arch = "wasm32"))]
    pub window_mode_before_pause: Option<WindowMode>,
}

impl PauseState {
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
    }
}
```

**Rationale**: Aligns with Constitution Principle I (ECS-First).
State stored in resource, systems query and mutate.

---

### System Ordering

**Decision**: Explicit system ordering for pause input → physics control → window management → UI

**Schedule**:

```rust
app.add_systems(Update, (
    handle_pause_input,
    handle_resume_input,
    apply_pause_to_physics,
    apply_pause_to_window_mode,
    spawn_despawn_pause_overlay,
).chain());
```

**Rationale**:

- Input systems run first to update PauseState
- Physics control responds to state change (freeze/resume)
- Window mode switching follows physics (visual feedback)
- UI overlay spawned last (ensure physics already frozen)
- Chain ordering prevents frame-delay between pause input and physics freeze

---

### Input Handling Modularity

**Decision**: Separate systems for ESC (pause) and click (resume)

**Rationale**:

- FR-007: ESC ignored when already paused (different logic than toggle)
- FR-011: Click only processes when paused (different condition than ESC)
- Independent testability (User Story 1 and 2 independently testable)
- Clear separation of concerns (pause vs resume logic)

---

## Best Practices Summary

### ECS Patterns

1. **Resource for global state**: `PauseState` resource
2. **Component markers**: `PauseOverlay` component for UI entity
3. **System ordering**: `.chain()` for sequential dependencies
4. **Run conditions**: Check `PauseState.is_paused` in system run conditions

### Platform Compatibility

1. **Conditional compilation**: Use `#[cfg(target_arch = "wasm32")]` for window mode logic
2. **Graceful degradation**: WASM skips window switching (no user-facing error)
3. **Documentation**: Document platform differences in README/quickstart

### Performance

1. **Zero allocations**: Pause overlay spawned once, despawned once
2. **Minimal frame impact**: Physics freeze is binary flag check (no iteration)
3. **Input efficiency**: `just_pressed()` avoids per-frame processing

### Testing Strategy

1. **Unit tests**: State transitions (`Active → Paused → Active`)
2. **Integration tests**: Physics freeze verification (ball velocity preserved)
3. **Manual tests**: Window mode switching on native, WASM compatibility
4. **Performance tests**: Measure pause latency (<16ms target)

---

## Open Questions / Future Enhancements

1. **Custom pause overlay styling**: Current design uses default font and simple text.
   Future: custom font, animations, semi-transparent background.
2. **Gamepad support**: Out of scope (FR-015), but could be added by listening to gamepad button events (e.g., Start button).
3. **Touch support**: Out of scope (FR-015), but could be added by listening to touch events.
4. **Pause during level transitions**: FR-012 requires blocking pause during transitions.
   Implementation: add run condition checking `LevelAdvanceState` resource.

---

## References

- Bevy 0.16 Window API: `bevy::window::Window`, `WindowMode`
- Rapier3D 0.31: `RapierConfiguration::physics_pipeline_active`
- Bevy Input: `ButtonInput::just_pressed()`
- Project Constitution: `.specify/memory/constitution.md`
- Existing window mode code: `src/lib.rs:150-177`
- Existing input patterns: `src/lib.rs:591-612` (grab_mouse function)
