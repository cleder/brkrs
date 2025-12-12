# Data Model: Paddle Size Powerups

**Phase**: 1 - Design & Contracts **Date**: 2025-12-12 **Status**: Complete

## Core Entities

### Paddle (Extended)

The player-controlled horizontal bar.
This entity gains a `PaddleSizeEffect` component when a powerup brick is hit.

**Base Entity** (existing):

- Entity: Paddle game object
- Transform: Position, scale, rotation
- Collider: Physics collider (Rapier3D)
- Material: Visual rendering properties
- [other existing components]

**New Component - PaddleSizeEffect**:

```rust
#[derive(Component, Clone)]
pub struct PaddleSizeEffect {
    /// Type of effect: Shrink (0.7x) or Enlarge (1.5x)
    pub effect_type: SizeEffectType,

    /// Time remaining in seconds before effect expires
    pub remaining_duration: f32,

    /// Original paddle width before effect applied (for reset calculation)
    pub base_width: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SizeEffectType {
    /// Shrink paddle to 70% of base size (minimum 10 units)
    Shrink,
    /// Enlarge paddle to 150% of base size (maximum 30 units)
    Enlarge,
}
```

**Relationships**:

- One paddle entity → zero or one active `PaddleSizeEffect` component
- Effect component is replaced (not stacked) when new brick is hit

**State Transitions**:

- No effect → Effect (on brick 30/32 hit)
- Shrink effect → Enlarge effect (on brick 32 hit while shrunk)
- Enlarge effect → Shrink effect (on brick 30 hit while enlarged)
- Any effect → No effect (on duration expiry, life loss, or level change)

**Lifecycle Rules**:

- Created: On collision with brick 30 or brick 32
- Updated: Every frame (duration countdown)
- Deleted: When duration ≤ 0, player loses life, or level changes
- Visual representation: Applied to material immediately upon creation

---

### Brick Type 30 (Shrink)

Special brick that triggers paddle shrinkage when destroyed by ball collision.

**Entity Marker Component**:

```rust
#[derive(Component)]
pub struct BrickType30;
```

**Required Sibling Components** (from level loading):

- Transform
- Collider (Rapier3D)
- Material/Mesh for rendering
- [standard brick components]

**Behavior**:

- Collision with ball → triggers `PaddleSizeEffect { effect_type: SizeEffectType::Shrink, ... }`
- Destroyed and removed from level (existing brick behavior)

**Relationships**:

- One-to-one with brick entity
- One brick 30 hit → one paddle `PaddleSizeEffect` (replaces any existing)

---

### Brick Type 32 (Enlarge)

Special brick that triggers paddle enlargement when destroyed by ball collision.

**Entity Marker Component**:

```rust
#[derive(Component)]
pub struct BrickType32;
```

**Required Sibling Components**:

- Transform
- Collider (Rapier3D)
- Material/Mesh for rendering
- [standard brick components]

**Behavior**:

- Collision with ball → triggers `PaddleSizeEffect { effect_type: SizeEffectType::Enlarge, ... }`
- Destroyed and removed from level (existing brick behavior)

**Relationships**:

- One-to-one with brick entity
- One brick 32 hit → one paddle `PaddleSizeEffect` (replaces any existing)

---

## Data Transformations & Calculations

### Size Calculation

When a `PaddleSizeEffect` is active, the paddle's visual width is calculated as:

```text
calculated_width = base_width × multiplier
actual_width = clamp(calculated_width, 10.0, 30.0)
```

Where:

- `base_width` = 20 units (standard paddle width)
- `multiplier` = 0.7 for Shrink, 1.5 for Enlarge
- `clamp(value, min, max)` = constrain to [10, 30] range

**Examples**:

| Scenario | Base | Multiplier | Calculated | Clamped | Effect Active |
|----------|------|-----------|------------|---------|---|
| Normal paddle | 20 | 1.0 | 20 | 20 | None |
| Brick 30 hit | 20 | 0.7 | 14 | 14 | Shrink (10s) |
| At min, brick 30 hit | 10 | 0.7 | 7 | 10 | Shrink (10s) |
| Brick 32 hit | 20 | 1.5 | 30 | 30 | Enlarge (10s) |
| At max, brick 32 hit | 30 | 1.5 | 45 | 30 | Enlarge (10s) |

### Visual Feedback Mapping

**Color Tints**:

| Effect | Base Color | Emission |
|--------|-----------|----------|
| None (normal) | Default paddle material | None |
| Shrink | Red: (1.0, 0.3, 0.3) | Red glow: (0.3, 0.0, 0.0) |
| Enlarge | Green: (0.3, 1.0, 0.3) | Green glow: (0.0, 0.3, 0.0) |

**Glow Intensity**: Subtle (emission values ≤ 0.3) to avoid overwhelming visuals

### Audio Mapping

| Event | Sound File | Trigger |
|-------|-----------|---------|
| Brick 30 hit | `assets/audio/paddle_shrink.ogg` | On collision start |
| Brick 32 hit | `assets/audio/paddle_enlarge.ogg` | On collision start |

---

## State Lifecycle Diagram

```text
┌─────────────────────────────────────────────────────────────────┐
│                     PADDLE SIZE STATE MACHINE                    │
└─────────────────────────────────────────────────────────────────┘

         ┌──────────────────┐
         │   NO EFFECT      │ (default state)
         │ (Normal 20 units)│
         └────────┬─────────┘
                  │
        ┌─────────┴──────────┐
        │                    │
   [Brick 30]          [Brick 32]
        │                    │
        ▼                    ▼
   ┌─────────┐          ┌─────────┐
   │ SHRINK  │          │ ENLARGE │
   │ 14 u    │          │ 30 u    │
   │ Timer:  │          │ Timer:  │
   │ 10s     │          │ 10s     │
   └────┬────┘          └────┬────┘
        │                    │
        │ [Brick 32]    [Brick 30]
        ├──────────┬─────────┤
        │          │         │
        │      Timer=0  Level change
        │      Loss life    Loss life
        │          │         │
        └──────────┴─────────┘
                   │
                   ▼
            [EFFECT CLEARED]
              (return to normal)
```

---

## Validation & Constraints

### Invariants

- `remaining_duration ≥ 0.0` at all times
- Only one `PaddleSizeEffect` component per paddle entity
- `base_width` is always 20.0 units (constant)
- `actual_width` is always in [10.0, 30.0] units

### Edge Case Handling

| Condition | Behavior | Rationale |
|-----------|----------|-----------|
| Hit brick 30 while already at 10 units | Effect activates, timer resets, size stays 10 | Honors effect intent |
| Hit brick 32 while already at 30 units | Effect activates, timer resets, size stays 30 | Honors effect intent |
| Hit same brick type twice in 10s | Effect timer resets, size unchanged | Allows duration extension |
| Hit different brick type | Replace effect, reset timer, recalculate size | Only one active at a time |
| Level ends with active effect | Component removed, paddle resets to 20 | Clean slate per level |
| Player loses life with active effect | Component removed, paddle resets to 20 | Clean slate per life |

---

## Acceptance Criteria Coverage

| Requirement | Data Model Element | Testability |
|-------------|-------------------|-------------|
| FR-003: Shrink to 70% | `SizeEffectType::Shrink` + calculation | ✓ Unit test multiplier |
| FR-004: Enlarge to 150% | `SizeEffectType::Enlarge` + calculation | ✓ Unit test multiplier |
| FR-006: 10s duration | `remaining_duration: f32` timer | ✓ Integration test |
| FR-007: Timer reset on re-hit | Component replacement | ✓ Integration test |
| FR-009: Min 10 units | `clamp(value, 10.0, 30.0)` | ✓ Unit test boundary |
| FR-010: Max 30 units | `clamp(value, 10.0, 30.0)` | ✓ Unit test boundary |
| FR-011: Clear on life loss | Event listener removes component | ✓ Integration test |
| FR-011b: Clear on level change | Event listener removes component | ✓ Integration test |

---

## Integration Points

### Existing Systems Used

- **Bevy ECS**: Component insertion/removal
- **Rapier3D**: Collision event stream
- **Transform**: Paddle position/scale
- **Material system**: Color and emission updates
- **Audio plugin**: Sound playback
- **Time resource**: Delta timing

### No New External Dependencies

All functionality achievable with existing Bevy ecosystem.
No additional crates required beyond project baseline (serde, ron for asset loading already in use).
