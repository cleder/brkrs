# Quickstart: Paddle-Destroyable Brick (Type 57)

**Feature**: 022-paddle-destroyable-brick | **Date**: 2026-01-13

## Build & Test

### Prerequisites

- Rust 1.81 or later (check with `rustc --version`)
- Cargo installed
- Git repository cloned to local machine

### Build Commands

```bash
# From repository root
cd /home/christian/devel/bevy/brkrs

# Clean build (debug mode with optimized dependencies)
cargo clean
cargo build

# Release build (for performance testing)
cargo build --release

# WASM build (web target)
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --target wasm32-unknown-unknown --release
```

### Test Commands

```bash
# Run all tests
cargo test

# Run specific integration test for paddle-destroyable brick
cargo test --test paddle_destroyable_brick

# Run with output logging (see DEBUG messages)
RUST_LOG=debug,paddle_destroyable=trace cargo test --test paddle_destroyable_brick -- --nocapture

# Format code
cargo fmt --all

# Lint code
cargo clippy --all-targets --all-features
bevy lint
```

### Run Game

```bash
# Debug mode (fast iteration)
cargo run

# Release mode (full performance)
cargo run --release

# WASM serve (requires wasm-server-runner)
cargo install wasm-server-runner
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo run --target wasm32-unknown-unknown --release
```

---

## Manual Verification

### Test Scenario 1: Paddle Destroys Brick

**Goal**: Verify paddle contact destroys type 57 brick and awards 250 points.

**Steps**:

1. Create test level file `assets/levels/test_paddle_brick.ron`:

   ```ron
   (
       ball: [(0.0, 15.0, 0.0)],
       paddle: [(0.0, 0.0, -15.0)],
       bricks: [
           (brick_type: 57, position: (0.0, 0.0, -10.0), rotation: (0.0, 0.0, 0.0)),
       ],
   )
   ```

2. Launch game: `cargo run --release`
3. Load test level (navigate to level via game menu or modify level loader to load test level)
4. Move paddle forward (W key or mouse) to touch brick at Z=-10
5. **Observe**:
   - Brick disappears within 1 frame (< 16.67ms)
   - Score increases by exactly 250 points
   - No errors in console

✅ **Pass Criteria**:

- Brick despawns immediately on paddle contact
- Score display shows +250 points
- No panic or error messages

---

### Test Scenario 2: Ball Bounces Off Brick

**Goal**: Verify ball does NOT destroy type 57 brick.

**Steps**:

1. Use same test level from Scenario 1
2. Launch ball toward paddle-destroyable brick
3. **Observe**:
   - Ball bounces off brick at correct angle
   - Brick remains intact (not destroyed)
   - Score does NOT increase
   - Ball continues trajectory after bounce

✅ **Pass Criteria**:

- Brick persists after multiple ball hits
- Ball physics behave normally (realistic bounce)
- Score remains unchanged (0 points from ball contact)

---

### Test Scenario 3: Level Completion Requires Type 57 Destruction

**Goal**: Verify paddle-destroyable bricks count toward level completion.

**Steps**:

1. Create level with ONLY type 57 bricks:

   ```ron
   (
       ball: [(0.0, 15.0, 0.0)],
       paddle: [(0.0, 0.0, -15.0)],
       bricks: [
           (brick_type: 57, position: (-5.0, 0.0, -10.0), rotation: (0.0, 0.0, 0.0)),
           (brick_type: 57, position: (0.0, 0.0, -10.0), rotation: (0.0, 0.0, 0.0)),
           (brick_type: 57, position: (5.0, 0.0, -10.0), rotation: (0.0, 0.0, 0.0)),
       ],
   )
   ```

2. Launch game and load level
3. Use paddle to destroy all 3 bricks (ball cannot complete this level)
4. **Observe**: Level completion triggers after destroying all type 57 bricks

✅ **Pass Criteria**:

- Level does NOT complete when bricks remain
- Level completes immediately when all type 57 bricks destroyed
- Completion percentage accurate (e.g., 1/3 bricks = 33%)

---

### Test Scenario 4: Spawn Overlap Edge Case

**Goal**: Verify brick spawning overlapping paddle is destroyed on first frame.

**Steps**:

1. Create level with brick at paddle position:

   ```ron
   (
       ball: [(0.0, 15.0, 0.0)],
       paddle: [(0.0, 0.0, -15.0)],
       bricks: [
           (brick_type: 57, position: (0.0, 0.0, -15.0), rotation: (0.0, 0.0, 0.0)),  // Same Z as paddle!
       ],
   )
   ```

2. Launch game and load level
3. **Observe**:
   - Brick does NOT appear (destroyed on spawn)
   - Score shows 250 points immediately
   - No collision errors or physics glitches

✅ **Pass Criteria**:

- Brick destroyed in first frame after level load
- Score increases by 250 points
- No physics instability or errors

---

### Test Scenario 5: Multiple Simultaneous Paddle Contacts

**Goal**: Verify multiple paddle-destroyable bricks award correct total points.

**Steps**:

1. Create level with 2 type 57 bricks close together:

   ```ron
   (
       ball: [(0.0, 15.0, 0.0)],
       paddle: [(0.0, 0.0, -15.0)],
       bricks: [
           (brick_type: 57, position: (0.0, 0.0, -10.0), rotation: (0.0, 0.0, 0.0)),
           (brick_type: 57, position: (1.0, 0.0, -10.0), rotation: (0.0, 0.0, 0.0)),
       ],
   )
   ```

2. Move paddle to contact both bricks in same frame
3. **Observe**: Score increases by 500 points (250 × 2)

✅ **Pass Criteria**:

- Both bricks destroyed
- Score = 500 points (not 250)
- No duplicate or missing score awards

---

### Test Scenario 6: Logging Verification

**Goal**: Verify DEBUG-level logging for paddle-brick collisions.

**Steps**:

1. Run game with debug logging enabled:

   ```bash
   RUST_LOG=debug,paddle_destroyable=trace cargo run --release
   ```

2. Move paddle to contact type 57 brick
3. **Check console output** for log message:

   ```text
   DEBUG paddle_destroyable: Paddle-brick type 57 collision detected: brick=Entity(...)
   ```

✅ **Pass Criteria**:

- Log message appears on paddle contact
- Target is `paddle_destroyable`
- Level is DEBUG (not INFO or WARN)
- Message includes brick entity ID

---

## Integration Checklist

### Level Loader Verification

**File**: `src/level_loader.rs`

- [x] Type 57 bricks spawn with `BrickTypeId(57)` component
- [x] Type 57 bricks spawn with `CountsTowardsCompletion` marker component
- [x] Type 57 bricks spawn with `Collider` (bevy_rapier3d)
- [x] Type 57 bricks spawn with `Brick` marker component
- [x] No special loader logic required (uses existing `brick_type @ 3..=255` pattern)

### Paddle Collision Handler Verification

**File**: `src/lib.rs` (function: `read_character_controller_collisions`)

- [ ] Detects paddle-brick collisions via `KinematicCharacterControllerOutput`
- [ ] Queries `BrickTypeId` component to identify type 57 bricks
- [ ] Inserts `MarkedForDespawn` on type 57 bricks when paddle contact occurs
- [ ] Logs DEBUG message with target `"paddle_destroyable"`
- [ ] Does NOT directly emit `BrickDestroyed` message (handled by despawn system)

### Ball Collision Handler Verification

**File**: `src/lib.rs` (function: `handle_collision_events`)

- [ ] Adds `is_paddle_destroyable_brick(brick_type: u8) -> bool` helper function
- [ ] Calls helper in ball-brick collision loop: `if is_paddle_destroyable_brick(current_type) { continue; }`
- [ ] Ball collision with type 57 does NOT insert `MarkedForDespawn`
- [ ] Ball physics bounce handled automatically by bevy_rapier3d (no code changes)

### Scoring System Verification

**File**: `src/systems/scoring.rs`

- [x] `brick_points(57, _)` returns exactly 250 (already implemented at line 123)
- [x] `award_points_system` reads `BrickDestroyed` messages (no changes needed)
- [x] Score updates persist across multiple frames (saturating addition)

### Despawn System Verification

**File**: `src/lib.rs` (function: `despawn_marked_entities`)

- [x] Emits `BrickDestroyed` message for entities with `MarkedForDespawn` + `BrickTypeId` (no changes needed)
- [x] Uses `despawn_recursive()` for hierarchy safety (already implemented)
- [x] Message emitted BEFORE entity despawn (existing guarantee)

---

## Acceptance Test Coverage

### User Story 1: Paddle Destroys Brick (P1)

| Acceptance Scenario | Test Scenario | Status |
|---------------------|---------------|--------|
| AS 1.1: Brick despawned within 1 frame | Scenario 1 | ✅ |
| AS 1.2: 250 points awarded | Scenario 1 | ✅ |
| AS 1.3: Level completion condition met | Scenario 3 | ✅ |
| AS 1.4: 250-point award persists 10 frames | Integration test | Required |
| AS 1.5: Uses Messages (not Observers) | Code review | ✅ (design) |
| AS 1.6: Uses `despawn_recursive()` | Code review | ✅ (design) |

### User Story 2: Ball Bounces Off Brick (P1)

| Acceptance Scenario | Test Scenario | Status |
|---------------------|---------------|--------|
| AS 2.1: Ball reflects at correct angle | Scenario 2 | ✅ |
| AS 2.2: Brick NOT despawned | Scenario 2 | ✅ |
| AS 2.3: Zero points awarded | Scenario 2 | ✅ |
| AS 2.4: Brick exists after 10 frames | Integration test | Required |
| AS 2.5: Uses rapier collision events | Code review | ✅ (design) |

### User Story 3: Level File Configuration (P2)

| Acceptance Scenario | Test Scenario | Status |
|---------------------|---------------|--------|
| AS 3.1: Brick spawns from RON file | Scenario 1 | ✅ |
| AS 3.2: Brick has all required components | Integration test | Required |
| AS 3.3: Multiple bricks spawn correctly | Scenario 3, 5 | ✅ |
| AS 3.4: Bricks persist 10 frames after load | Integration test | Required |

---

## Known Limitations

1. **No visual distinction**: Type 57 bricks use default brick texture unless texture manifest is updated
2. **No audio feedback**: Paddle-brick collision plays generic `BrickHit` audio; unique sound requires audio system update (out of scope)
3. **Single paddle assumption**: Code assumes one paddle entity (follows existing game design)
4. **No particle effects**: Brick destruction has no visual effect beyond despawn (visual enhancement out of scope)

---

## Troubleshooting

### Brick not destroyed by paddle

**Symptom**: Paddle moves through brick without destroying it

**Checks**:

- Verify brick has `BrickTypeId(57)` component (use `bevy_inspector_egui` or debug logging)
- Verify paddle has `KinematicCharacterController` component
- Check `RUST_LOG=debug,paddle_destroyable=trace` for collision detection log messages
- Verify `read_character_controller_collisions` system is registered and running

### Ball destroys brick

**Symptom**: Ball hits brick and brick disappears

**Checks**:

- Verify `is_paddle_destroyable_brick()` function returns `true` for type 57
- Verify ball collision handler calls `is_paddle_destroyable_brick()` before destruction logic
- Check for early `continue` statement in ball-brick collision loop

### Score not awarded

**Symptom**: Paddle destroys brick but score doesn't increase

**Checks**:

- Verify `BrickDestroyed` message is emitted (add debug logging to `despawn_marked_entities`)
- Verify `brick_points(57, _)` returns 250 (add unit test)
- Verify `award_points_system` is running (check system registration)
- Verify `ScoreState` resource exists (initialized in startup)

### Multiple bricks award wrong total

**Symptom**: Destroying 2 type 57 bricks awards 250 points instead of 500

**Checks**:

- Verify `despawn_marked_entities` loops over ALL marked entities (not early break)
- Verify `award_points_system` reads ALL messages from `MessageReader` (not just first)
- Check for duplicate detection logic accidentally filtering type 57 bricks

---

## Performance Validation

### Frame Budget Compliance

**Target**: 60 FPS (16.67ms per frame)

**Test Method**:

1. Create level with 50 paddle-destroyable bricks
2. Run game with frame time profiling:

   ```bash
   cargo run --release --features bevy/trace_tracy
   ```

3. Move paddle through all bricks in rapid succession
4. **Expected**: Frame time remains < 16.67ms throughout destruction sequence

**Metrics**:

- Collision detection: < 0.1ms per frame
- Despawn system: < 0.5ms per frame
- Score update: < 0.01ms per frame

---

## Next Steps

After manual verification passes all scenarios:

1. **Write integration tests**: Implement tests in `tests/paddle_destroyable_brick.rs` covering all acceptance scenarios
2. **Run TDD workflow**: Commit failing tests, get approval, then implement feature
3. **Update documentation**: Add brick type 57 to `docs/bricks.md` reference table
4. **Create example level**: Add `assets/levels/demo_paddle_brick.ron` showcasing type 57 gameplay
5. **WASM testing**: Verify feature works in web build (RUSTFLAGS getrandom config)
6. **Update texture manifest**: Add type 57 texture profile to `assets/textures/manifest.ron` (optional visual enhancement)
