# Data Model: Pause and Resume System

**Feature**: `004-pause-system` **Date**: 2025-11-28 **Spec**: [spec.md](spec.md) **Research**: [research.md](research.md)

## Overview

This document defines the data structures, components, resources, and their relationships for the pause/resume system.
All entities follow Bevy's ECS architecture (Constitution Principle I).

---

## Resources

### PauseState

**Purpose**: Global pause state tracking

**Location**: `src/pause.rs`

**Structure**:

```rust
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseState {
    #[default]
    Active,
    Paused {
        #[cfg(not(target_arch = "wasm32"))]
        window_mode_before_pause: WindowMode,
    },
}
```

**Behavior**:

- `Active`: Normal gameplay, physics running, no overlay
- `Paused`: Physics frozen, overlay visible, window mode captured (native only)
- State transitions: `Active → Paused` (ESC key), `Paused → Active` (mouse click)

**Platform Differences**:

- **Native**: Stores `WindowMode` before pause to enable restoration on resume
- **WASM**: No window mode storage (window switching not supported)

**Usage**:

```rust
// System run condition
fn is_paused(state: Res<PauseState>) -> bool {
    matches!(*state, PauseState::Paused { .. })
}

// System run condition
fn is_active(state: Res<PauseState>) -> bool {
    matches!(*state, PauseState::Active)
}
```

---

## Components

### PauseOverlay

**Purpose**: Marker component for the pause UI overlay entity

**Location**: `src/ui/pause_overlay.rs`

**Structure**:

```rust
#[derive(Component, Debug)]
pub struct PauseOverlay;
```

**Behavior**:

- Attached to UI entity spawned when game pauses
- Used to query and despawn overlay when game resumes
- Entity structure:
  - Root node (full-screen container)
  - Text bundle (pause message)
  - Both tagged with `PauseOverlay`

**Lifecycle**:

- **Spawned**: When `PauseState` transitions to `Paused`
- **Despawned**: When `PauseState` transitions to `Active`

---

## Systems

### Input Handling

#### handle_pause_input

**Purpose**: Detect ESC key press and transition to paused state

**Parameters**:

- `keyboard: Res<ButtonInput<KeyCode>>`
- `pause_state: ResMut<PauseState>`
- `window: Single<&Window, With<PrimaryWindow>>` (read window mode before pause)

**Logic**:

```rust
if keyboard.just_pressed(KeyCode::Escape) && matches!(*pause_state, PauseState::Active) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        *pause_state = PauseState::Paused {
            window_mode_before_pause: window.mode.clone(),
        };
    }
    #[cfg(target_arch = "wasm32")]
    {
        *pause_state = PauseState::Paused;
    }
}
```

**Run Condition**: Always active (checks pause state internally)

**Related Requirements**: FR-003, FR-007, FR-014

---

#### handle_resume_input

**Purpose**: Detect mouse click and transition to active state

**Parameters**:

- `mouse: Res<ButtonInput<MouseButton>>`
- `pause_state: ResMut<PauseState>`

**Logic**:

```rust
if mouse.just_pressed(MouseButton::Left) && matches!(*pause_state, PauseState::Paused { .. }) {
    *pause_state = PauseState::Active;
}
```

**Run Condition**: Always active (checks pause state internally)

**Related Requirements**: FR-004, FR-011

---

### Physics Control

#### apply_pause_to_physics

**Purpose**: Freeze/resume physics simulation based on pause state

**Parameters**:

- `pause_state: Res<PauseState>`
- `rapier_config: ResMut<RapierConfiguration>`

**Logic**:

```rust
match *pause_state {
    PauseState::Active => {
        rapier_config.physics_pipeline_active = true;
    }
    PauseState::Paused { .. } => {
        rapier_config.physics_pipeline_active = false;
    }
}
```

**Run Condition**: Run on `PauseState` change (Bevy change detection)

**Related Requirements**: FR-001, FR-006

---

### Window Management

#### apply_pause_to_window_mode

**Purpose**: Switch window mode when entering/leaving pause state (native only)

**Parameters**:

- `pause_state: Res<PauseState>`
- `window: Single<&mut Window, With<PrimaryWindow>>`

**Logic**:

```rust
#[cfg(not(target_arch = "wasm32"))]
{
    match *pause_state {
        PauseState::Active => {
            // Restore original window mode from snapshot
            if let PauseState::Paused { window_mode_before_pause } = /* previous state */ {
                window.mode = window_mode_before_pause;
            }
        }
        PauseState::Paused { window_mode_before_pause } => {
            // Switch to windowed mode if was fullscreen
            if matches!(window_mode_before_pause, WindowMode::BorderlessFullscreen(_)) {
                window.mode = WindowMode::Windowed;
            }
            // If already windowed, no change (FR-010)
        }
    }
}
#[cfg(target_arch = "wasm32")]
{
    // No-op: WASM does not support window mode switching
}
```

**Run Condition**: Run on `PauseState` change

**Related Requirements**: FR-008, FR-009, FR-010, FR-013

---

### UI Management

#### spawn_pause_overlay

**Purpose**: Create pause overlay UI when game pauses

**Parameters**:

- `commands: Commands`
- `overlay_query: Query<(), With<PauseOverlay>>`

**Logic**:

```rust
// Only spawn if overlay doesn't exist (prevent duplicates)
if overlay_query.is_empty() {
    commands.spawn((
        Text::new("PAUSED\nClick to Resume"),
        TextFont {
            font_size: 60.0,
            ..default()
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
        PauseOverlay,
    ));
}
```

**Run Condition**: Run when `PauseState == Paused`

**Related Requirements**: FR-002

---

#### despawn_pause_overlay

**Purpose**: Remove pause overlay UI when game resumes

**Parameters**:

- `commands: Commands`
- `overlay_query: Query<Entity, With<PauseOverlay>>`

**Logic**:

```rust
for entity in overlay_query.iter() {
    commands.entity(entity).despawn_recursive();
}
```

**Run Condition**: Run when `PauseState == Active`

**Related Requirements**: FR-005

---

## State Transitions

### Pause Activation (ESC Key)

```text
[Active]
   ↓ (ESC pressed)
   ↓ handle_pause_input
   ↓ PauseState → Paused { window_mode_before_pause }
   ├─→ apply_pause_to_physics (physics_pipeline_active = false)
   ├─→ apply_pause_to_window_mode (switch to windowed if fullscreen)
   └─→ spawn_pause_overlay (UI appears)
[Paused]
```

### Resume Activation (Mouse Click)

```text
[Paused]
   ↓ (Click anywhere)
   ↓ handle_resume_input
   ↓ PauseState → Active
   ├─→ apply_pause_to_physics (physics_pipeline_active = true)
   ├─→ apply_pause_to_window_mode (restore original window mode)
   └─→ despawn_pause_overlay (UI disappears)
[Active]
```

---

## Integration with Existing Systems

### Level Transition Blocking (FR-012)

**Integration Point**: `src/level_loader.rs` systems

**Approach**: Add run condition to pause input system checking `LevelAdvanceState`

**Logic**:

```rust
fn can_pause(
    level_state: Res<LevelAdvanceState>,
    pause_state: Res<PauseState>,
) -> bool {
    matches!(*level_state, LevelAdvanceState::None)
        && matches!(*pause_state, PauseState::Active)
}

// Apply to handle_pause_input system
app.add_systems(Update, handle_pause_input.run_if(can_pause));
```

**Rationale**: Prevents pause during `SpawningBricks`, `GrowingPaddle`, `FadingOut`, etc.

---

## Data Flow Diagram

```text
┌─────────────────────┐
│   User Input        │
│  (ESC / Click)      │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Input Systems      │
│ • handle_pause      │
│ • handle_resume     │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  PauseState         │
│  (Resource)         │
└──────────┬──────────┘
           │
           ├──────────────────────────┐
           │                          │
           ▼                          ▼
┌─────────────────────┐    ┌─────────────────────┐
│ Physics Control     │    │ Window Management   │
│ • RapierConfig      │    │ • WindowMode        │
│ • pipeline_active   │    │   (native only)     │
└─────────────────────┘    └─────────────────────┘
           │                          │
           └──────────┬───────────────┘
                      │
                      ▼
           ┌─────────────────────┐
           │  UI Management      │
           │ • spawn_overlay     │
           │ • despawn_overlay   │
           └─────────────────────┘
```

---

## Performance Characteristics

### Memory Footprint

- `PauseState` resource: 2-8 bytes (enum discriminant + optional WindowMode)
- `PauseOverlay` UI entity: ~100 bytes (Node + Text + components)
- **Total**: <1 KB per pause/resume cycle

### Frame Impact

- **Pause activation**: 1 frame (input → state → physics freeze → UI spawn)
- **Resume activation**: 1 frame (input → state → physics resume → UI despawn)
- **Physics freeze**: Zero CPU impact (systems skip execution)

### Allocations

- UI spawning: Single allocation for entity archetype (Bevy manages)
- UI despawning: Deferred command (no immediate allocation)
- State transitions: Stack-only (enum copy)

---

## Testing Strategy

### Unit Tests

```rust
#[test]
fn pause_state_transitions() {
    // Active → Paused
    // Paused → Active
    // Paused → Paused (no-op)
}

#[test]
fn window_mode_snapshot_restore() {
    // Fullscreen → Windowed (on pause)
    // Windowed → Fullscreen (on resume)
    // Windowed → Windowed (no change on pause)
}
```

### Integration Tests

```rust
#[test]
fn physics_freeze_preserves_velocity() {
    // Spawn ball with velocity
    // Pause game
    // Verify ball position unchanged after 1 second
    // Resume game
    // Verify ball resumes movement from exact position
}

#[test]
fn pause_blocks_during_level_transition() {
    // Start level transition
    // Press ESC
    // Verify pause_state remains Active
}
```

### Manual Tests

- Pause during active gameplay (ESC) → verify freeze + overlay
- Resume from pause (click) → verify physics resumes
- Pause from fullscreen → verify switch to windowed (native)
- Resume from windowed → verify switch back to fullscreen (native)
- Rapid ESC presses → verify no stutter or crash
- WASM: verify window mode unchanged on pause

---

## Future Enhancements

1. **Pause Menu**: Replace simple text overlay with interactive menu (resume/restart/quit buttons)
2. **Pause Animation**: Fade in/out pause overlay (similar to `FadeOverlay` in level transitions)
3. **Audio Pause**: Mute/pause audio playback when paused (requires audio system integration)
4. **Gamepad Support**: Listen to gamepad Start button for pause/resume (FR-015 follow-up)
5. **Touch Support**: Detect tap gestures for pause/resume (FR-015 follow-up)

---

## References

- Feature Specification: [spec.md](spec.md)
- Research Findings: [research.md](research.md)
- Bevy ECS: <https://bevyengine.org/learn/book/getting-started/ecs/>
- Rapier3D Configuration: <https://rapier.rs/docs/>
- Project Constitution: `.specify/memory/constitution.md`
