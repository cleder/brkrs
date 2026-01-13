# Data Model: Gravity Indicator UI

**Date**: 2026-01-11 | **Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)

## Phase 1: Component & Resource Design

This document defines the ECS components, resources, and data structures for the gravity indicator feature.

## Components

### `GravityIndicator`

**Purpose**: Marker component for the gravity indicator UI entity.

**Type**: Unit struct (no fields)

```rust
#[derive(Component)]
pub struct GravityIndicator;
```

**Lifecycle**:

- Spawned once per game session when GravityConfiguration and textures are available
- Persists across level transitions
- No despawn unless explicitly removed (not tied to game state)

**Usage**:

- Query filter for update system: `Query<&mut ImageNode, With<GravityIndicator>>`
- Existence check for spawn system: `Query<Entity, With<GravityIndicator>>`

## Resources

### `GravityIndicatorTextures`

**Purpose**: Cache of preloaded indicator texture handles.

**Type**: Resource struct

```rust
#[derive(Resource)]
pub struct GravityIndicatorTextures {
    pub question: Handle<Image>,
    pub weight0: Handle<Image>,
    pub weight2: Handle<Image>,
    pub weight10: Handle<Image>,
    pub weight20: Handle<Image>,
}
```

**Lifecycle**:

- Initialized in `setup_ui_assets` system (Startup schedule)
- Persists for entire game session
- Read-only after initialization

**Usage**:

- Accessed via `Option<Res<GravityIndicatorTextures>>` in spawn/update systems
- Handles cloned when setting `ImageNode`

### `GravityConfiguration` (existing)

**Purpose**: Tracks current and default gravity for gameplay.

**Type**: Resource struct (defined in `src/lib.rs`)

```rust
#[derive(Resource, Clone, Copy, Debug)]
pub struct GravityConfiguration {
    pub current: Vec3,
    pub level_default: Vec3,
    pub last_level_number: Option<u32>,
}
```

**Relevant Fields for Indicator**:

- `current: Vec3` - Current gravity vector (read by indicator for mapping)

**Change Detection**:

- Update system uses `gravity_cfg.is_changed()` to detect modifications
- Changes triggered by gravity brick destruction, level load, or life loss

## Enums

### `GravityLevel`

**Purpose**: Discrete gravity levels for mapping and testing.

**Type**: Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GravityLevel {
    L0,
    L2,
    L10,
    L20,
    Unknown,
}
```

**Mapping Logic**:

- `L0`: Magnitude 0 detected on X or Z axis (within ±0.5 tolerance)
- `L2`: Magnitude 2 detected (within ±0.5 tolerance)
- `L10`: Magnitude 10 detected (within ±0.5 tolerance)
- `L20`: Magnitude 20 detected (within ±0.5 tolerance)
- `Unknown`: No recognized magnitude within tolerance

**Selection Priority**: Highest recognized magnitude among X/Z axes wins.

## UI Entity Structure

**Indicator Entity**:

```rust
commands.spawn((
    ImageNode::new(handle),          // Current gravity icon
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(12.0),         // Lower-left corner
        bottom: Val::Px(12.0),
        ..Default::default()
    },
    GravityIndicator,                // Marker component
));
```

**Component Breakdown**:

- `ImageNode`: Bevy UI component for displaying an image (replaces deprecated `UiImage`)
- `Node`: Layout properties (absolute positioning, bottom-left anchor)
- `GravityIndicator`: Query filter marker

**Update Pattern**:

```rust
*image_node = ImageNode::new(new_handle.clone());
```

## Helper Functions

### `map_gravity_to_level(g: Vec3) -> GravityLevel`

**Purpose**: Convert gravity vector to discrete level enum.

**Logic**:

1. Extract X and Z components (Y ignored)
2. Round each to nearest integer
3. Check if rounded value is within ±0.5 of original
4. Keep highest absolute value that passes tolerance check
5. Map to enum: 20 → L20, 10 → L10, 2 → L2, 0 → L0, else → Unknown

**Example**:

- `Vec3::new(10.3, 0.0, 2.1)` → rounds to 10 and 2 → highest is 10 → `L10`
- `Vec3::new(9.4, 0.0, 0.0)` → 9.4 rounds to 9, but 0.6 away → `Unknown`

### `select_texture(level: GravityLevel, textures: &GravityIndicatorTextures) -> &Handle<Image>`

**Purpose**: Map level enum to corresponding texture handle.

**Logic**: Simple match expression:

```rust
match level {
    L20 => &textures.weight20,
    L10 => &textures.weight10,
    L2 => &textures.weight2,
    L0 => &textures.weight0,
    Unknown => &textures.question,
}
```

## System Data Flow

```text
[Startup: setup_ui_assets]
    ↓
Loads textures into GravityIndicatorTextures resource
    ↓
[Update: spawn_gravity_indicator] (runs once)
    ↓
Checks: !existing.is_empty() && GravityConfiguration exists && GravityIndicatorTextures exists
    ↓
Maps GravityConfiguration.current → GravityLevel → Handle<Image>
    ↓
Spawns entity with ImageNode + Node + GravityIndicator
    ↓
[Update: update_gravity_indicator] (runs on Changed<GravityConfiguration>)
    ↓
Maps new GravityConfiguration.current → GravityLevel → Handle<Image>
    ↓
Updates entity's ImageNode to new texture
```

## Validation Rules

1. **Tolerance**: ±0.5 on rounded integer values
2. **Axis Priority**: X/Z only; Y always ignored
3. **Magnitude Selection**: Highest recognized magnitude wins
4. **Spawn Guard**: Only spawn if indicator doesn't exist
5. **Update Guard**: Only update if `gravity_cfg.is_changed()`

## Testing Strategy

**Unit Tests** (in `src/ui/gravity_indicator.rs`):

- `map_gravity_to_level` with exact values (0.0, 2.0, 10.0, 20.0)
- `map_gravity_to_level` with tolerance edges (1.5, 2.49, 9.51, 10.4)
- `map_gravity_to_level` with mixed axes (X=2, Z=10 → L10)
- `map_gravity_to_level` with unknown values (9.4, 2.6)

**Integration Tests** (in `tests/gravity_indicator_ui.rs`):

- Spawn timing: indicator appears when both GravityConfiguration and textures ready
- Idempotence: spawn system doesn't create duplicates
- Update correctness: indicator changes when gravity changes
- Multi-frame persistence: indicator remains correct for 10+ frames
- Level transition: indicator updates to new level default

## Summary

Data model is minimal and focused:

- 1 marker component (`GravityIndicator`)
- 1 resource (`GravityIndicatorTextures`) + 1 existing (`GravityConfiguration`)
- 1 enum for mapping logic (`GravityLevel`)
- 2 helper functions (mapping + texture selection)
- UI entity with standard Bevy components (`ImageNode`, `Node`)

No complex state management.
No hierarchies.
No custom events.
Leverages existing ECS change detection.
