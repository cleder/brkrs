# Quickstart: Brick Types 42 & 91 — Paddle Life Loss

**Feature**: 023-brick-42-91-life-loss **Status**: Ready for Implementation

## Overview

This feature adds two new brick types to the game:

- **Brick Type 42 (Killer)**: Destructible by ball collision; awards 90 points; paddle contact causes life loss
- **Brick Type 91 (Indestructible)**: Indestructible by ball collision; paddle contact causes life loss; does not count toward level completion

## Key Changes

### 1. Level Format (Brick Spawning)

Bricks are defined in level matrices using numeric IDs.
Type 42 and 91 can be added to any level RON file:

```ron
LevelDefinition {
    number: 1,
    matrix: [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 20, 20, 20, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 20, 20, 20, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 42, 91, 42, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 91, 42, 91, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        // ... more rows ...
    ],
}
```

- Type `20`: Simple stone (destructible, counts toward completion)
- Type `42`: Killer brick (destructible, 90 pts, life loss on paddle hit, counts toward completion)
- Type `91`: Indestructible (never destroyed by ball, life loss on paddle hit, does NOT count toward completion)

### 2. Game Mechanics

#### Ball Collision with Type 42

- Brick is destroyed
- 90 points awarded
- `BrickDestroyed` message emitted
- Contributes to level completion (when all type 42 destroyed, level completes)

#### Ball Collision with Type 91

- Brick remains (indestructible)
- 0 points awarded
- No `BrickDestroyed` message
- Does NOT count toward level completion

#### Paddle Collision with Type 42

- Life is lost (lives_remaining -= 1)
- Standard respawn sequence begins
- Brick remains (paddle collision does not destroy bricks)

#### Paddle Collision with Type 91

- Life is lost (lives_remaining -= 1)
- Standard respawn sequence begins
- Brick remains

#### Multi-Contact Policy

- If the paddle contacts multiple hazard bricks (both types 42 and 91) in the same frame, only **one life is lost** (not one per brick)
- This prevents unfair multi-loss bursts when bricks are stacked or overlapping

### 3. Implementation Checklist

#### Phase 0: Constants & Helpers

- [ ] Add `HAZARD_BRICK_91: u8 = 91` constant to `src/level_format/mod.rs`
- [ ] Add `fn is_hazard_brick(type_id: u8) -> bool` helper
- [ ] Update level loader: Do NOT insert `CountsTowardsCompletion` for type 91

#### Phase 1: Paddle Collision

- [ ] Extend `read_character_controller_collisions()` to emit `LifeLostEvent` on hazard brick contact
- [ ] Add `Local<bool>` frame flag to enforce one loss per frame
- [ ] Add system to reset frame flag each update

#### Phase 2: Ball Collision

- [ ] Extend `mark_brick_on_ball_collision()` to skip destruction for type 91
- [ ] Verify `BrickDestroyed` only emitted for type 42

#### Phase 3: Level Completion & Textures

- [ ] Verify level completion query only counts `CountsTowardsCompletion` bricks
- [ ] Add type 91 to `assets/textures/manifest.ron` if needed

#### Phase 4: Testing

- [ ] Write unit tests in `tests/brick_42_91_life_loss.rs`
- [ ] Run `cargo test --all`
- [ ] Run `cargo clippy --all-targets --all-features`

## Testing the Feature

### Manual Test: Destroy Brick 42 & Get Points

1. Create a simple level with brick type 42
2. Run the game and hit the brick with the ball
3. Observe: Brick disappears, score increases by 90 points

```bash
cargo run --release
# In-game: Level with type 42 brick
# Action: Hit with ball
# Expected: Brick gone, score += 90
```

### Manual Test: Indestructible Brick 91

1. Create a level with both brick types 42 and 91
2. Hit both with the ball
3. Observe: Type 42 destroyed, type 91 remains

```bash
cargo run --release
# In-game: Level with types 42 and 91
# Action: Hit both with ball
# Expected: Type 42 gone (90 pts), type 91 remains (0 pts)
```

### Manual Test: Paddle Life Loss

1. Create a level with hazard bricks
2. Move the paddle into a brick
3. Observe: Life counter decreases by 1, respawn sequence begins

```bash
cargo run --release
# In-game: Level with hazard bricks (42 or 91)
# Action: Move paddle into brick
# Expected: Lives: 3 → 2, respawn begins
```

### Automated Tests

```bash
# Run all tests for this feature
cargo test brick_42_91 --lib

# Run specific test
cargo test brick_42_91::test_brick_42_ball_collision_awards_90_points -- --nocapture

# Full suite
cargo test --all
cargo clippy --all-targets --all-features
cargo fmt --all
```

## Architecture Overview

```text
read_character_controller_collisions
  ├─ Detect paddle-brick contact
  ├─ Check if brick is hazard (type 42 or 91)
  ├─ Emit LifeLostEvent (once per frame max)
  └─ Uses Local<bool> to enforce single-loss-per-frame

mark_brick_on_ball_collision
  ├─ Detect ball-brick contact
  ├─ Skip destruction for type 91
  └─ Mark type 42 for despawn → BrickDestroyed → +90 pts

Level Completion
  ├─ Query: bricks with CountsTowardsCompletion
  ├─ Type 42: has marker (counts toward completion)
  └─ Type 91: no marker (does not block completion)

Lives System (existing)
  ├─ Consume LifeLostEvent (from ball loss OR paddle collision)
  ├─ Decrement lives
  └─ Respawn sequence / Game Over if lives = 0
```

## Reference Implementation Patterns

### Paddle Collision Handler (Skeleton)

```rust
fn read_character_controller_collisions(
    // ... existing parameters ...
    brick_types: Query<&BrickTypeId, With<Brick>>,
    mut life_lost_writer: MessageWriter<LifeLostEvent>,
    mut local_loss_sent: Local<bool>,
) {
    // ... existing code ...

    for collision in output.collisions.iter() {
        for brick in bricks.iter() {
            if collision.entity == brick {
                if let Ok(brick_type) = brick_types.get(brick) {
                    if is_hazard_brick(brick_type.0) && !*local_loss_sent {
                        // Find a ball to attribute the loss to
                        if let Some(ball_entity) = balls.iter().next() {
                            let ball_spawn = ball_handles
                                .get(ball_entity)
                                .map(|h| h.spawn)
                                .unwrap_or_else(|_| spawn_points.ball_spawn());

                            life_lost_writer.write(LifeLostEvent {
                                ball: ball_entity,
                                cause: LifeLossCause::LowerGoal,
                                ball_spawn,
                            });
                            *local_loss_sent = true;
                        }
                    }
                }
                // ... rest of collision handling ...
            }
        }
    }
}

fn clear_life_loss_frame_flag(mut local_loss_sent: Local<bool>) {
    *local_loss_sent = false;
}
```

### Ball Collision Handler (Skeleton)

```rust
fn mark_brick_on_ball_collision(
    // ... existing parameters ...
) {
    // ... existing code ...

    if is_multi_hit_brick(current_type) {
        // ... multi-hit logic ...
    } else {
        // Skip destruction for type 91 (indestructible)
        if !is_hazard_brick(current_type) || current_type != 91 {
            // Mark for despawn and emit BrickDestroyed
            commands.entity(entity).insert(MarkedForDespawn);
        }
    }
}
```

## Expected Behavior Summary

| Scenario | Expected Result |
|----------|-----------------|
| Ball hits type 42 | Brick destroyed, +90 points, `BrickDestroyed` message |
| Ball hits type 91 | Brick remains, 0 points, no message |
| Paddle hits type 42 | Life lost, respawn sequence |
| Paddle hits type 91 | Life lost, respawn sequence |
| Paddle hits 42 & 91 same frame | 1 life lost (not 2) |
| Level with 3×42 + 2×91 | Completes when all 42 destroyed (91 remain) |
| Destroy 42 then pause 10 frames | Score persists at +90 |
| Lose 1 life then pause 10 frames | Lives persists at -1 |
