# Contracts: Direction Bricks

**Document Type**: Phase 1 API Design **Feature**: Direction Bricks **Implementation Plan**: [plan.md](../plan.md)

## Observer Event Contract

### Event Type: DirectionBrickEffect

**Namespace**: `brkrs::systems::brick_effects`

**Rust Type**:

```rust
#[derive(Event, Clone, Debug)]
pub struct DirectionBrickEffect {
    pub ball_entity: Entity,
    pub brick_type: u32,
    pub brick_position: Vec3,
    pub velocity_before: Vec3,
}
```

**Trigger Point**: Emitted by brick destruction system immediately when direction brick (ID 43-48 or 52) is destroyed

**Trigger Timing**: Before `BrickDestroyed` message is read by scoring system (same update cycle)

**Observer Function Signature**:

```rust
pub fn apply_direction_brick_effects(
    trigger: Trigger<DirectionBrickEffect>,
    mut query: Query<&mut LinearVelocity, With<Ball>>,
)
```

**Constraints**:

- `ball_entity` MUST reference a valid Ball entity with `LinearVelocity` component
- `brick_type` MUST be 43, 44, 45, 46, 47, 48, or 52 (others silently ignored)
- `velocity_before` is for logging/tracing only; not used by physics logic
- Observer runs in `Update` schedule, before physics integration

**Guarantees**:

- Observer runs exactly once per `Trigger<DirectionBrickEffect>` event
- Velocity modification is applied atomically (no partial updates)
- Z-axis velocity is never modified
- Points are awarded independently (via `BrickDestroyed` message) before impulse

**Side Effects**:

- Modifies `LinearVelocity::linvel` (X and Y components only)
- Emits tracing spans with brick ID, entity ID, velocity deltas
- No other components or resources modified

## System Signature

### Module: `brick_effects`

**File**: `src/systems/brick_effects.rs`

**Public Functions**:

```rust
/// Observer system for direction brick effects.
/// 
/// Applies velocity impulses to balls when direction bricks (43-48, 52) are destroyed.
/// Emits structured tracing spans for observability.
pub fn apply_direction_brick_effects(
    trigger: Trigger<DirectionBrickEffect>,
    mut query: Query<&mut LinearVelocity, With<Ball>>,
)

/// Applies randomized velocity for brick 52.
/// 
/// Generates random magnitude (5.0..=15.0) and direction (0..360°).
/// Internal helper function; may be extracted if reused.
fn apply_randomizer(velocity: &mut LinearVelocity)
```

**Plugin Registration**:

```rust
impl Plugin for DirectionBricksPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(apply_direction_brick_effects);
    }
}
```

## Integration Points

### Messaging Chain

```text
Brick Destruction System:
  ├─ Emit: BrickDestroyed message
  │   └─ Read by: Scoring system
  │       └─ Update score (next message read phase)
  │
  └─ Trigger: DirectionBrickEffect
      └─ Received by: apply_direction_brick_effects observer
          └─ Update ball LinearVelocity (same frame)
```

### Scoring Integration

**Location**: `src/systems/scoring.rs`

**Change**: Extend brick_type match statement to include 43-48, 52

**Before**:

```rust
match brick_type {
    1..=42 => { /* existing logic */ },
    49..=51 => { /* existing logic */ },
    53.. => { /* existing logic */ },
    _ => {}
}
```

**After**:

```rust
match brick_type {
    1..=42 => { /* existing logic */ },
    43 | 44 | 45 | 46 => score += 75,     // Cardinal impulses
    47 | 48 => score += 100,               // Diagonal impulses
    49..=51 => { /* existing logic */ },
    52 => score += 125,                    // Randomizer
    53.. => { /* existing logic */ },
    _ => {}
}
```

**No removal**: Existing scoring logic remains untouched; direction bricks are additive

### Physics System Interaction

**Framework**: bevy_rapier3d 0.32.0

**Component Modified**: `LinearVelocity`

**Properties Accessed**:

- `velocity.linvel.x` (read/write)
- `velocity.linvel.y` (read/write)
- `velocity.linvel.z` (read only; never modified)
- `velocity.angvel` (untouched)

**Constraints Respected**: No modification of `LockedAxes` or other physics constraints

**Physics Propagation**: Velocity changes propagate naturally through physics system in next frame (gravity, collisions, damping all apply to modified velocity)

## Level File Contract

### Brick Type Registration

**Location**: Level loader (existing)

**Required Support**: Level loader MUST recognize brick type IDs 43-48 and 52 in RON files

**No Changes Needed**: Existing loader already supports arbitrary brick type IDs

**Example Level Entry**:

```ron
(position: (4.0, 5.0, 10.0), brick_type: 43)
```

**Validation**: Loader should accept any u32 brick_type; unknown types are rendered as default brick visual or skipped per existing logic

## Component Requirements

### Required on Ball Entity

```rust
#[derive(Component)]
pub struct Ball;  // Marker component

#[derive(Component)]
pub struct LinearVelocity {  // From bevy_rapier3d
    pub linvel: Vec3,
    pub angvel: Vec3,
}
```

### Required on Brick Entity (pre-existing)

```rust
#[derive(Component)]
pub struct Transform {  // From Bevy
    pub translation: Vec3,
    // ... other fields
}
```

## Error Handling

### Invalid Ball Entity

**Scenario**: `DirectionBrickEffect` references entity that doesn't exist or lacks `LinearVelocity`

**Behavior**: Observer query returns `Err`; impulse is silently skipped

**Logging**: If enabled, tracing logs the event context for debugging

**Recovery**: No recovery needed; next brick destruction can modify same or different ball

### Unknown Brick Type

**Scenario**: `DirectionBrickEffect.brick_type` is not 43-48 or 52

**Behavior**: Match statement falls through to `_ => {}`; no impulse applied

**Logging**: Event is still traced if tracing is enabled; unknown type is logged

**Recovery**: No recovery needed; velocity unchanged

### RNG Failure (Brick 52)

**Scenario**: `rand::thread_rng()` fails (extremely rare)

**Behavior**: Panic avoided by using `gen_range()` which doesn't fail

**Guarantee**: RNG will never produce magnitude < 5.0 or > 15.0

## Performance Characteristics

### Computational Complexity

- **Cardinal bricks (43-46)**: O(1) - single float operation
- **Diagonal bricks (47-48)**: O(1) - two float operations
- **Randomizer (52)**: O(1) - RNG call, two float operations

### Memory Usage

- **DirectionBrickEffect event**: ~40 bytes (Entity + u32 + Vec3 + Vec3)
- **Tracing span overhead**: Negligible in tests; configurable in release

### Query Performance

- Observer query filters for `With<Ball>`; uses existing component filter
- No additional index contention

## Testing Contract

### Test Framework

**Framework**: `cargo test` (standard Rust testing)

**Required Tests**:

- Unit tests: Each brick type impulse calculation (7 tests)
- Randomizer tests: Magnitude range, direction distribution
- Multi-frame tests: Persistence over 10+ frames
- Edge case tests: Stationary ball, rapid succession, Z-axis independence
- Scoring tests: Point values verified (7 values)
- Integration tests: No regression in existing systems

### Test Data Requirements

See [data-model.md](../data-model.md) for test fixtures and expected outputs.

## Observability Contract

### Tracing Spans

**Span Name**: `"direction_brick_effect"`

**Fields**:

- `brick_type: u32` (ID 43-48 or 52)
- `ball_entity: Entity` (target entity)
- `velocity_before: Vec3` (pre-impulse velocity)
- `velocity_after: Vec3` (post-impulse velocity)
- `points: i32` (points awarded, 75/100/125)

**Log Level**: `info!` (visible in test output)

**Example**:

```text
direction_brick_effect{brick_type=45 ball_entity=Entity { index: 3, generation: 1 } velocity_before=[3.0, 2.0, 0.0] velocity_after=[8.0, 2.0, 0.0] points=75}: direction_brick_effect: Direction brick impulse applied
```

### Tracing Configuration

**Test Harness**: Tracing enabled by default in tests (via `tracing_subscriber`)

**Production**: Tracing can be disabled at compile time via feature flags (no-op spans)

## Backward Compatibility

### Breaking Changes

**None**: Existing brick types 1-42, 49-51, 53+ are unaffected.

### Non-Breaking Additions

- New observer registration (added to plugin init)
- New event type (doesn't conflict with existing messages)
- Scoring system extended (additive case in match, no removal)

### Migration Path

**For Existing Levels**: Levels without direction bricks (43-48, 52) work unchanged.

**For New Levels**: Simply add brick_type entries for 43-48 or 52.

## External Dependencies

### New Crate: `rand` 0.8

**License**: MIT OR Apache-2.0 (compatible with project)

**Used For**: Randomization brick (52) magnitude and direction generation

**Impact**: Adds ~50 KB to compiled binary (already in `Cargo.toml`)

### Existing Crates (No Version Changes)

- `bevy 0.17.3`: Observers, Events, ECS
- `bevy_rapier3d 0.32.0`: LinearVelocity component, physics
- `tracing 0.1`: Structured logging

## Success Criteria for Contract Compliance

- ✅ Observer system correctly receives and triggers on DirectionBrickEffect
- ✅ Velocity impulses calculated per specification (±5.0 units/sec)
- ✅ Randomizer generates magnitude 5.0-15.0, direction 0-360°
- ✅ Tracing spans emitted with correct context
- ✅ Scoring system awards correct points (75, 100, 125)
- ✅ Multi-frame persistence verified (10+ frames)
- ✅ Z-axis velocity never modified
- ✅ All acceptance scenarios pass
- ✅ No regression in existing brick types
