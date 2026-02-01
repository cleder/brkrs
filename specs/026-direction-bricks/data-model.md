# Data Model: Direction Bricks

**Document Type**: Phase 1 Design **Feature**: [spec.md](spec.md) **Implementation Plan**: [plan.md](plan.md)

## Domain Entities

### 1. Direction Brick (Abstract Base)

**Purpose**: Game entity representing destructible brick with velocity modification behavior

**Component/Marker**: `DirectionBrick` (component storing brick type ID)

**Attributes**:

- `brick_type: u32` - Brick ID (43-48 or 52)
- `position: Transform` - XZ position on game board
- `health: i32` - Durability (destroyed at 1 hit in standard mode)
- `visual: Handle<Image>` - Texture handle from asset system

**Lifecycle**:

1. **Spawn**: Level loader creates brick entity with Transform, image handle, brick_type component
2. **Collision Detection**: Ball entity collides with brick collider
3. **Destruction Signal**: Brick destruction system emits `BrickDestroyed` message
4. **Observer Trigger**: Brick destruction system triggers `Trigger<DirectionBrickEffect>`
5. **Removal**: Brick entity despawned, replaced with particle effects and audio

**Relationships**:

- **Depends On**: Ball entity (observes its `LinearVelocity` for impulse application)
- **Depends On**: Scoring system (receives points via `BrickDestroyed` message)
- **Depends On**: Physics system (collides with ball via Rapier3D collider)

### 2. Direction Brick Effects (Subtypes)

#### Brick 43: Down Impulse

**Brick Type ID**: 43 **Effect**: `velocity.y -= 5.0` (instantaneous, applied once per destruction) **Points**: 75

**Behavior Example**:

```text
Input: Ball velocity = (3.0, 2.0, 0.0)
Impulse: Apply -5.0 to Y
Output: Ball velocity = (3.0, -3.0, 0.0)
```

#### Brick 44: Left Impulse

**Brick Type ID**: 44 **Effect**: `velocity.x -= 5.0` **Points**: 75

**Behavior Example**:

```text
Input: Ball velocity = (3.0, 2.0, 0.0)
Impulse: Apply -5.0 to X
Output: Ball velocity = (-2.0, 2.0, 0.0)
```

#### Brick 45: Right Impulse

**Brick Type ID**: 45 **Effect**: `velocity.x += 5.0` **Points**: 75

**Behavior Example**:

```text
Input: Ball velocity = (-3.0, 2.0, 0.0)
Impulse: Apply +5.0 to X
Output: Ball velocity = (2.0, 2.0, 0.0)
```

#### Brick 46: Up Impulse

**Brick Type ID**: 46 **Effect**: `velocity.y += 5.0` **Points**: 75

**Behavior Example**:

```text
Input: Ball velocity = (3.0, -2.0, 0.0)
Impulse: Apply +5.0 to Y
Output: Ball velocity = (3.0, 3.0, 0.0)
```

#### Brick 47: Up-Right Impulse

**Brick Type ID**: 47 **Effect**: `velocity.x += 5.0; velocity.y += 5.0` (both applied simultaneously) **Points**: 100

**Behavior Example**:

```text
Input: Ball velocity = (1.0, 1.0, 0.0)
Impulse: Apply +5.0 to X and +5.0 to Y
Output: Ball velocity = (6.0, 6.0, 0.0)
```

#### Brick 48: Up-Left Impulse

**Brick Type ID**: 48 **Effect**: `velocity.x -= 5.0; velocity.y += 5.0` (both applied simultaneously) **Points**: 100

**Behavior Example**:

```text
Input: Ball velocity = (3.0, 1.0, 0.0)
Impulse: Apply -5.0 to X and +5.0 to Y
Output: Ball velocity = (-2.0, 6.0, 0.0)
```

#### Brick 52: Randomizer

**Brick Type ID**: 52 **Effect**: Replace velocity with random vector (not impulse; complete replacement) **Points**: 125 **Random Generation**:

```text
magnitude = rand::thread_rng().gen_range(5.0..=15.0)
direction_angle = rand::thread_rng().gen_range(0.0..360.0)
velocity.x = magnitude * direction_angle.cos()
velocity.y = magnitude * direction_angle.sin()
velocity.z = 0.0  # Z-axis unchanged
```

**Behavior Example**:

```text
Input: Ball velocity = (10.0, 0.0, 3.0)
RNG Output: magnitude = 8.5, angle = 45°
Output: Ball velocity = (6.01, 6.01, 3.0)  # Z preserved
```

## System Design

### Observer Event Type

**Event Name**: `DirectionBrickEffect` (custom trigger type)

**Payload**:

```rust
pub struct DirectionBrickEffect {
    pub ball_entity: Entity,
    pub brick_type: u32,
    pub brick_position: Vec3,
    pub velocity_before: Vec3,
}
```

**Trigger Point**: Emitted by brick destruction system immediately after `BrickDestroyed` message is created

**Observer System**: Listening to `Trigger<DirectionBrickEffect>`, applies impulse to `ball_entity`'s `LinearVelocity`

### Velocity Modification Logic

**Location**: `src/systems/brick_effects.rs`

**System Function Signature**:

```rust
pub fn apply_direction_brick_effects(
    trigger: Trigger<DirectionBrickEffect>,
    mut query: Query<&mut LinearVelocity, With<Ball>>,
) {
    let ball_entity = trigger.event().ball_entity;
    let brick_type = trigger.event().brick_type;
    let velocity_before = trigger.event().velocity_before;

    if let Ok(mut velocity) = query.get_mut(ball_entity) {
        match brick_type {
            43 => velocity.linvel.y -= 5.0,
            44 => velocity.linvel.x -= 5.0,
            45 => velocity.linvel.x += 5.0,
            46 => velocity.linvel.y += 5.0,
            47 => {
                velocity.linvel.x += 5.0;
                velocity.linvel.y += 5.0;
            }
            48 => {
                velocity.linvel.x -= 5.0;
                velocity.linvel.y += 5.0;
            }
            52 => {
                let mut rng = rand::thread_rng();
                let magnitude = rng.gen_range(5.0..=15.0);
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                velocity.linvel.x = magnitude * angle.cos();
                velocity.linvel.y = magnitude * angle.sin();
            }
            _ => {}
        }

        // Emit tracing span
        tracing::info_span!(
            "direction_brick_effect",
            brick_type,
            brick_position = ?trigger.event().brick_position,
            velocity_before = ?velocity_before,
            velocity_after = ?velocity.linvel,
        ).in_scope(|| {
            tracing::info!("Direction brick impulse applied");
        });
    }
}
```

### Scoring Integration

**Existing Pattern**: Scoring system reads `MessageReader<BrickDestroyed>` and updates score based on brick type

**Direction Brick Addition**: Extend brick type match statement to include 43-48, 52 with corresponding point values:

```rust
match brick_type {
    43 | 44 | 45 | 46 => score += 75,   // Cardinal impulses
    47 | 48 => score += 100,             // Diagonal impulses
    52 => score += 125,                  // Randomizer
    // ... existing brick types
    _ => {}
}
```

## Data Persistence

**Storage Model**: In-memory ECS only (no persistence to disk per project design)

**Level Definition**: RON files in `assets/levels/` specify brick positions and types

**Example Level File** (excerpt):

```ron
(
  bricks: [
    // Cardinal impulse bricks
    (position: (1.0, 5.0, 10.0), brick_type: 43),
    (position: (3.0, 5.0, 10.0), brick_type: 44),
    (position: (5.0, 5.0, 10.0), brick_type: 45),
    (position: (7.0, 5.0, 10.0), brick_type: 46),
    // Diagonal impulse bricks
    (position: (2.0, 5.0, 12.0), brick_type: 47),
    (position: (6.0, 5.0, 12.0), brick_type: 48),
    // Randomizer
    (position: (4.0, 5.0, 14.0), brick_type: 52),
  ],
)
```

## State Transitions

### Normal Brick Destruction Flow

```text
Spawned → Collision Detected → BrickDestroyed Message Sent + DirectionBrickEffect Triggered
    ↓
Ball receives velocity impulse (immediate via Observer)
Ball receives tracing span (immediate via Observer)
Score updated (next frame via MessageReader)
Brick entity despawned
```

### Edge Cases

#### 1. Stationary Ball Hits Direction Brick

**Initial State**: Ball has `velocity = (0.0, 0.0, 0.0)`

**Impulse Application**: Velocity becomes `(0.0, 5.0, 0.0)` for brick 46 (Up)

**Result**: Ball begins moving in specified direction

#### 2. Multiple Direction Bricks Hit in Rapid Succession

**Frame 1**: Ball hits brick 45 (Right) → `velocity = (3.0 + 5.0, -2.0, 0.0) = (8.0, -2.0, 0.0)`

**Frame 2**: Ball hits brick 46 (Up) → `velocity = (8.0, -2.0 + 5.0, 0.0) = (8.0, 3.0, 0.0)`

**Result**: Impulses stack; each is applied to current velocity, not original

#### 3. Brick 52 Randomizer Hit Multiple Times

**Hit 1**: Random generation → `velocity = (12.3, 4.1, 0.0)` (example)

**Hit 2**: Random generation → `velocity = (-7.2, 8.5, 0.0)` (new random, replaces previous)

**Result**: Each hit produces independent random velocity; no accumulation

#### 4. Z-Velocity Preserved During Impulse

**Input**: Ball has `velocity = (2.0, 3.0, 5.0)` (moving forward/backward)

**Brick 45 (Right)**: Apply +5.0 to X

**Output**: `velocity = (7.0, 3.0, 5.0)` (Z unchanged)

**Result**: Forward momentum preserved; only X direction accelerated

## Validation Rules

### Pre-Impulse Validation

- Ball entity must exist and have `LinearVelocity` component (error: log and skip)
- Brick type must be 43-48 or 52 (error: skip unknown types)

### Post-Impulse Validation

- No validation or clamping of resulting velocity (physics system owns bounds)
- Physics system will apply gravity/forces on next frame as normal

### Scoring Validation

- Points awarded before impulse is applied (ensures reward even if velocity application fails)

## Testing Data

### Test Fixtures

**Stationary Ball**:

```rust
let ball = Ball {
    velocity: LinearVelocity { linvel: Vec3::ZERO, angvel: Vec3::ZERO },
    ..default()
};
```

**Moving Ball**:

```rust
let ball = Ball {
    velocity: LinearVelocity { linvel: Vec3::new(3.0, -2.0, 0.0), angvel: Vec3::ZERO },
    ..default()
};
```

**Direction Brick 43 Effect**:

```rust
let effect = DirectionBrickEffect {
    ball_entity: ball_id,
    brick_type: 43,
    brick_position: Vec3::new(4.0, 5.0, 10.0),
    velocity_before: Vec3::new(3.0, -2.0, 0.0),
};
```

### Expected Outcomes

| Brick Type | Input Velocity | Expected Output Velocity |
|------------|----------------|--------------------------|
| 43 (Down) | (3.0, 2.0, 0.0) | (3.0, -3.0, 0.0) |
| 44 (Left) | (3.0, 2.0, 0.0) | (-2.0, 2.0, 0.0) |
| 45 (Right) | (-3.0, 2.0, 0.0) | (2.0, 2.0, 0.0) |
| 46 (Up) | (3.0, -2.0, 0.0) | (3.0, 3.0, 0.0) |
| 47 (Up-Right) | (2.0, 2.0, 0.0) | (7.0, 7.0, 0.0) |
| 48 (Up-Left) | (3.0, 1.0, 0.0) | (-2.0, 6.0, 0.0) |
| 52 (Random) | (10.0, 0.0, 3.0) | Magnitude 5.0-15.0, angle 0-360° (Z=3.0) |
