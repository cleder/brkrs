# Data Model: Gravity Switching Bricks

**Feature**: 020-gravity-bricks | **Date**: 2026-01-10 | **Phase**: 1 Design

## Overview

The gravity switching bricks feature introduces:

- A new `GravityChanged` message for communicating gravity updates
- A `GravityConfiguration` resource tracking current and level default gravity
- A `GravityBrick` component marker for identifying gravity brick entities
- Updates to `LevelDefinition` to include optional default gravity configuration

## Core Entities & Components

### 1. GravityBrick Component

**Purpose**: Mark a brick entity as a gravity brick and store its gravity output value.

```rust
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct GravityBrick {
    /// Brick index (21-25 per specification)
    pub index: u32,
    /// Gravity vector applied when this brick is destroyed
    /// Y-axis (vertical), with optional X/Z for directional gravity
    pub gravity: Vec3,
}
```

**Fields**:

- `index`: Unique identifier (21 = Zero G, 22 = 2G, 23 = 10G, 24 = 20G, 25 = Queer Gravity)
- `gravity`: Output gravity vector in Bevy coordinates (Y = vertical, XZ = horizontal)

**Relationships**:

- Attached to brick entities in level maps
- Read by brick destruction system to determine gravity output
- Used by score system to award correct points (75-250 points)

**Validation**:

- `index` must be in range [21, 25]
- `gravity` vector components must be within [-20.0, +20.0] for X, [-0.1, +30.0] for Y, [-10.0, +10.0] for Z
- For Queer Gravity (index 25), `gravity` is set at runtime (RNG in destruction handler)

**Creation**: Populated by level metadata loader when parsing brick map from RON file

---

### 2. GravityConfiguration Resource

**Purpose**: Track the current gravity applied to the world and the level's default gravity for reset.

```rust
#[derive(Resource, Clone, Copy, Debug)]
pub struct GravityConfiguration {
    /// Currently applied gravity vector
    current: Vec3,
    /// Level's starting/default gravity (loaded from level metadata)
    level_default: Vec3,
}
```

**Fields**:

- `current`: The gravity currently affecting the ball's physics
  - Initialized to `level_default` at level start
  - Updated by `gravity_application_system` when `GravityChanged` message received
- `level_default`: The gravity defined in the level RON metadata
  - Loaded by `gravity_configuration_loader_system` at level start
  - Defaults to `Vec3::ZERO` if level metadata doesn't specify gravity
  - Used to reset gravity when ball is lost

**Relationships**:

- Singleton resource (one per game world)
- Read by physics gravity application system
- Updated by gravity update system
- Reset by ball loss system

**Access Pattern**:

- Gravity application system reads `current` value
- Gravity reset system writes `level_default` back to `current`
- Ball physics system may query `current` to apply forces (indirect: gravity affects Rapier physics)

---

### 3. GravityChanged Message

**Purpose**: Communicate gravity changes from brick destruction to physics system.

```rust
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct GravityChanged {
    /// New gravity vector to apply
    pub gravity: Vec3,
}
```

**Fields**:

- `gravity`: The output gravity vector from the destroyed brick
  - For static bricks (21-24): constant value defined in `GravityBrick` component
  - For Queer Gravity (25): randomly generated at destruction time

**Semantics**:

- Sent by: `brick_destruction_system` when gravity brick is destroyed
- Received by: `gravity_application_system` via `MessageReader<GravityChanged>`
- Lifecycle: Buffered message queue, read in next/same schedule step
- Frequency: One message per gravity brick destruction (multiple bricks → multiple messages)
- No ordering guarantee between sequential gravity brick destructions (each message is independent)

**Message Queue Behavior**:

- Messages are buffered and processed in order of destruction
- If two gravity bricks destroyed in same frame, both messages queue
- Each message independently updates `GravityConfiguration::current`
- Last message "wins" if multiple queued (deterministic application order)

---

### 4. GravityChanged RNG Implementation Strategy (Queer Gravity)

**Purpose**: Document RNG approach for Queer Gravity brick 25 randomization.

**RNG Choice**: Use `rand::thread_rng()` for non-deterministic randomization.

**Seeding Strategy**:

- Each gravity brick 25 destruction calls `thread_rng().gen_range()` independently
- No explicit seeding; uses OS entropy pool (non-deterministic across runs)
- Each call to `gen_range()` pulls from the thread-local RNG state
- Satisfies "independent seeded RNG" requirement from FR-005

**Platform-Specific Considerations**:

- **Native (Linux/Windows/macOS)**: `rand` + `getrandom` = standard OS entropy (works out of box)
- **WASM**: Requires explicit `getrandom` backend configuration:

  ```bash
  RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --target wasm32-unknown-unknown
  ```

  This enables JavaScript `crypto.getRandomValues()` as entropy source.

- **Testing**: Use `#[cfg(test)]` mocks if deterministic test gravity is needed (seeded Rng for reproducibility)

**Implementation Location**: `brick_destruction_gravity_handler` system in `src/systems/gravity/mod.rs` detects brick index 25, calls `rand::thread_rng().gen_range(-2.0..=15.0)` for X, always Y=0.0, and `gen_range(-5.0..=5.0)` for Z.

---

### 5. LevelDefinition (Modification)

**Purpose**: Extend existing level metadata to include optional default gravity configuration.

**New Field**:

```rust
pub struct LevelDefinition {
    // ... existing fields ...

    /// Optional default gravity for this level
    /// If None or missing from RON, defaults to Vec3::ZERO (zero gravity)
    /// Format: [x, y, z] in Bevy coordinates
    /// BACKWARD COMPATIBLE: Existing level files without this field will automatically
    /// deserialize with None (fallback to zero gravity) due to #[serde(default)] attribute
    #[serde(default)]
    pub default_gravity: Option<Vec3>,
}
```

**Deserialization Behavior**:

- Old RON files (no `default_gravity` field): serde assigns `None` → loads zero gravity fallback
- New RON files (with `default_gravity` field): serde parses value → loads specified gravity
- **No migration needed**: Existing levels automatically work with zero gravity fallback

**RON Format**:

```ron
(
    name: "Level 1: Easy",
    bricks: [/* ... */],
    default_gravity: Some((0.0, 10.0, 0.0)),  // Earth gravity
    // ... other fields ...
)
```

Or with fallback:

```ron
(
    name: "Level 0: Classic",
    bricks: [/* ... */],
    // no default_gravity field → defaults to zero gravity
    // ... other fields ...
)
```

**Loading Behavior**:

- Loaded by `gravity_configuration_loader_system` at level start
- If field missing or `None`: defaults to `Vec3::ZERO`
- Stored in `GravityConfiguration::level_default`

---

## Brick Destruction Detection Mechanism

**Pattern**: Query for `RemovedComponents<GravityBrick>` to detect destroyed gravity bricks.

**Implementation Strategy**:

```rust
fn brick_destruction_gravity_handler(
    mut removed: RemovedComponents<GravityBrick>,
    gravity_bricks: Query<&GravityBrick>,
    mut gravity_writer: MessageWriter<GravityChanged>,
) {
    for entity in removed.read() {
        // Entity is being removed; if it had GravityBrick component, it was a gravity brick
        // Read GravityBrick data before despawn OR store it in despawn event payload
        // Write GravityChanged message with the brick's output gravity
    }
}
```

**Timing**: `RemovedComponents<T>` detects component removal in the same frame, enabling immediate gravity message on destruction.

**Alternative**: If brick destruction is signaled via event (e.g., `BrickDestroyedEvent`), extend that event to include `GravityBrick` data and handle in the event reader.

---

## Data Relationships Diagram

```text
Level Start
    ↓
[LevelDefinition loaded from RON]
    ↓
[default_gravity field parsed with #[serde(default)]]
    ↓
[GravityConfiguration resource created]
├─ current: Level's default_gravity (or zero gravity fallback)
└─ level_default: Level's default_gravity (or zero gravity fallback)
    ↓
[Brick entities spawned with GravityBrick component if index 21-25]
    ↓
[Ball physics updates using GravityConfiguration::current]
    ↓
Ball destroys gravity brick (21-25)
    ↓
[brick_destruction_system detects via RemovedComponents<GravityBrick>]
    ↓
[Reads GravityBrick data (or retrieves from despawn event)]
    ↓
[Computes gravity output (for brick 25, generates random X/Z with Y=0.0)]
    ↓
[Writes GravityChanged message]
    ↓
[gravity_application_system reads message via MessageReader<GravityChanged>]
    ↓
[Updates GravityConfiguration::current]
    ↓
[Next physics frame applies new gravity to ball]
    ↓
Ball is lost
    ↓
[gravity_reset_on_life_loss_system detects]
    ↓
[Resets GravityConfiguration::current to level_default]
    ↓
[Next ball spawn physics uses default gravity]
```

---

## Type Definitions Summary

| Type | Category | Purpose | Scope |
|------|----------|---------|-------|
| `GravityBrick` | Component | Mark and store gravity output for brick entities | Per-entity |
| `GravityConfiguration` | Resource | Track current and default gravity | Global (singleton) |
| `GravityChanged` | Message | Communicate gravity updates | Buffered event stream |
| `LevelDefinition` | Configuration | Store level metadata including gravity | Per-level (static) |

---

## State Transitions

### Gravity State Machine

```text
State: [Default Gravity from Level Metadata]
  ↓
Event: Gravity brick destroyed
  → GravityChanged message sent
  → GravityConfiguration::current updated
  → New state: [New gravity from brick]
  ↓
Event: Another gravity brick destroyed
  → GravityChanged message sent
  → GravityConfiguration::current updated (overrides previous)
  → New state: [Different gravity]
  ↓
Event: Ball is lost
  → Gravity reset message/trigger sent
  → GravityConfiguration::current = GravityConfiguration::level_default
  → New state: [Default gravity from level]
  ↓
Event: Next ball spawned
  → Physics system uses current gravity
  → Ball falls/floats with default gravity
```

---

## Validation Rules

### GravityBrick Component

- `index` must be exactly one of: 21, 22, 23, 24, 25
- `gravity` vector components must be finite (not NaN or Inf)
- For indices 21-24: gravity is constant and pre-defined
- For index 25 (Queer Gravity): gravity is generated at destruction time with RNG

### GravityConfiguration Resource

- `current` must always be finite
- `level_default` must always be finite
- Both should be within reasonable physics range (typically [-30, +30] per axis)

### LevelDefinition::default_gravity

- If `Some(Vec3)`: must have finite components
- If `None`: treated as zero gravity `Vec3::ZERO`

---

## Performance Considerations

- **GravityConfiguration**: Single resource, `Copy` type, O(1) reads/writes
- **GravityChanged messages**: Buffered, read once per frame; O(n) where n = gravity bricks destroyed per frame (typically 0-2)
- **GravityBrick component**: Per-brick storage, included only for bricks 21-25 (typically 0-20 per level)
- **Physics application**: No additional per-frame cost; gravity value already read by physics system (just updating the magnitude)

---

## Testing Concerns

### Component Tests

- ✅ `GravityBrick` creation with valid/invalid indices
- ✅ `GravityConfiguration` initialization and updates
- ✅ Message serialization/deserialization (if applicable)

### Integration Tests

- ✅ Message flow: destruction → message write → gravity update
- ✅ Gravity reset on ball loss
- ✅ Sequential gravity changes
- ✅ Zero gravity fallback for undefined levels
- ✅ Queer Gravity RNG within specified ranges

### State Transition Tests

- ✅ Default gravity applied at level start
- ✅ Gravity changes immediately on brick destruction
- ✅ Gravity resets before next ball spawn
- ✅ No gravity state bleeding between levels

---

## Migration & Backwards Compatibility

**Existing Levels**:

- All existing level RON files are backwards compatible
- Levels without `default_gravity` field automatically use zero gravity fallback
- No migration script required
- Optional: developers can add `default_gravity` field to levels to set custom defaults

**Existing Components**:

- `LevelDefinition` struct extended with optional `default_gravity` field
- Deserialization handles missing field gracefully (defaults to `None`)
- No breaking changes to existing level format

---

## Future Extensions

- **Per-entity gravity zones**: Extend to apply gravity to specific game areas
- **Gravity curve/easing**: Smooth transitions between gravity values (currently instant)
- **Gravity multipliers**: Stack gravity effects from multiple sources
- **Gravity momentum**: Preserve ball velocity direction when gravity changes
