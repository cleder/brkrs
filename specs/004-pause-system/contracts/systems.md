# System Contracts: Pause and Resume System

**Feature**: `004-pause-system`
**Date**: 2025-11-28

## Overview

This document defines the contracts (interfaces, behaviors, guarantees) for the pause system's internal systems and their interactions with the rest of the game.

---

## PauseState Resource Contract

**Type**: `Resource`
**Module**: `src/pause.rs`

### Interface

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

impl PauseState {
    pub fn is_paused(&self) -> bool;
    pub fn is_active(&self) -> bool;
}
```

### Guarantees

1. **Single Source of Truth**: PauseState is the **only** authoritative source for pause status
2. **Immutable Queries**: Systems MAY read `Res<PauseState>` without side effects
3. **Mutable Exclusivity**: Only pause input systems MAY write `ResMut<PauseState>`
4. **State Consistency**: State transitions are atomic (no partial updates)
5. **Platform Variant**: WASM variant does not store `window_mode_before_pause`

### Dependencies

- `bevy::window::WindowMode` (native only)

### Usage Example

```rust
// Run condition for gameplay systems
fn is_active(pause_state: Res<PauseState>) -> bool {
    pause_state.is_active()
}

app.add_systems(Update, gameplay_system.run_if(is_active));
```

---

## Input Systems Contract

### handle_pause_input

**System Type**: Input Handler
**Execution Schedule**: `Update` (runs every frame)
**Module**: `src/pause.rs`

#### Parameters

```rust
fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut pause_state: ResMut<PauseState>,
    window: Single<&Window, With<PrimaryWindow>>,
    level_state: Res<LevelAdvanceState>,
) -> ()
```

#### Behavior Guarantees

1. **Input Check**: Processes `KeyCode::Escape` via `just_pressed()` (frame-level debouncing)
2. **State Guard**: Only transitions if `pause_state.is_active()` (ignores ESC when paused)
3. **Blocking Check**: Respects `LevelAdvanceState` (no pause during level transitions - FR-012)
4. **Window Snapshot**: Captures current `window.mode` before transition (native only)
5. **No Side Effects**: Does not mutate window, physics, or UI directly

#### Preconditions

- Window must exist (guaranteed by Bevy's `PrimaryWindow` marker)
- `LevelAdvanceState` resource must exist (initialized by `LevelLoaderPlugin`)

#### Postconditions

- If conditions met: `PauseState::Active → PauseState::Paused`
- Window mode captured in `Paused` variant (native only)
- State change triggers dependent systems via Bevy change detection

#### Error Handling

- Missing window: System panics (acceptable - window must exist)
- Missing resources: System panics (acceptable - resources guaranteed by plugin init)

---

### handle_resume_input

**System Type**: Input Handler
**Execution Schedule**: `Update` (runs every frame)
**Module**: `src/pause.rs`

#### Parameters

```rust
fn handle_resume_input(
    mouse: Res<ButtonInput<MouseButton>>,
    mut pause_state: ResMut<PauseState>,
) -> ()
```

#### Behavior Guarantees

1. **Input Check**: Processes `MouseButton::Left` via `just_pressed()`
2. **State Guard**: Only transitions if `pause_state.is_paused()` (ignores clicks when active)
3. **Unconditional**: Accepts clicks anywhere in window (FR-011)
4. **No Side Effects**: Does not mutate window, physics, or UI directly

#### Preconditions

- None (mouse input always available)

#### Postconditions

- If conditions met: `PauseState::Paused → PauseState::Active`
- State change triggers dependent systems via Bevy change detection

---

## Physics Control Contract

### apply_pause_to_physics

**System Type**: State Reactor
**Execution Schedule**: `Update` (runs on `PauseState` change)
**Module**: `src/pause.rs`

#### Parameters

```rust
fn apply_pause_to_physics(
    pause_state: Res<PauseState>,
    mut rapier_config: ResMut<RapierConfiguration>,
) -> ()
```

#### Behavior Guarantees

1. **Freeze on Pause**: Sets `physics_pipeline_active = false` when `PauseState::Paused`
2. **Resume on Active**: Sets `physics_pipeline_active = true` when `PauseState::Active`
3. **State Preservation**: Velocities, positions, forces preserved during freeze (FR-006)
4. **Immediate Effect**: Physics freeze takes effect on the same frame as state change

#### Preconditions

- `RapierConfiguration` resource must exist (initialized by `RapierPhysicsPlugin`)

#### Postconditions

- Physics pipeline state synchronized with pause state
- No physics simulation steps occur while paused
- Physics resumes from exact pre-pause state when unpaused

#### Performance Contract

- **Frame Impact**: <1ms (single boolean write)
- **Memory Impact**: Zero allocations
- **State Corruption**: Zero risk (physics state immutable during freeze)

---

## Window Management Contract

### apply_pause_to_window_mode

**System Type**: State Reactor (Native Only)
**Execution Schedule**: `Update` (runs on `PauseState` change)
**Module**: `src/pause.rs`

#### Parameters

```rust
#[cfg(not(target_arch = "wasm32"))]
fn apply_pause_to_window_mode(
    pause_state: Res<PauseState>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) -> ()
```

#### Behavior Guarantees (Native)

1. **Windowed on Pause**: Switches to `WindowMode::Windowed` if was fullscreen (FR-008)
2. **Restore on Resume**: Restores `window_mode_before_pause` from state (FR-009)
3. **No-Op if Windowed**: Does not change mode if already windowed (FR-010)
4. **Graceful Failure**: Handles display limitations (FR-013) - window manager rejects → no crash

#### Behavior Guarantees (WASM)

- **No-Op**: System not compiled on WASM (window mode switching unsupported)

#### Preconditions

- Window must exist (guaranteed by `PrimaryWindow` marker)
- `PauseState::Paused` variant must contain `window_mode_before_pause` (native only)

#### Postconditions

- Window mode synchronized with pause state
- User can interact with OS during pause (windowed mode)
- Fullscreen restored on resume (if was fullscreen originally)

#### Error Handling

- Display does not support fullscreen: Window manager silently rejects → remain windowed (no crash)
- Manual window mode change during pause: Respected (user override honored)

#### Performance Contract

- **Frame Impact**: <100ms (window mode switching latency - FR-006, FR-007)
- **Memory Impact**: Zero allocations
- **User Perception**: Smooth transition (no flicker or delay)

---

## UI Management Contracts

### spawn_pause_overlay

**System Type**: UI Spawner
**Execution Schedule**: `Update` (runs when entering `PauseState::Paused`)
**Module**: `src/ui/pause_overlay.rs`

#### Parameters

```rust
fn spawn_pause_overlay(
    mut commands: Commands,
    existing_overlay: Query<(), With<PauseOverlay>>,
) -> ()
```

#### Behavior Guarantees

1. **Idempotency**: Only spawns if no overlay exists (prevents duplicates)
2. **Message Content**: Displays "PAUSED\nClick to Resume" (FR-002)
3. **Full-Screen**: Overlay covers entire window (centered text)
4. **Immediate Visibility**: Appears within 1 frame (<16ms at 60 FPS - SC-003)
5. **Marker Component**: Entity tagged with `PauseOverlay` for later cleanup

#### Preconditions

- Bevy UI plugin initialized (guaranteed by `DefaultPlugins`)
- Default font available (embedded in Bevy 0.16)

#### Postconditions

- UI entity spawned with `PauseOverlay` marker
- Text visible at center of screen
- Entity queryable by `Query<Entity, With<PauseOverlay>>`

#### Performance Contract

- **Frame Impact**: <1ms (single entity spawn)
- **Memory Impact**: ~100 bytes (UI entity archetype)
- **Allocation**: Single allocation (deferred command)

---

### despawn_pause_overlay

**System Type**: UI Cleanup
**Execution Schedule**: `Update` (runs when entering `PauseState::Active`)
**Module**: `src/ui/pause_overlay.rs`

#### Parameters

```rust
fn despawn_pause_overlay(
    mut commands: Commands,
    overlay_query: Query<Entity, With<PauseOverlay>>,
) -> ()
```

#### Behavior Guarantees

1. **Complete Cleanup**: Despawns all entities with `PauseOverlay` marker
2. **Recursive**: Uses `despawn_recursive()` to clean up child entities
3. **Immediate Effect**: Overlay disappears within 1 frame (<16ms at 60 FPS - SC-004)
4. **No Leaks**: All UI entities removed (no dangling entities)

#### Preconditions

- `PauseOverlay` entities exist (spawned by `spawn_pause_overlay`)

#### Postconditions

- No entities with `PauseOverlay` marker remain
- UI overlay invisible
- Memory reclaimed (entity archetype freed)

#### Performance Contract

- **Frame Impact**: <1ms (single entity despawn)
- **Memory Impact**: ~100 bytes freed
- **Allocation**: Deferred command (no immediate allocation)

---

## Cross-System Integration Contracts

### Level Transition Blocking

**Contract**: Pause input MUST be blocked during level transitions

**Implementation**:

```rust
fn can_pause(
    level_state: Res<LevelAdvanceState>,
) -> bool {
    matches!(*level_state, LevelAdvanceState::None)
}

app.add_systems(Update, handle_pause_input.run_if(can_pause));
```

**Guarantees**:

- Pause request ignored during `SpawningBricks`, `GrowingPaddle`, `FadingOut`, etc.
- Level transition completes uninterrupted
- Pause activates only when `LevelAdvanceState::None` (FR-012)

---

### Respawn Sequence Blocking

**Contract**: Pause input SHOULD be blocked during ball respawn delay

**Implementation**:

```rust
fn can_pause(
    respawn_state: Res<RespawnVisualState>,
) -> bool {
    !respawn_state.is_respawning()
}
```

**Guarantees**:

- Pause request ignored during respawn countdown
- Ball appears before pause can activate
- Prevents confusing pause during respawn animation

---

## Testing Contracts

### Unit Test Requirements

Each system MUST have unit tests verifying:

1. **State Transitions**: `Active ↔ Paused` transitions work correctly
2. **Idempotency**: Repeated calls with same state are no-ops
3. **Input Handling**: `just_pressed()` triggers state change
4. **Platform Variants**: WASM vs native behavior differs correctly

### Integration Test Requirements

Feature MUST have integration tests verifying:

1. **Physics Freeze**: Ball velocity unchanged during pause
2. **State Preservation**: Resume from exact pre-pause state (10 second pause test)
3. **Window Mode**: Fullscreen → windowed → fullscreen cycle works (native)
4. **UI Lifecycle**: Overlay spawns and despawns correctly
5. **Cycle Stability**: 10 consecutive pause/resume cycles succeed (SC-009)

### Performance Test Requirements

Feature MUST verify:

1. **Pause Latency**: <16ms from ESC press to physics freeze (SC-003)
2. **Resume Latency**: <16ms from click to physics resume (SC-004)
3. **Window Switching**: <100ms for mode transition (SC-006, SC-007)
4. **Frame Rate**: 60 FPS maintained during transitions

---

## Versioning & Compatibility

### API Stability

- **PauseState Resource**: Public API (other systems may query)
- **Internal Systems**: Private (implementation detail)
- **Breaking Changes**: Require major version bump (per constitution)

### Backward Compatibility

- **WASM Builds**: Must continue to work (window mode no-op)
- **Existing Input**: Must not conflict with existing keyboard/mouse handling
- **Level Transitions**: Must respect existing `LevelAdvanceState` blocking

---

## References

- Feature Specification: [spec.md](spec.md)
- Data Model: [data-model.md](data-model.md)
- Research: [research.md](research.md)
- Bevy System API: <https://bevyengine.org/learn/book/getting-started/systems/>
- Rapier3D Configuration: <https://rapier.rs/docs/>
