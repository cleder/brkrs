# Implementation Plan: Brick Types 42 & 91 — Paddle Life Loss

**Feature Branch**: `023-brick-42-91-life-loss` **Status**: Ready for Implementation **Last Updated**: 2026-01-13

## Technical Context

### Technology Stack

- **Rust 1.81** (Edition 2021) + **Bevy 0.17.3** + **bevy_rapier3d 0.32.0**
- **ECS state**: In-memory only; no persistent storage
- **Event system**: Bevy Messages (not Observers)
- **Hierarchy**: `commands.entity().insert()` and despawn patterns

### Integration Points

- **Lives System**: Existing `LivesState` resource; `LifeLostEvent` message consumption
- **Scoring System**: Existing `brick_points()` function in `src/systems/scoring.rs`; `BrickDestroyed` message
- **Level Completion**: Query on `CountsTowardsCompletion` marker component; destruction tracking
- **Paddle-Brick Collision**: Existing paddle collision logic in `src/lib.rs:read_character_controller_collisions`
- **Ball-Brick Collision**: Existing `mark_brick_on_ball_collision` system

### Known Dependencies

- Brick type 57 (paddle-destroyable) already implemented; can serve as reference
- Brick type 91 overlap with indestructible bricks (index 90); require integration with existing `INDESTRUCTIBLE_BRICK` constant
- Type 42 already has a scoring entry (90 points); no scoring system changes needed

---

## Data Model

### Components

#### `BrickTypeId`

**Location**: `src/lib.rs` (existing) **Purpose**: Identifies brick type via numeric ID (u8) **Used for**: Type 42 and Type 91 identification in collision detection and scoring

#### `CountsTowardsCompletion`

**Location**: `src/lib.rs` (existing) **Purpose**: Marker component indicating brick contributes to level completion **Rule**: Type 42 bricks MUST have this component; Type 91 bricks MUST NOT have this component

### Events/Messages

#### `BrickDestroyed`

**Location**: `src/signals.rs` (existing) **Contract**: Emitted when a brick is destroyed by ball collision **Payload**:

- `brick_entity: Entity`
- `brick_type: u8` (e.g., 42)
- `destroyed_by: Option<Entity>` (None = paddle, Some = ball entity)

**When emitted**:

- Type 42: On ball collision (via `mark_brick_on_ball_collision`)
- Type 91: Never (indestructible)

#### `LifeLostEvent`

**Location**: `src/systems/respawn.rs` (existing) **Contract**: Emitted when a life is lost **Payload**:

- `ball: Entity`
- `cause: LifeLossCause` (enum: `LowerGoal` or extend with `PaddleHazard`)
- `ball_spawn: SpawnTransform`

**When emitted**:

- Paddle collision with Type 42 or Type 91 (new behavior)
- Ball goes below lower goal (existing behavior)

#### `ScoreState` (Resource)

**Location**: `src/systems/scoring.rs` (existing) **Update**: Read `brick_type: 42` from `BrickDestroyed`; add 90 points per destruction

### Level Completion Integration

**Location**: `src/level_loader.rs` (existing level completion tracking) **Rule**:

- Type 42 bricks: Include `CountsTowardsCompletion` during spawn (already done)
- Type 91 bricks: Do NOT include `CountsTowardsCompletion` during spawn
- Level completes when all bricks with `CountsTowardsCompletion` are destroyed

---

## Contracts & Specifications

### Life-Loss Policy Contract

**Multi-Frame Paddle Collision Handling**:

1. A single `LifeLostEvent` is emitted per frame, regardless of how many hazardous bricks (42 or 91) the paddle contacts
2. Implementation: Use a `Local<bool>` flag in the paddle collision handler to track whether a loss has already been sent this frame
3. Reset flag at start of each frame in a dedicated system

**Example**:

- Frame N: Paddle contacts both brick 42 and brick 91 → emit one `LifeLostEvent`
- Frame N+1: No contacts → no event emitted
- Frame N+2: Paddle contacts brick 42 again → emit one `LifeLostEvent`

### Scoring Contract

**Brick 42 Point Award**:

- Ball collision with Type 42 → `BrickDestroyed { brick_type: 42, ... }` emitted
- Scoring system reads the message and adds 90 points to `ScoreState.current_score`
- No special handling needed; type 42 already configured in `brick_points()` function

**Brick 91 Scoring**:

- Type 91 is indestructible; `BrickDestroyed` is never emitted for it
- Paddle collision with Type 91 does not emit `BrickDestroyed` (no score award)
- Result: Type 91 always awards 0 points

---

## Implementation Phases

### Phase 0: Update Constants & Add Type 91 Brick Type Constant

**Goal**: Ensure type 91 is recognized as a valid brick type; add helper functions for identification.

**Tasks**:

1. Add constant `HAZARD_BRICK_91: u8 = 91` to `src/level_format/mod.rs`
2. Add helper function `fn is_hazard_brick(type_id: u8) -> bool` returning `true` for types 42 and 91
3. Update level loader: Do NOT insert `CountsTowardsCompletion` for type 91 bricks
   - Check: `if brick_type_id != INDESTRUCTIBLE_BRICK && brick_type_id != 91 { entity.insert(CountsTowardsCompletion); }`

**Acceptance**:

- `cargo clippy --all-targets` passes with new functions
- Level loader spawns type 91 bricks without `CountsTowardsCompletion` marker

### Phase 1: Implement Paddle-Brick Collision Life-Loss System

**Goal**: Emit `LifeLostEvent` when paddle contacts hazard bricks; limit to one loss per frame.

**Tasks**:

1. Extend `src/lib.rs:read_character_controller_collisions()` to:
   - Add query for `brick_types: Query<&BrickTypeId>`
   - Add `mut life_lost_writer: MessageWriter<LifeLostEvent>`
   - When paddle collides with a brick, check `is_hazard_brick(type_id)`
   - If hazard, emit `LifeLostEvent` with the first ball entity found
2. Add frame-scoped life-loss flag system:
   - Create `Local<bool>` in paddle collision handler to track if loss already sent this frame
   - Initialize to `false` at frame start
   - Check flag before emitting; set to `true` after emit
3. Add system to reset flag at start of each frame (e.g., in a "clear" phase)

**Acceptance**:

- Paddle collision with type 42 emits exactly one `LifeLostEvent`
- Paddle collision with type 91 emits exactly one `LifeLostEvent`
- Multiple hazardous contacts in same frame emit only one `LifeLostEvent`
- Flag resets each frame

**Tests** (see Phase 3):

- `test_paddle_brick_42_life_loss`
- `test_paddle_brick_91_life_loss`
- `test_single_life_loss_per_frame_multi_contact`

### Phase 2: Ensure Ball Collision Destroys Type 42 but Not Type 91

**Goal**: Verify ball-brick collision system respects indestructibility of type 91.

**Tasks**:

1. Extend `src/lib.rs:mark_brick_on_ball_collision()` to:
   - Check `is_hazard_brick(current_type)` and skip destruction for type 91
   - Continue destruction for type 42 as normal
   - Ensure `BrickDestroyed` event is emitted for type 42 only
2. Verify that type 91 does not transition or despawn on ball collision

**Acceptance**:

- Ball collision with type 42 → brick removed, `BrickDestroyed` event emitted, 90 points awarded
- Ball collision with type 91 → brick remains, no event emitted, no points awarded

**Tests** (see Phase 3):

- `test_ball_brick_42_destroyed_scores_90`
- `test_ball_brick_91_indestructible`

### Phase 3: Level Completion and Indestructibility Integration

**Goal**: Ensure type 91 bricks do not block level completion.

**Tasks**:

1. Verify level completion query only counts entities with `CountsTowardsCompletion`
   - No changes needed if already implemented correctly
   - Test by spawning level with only type 91 bricks and validating completion
2. Add texture/material support for type 91 in `assets/textures/manifest.ron` (if not already present)
   - Reference: Type 90 (indestructible) for visual consistency

**Acceptance**:

- Level with type 42 and type 91 completes when all type 42 are destroyed
- Level with only type 91 bricks is immediately complete or marked completable
- Texture manifest includes type 91 with indestructible profile

### Phase 4: Testing & Validation

**Goal**: Validate all behaviors via unit and integration tests.

**Test File**: `tests/brick_42_91_life_loss.rs`

**Tests**:

1. `test_brick_42_ball_collision_awards_90_points` — Ball destroys brick 42, awards 90 points
2. `test_brick_91_ball_collision_indestructible` — Ball does not destroy brick 91, awards 0 points
3. `test_paddle_brick_42_life_loss` — Paddle contact with type 42 triggers life loss
4. `test_paddle_brick_91_life_loss` — Paddle contact with type 91 triggers life loss
5. `test_single_life_loss_per_frame_multi_contact` — Multiple hazardous contacts = one life loss
6. `test_brick_42_contributes_to_completion` — Destroying all type 42 completes level with type 91 present
7. `test_brick_91_not_counted_in_completion` — Type 91 bricks do not block completion
8. `test_score_persists_across_frames` — Score updates persist across 10+ frames (multi-frame test)
9. `test_lives_persist_across_frames` — Lives decrements persist across 10+ frames (multi-frame test)

**Validation**:

- All tests pass
- `cargo test --all`
- `cargo clippy --all-targets --all-features`
- `cargo fmt --all`
- `bevy lint` (if available)

---

## Quickstart

### Quick Try

```bash
cd /home/christian/devel/bevy/brkrs
cargo test brick_42_91_life_loss --lib
```

### Build & Run

```bash
cargo build
cargo run
```

### Before Submitting

```bash
cargo test --all
cargo clippy --all-targets --all-features
cargo fmt --all
```

---

## Architecture Notes

### Coordinate System

- **XZ plane**: Horizontal movement (paddle left/right)
- **Y axis**: Vertical (up/down); paddle height locked via `LockedAxes::TRANSLATION_LOCKED_Y`
- **Brick collisions**: Physics-based; rapier3d handles contact events

### Design Decisions

1. **Why `Local<bool>` for life-loss frame tracking?**
   - Simplest approach for single-loss-per-frame policy
   - Avoids additional resource or component overhead
   - Scoped to system, automatically reset on system re-run

2. **Why extend existing `LifeLostEvent` instead of new message?**
   - Reuses existing lives/respawn flow
   - Maintains Message-Event Separation contract
   - Minimal diff; leverages proven pattern (ball→lower goal life loss)

3. **Why `CountsTowardsCompletion` marker for type 42 but not 91?**
   - Aligns with existing indestructible brick pattern (type 90)
   - Level completion already queries this marker
   - No changes needed to completion system

4. **Why `is_hazard_brick()` helper instead of separate constants?**
   - Centralizes hazard identification logic
   - Easier to extend if more hazard types added in future
   - Single source of truth for paddle-collision behavior

---

## Risks & Mitigations

| Risk | Likelihood | Mitigation |
|------|------------|-----------|
| Multi-frame life-loss flag not reset properly | Medium | Add dedicated "clear flags" system; test frame boundary conditions |
| Type 91 overlaps with existing indestructible brick behavior | Low | Verify level loader handles both types 90 and 91 correctly |
| Score milestone triggers on 90-point increments incorrectly | Low | Test milestone detection with 90-point awards; verify no off-by-one errors |
| Paddle collisions simultaneously with ball collisions (same brick) | Low | Test multi-ball scenarios; verify one event per actor type (paddle vs ball) |

---

## Dependencies & Integration

### Existing Systems Used

- `read_character_controller_collisions` (paddle collision detection)
- `mark_brick_on_ball_collision` (ball-brick destruction)
- `award_points_system` (scoring)
- `despawn_marked_entities` (brick removal)
- Lives tracking (`LivesState`, respawn flow)
- Level completion query

### Files Modified

- `src/level_format/mod.rs` (constants, helper functions)
- `src/lib.rs` (paddle collision handler)
- `src/systems/scoring.rs` (verify type 42 point mapping; no changes needed)
- `tests/brick_42_91_life_loss.rs` (new test file)
- `assets/textures/manifest.ron` (optional: type 91 material)

### No Changes Needed

- Event/message definitions (reuse existing)
- Lives system (reuse `LifeLostEvent`)
- Scoring system (type 42 already configured)
