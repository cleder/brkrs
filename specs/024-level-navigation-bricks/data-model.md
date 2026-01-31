# Data Model: Level Navigation Bricks (Bricks 50 & 54)

**Date**: 2026-01-31 **Feature**: Level Navigation Bricks **Phase**: Phase 1 - Design & Contracts

## Overview

This document defines the data structures, components, resources, and messages for level navigation bricks (50 and 54).
The design follows established patterns from existing special bricks (41, 42, 57, 91) and integrates with the existing level transition system.

---

## Entities

### Brick 50 (Level Up)

**Purpose**: Destructible brick that advances to the next level when destroyed by ball collision.

**Entity Marker Component**:

- `BrickTypeId(50)` - Standard brick type identification
- No special marker component needed

**Required Sibling Components**:

- `Transform` - Spatial position
- `Collider` (Rapier3D) - Physics collision detection
- `MeshMaterial3d<StandardMaterial>` - Visual rendering
- `Mesh3d` - Geometry
- `Brick` - Marker component for all bricks
- `CountsTowardsCompletion` - Destructible brick marker

**Behavior**:

- Ball collision → mark for despawn → emit `BrickDestroyed(brick_type: 50)` → emit `LevelSwitchRequested { direction: Next }`
- If on final level → show victory screen instead of transition
- Awards 0 points (utility brick)
- Plays unique "level up" sound on destruction

**Relationships**:

- One-to-one with brick entity
- Emits messages consumed by level switch system, audio system, scoring system

---

### Brick 54 (Level Down)

**Purpose**: Destructible brick that returns to the previous level when destroyed by ball collision.

**Entity Marker Component**:

- `BrickTypeId(54)` - Standard brick type identification
- No special marker component needed

**Required Sibling Components**:

- `Transform` - Spatial position
- `Collider` (Rapier3D) - Physics collision detection
- `MeshMaterial3d<StandardMaterial>` - Visual rendering
- `Mesh3d` - Geometry
- `Brick` - Marker component for all bricks
- `CountsTowardsCompletion` - Destructible brick marker

**Behavior**:

- Ball collision → mark for despawn → emit `BrickDestroyed(brick_type: 54)` → emit `LevelSwitchRequested { direction: Previous }`
- If on level 1 → no transition (brick destroyed normally, no level change)
- Awards 0 points (utility brick)
- Plays unique "level down" sound on destruction

**Relationships**:

- One-to-one with brick entity
- Emits messages consumed by level switch system, audio system, scoring system

---

## Components

### Existing Components (No Changes Required)

#### `BrickTypeId`

**Location**: `src/lib.rs` (existing)

**Purpose**: Identifies brick type via numeric ID (u8)

**Structure**:

```rust
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrickTypeId(pub u8);
```

**Usage**: Bricks 50 and 54 use `BrickTypeId(50)` and `BrickTypeId(54)` respectively

---

#### `CountsTowardsCompletion`

**Location**: `src/lib.rs` (existing)

**Purpose**: Marker component indicating brick contributes to level completion count

**Rule**: Both bricks 50 and 54 MUST have this component (they are destructible)

---

#### `MarkedForDespawn`

**Location**: `src/lib.rs` (existing)

**Purpose**: Marker component indicating entity should be despawned

**Usage**: Inserted by collision handler when brick 50/54 is hit

---

## Resources

### Existing Resources (Extended)

#### `LevelSwitchState`

**Location**: `src/systems/level_switch.rs` (existing)

**Purpose**: Manages level sequence and transition state

**Structure**:

```rust
#[derive(Resource, Debug)]
pub struct LevelSwitchState {
    ordered_levels: Vec<LevelSlot>,
    trigger_file: PathBuf,
    pending_transition: bool,
}
```

**Methods** (existing):

- `next_level_after(current: u32) -> Option<&LevelSlot>` - Returns next level or `None` if at end
- `previous_level_before(current: u32) -> Option<&LevelSlot>` - Returns previous level or `None` if at start

**Extension Required**:

- No structural changes needed
- Logic extension in `process_level_switch_requests` to handle boundary conditions

---

#### `CurrentLevel`

**Location**: `src/level_loader.rs` (existing)

**Purpose**: Tracks the currently active level

**Structure**:

```rust
#[derive(Resource, Clone)]
pub struct CurrentLevel(pub LevelDefinition);
```

**Fields**:

- `LevelDefinition.number: u32` - Current level number
- `LevelDefinition.matrix: Vec<Vec<u8>>` - Brick layout
- `LevelDefinition.gravity: Option<(f32, f32, f32)>` - Level-specific gravity

**Usage**: Updated during level transitions triggered by bricks 50/54

---

#### `GameProgress`

**Location**: `src/lib.rs` (existing)

**Purpose**: Tracks overall game completion state

**Structure**:

```rust
#[derive(Resource, Default)]
pub struct GameProgress {
    pub finished: bool,
}
```

**Extension Required**:

- Set `finished = true` when brick 50 hit on final level (victory condition)

---

## Messages

### Existing Messages (Extended)

#### `LevelSwitchRequested`

**Location**: `src/systems/level_switch.rs` (existing)

**Purpose**: Requests a level transition

**Structure**:

```rust
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelSwitchRequested {
    pub source: LevelSwitchSource,
    pub direction: LevelSwitchDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelSwitchSource {
    Keyboard,
    Brick,  // NEW: Add this variant for bricks 50/54
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelSwitchDirection {
    Next,
    Previous,
}
```

**Extension Required**:

- Add `Brick` variant to `LevelSwitchSource` enum (if not already present)
- Emitted by collision handler when brick 50 (`direction: Next`) or brick 54 (`direction: Previous`) is hit

**Emitter**: `mark_brick_on_ball_collision` system (extended)

**Consumer**: `process_level_switch_requests` system (extended for boundary conditions)

---

#### `BrickDestroyed`

**Location**: `src/signals.rs` (existing)

**Purpose**: Signals that a brick was destroyed for audio/scoring systems

**Structure**:

```rust
#[derive(Message, Debug, Clone, Copy)]
pub struct BrickDestroyed {
    pub brick_entity: Entity,
    pub brick_type: u8,
    pub destroyed_by: Option<Entity>,
}
```

**Usage**: Emitted for bricks 50 and 54 with `brick_type: 50` or `brick_type: 54`

**Emitter**: `despawn_marked_entities` system (existing)

**Consumers**:

- `award_points_system` - Awards 0 points for types 50 and 54
- Audio system - Maps to unique sounds `Brick50LevelUp` or `Brick54LevelDown`

---

## Constants

### New Constants (to be added)

**Location**: `src/level_format/mod.rs`

```rust
/// Brick type ID for Level Up brick (advances to next level)
pub const BRICK_50: u8 = 50;

/// Brick type ID for Level Down brick (returns to previous level)
pub const BRICK_54: u8 = 54;
```

**Usage**: Checked in collision handler to identify navigation bricks

---

## Data Transformations & Calculations

### Brick Collision Detection Flow

```text
Ball collision with brick
    ↓
mark_brick_on_ball_collision() system
    ↓
Query brick_type via BrickTypeId component
    ↓
Is brick_type == 50 or 54?
    ├─ YES → Emit LevelSwitchRequested message
    │        - brick_type 50 → direction: Next
    │        - brick_type 54 → direction: Previous
    │        └─ Insert MarkedForDespawn component
    └─ NO → Standard brick handling (multi-hit, hazard, etc.)
    ↓
despawn_marked_entities() system (next frame)
    ↓
Emit BrickDestroyed message before despawning
    ↓
Despawn brick entity
```

### Level Transition Flow

```text
LevelSwitchRequested message emitted
    ↓
process_level_switch_requests() system reads message
    ↓
Check current level number from CurrentLevel resource
    ↓
Query LevelSwitchState for target level
    ├─ direction: Next → next_level_after(current)
    │   ├─ Returns Some(level) → Proceed with transition
    │   └─ Returns None → Brick 50 on final level
    │       └─ Spawn victory screen, set GameProgress.finished = true
    └─ direction: Previous → previous_level_before(current)
        ├─ Returns Some(level) → Proceed with transition
        └─ Returns None → Brick 54 on level 1 (no-op, already handled)
    ↓
force_load_level_from_path() (if transition proceeds)
    ├─ Despawn all bricks, paddle, ball, merkaba
    ├─ Load new level definition from RON file
    ├─ Update CurrentLevel resource
    ├─ Spawn new entities from level definition
    └─ Preserve LivesState and ScoreState resources
```

### Scoring Calculation

**Function**: `brick_points(brick_type: u8, rng: &mut impl Rng) -> u32`

**Location**: `src/systems/scoring.rs`

**Extension Required**:

```rust
fn brick_points(brick_type: u8, rng: &mut impl Rng) -> u32 {
    match brick_type {
        // ... existing match arms ...
        41 => 0,  // Extra Ball (existing)
        50 => 0,  // Level Up (NEW)
        54 => 0,  // Level Down (NEW)
        // ... remaining arms ...
    }
}
```

---

## Validation Rules

### Brick Type Constraints

- Brick type IDs 50 and 54 MUST NOT conflict with existing types
- Current special bricks: 10-13 (multi-hit), 20 (simple), 21-25 (gravity), 30/32 (paddle size), 41 (extra life), 42/91 (hazard), 57 (paddle-destroyable), 90 (indestructible)
- **Validated**: Types 50 and 54 are unused ✅

### Component Requirements

- Bricks 50 and 54 MUST have `BrickTypeId` component with value 50 or 54
- Bricks 50 and 54 MUST have `CountsTowardsCompletion` component (destructible)
- Bricks 50 and 54 MUST NOT have `Indestructible` marker (they are destructible)

### Level Transition Constraints

- Level transitions MUST clear all active balls, powerups, and temporary effects (per FR-010)
- Level transitions MUST preserve `LivesState` and `ScoreState` resources
- Level number MUST persist across minimum 10 frames after transition (multi-frame persistence requirement)

---

## State Transitions

### Brick 50 State Machine

```text
State: Spawned (BrickTypeId(50), CountsTowardsCompletion)
    ↓ (ball collision event)
State: Marked (MarkedForDespawn inserted)
    ↓ (emit LevelSwitchRequested { direction: Next })
    ↓ (emit BrickDestroyed { brick_type: 50 })
State: Despawned
    ↓
Level Transition Flow:
    ├─ If next level exists → Load next level
    └─ If on final level → Show victory screen
```

### Brick 54 State Machine

```text
State: Spawned (BrickTypeId(54), CountsTowardsCompletion)
    ↓ (ball collision event)
State: Marked (MarkedForDespawn inserted)
    ↓ (emit LevelSwitchRequested { direction: Previous })
    ↓ (emit BrickDestroyed { brick_type: 54 })
State: Despawned
    ↓
Level Transition Flow:
    ├─ If previous level exists → Load previous level
    └─ If on level 1 → No-op (transition skipped)
```

---

## Audio Integration (Extended)

### Sound Type Enumeration (Extended)

**Location**: `src/systems/audio.rs` (or equivalent)

**Extension Required**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundType {
    // ... existing variants ...
    BrickDestroy,
    Brick41ExtraLife,
    Brick50LevelUp,      // NEW
    Brick54LevelDown,    // NEW
    // ... remaining variants ...
}
```

### Sound Mapping Logic (Extended)

**Function**: Map `BrickDestroyed.brick_type` to `SoundType`

**Extension Required**:

```rust
fn brick_sound(brick_type: u8) -> SoundType {
    match brick_type {
        41 => SoundType::Brick41ExtraLife,
        50 => SoundType::Brick50LevelUp,   // NEW
        54 => SoundType::Brick54LevelDown, // NEW
        _ => SoundType::BrickDestroy,      // Fallback
    }
}
```

**Fallback Behavior** (per FR-006):

- If dedicated sound asset missing → use `SoundType::BrickDestroy`
- Gameplay proceeds normally (no blocking)

---

## Relationships

```text
Brick 50/54 Entity
    ├─ HAS component: BrickTypeId(50 or 54)
    ├─ HAS component: CountsTowardsCompletion
    ├─ HAS component: Transform, Collider, Mesh3d, etc.
    └─ EMITS (when destroyed):
        ├─ LevelSwitchRequested message → LevelSwitchState resource
        └─ BrickDestroyed message → Audio system, Scoring system

Level File (RON)
    └─ DEFINES brick layout with type IDs 50 and 54

LevelSwitchState resource
    ├─ PROVIDES next/previous level queries
    └─ CONSUMED BY process_level_switch_requests system

CurrentLevel resource
    ├─ UPDATED BY level transition
    └─ PERSISTS across multiple frames (multi-frame requirement)

GameProgress resource
    └─ SET finished=true when brick 50 hit on final level
```

---

## Assumptions

- Level files are stored in `assets/levels/level_XXX.ron` with sequential numbering
- `LevelSwitchState` is initialized during startup with all available level files
- Audio system is configured to play sounds based on `SoundType` mapping
- Scoring system already handles 0-point brick types (brick 41 precedent)
- Level transition system already clears entities and resets state (existing behavior)
- Multi-frame persistence testing will verify `CurrentLevel` stability

---

## Notes

- No new ECS archetypes created (bricks 50/54 use same components as standard bricks)
- No new resources or global state required
- Integration points: collision handler, level switch system, scoring system, audio system
- All changes are additive (no breaking changes to existing systems)
- Victory screen logic reuses existing "You Win!"
  UI from game completion flow
