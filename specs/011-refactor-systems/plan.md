# Implementation Plan: Systems Constitution Refactor

**Branch**: `copilot/refactor-legacy-code-systems` | **Date**: 2025-12-19 | **Spec**: [specs/011-refactor-systems/spec.md](spec.md)
**Input**: Feature specification from `/specs/011-refactor-systems/spec.md`

## Summary

Bring `src/systems/` into compliance with the Brkrs Constitution by:

- Producing a complete compliance audit artifact for `src/systems`
- Refactoring systems code to satisfy Bevy 0.17 mandates/prohibitions (fallible systems, change detection, asset handle reuse, required components)
- Preserving all player-facing system behavior (audio, scoring, paddle effects, respawn, level switching)

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021)
**Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1
**Storage**: N/A (in-memory ECS state only)
**Testing**: `cargo test` (integration tests in `tests/`)
**Target Platform**: Native + WASM
**Project Type**: Single project (`src/`, `tests/`)
**Performance Goals**: 60 FPS (avoid per-frame updates where data unchanged)
**Constraints**: Minimal scope outside `src/systems` (supporting edits only when required)
**Scale/Scope**: One module subtree (`src/systems`) + minimal supporting changes

## Constitution Check

*GATE: Must pass before implementation tasks begin. Re-check after each story.*

**TDD gates (Constitution VII)**

- Tests are authored and committed before any implementation changes for each user story.
- A proof-of-failure commit (tests FAIL) exists in branch history before implementation commits.
- Tests are reviewed/approved by the requestor before implementation proceeds.

**Bevy 0.17 gates (Constitution VIII)**

- Systems are fallible (`Result`), do not panic on query outcomes (`?`, no `.unwrap()` on queries).
- Reactive systems use `Changed<T>` and are not executed every frame without data changes.
- Message vs Event usage is correct (`MessageReader/Writer` vs observers).
- Asset handles are loaded once and cached in Resources (no repeated `asset_server.load()`).
- Marker components use `#[require(Transform, Visibility)]` where appropriate.
- Systems organized into system sets with `*Systems` suffix.
- Plugin-based architecture for each subsystem.

## Project Structure

### Documentation (this feature)

```text
specs/011-refactor-systems/
├── plan.md
├── spec.md
├── compliance-audit.md
├── refactoring-plan.md
├── checklists/
│   ├── requirements.md
│   ├── compliance.md
│   └── compliance-lightweight.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── systems/
│   ├── audio.rs
│   ├── cheat_mode.rs
│   ├── grid_debug.rs
│   ├── level_switch.rs
│   ├── mod.rs
│   ├── multi_hit.rs
│   ├── paddle_size.rs
│   ├── respawn.rs
│   ├── scoring.rs
│   └── textures/
└── ...

tests/
├── ...
└── (new compliance tests)
```

**Structure Decision**: Single Rust crate with Bevy ECS; changes are primarily within `src/systems`, with minimal supporting edits allowed outside when required.

## Systems Inventory

### Audio System (`audio.rs`)

- **Purpose**: Event-driven audio playback for game events
- **Key Components**: `AudioPlugin`, `AudioConfig`, `AudioAssets`, `ActiveSounds`
- **Compliance Concerns**: Asset handle caching, fallible observers

### Cheat Mode (`cheat_mode.rs`)

- **Purpose**: Debug mode toggle for testing
- **Key Components**: `CheatModePlugin`, `CheatModeState`, `CheatModeToggled`
- **Compliance Concerns**: Fallible input system, message usage

### Grid Debug (`grid_debug.rs`)

- **Purpose**: Visual debug grid overlay
- **Key Components**: TBD (needs inspection)
- **Compliance Concerns**: Required components, fallible spawning

### Level Switch (`level_switch.rs`)

- **Purpose**: Level transition orchestration
- **Key Components**: `LevelSwitchPlugin`, `LevelSwitchRequested`, `LevelSwitchState`
- **Compliance Concerns**: Fallible state transitions, error recovery

### Multi-Hit Bricks (`multi_hit.rs`)

- **Purpose**: Multi-hit brick collision handling
- **Key Components**: `MultiHitBrickHit` message
- **Compliance Concerns**: Fallible collision observers

### Paddle Size (`paddle_size.rs`)

- **Purpose**: Temporary paddle size powerups
- **Key Components**: `PaddleSizePlugin`, `PaddleSizeEffect`, `PaddleSizeEffectApplied`
- **Compliance Concerns**: Fallible effect application, timer updates

### Respawn (`respawn.rs`)

- **Purpose**: Ball respawn orchestration
- **Key Components**: `RespawnPlugin`, `RespawnSystems`, `LivesState`, `RespawnSchedule`
- **Compliance Concerns**: Fallible spawning, complex state machine

### Scoring (`scoring.rs`)

- **Purpose**: Score tracking and milestone detection
- **Key Components**: `ScoreState`, `BrickDestroyed`, `MilestoneReached`
- **Compliance Concerns**: Fallible score updates, message handling

### Textures (`textures/`)

- **Purpose**: Texture manifest loading
- **Key Components**: `TextureManifestPlugin`
- **Compliance Concerns**: Asset handle caching, fallible loading

## Refactoring Strategy

### Pattern 1: Fallible Systems

**Before**:
```rust
fn system(query: Query<&Component>) {
    let component = query.single();
    // ...
}
```

**After**:
```rust
fn system(query: Query<&Component>) -> Result<(), Box<dyn Error>> {
    let component = query.get_single()?;
    // ...
    Ok(())
}
```

### Pattern 2: Change Detection

**Before**:
```rust
fn update_system(query: Query<&Component>) {
    // Runs every frame
}
```

**After**:
```rust
fn update_system(query: Query<&Component, Changed<Component>>) -> Result<(), Box<dyn Error>> {
    // Only runs when Component changes
    if query.is_empty() {
        return Ok(());
    }
    // ...
    Ok(())
}
```

### Pattern 3: Asset Handle Caching

**Before**:
```rust
fn spawn_system(asset_server: Res<AssetServer>, mut commands: Commands) {
    let texture = asset_server.load("texture.png"); // Repeated loading
    commands.spawn(Sprite::from_image(texture));
}
```

**After**:
```rust
#[derive(Resource)]
struct CachedTextures {
    texture: Handle<Image>,
}

fn startup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(CachedTextures {
        texture: asset_server.load("texture.png"), // Load once
    });
}

fn spawn_system(textures: Res<CachedTextures>, mut commands: Commands) -> Result<(), Box<dyn Error>> {
    commands.spawn(Sprite::from_image(textures.texture.clone())); // Reuse handle
    Ok(())
}
```

### Pattern 4: System Sets

**Before**:
```rust
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (system1, system2, system3));
    }
}
```

**After**:
```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MySystems {
    Input,
    Logic,
    Cleanup,
}

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (
            MySystems::Input,
            MySystems::Logic,
            MySystems::Cleanup,
        ).chain());
        
        app.add_systems(Update, (
            system1.in_set(MySystems::Input),
            system2.in_set(MySystems::Logic),
            system3.in_set(MySystems::Cleanup),
        ));
    }
}
```

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |
