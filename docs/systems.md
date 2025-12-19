# Game Systems Documentation

**Last Updated**: 2025-12-19 | **Constitution Version**: 1.3.0

## Overview

This document describes the game systems architecture in `src/systems/`, their organization, error handling patterns, and Constitution compliance guidelines.

## System Modules

### Audio System (`audio.rs`)

**Purpose**: Event-driven audio playback for game events (brick destruction, wall bounces, level transitions).

**Plugin**: `AudioPlugin`  
**System Sets**: `AudioSystems::Startup`, `AudioSystems::Update`

**Key Components**:
- `AudioConfig` - User-adjustable volume and mute settings
- `AudioAssets` - Loaded audio handles (cached at startup)
- `ActiveSounds` - Concurrent playback tracking (max 4 per type)

**Events**:
- `BrickDestroyed` - Standard brick destruction
- `BallWallHit` - Ball bounces off wall
- `LevelStarted` / `LevelCompleted` - Level transitions

**Constitution Compliance**:
- ✓ Asset handles cached at startup (no repeated loads)
- ✓ Observer pattern for event handling
- ✓ Fallible systems (return Result)

---

### Cheat Mode (`cheat_mode.rs`)

**Purpose**: Debug mode toggle for testing.

**Plugin**: `CheatModePlugin`  
**System Sets**: `CheatModeSystems::Input`

**Key Components**:
- `CheatModeState` - Active/inactive state tracking
- `CheatModeToggled` - Message emitted on toggle

**Constitution Compliance**:
- ✓ Message-based communication
- ✓ Fallible input system

---

### Grid Debug (`grid_debug.rs`)

**Purpose**: Visual debug grid overlay for alignment and debugging.

**Plugin**: `GridDebugPlugin`  
**System Sets**: `GridDebugSystems::Spawn`, `GridDebugSystems::Update`

**Key Components**:
- `GridOverlay` - Marker for grid entities

**Constitution Compliance**:
- ✓ Required components (`Transform`, `Visibility`)
- ✓ Fallible spawn system

---

### Level Switch (`level_switch.rs`)

**Purpose**: Level transition orchestration.

**Plugin**: `LevelSwitchPlugin`  
**System Sets**: `LevelSwitchSystems::Input`, `LevelSwitchSystems::Logic`

**Key Components**:
- `LevelSwitchState` - Ordered level list and transition state
- `LevelSwitchRequested` - Message requesting level change

**Constitution Compliance**:
- ✓ Message-based communication
- ✓ Fallible state transitions

---

### Multi-Hit Bricks (`multi_hit.rs`)

**Purpose**: Handle bricks requiring multiple ball collisions (indices 10-13).

**Event**: `MultiHitBrickHit`

**Key Features**:
- Brick lifecycle: Index 13 → 12 → 11 → 10 → 20 (stone) → Destroyed
- Change detection for brick type transitions

**Constitution Compliance**:
- ✓ `Changed<BrickTypeId>` filter for reactive updates
- ✓ Event-based communication

---

### Paddle Size (`paddle_size.rs`)

**Purpose**: Temporary paddle size powerups (shrink/enlarge).

**Plugin**: `PaddleSizePlugin`  
**System Sets**: `PaddleSizeSystems::Detection`, `PaddleSizeSystems::Application`, `PaddleSizeSystems::Cleanup`

**Key Components**:
- `PaddleSizeEffect` - Active effect tracking with timer
- `SizeEffectType` - Shrink (70%) or Enlarge (150%)

**Brick Types**:
- Type 30: Shrink paddle for 10 seconds
- Type 32: Enlarge paddle for 10 seconds

**Constitution Compliance**:
- ✓ System sets with logical grouping
- ✓ Fallible effect application
- ✓ Cleanup on life loss or level switch

---

### Respawn System (`respawn.rs`)

**Purpose**: Ball respawn orchestration with visual feedback.

**Plugin**: `RespawnPlugin`  
**System Sets**: `RespawnSystems::LifeLoss`, `RespawnSystems::Spawn`, `RespawnSystems::Animation`, `RespawnSystems::Input`

**Key Components**:
- `LivesState` - Remaining lives tracking
- `RespawnSchedule` - Pending respawn queue with timer
- `RespawnFadeOverlay` - Visual fade effect during respawn

**Constitution Compliance**:
- ✓ **Best practice example** for system set organization
- ✓ Proper set chaining (.chain())
- ✓ Fallible systems
- ✓ Error recovery patterns

**Note**: Use this as the reference implementation for system organization!

---

### Scoring (`scoring.rs`)

**Purpose**: Score tracking and milestone detection.

**Plugin**: `ScoringPlugin`  
**System Sets**: `ScoringSystems::Awards`, `ScoringSystems::Milestones`

**Key Components**:
- `ScoreState` - Current score and milestone tracking
- `BrickDestroyed` - Message for point awards
- `MilestoneReached` - Message for 5000-point thresholds

**Point Values**:
- Multi-hit bricks (10-13): 50 points
- Simple stone (20): 25 points
- Question brick (53): Random 25-300 points
- (See `docs/bricks.md` for complete list)

**Constitution Compliance**:
- ✓ Message-based communication
- ✓ Fallible systems

---

### Textures Subsystem (`textures/`)

**Purpose**: Texture manifest loading and material management.

**Plugin**: `TextureManifestPlugin`  
**System Sets**: `TextureSystems::Startup`, `TextureSystems::Update`

**Submodules**:
- `loader.rs` - Manifest loading and type change watching
- `materials.rs` - Material setup and registration
- `contracts.rs` - Validation and serialization
- `overrides.rs` - Override loading and parsing

**Constitution Compliance**:
- ✓ Asset handles cached at startup
- ✓ `Changed<T>` filters for reactive material swaps
- ✓ Fallible systems

---

## Constitution Compliance Patterns

### Fallible Systems

**Mandate**: All systems MUST return `Result` and use `?` operator for error propagation.

**Pattern**:
```rust
fn my_system(
    query: Query<&Component>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Early return on expected failures
    if query.is_empty() {
        return Ok(());
    }
    
    // Use ? for queries
    let component = query.get_single()?;
    
    // ... system logic
    
    Ok(())
}
```

---

### Query Error Handling

**0 entities (expected)**:
```rust
if query.is_empty() {
    return Ok(());
}
```

**1 entity (required)**:
```rust
let entity = query.get_single()?;
```

**1 entity (optional)**:
```rust
if let Ok(entity) = query.get_single() {
    // ... handle entity
}
```

**1 entity (safe alternative)**:
```rust
let Ok(entity) = query.get_single() else {
    return Ok(());
};
```

**Many entities**:
```rust
for entity in query.iter() {
    // ... process each entity
}
```

---

### Required Components

**Mandate**: Component structs MUST use `#[require()]` for Transform/Visibility.

**Pattern**:
```rust
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct MyMarker;

// Spawning - Transform and Visibility added automatically
commands.spawn(MyMarker);
```

---

### System Organization

**Mandate**: Define system sets with `*Systems` suffix and use `.configure_sets()`.

**Pattern**:
```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MySystems {
    Input,
    Logic,
    Cleanup,
}

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        // Configure set ordering
        app.configure_sets(Update, (
            MySystems::Input,
            MySystems::Logic,
            MySystems::Cleanup,
        ).chain());
        
        // Add systems to sets
        app.add_systems(Update, (
            input_system.in_set(MySystems::Input),
            logic_system.in_set(MySystems::Logic),
            cleanup_system.in_set(MySystems::Cleanup),
        ));
    }
}
```

**Important**: Only chain sets, not individual systems. Systems within a set run in parallel.

---

### Asset Handle Reuse

**Mandate**: Load assets once and store handles in Resources.

**Anti-pattern** (❌):
```rust
fn spawn_system(asset_server: Res<AssetServer>) {
    let texture = asset_server.load("texture.png"); // ❌ Repeated loading
}
```

**Correct pattern** (✓):
```rust
#[derive(Resource)]
struct CachedAssets {
    texture: Handle<Image>,
}

fn startup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(CachedAssets {
        texture: asset_server.load("texture.png"), // ✓ Load once
    });
}

fn spawn_system(assets: Res<CachedAssets>) {
    let texture = assets.texture.clone(); // ✓ Reuse handle
}
```

---

### Change Detection

**Mandate**: Reactive systems MUST use `Changed<T>` filters.

**Anti-pattern** (❌):
```rust
fn update_ui(score: Res<ScoreState>) {
    // ❌ Runs every frame even when score unchanged
}
```

**Correct pattern** (✓):
```rust
fn update_ui(score: Res<ScoreState>) {
    if !score.is_changed() {
        return; // ✓ Early return when no changes
    }
    // ... update UI
}
```

**For queries**:
```rust
fn update_materials(
    query: Query<&BrickTypeId, Changed<BrickTypeId>>  // ✓ Only changed entities
) {
    for brick_type in query.iter() {
        // ... update material
    }
}
```

---

## Testing Systems

### Unit Testing

**Pattern**: Test system logic in isolation using `App::update()`.

```rust
#[test]
fn test_my_system() {
    let mut app = App::new();
    app.add_systems(Update, my_system);
    app.insert_resource(MyResource::default());
    
    // Spawn test entities
    app.world_mut().spawn(MyComponent);
    
    // Run systems
    app.update();
    
    // Assert results
    let resource = app.world().resource::<MyResource>();
    assert_eq!(resource.value, expected_value);
}
```

### Integration Testing

**Pattern**: Test system interactions with full app setup.

```rust
#[test]
fn test_system_interaction() {
    let mut app = App::new();
    app.add_plugins(MyPlugin);
    
    // Trigger event
    app.world_mut().send_event(MyEvent);
    
    // Run systems
    app.update();
    
    // Verify side effects
    // ...
}
```

---

## Common Pitfalls

### ❌ Don't: Use `.unwrap()` on query results

```rust
let entity = query.single(); // ❌ Panics if 0 or >1 entities
let entity = query.get_single().unwrap(); // ❌ Panics on error
```

### ✓ Do: Use `?` operator or safe patterns

```rust
let entity = query.get_single()?; // ✓ Returns error
let Ok(entity) = query.get_single() else { return Ok(()); }; // ✓ Early return
```

---

### ❌ Don't: Chain individual systems

```rust
app.add_systems(Update, (system1, system2, system3).chain()); // ❌
```

### ✓ Do: Chain sets, not systems

```rust
app.configure_sets(Update, (Set1, Set2, Set3).chain()); // ✓
app.add_systems(Update, (
    system1.in_set(Set1),
    system2.in_set(Set2),
    system3.in_set(Set3),
));
```

---

### ❌ Don't: Update every frame without change detection

```rust
fn update_ui(score: Res<ScoreState>) {
    // ❌ Runs every frame
}
```

### ✓ Do: Use change detection

```rust
fn update_ui(score: Res<ScoreState>) {
    if !score.is_changed() { return; } // ✓ Only when changed
}
```

---

## See Also

- [Constitution](.specify/memory/constitution.md) - Architectural principles
- [UI Systems](ui-systems.md) - UI-specific system patterns
- [Developer Guide](developer-guide.md) - General development guidelines
- [Architecture](architecture.md) - High-level architecture overview

---

**Document Version**: 1.0  
**Last Reviewed**: 2025-12-19  
**Next Review**: After major Constitution updates
