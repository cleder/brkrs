# Compliance Audit: src/systems Constitution Violations

**Branch**: `copilot/refactor-legacy-code-systems` | **Date**: 2025-12-19
**Audited Against**: Constitution v1.3.0, Section VIII (Bevy 0.17 Mandates & Prohibitions)

## Executive Summary

This audit reviews all files in `src/systems/` for compliance with the Brkrs Constitution, specifically Section VIII: Bevy 0.17 Mandates & Prohibitions. The audit identifies violations across 9 core systems files and the textures subsystem.

**Overall Status**: ❌ FAIL - Multiple Constitution violations found

**Files Audited**: 14 files
- Core systems: 9 files (`*.rs` in `src/systems/`)
- Textures subsystem: 5 files (`src/systems/textures/*.rs`)

**Violation Summary**:
- Fallible Systems: 13 violations (all system functions)
- NO Panicking Queries: 3 potential violations
- Asset Handle Reuse: Already compliant ✓
- Required Components: 2 violations
- Plugin-Based Architecture: 1 violation (missing system sets organization)
- System Organization: 1 violation (no explicit system sets with *Systems suffix)

---

## File-by-File Findings

### 1. src/systems/audio.rs (929 lines)

**Status**: ❌ FAIL - Multiple violations

#### Violation 1.1: Non-Fallible Systems (Constitution VIII: Fallible Systems)

**Lines**: 216-228 (cleanup_finished_sounds), 238-317 (load_audio_config), 320-407 (save_audio_config_on_change), 412-478 (load_audio_assets), All observer functions (590-817)

**Issue**: All system functions do not return `Result`. They return `()` implicitly or explicitly.

**Constitution Rule**: "All systems MUST return `Result` and use the `?` operator for error propagation."

**Evidence**:
```rust
fn cleanup_finished_sounds(
    mut removed: RemovedComponents<AudioPlayer>,
    mut active_instances: ResMut<ActiveAudioInstances>,
    mut active_sounds: ResMut<ActiveSounds>,
) {  // ❌ No Result return type
    // ...
}

fn load_audio_config(mut commands: Commands) {  // ❌ No Result
    // ...
}
```

**Recommendation**: Add `-> Result<(), Box<dyn std::error::Error>>` return type and use `?` operator where appropriate.

#### Violation 1.2: Potential Panicking Pattern (Constitution VIII: NO Panicking Queries)

**Line**: 174

**Issue**: `.unwrap_or(&0)` used on HashMap lookup (not a panic risk, but pattern should be consistent)

**Evidence**:
```rust
*self.counts.get(&sound_type).unwrap_or(&0)
```

**Recommendation**: This is actually safe (unwrap_or provides fallback), but document why it's not a panic risk.

#### Finding 1.3: ✓ Asset Handle Caching COMPLIANT

**Assessment**: Audio assets are loaded once in `load_audio_assets` startup system and stored in `AudioAssets` Resource. Handles are reused via `.get()` method. No repeated `asset_server.load()` calls in observers.

**Evidence**: Line 447-454 shows one-time loading, lines 510-518 show handle reuse in `play_sound`.

#### Finding 1.4: ✓ Plugin Architecture COMPLIANT  

**Assessment**: `AudioPlugin` exists with complete plugin implementation (lines 192-213). All resources and systems properly registered.

#### Violation 1.5: System Organization (Constitution VIII: System Organization)

**Issue**: Systems are registered individually, not organized into system sets with `*Systems` suffix.

**Evidence**:
```rust
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioAssets>()
            .add_systems(Startup, (load_audio_config, load_audio_assets).chain())
            .add_systems(Update, save_audio_config_on_change)
            .add_systems(Update, cleanup_finished_sounds)
            .add_observer(on_multi_hit_brick_sound)
            // ... more observers
    }
}
```

**Recommendation**: Define `AudioSystems` enum with `Startup`, `Update` sets and group systems accordingly.

---

### 2. src/systems/cheat_mode.rs (88 lines)

**Status**: ❌ FAIL - Non-fallible system

#### Violation 2.1: Non-Fallible System (Constitution VIII: Fallible Systems)

**Lines**: 52-65 (toggle_cheat_mode_input)

**Issue**: System does not return `Result`.

**Evidence**:
```rust
fn toggle_cheat_mode_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cheat_state: ResMut<CheatModeState>,
    mut writer: MessageWriter<CheatModeToggled>,
) {  // ❌ No Result return
    // ...
}
```

**Recommendation**: Add `-> Result<(), Box<dyn std::error::Error>>` return type.

#### Finding 2.2: ✓ Plugin Architecture COMPLIANT

**Assessment**: `CheatModePlugin` exists with proper plugin implementation.

#### Violation 2.3: System Organization (Constitution VIII: System Organization)

**Issue**: System registered directly without system sets.

**Recommendation**: Define `CheatModeSystems` enum and organize system.

---

### 3. src/systems/grid_debug.rs (86 lines)

**Status**: ❌ FAIL - Multiple violations

#### Violation 3.1: Non-Fallible Systems (Constitution VIII: Fallible Systems)

**Lines**: 15-62 (spawn_grid_overlay), 67-80 (toggle_grid_visibility)

**Issue**: Systems do not return `Result`.

**Recommendation**: Add `-> Result<(), Box<dyn std::error::Error>>` return types.

#### Violation 3.2: Required Components (Constitution VIII: Required Components)

**Lines**: 39-45, 54-60

**Issue**: `GridOverlay` marker spawned with manual `Transform` and `Visibility` components instead of using `#[require()]`.

**Evidence**:
```rust
commands.spawn((
    Mesh3d(line_mesh),
    MeshMaterial3d(grid_material.clone()),
    Transform::from_xyz(0.0, 2.0, z_pos),
    GridOverlay,  // ❌ Should require Transform/Visibility
    Visibility::Hidden,
));
```

**Recommendation**: Add `#[require(Transform, Visibility)]` to `GridOverlay` component definition.

#### Finding 3.3: ⚠️ No Plugin Implementation

**Issue**: No `GridDebugPlugin` found. Systems likely registered in main app.

**Recommendation**: Create `GridDebugPlugin` for modularity.

---

### 4. src/systems/level_switch.rs (333 lines)

**Status**: ❌ FAIL - Multiple violations

#### Violation 4.1: Non-Fallible Systems (Constitution VIII: Fallible Systems)

**Lines**: 178-191 (poll_file_trigger), 196-248 (keyboard_level_switch), 253-298 (execute_pending_transition)

**Issue**: All systems do not return `Result`.

**Recommendation**: Add `-> Result<(), Box<dyn std::error::Error>>` return types and use `?` for error propagation.

#### Finding 4.2: ✓ Plugin Architecture COMPLIANT

**Assessment**: `LevelSwitchPlugin` exists with proper implementation.

#### Violation 4.3: System Organization (Constitution VIII: System Organization)

**Issue**: Systems registered individually without system sets.

**Recommendation**: Define `LevelSwitchSystems` enum with `Input`, `Logic` sets.

---

### 5. src/systems/mod.rs (27 lines)

**Status**: ✓ PASS - Module exports only

**Assessment**: This file only contains module declarations and pub use statements. No systems or logic to audit.

---

### 6. src/systems/multi_hit.rs (130 lines)

**Status**: ❌ FAIL - Non-fallible system

#### Violation 6.1: Non-Fallible System (Constitution VIII: Fallible Systems)

**Lines**: 66-95 (watch_brick_type_changes)

**Issue**: System does not return `Result`.

**Evidence**:
```rust
#[cfg(feature = "texture_manifest")]
pub fn watch_brick_type_changes(
    mut bricks: Query<
        (&BrickTypeId, &mut MeshMaterial3d<StandardMaterial>),
        (With<crate::Brick>, Changed<BrickTypeId>),
    >,
    type_registry: Res<crate::systems::textures::TypeVariantRegistry>,
    fallback: Option<Res<crate::systems::textures::FallbackRegistry>>,
) {  // ❌ No Result
    // ...
}
```

**Recommendation**: Add `-> Result<(), Box<dyn std::error::Error>>` return type.

#### Finding 6.2: ✓ Change Detection COMPLIANT

**Assessment**: System uses `Changed<BrickTypeId>` filter (line 69), following Constitution mandate.

#### Finding 6.3: ⚠️ No Plugin Implementation

**Issue**: No `MultiHitPlugin` found. System likely registered elsewhere.

**Recommendation**: Create `MultiHitPlugin` or integrate into appropriate plugin.

---

### 7. src/systems/paddle_size.rs (308 lines)

**Status**: ❌ FAIL - Multiple violations

#### Violation 7.1: Non-Fallible Systems (Constitution VIII: Fallible Systems)

**Lines**: 131-175 (detect_paddle_brick_collision), 180-215 (apply_size_effect), 220-236 (tick_active_effects), 241-266 (restore_on_life_lost), 271-287 (restore_on_level_switch)

**Issue**: All 5 systems do not return `Result`.

**Recommendation**: Add `-> Result<(), Box<dyn std::error::Error>>` return types to all systems.

#### Finding 7.2: ✓ Plugin Architecture COMPLIANT

**Assessment**: `PaddleSizePlugin` exists with proper implementation (lines 96-118).

#### Violation 7.3: System Organization (Constitution VIII: System Organization)

**Issue**: Systems registered individually in Update set without explicit system sets.

**Evidence** (lines 106-117):
```rust
.add_systems(
    Update,
    (
        detect_paddle_brick_collision,
        apply_size_effect,
        tick_active_effects,
        restore_on_life_lost,
        restore_on_level_switch,
    ),
)
```

**Recommendation**: Define `PaddleSizeSystems` enum with sets like `Detection`, `Application`, `Cleanup`.

---

### 8. src/systems/respawn.rs (1189 lines)

**Status**: ❌ FAIL - Multiple violations

#### Violation 8.1: Non-Fallible Systems (Constitution VIII: Fallible Systems)

**Lines**: Multiple system functions (594-679 spawn_paddle_and_ball, 684-718 handle_life_loss, 723-736 handle_milestone_awards, 741-785 respawn_fade_animation, 790-809 handle_input_lock, 814-848 test_input_lock_integration)

**Issue**: All major systems do not return `Result`.

**Recommendation**: Add `-> Result<(), Box<dyn std::error::Error>>` return types.

#### Violation 8.2: Potential Panicking Pattern (Constitution VIII: Error Recovery Patterns)

**Line**: 603

**Issue**: `.unwrap()` called on `pending.take()` without fallback.

**Evidence**:
```rust
let request = respawn_schedule.pending.take().unwrap();
```

**Recommendation**: Use `let Some(request) = respawn_schedule.pending.take() else { return Ok(()); };` pattern.

#### Finding 8.3: ✓ Plugin Architecture COMPLIANT

**Assessment**: `RespawnPlugin` exists with system sets already defined!

**Evidence** (lines 466-521): `RespawnSystems` enum exists with `LifeLoss`, `Spawn`, `Animation`, `Input` sets. This is the best example in the codebase.

#### Finding 8.4: ✓ System Organization COMPLIANT (Partially)

**Assessment**: System sets ARE defined with `*Systems` suffix (`RespawnSystems`). However, systems still need to return `Result`.

---

### 9. src/systems/scoring.rs (223 lines)

**Status**: ❌ FAIL - Non-fallible systems

#### Violation 9.1: Non-Fallible Systems (Constitution VIII: Fallible Systems)

**Lines**: 155-180 (award_brick_points), 185-204 (detect_milestones)

**Issue**: Both systems do not return `Result`.

**Recommendation**: Add `-> Result<(), Box<dyn std::error::Error>>` return types.

#### Finding 9.2: ⚠️ No Plugin Implementation

**Issue**: No `ScoringPlugin` found. Systems likely registered in main app.

**Recommendation**: Create `ScoringPlugin` with proper system sets.

---

### 10. src/systems/textures/mod.rs (21 lines)

**Status**: ✓ PASS - Module exports only

**Assessment**: Module declarations and re-exports only.

---

### 11. src/systems/textures/contracts.rs (287 lines)

**Status**: ⚠️ ADVISORY - No systems, utility functions only

**Assessment**: Contains validation and serialization helpers. No system functions to audit. One `.unwrap_or` pattern (line 208) is safe (provides fallback).

---

### 12. src/systems/textures/loader.rs (334 lines)

**Status**: ❌ FAIL - Non-fallible systems

#### Violation 12.1: Non-Fallible Systems (Constitution VIII: Fallible Systems)

**Lines**: 121-172 (load_texture_manifest), 177-262 (watch_ball_type_changes), 267-320 (watch_paddle_type_changes)

**Issue**: All 3 systems do not return `Result`.

**Recommendation**: Add `-> Result<(), Box<dyn std::error::Error>>` return types.

#### Finding 12.2: ✓ Change Detection COMPLIANT

**Assessment**: Both watch systems use `Changed<T>` filters (lines 178, 268).

---

### 13. src/systems/textures/materials.rs (606 lines)

**Status**: ❌ FAIL - Non-fallible system

#### Violation 13.1: Non-Fallible System (Constitution VIII: Fallible Systems)

**Lines**: 429-537 (setup_texture_materials)

**Issue**: Startup system does not return `Result`.

**Recommendation**: Add `-> Result<(), Box<dyn std::error::Error>>` return type.

#### Finding 13.2: ⚠️ Safe Unwrap Patterns

**Lines**: 126, 195, 225

**Assessment**: Multiple `.unwrap_or` and `.unwrap_or_else` calls with fallbacks. These are safe patterns, not panics.

---

### 14. src/systems/textures/overrides.rs (310 lines)

**Status**: ⚠️ ADVISORY - No systems, utility functions only

**Assessment**: Contains override loading and parsing logic. No system functions to audit.

---

## Constitution Section VIII Coverage

### ✅ Compliant Areas

1. **Asset Handle Reuse**: Audio system loads assets once and caches handles ✓
2. **Change Detection**: Multi-hit and texture watch systems use `Changed<T>` filters ✓  
3. **Plugin Architecture**: Audio, CheatMode, LevelSwitch, PaddleSize, Respawn plugins exist ✓
4. **System Sets** (Partial): Respawn has proper `RespawnSystems` enum ✓

### ❌ Violation Areas

1. **Fallible Systems**: ALL 13+ system functions need `Result` return types
2. **Panicking Queries**: 1 `.unwrap()` on Option (respawn.rs:603) needs error recovery
3. **Required Components**: GridOverlay needs `#[require(Transform, Visibility)]`
4. **System Organization**: 6 plugins lack explicit system sets with `*Systems` suffix
5. **Plugin Architecture**: 2 subsystems (grid_debug, scoring) lack dedicated plugins

---

## Refactoring Priorities

### P1: Critical (Constitution Mandates)

1. **Add Result return types to all systems** (13 files affected)
2. **Replace `.unwrap()` in respawn.rs:603** with safe error recovery
3. **Add `#[require()]` to GridOverlay component**
4. **Create missing plugins** (GridDebugPlugin, ScoringPlugin)

### P2: Important (Best Practices)

5. **Define system sets** for 6 plugins lacking `*Systems` enums:
   - AudioSystems
   - CheatModeSystems  
   - LevelSwitchSystems
   - PaddleSizeSystems
   - TextureSystems (for textures subsystem)
   - ScoringPlugin (once created)

### P3: Nice-to-Have (Code Quality)

6. **Add rustdoc** for missing public items
7. **Consolidate system sets** into consistent patterns across all plugins

---

## Test Coverage Gaps

Based on this audit, the following test scenarios are needed:

1. **Fallible Systems Test**: Verify all systems return `Result` (compile-time check or integration test)
2. **Error Recovery Test**: Verify respawn system handles missing pending requests gracefully
3. **Required Components Test**: Verify GridOverlay spawns with Transform/Visibility automatically
4. **System Sets Test**: Verify all plugins register systems in properly configured sets

---

## Conclusion

The `src/systems/` directory has **major** Constitution violations that must be addressed:

- **13+ systems** do not return `Result` (violates Fallible Systems mandate)
- **1 panic risk** in respawn.rs (violates NO Panicking Queries prohibition)
- **2 components** lack required component declarations
- **6 plugins** lack proper system set organization

However, some areas show good practices:
- Asset handles are properly cached
- Change detection is used where appropriate
- Plugin architecture is mostly in place
- Respawn system has exemplary system set organization

**Recommendation**: Proceed with refactoring in the order specified (P1 → P2 → P3) to bring all systems into full Constitution compliance.

---

**Audit Date**: 2025-12-19
**Auditor**: Automated Constitution Compliance Tool
**Next Review**: After refactoring completion
