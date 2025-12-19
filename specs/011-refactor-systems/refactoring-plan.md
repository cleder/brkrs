# Refactoring Plan: Systems Constitution Compliance

**Branch**: `copilot/refactor-legacy-code-systems` | **Date**: 2025-12-19
**Based on**: [compliance-audit.md](compliance-audit.md)

## Overview

This document outlines the specific refactoring steps needed to bring `src/systems/` into full Constitution compliance based on the compliance audit findings.

## Refactoring Strategy

### Phase 1: Foundation (P1 - Critical)

These changes are **REQUIRED** before the feature can be considered complete.

#### 1.1: Define SystemsError Type

**File**: `src/systems/mod.rs`

**Action**: Create a shared error type for all systems to return.

```rust
/// Error type for systems that can fail.
#[derive(Debug)]
pub enum SystemsError {
    /// Query returned unexpected number of entities
    QueryError(String),
    /// Resource not available
    MissingResource(String),
    /// Component not found
    MissingComponent(String),
    /// Other error
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for SystemsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryError(msg) => write!(f, "Query error: {}", msg),
            Self::MissingResource(msg) => write!(f, "Missing resource: {}", msg),
            Self::MissingComponent(msg) => write!(f, "Missing component: {}", msg),
            Self::Other(e) => write!(f, "System error: {}", e),
        }
    }
}

impl std::error::Error for SystemsError {}

/// Type alias for system result
pub type SystemResult = Result<(), SystemsError>;
```

**Rationale**: Provides a consistent error type for all systems, enabling fallible system pattern.

---

#### 1.2: Refactor All Systems to Return Result

**Files**: All system function signatures

**Pattern**: Add `-> Result<(), Box<dyn std::error::Error>>` or `-> SystemResult` to every system function.

**Before**:
```rust
fn my_system(query: Query<&Component>) {
    // ...
}
```

**After**:
```rust
fn my_system(query: Query<&Component>) -> Result<(), Box<dyn std::error::Error>> {
    // ...
    Ok(())
}
```

**Affected Files**:
1. `audio.rs`: 9 systems + all observers
2. `cheat_mode.rs`: 1 system
3. `grid_debug.rs`: 2 systems
4. `level_switch.rs`: 3 systems
5. `multi_hit.rs`: 1 system
6. `paddle_size.rs`: 5 systems
7. `respawn.rs`: 6 systems
8. `scoring.rs`: 2 systems
9. `textures/loader.rs`: 3 systems
10. `textures/materials.rs`: 1 system

**Total**: ~33 system functions

---

#### 1.3: Fix Panicking Pattern in respawn.rs

**File**: `src/systems/respawn.rs`
**Line**: 603

**Before**:
```rust
let request = respawn_schedule.pending.take().unwrap();
```

**After**:
```rust
let Some(request) = respawn_schedule.pending.take() else {
    // No pending respawn request - this can happen if the schedule was cleared
    return Ok(());
};
```

**Rationale**: Eliminates panic risk by using safe error recovery pattern.

---

#### 1.4: Add Required Components to GridOverlay

**File**: Component definition (likely in `src/lib.rs` or `src/components.rs`)

**Before**:
```rust
#[derive(Component)]
pub struct GridOverlay;
```

**After**:
```rust
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct GridOverlay;
```

**File**: `src/systems/grid_debug.rs`

**Update spawning code** (lines 39-45, 54-60):

**Before**:
```rust
commands.spawn((
    Mesh3d(line_mesh),
    MeshMaterial3d(grid_material.clone()),
    Transform::from_xyz(0.0, 2.0, z_pos),
    GridOverlay,
    Visibility::Hidden,
));
```

**After**:
```rust
commands.spawn((
    Mesh3d(line_mesh),
    MeshMaterial3d(grid_material.clone()),
    GridOverlay,  // Transform and Visibility added automatically
));
// Set transform and visibility after spawn if needed
```

**Rationale**: Eliminates manual component bundling, follows Bevy 0.17 required components pattern.

---

#### 1.5: Create Missing Plugins

##### GridDebugPlugin

**File**: `src/systems/grid_debug.rs`

**Add**:
```rust
/// Plugin for grid debug overlay functionality.
pub struct GridDebugPlugin;

impl Plugin for GridDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grid_overlay);
        
        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Update, toggle_grid_visibility);
    }
}
```

**File**: `src/systems/mod.rs`

**Add export**:
```rust
pub use grid_debug::GridDebugPlugin;
```

##### ScoringPlugin

**File**: `src/systems/scoring.rs`

**Add**:
```rust
/// Plugin for scoring and milestone tracking.
pub struct ScoringPlugin;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScoreState>()
            .add_message::<BrickDestroyed>()
            .add_message::<MilestoneReached>()
            .add_systems(Update, (
                award_brick_points,
                detect_milestones,
            ));
    }
}
```

**File**: `src/systems/mod.rs`

**Add export**:
```rust
pub use scoring::ScoringPlugin;
```

---

### Phase 2: System Organization (P2 - Important)

These changes improve code quality and follow Constitution best practices.

#### 2.1: Define System Sets for All Plugins

##### AudioSystems

**File**: `src/systems/audio.rs`

**Add**:
```rust
/// System sets for audio subsystem organization.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudioSystems {
    /// Startup: Load assets and configuration
    Startup,
    /// Update: Manage playback and cleanup
    Update,
}
```

**Update plugin**:
```rust
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, AudioSystems::Startup)
            .configure_sets(Update, AudioSystems::Update);
        
        app.init_resource::<AudioAssets>()
            .init_resource::<ActiveSounds>()
            .init_resource::<ActiveAudioInstances>()
            .add_message::<UiBeepEvent>()
            .add_systems(Startup, 
                (load_audio_config, load_audio_assets)
                    .chain()
                    .in_set(AudioSystems::Startup)
            )
            .add_systems(Update, 
                (save_audio_config_on_change, cleanup_finished_sounds)
                    .in_set(AudioSystems::Update)
            )
            .add_observer(on_multi_hit_brick_sound)
            // ... more observers
    }
}
```

##### CheatModeSystems

**File**: `src/systems/cheat_mode.rs`

**Add**:
```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheatModeSystems {
    Input,
}
```

**Update plugin**:
```rust
impl Plugin for CheatModePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PreUpdate, CheatModeSystems::Input);
        
        app.init_resource::<CheatModeState>()
            .add_message::<CheatModeToggled>()
            .add_systems(
                PreUpdate,
                toggle_cheat_mode_input
                    .run_if(crate::pause::not_paused)
                    .in_set(CheatModeSystems::Input),
            );
    }
}
```

##### LevelSwitchSystems

**File**: `src/systems/level_switch.rs`

**Add**:
```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LevelSwitchSystems {
    Input,
    Logic,
}
```

**Update plugin** to use sets and chain them.

##### PaddleSizeSystems

**File**: `src/systems/paddle_size.rs`

**Add**:
```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaddleSizeSystems {
    Detection,
    Application,
    Cleanup,
}
```

**Update plugin**:
```rust
impl Plugin for PaddleSizePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (
            PaddleSizeSystems::Detection,
            PaddleSizeSystems::Application,
            PaddleSizeSystems::Cleanup,
        ).chain());
        
        app.add_message::<PaddleSizeEffectApplied>()
            .add_systems(Update, 
                detect_paddle_brick_collision
                    .in_set(PaddleSizeSystems::Detection)
            )
            .add_systems(Update, 
                (apply_size_effect, tick_active_effects)
                    .in_set(PaddleSizeSystems::Application)
            )
            .add_systems(Update, 
                (restore_on_life_lost, restore_on_level_switch)
                    .in_set(PaddleSizeSystems::Cleanup)
            );
    }
}
```

##### GridDebugSystems

**File**: `src/systems/grid_debug.rs`

**Add**:
```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GridDebugSystems {
    Spawn,
    Update,
}
```

##### ScoringPlugin Systems

**File**: `src/systems/scoring.rs`

**Add**:
```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoringSystems {
    Awards,
    Milestones,
}
```

##### TextureSystems

**File**: `src/systems/textures/mod.rs`

**Add**:
```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureSystems {
    Startup,
    Update,
}
```

**Update TextureManifestPlugin** to use these sets.

---

### Phase 3: Documentation & Polish (P3 - Nice-to-Have)

#### 3.1: Fill Rustdoc Gaps

**Pattern**: Add module-level and function-level documentation for all public items.

**Example** (audio.rs):
```rust
/// Play a sound effect respecting volume, mute, and concurrent limits.
///
/// # Purpose
///
/// Spawns an audio player entity for the given sound type, applying volume
/// settings and enforcing a maximum of 4 concurrent sounds per type to prevent
/// audio distortion.
///
/// # When to Use
///
/// Called by audio observers when game events occur. Not intended for direct
/// external use.
fn play_sound(
    sound_type: SoundType,
    config: &AudioConfig,
    assets: &AudioAssets,
    active_sounds: &mut ActiveSounds,
    active_instances: &mut ActiveAudioInstances,
    commands: &mut Commands,
) {
    // ...
}
```

**Files**: All public functions in all systems files.

---

## Implementation Checklist

### Phase 1: Foundation

- [ ] 1.1: Create `SystemsError` type in mod.rs
- [ ] 1.2: Add `Result` return types to all 33+ system functions
- [ ] 1.3: Fix `.unwrap()` panic in respawn.rs:603
- [ ] 1.4: Add `#[require()]` to GridOverlay component
- [ ] 1.5a: Create GridDebugPlugin
- [ ] 1.5b: Create ScoringPlugin

### Phase 2: System Organization

- [ ] 2.1a: Define AudioSystems enum and reorganize
- [ ] 2.1b: Define CheatModeSystems enum and reorganize
- [ ] 2.1c: Define LevelSwitchSystems enum and reorganize
- [ ] 2.1d: Define PaddleSizeSystems enum and reorganize
- [ ] 2.1e: Define GridDebugSystems enum
- [ ] 2.1f: Define ScoringSystems enum
- [ ] 2.1g: Define TextureSystems enum

### Phase 3: Documentation

- [ ] 3.1: Fill rustdoc gaps in all public items

---

## Testing Strategy

After each phase, run:

```bash
# Verify compilation
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --all --check

# Run clippy
cargo clippy --all-targets --all-features

# Run bevy lint
bevy lint
```

Specific tests to add:
1. Fallible systems compilation test
2. Error recovery test for respawn
3. Required components test for GridOverlay
4. System sets registration test

---

## Risk Assessment

### Low Risk

- Adding `Result` return types (compile-time verification)
- Adding `#[require()]` attributes (Bevy 0.17 feature)
- Defining system sets (organizational change)

### Medium Risk

- Fixing `.unwrap()` in respawn.rs (behavior change - needs testing)
- Creating new plugins (integration risk - needs manual testing)

### Mitigation

- Write tests first (TDD approach)
- Test native + WASM builds
- Manual smoke testing after each phase

---

## Success Criteria

- [ ] All systems return `Result`
- [ ] No `.unwrap()` or `.expect()` on query results
- [ ] All components use `#[require()]` where appropriate
- [ ] All subsystems have dedicated plugins
- [ ] All plugins use system sets with `*Systems` suffix
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] No bevy lint warnings
- [ ] Manual smoke test passes (native + WASM)

---

**Plan Date**: 2025-12-19
**Estimated Effort**: 2-3 days
**Priority**: P1 (Critical Constitution Compliance)
