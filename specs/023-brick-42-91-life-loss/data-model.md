# Data Model: Brick Types 42 & 91 — Paddle Life Loss

**Feature**: 023-brick-42-91-life-loss **Created**: 2026-01-13

## Entity & Component Model

### Brick Entity (Type 42)

**Spawned by**: `src/level_loader.rs:spawn_level_entities()` **Destroyed by**: Ball collision via `mark_brick_on_ball_collision` system **Components**:

- `Brick` (marker)
- `BrickTypeId(42)`
- `CountsTowardsCompletion` (marker) — contributes to level completion
- `Transform` (position, rotation, scale)
- `RigidBody::Fixed`
- `Collider` (cuboid)
- `Restitution`, `Friction`
- `CollidingEntities`
- `ActiveEvents::COLLISION_EVENTS`

**State Transitions**:

- Spawned → (ball collision) → Marked with `MarkedForDespawn` → Despawned + `BrickDestroyed` message emitted

**Interactions**:

- **Ball collision**: Destroy and emit `BrickDestroyed { brick_type: 42, ... }`
- **Paddle collision**: Emit `LifeLostEvent` (via new behavior)

### Brick Entity (Type 91)

**Spawned by**: `src/level_loader.rs:spawn_level_entities()` **Destroyed by**: Never **Components**:

- `Brick` (marker)
- `BrickTypeId(91)`
- **NOT** `CountsTowardsCompletion` — does not affect level completion
- `Transform` (position, rotation, scale)
- `RigidBody::Fixed`
- `Collider` (cuboid)
- `Restitution`, `Friction`
- `CollidingEntities`
- `ActiveEvents::COLLISION_EVENTS`

**State Transitions**:

- Spawned → (collisions handled, no destruction) → Remains until level ends

**Interactions**:

- **Ball collision**: No change; ball bounces off; no event emitted
- **Paddle collision**: Emit `LifeLostEvent` (via new behavior)

### Paddle Entity

**Existing entity** (not changed by this feature) **Key behavior for this feature**:

- Collision detection via `KinematicCharacterControllerOutput`
- Detects contacts with bricks in `read_character_controller_collisions`
- Triggers `LifeLostEvent` on contact with hazard bricks (new)

### Lives State Resource

**Location**: `src/systems/respawn.rs` **Struct**:

```rust
pub struct LivesState {
    pub lives_remaining: u8,
    pub on_last_life: bool,
}
```

**Updates**:

- Decremented via `LifeLostEvent` message consumption (existing system)
- When lives reach 0, `GameOverRequested` message emitted (existing behavior)
- New life losses from paddle collisions use the same flow

### Score State Resource

**Location**: `src/systems/scoring.rs` **Struct**:

```rust
pub struct ScoreState {
    pub current_score: u32,
    pub last_milestone_reached: u32,
}
```

**Updates**:

- Incremented by 90 when `BrickDestroyed { brick_type: 42, ... }` is consumed (existing system)
- No changes needed; type 42 already configured in `brick_points()` function

---

## Message / Event Model

### `BrickDestroyed` Message

**Emitted by**: `despawn_marked_entities` system **Consumed by**: `award_points_system` (scoring), audio system **Type**: Bevy Message (not Observer)

**Payload**:

```rust
pub struct BrickDestroyed {
    pub brick_entity: Entity,
    pub brick_type: u8,
    pub destroyed_by: Option<Entity>,
}
```

**For Type 42**:

- Emitted when: Ball collides and brick is marked for despawn
- `brick_type`: 42
- `destroyed_by`: Some(ball_entity)
- Points awarded: 90 (via `brick_points(42, &mut rng)`)

**For Type 91**:

- Never emitted (indestructible)

### `LifeLostEvent` Message

**Emitted by**:

- `detect_ball_loss` system (ball below lower goal — existing)
- `read_character_controller_collisions` system (paddle hazard brick contact — NEW)

**Consumed by**:

- Respawn scheduling system (existing)
- Visual/audio feedback systems (existing)

**Type**: Bevy Message (not Observer)

**Payload**:

```rust
pub struct LifeLostEvent {
    pub ball: Entity,
    pub cause: LifeLossCause,
    pub ball_spawn: SpawnTransform,
}

pub enum LifeLossCause {
    LowerGoal,
    // Optionally add: PaddleHazard, for clarity
}
```

**For Paddle-Brick Collision (NEW)**:

- Emitted when: Paddle contacts type 42 or type 91 brick
- `ball`: First ball entity in the world
- `cause`: `LifeLossCause::LowerGoal` (existing enum; reused)
- `ball_spawn`: Cached spawn point from `RespawnHandle` or `SpawnPoints` resource
- Result: Lives decremented by 1; respawn sequence initiated if lives > 0

**Multi-Frame Policy**:

- Frame N: Paddle contacts brick 42 AND brick 91 → exactly one `LifeLostEvent` emitted
- Frame N+1: No event (contacts ended)
- Implementation: `Local<bool>` flag in `read_character_controller_collisions` tracks if loss already sent this frame

---

## System Execution Order

### Frame 0 (Clear Phase)

**System**: `clear_life_loss_frame_flag` (new)

- **Purpose**: Reset per-frame life-loss tracking flag
- **Order**: Early in Update schedule, before collision detection

### Frame 1 (Collision Detection)

**System**: `read_character_controller_collisions` (extended)

- **Purpose**: Detect paddle collisions
- **New behavior**:
  - Check if colliding entity is a hazard brick (type 42 or 91)
  - Emit `LifeLostEvent` if not already sent this frame
  - Use `Local<bool>` flag to ensure single loss per frame

**System**: `mark_brick_on_ball_collision` (extended)

- **Purpose**: Mark bricks for destruction on ball contact
- **New behavior**:
  - Skip destruction for type 91 (indestructible)
  - Continue normal destruction for type 42

### Frame 2 (Despawn & Messaging)

**System**: `despawn_marked_entities` (existing)

- **Purpose**: Remove marked entities and emit `BrickDestroyed` messages
- **Behavior**: Type 42 bricks emit `BrickDestroyed`; type 91 never reaches this system

**System**: `award_points_system` (existing)

- **Purpose**: Award points on `BrickDestroyed` message
- **Behavior**: Type 42 → +90 points; no change needed

### Frame 3 (Lives Processing)

**System**: Respawn flow consumes `LifeLostEvent`

- **Purpose**: Decrement lives and initiate respawn sequence
- **Behavior**: Existing system; handles paddle-triggered events same as ball→lower goal

---

## Level Completion Integration

### Completion Query

**Location**: `src/level_loader.rs` (exact system TBD) **Query**: All bricks with `With<CountsTowardsCompletion>` **Rule**: Level complete when count of such bricks reaches 0

### Type 42 Behavior

- Spawned with `CountsTowardsCompletion` ✓
- Contributes to completion count
- Destruction decrements count → may trigger level completion

### Type 91 Behavior

- Spawned WITHOUT `CountsTowardsCompletion` ✓
- Does NOT contribute to completion count
- Presence does not block completion

### Level Completion Scenario

**Level Matrix**:

```text
5 × Type 42 bricks
3 × Type 91 bricks (indestructible)
```

**Play Sequence**:

1. Player destroys all 5 type 42 bricks
2. Completion query finds 0 bricks with `CountsTowardsCompletion`
3. Level marked complete
4. Type 91 bricks remain visible but do not prevent completion

---

## Key Design Invariants

1. **Type 42 is destructible by ball collision**
   - Destruction is via `MarkedForDespawn` component insertion
   - `BrickDestroyed` message emitted in `despawn_marked_entities`
   - Scoring system reads message and awards 90 points

2. **Type 91 is indestructible by ball collision**
   - Ball-brick system skips it (guard: `if is_hazard_brick(type) { continue; }`)
   - `BrickDestroyed` message never emitted for type 91
   - Type 91 remains in scene until level ends

3. **Paddle collision with both types triggers life loss**
   - Once per frame maximum (via `Local<bool>` flag)
   - Uses existing `LifeLostEvent` message
   - Respawn flow handles like any other life loss

4. **Type 91 does not count toward completion**
   - Lacks `CountsTowardsCompletion` marker during spawn
   - Completion query ignores it
   - Level can complete with type 91 bricks still present

5. **Score increments from type 42 are immediate and persist**
   - Updated in `award_points_system` on `BrickDestroyed` message read
   - No per-frame overwrites (architecture responsibility)
   - Multi-frame persistence guaranteed by resource nature

---

## Testing Anchors

### Unit Test Fixtures

1. **Single Brick 42 + Ball**
   - Setup: Spawn brick 42 and ball
   - Action: Trigger collision
   - Assert: Brick despawned, `BrickDestroyed` message present, score += 90

2. **Single Brick 91 + Ball**
   - Setup: Spawn brick 91 and ball
   - Action: Trigger collision
   - Assert: Brick remains, no `BrickDestroyed` message, score unchanged

3. **Brick 42 + Paddle**
   - Setup: Spawn brick 42 and paddle with ball
   - Action: Trigger paddle collision
   - Assert: `LifeLostEvent` message present, lives decremented by 1

4. **Brick 91 + Paddle**
   - Setup: Spawn brick 91 and paddle with ball
   - Action: Trigger paddle collision
   - Assert: `LifeLostEvent` message present, lives decremented by 1

5. **Multi-Contact Same Frame (Paddle)**
   - Setup: Spawn brick 42 and brick 91, both contacting paddle in same frame
   - Action: Trigger both collisions
   - Assert: Exactly one `LifeLostEvent` emitted (not two)

6. **Multi-Frame Persistence (Score)**
   - Setup: Spawn brick 42 and ball; destroy brick
   - Action: Run 10 updates, verify score unchanged
   - Assert: Score remains 90 across all frames

7. **Level Completion (Mixed Bricks)**
   - Setup: Level with 3 type 42 and 2 type 91; spawn ball
   - Action: Destroy all type 42 bricks
   - Assert: Level completion triggered; type 91 remain visible
