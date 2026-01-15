# Quickstart: Add Scoring System

**Feature**: Add Scoring System **Branch**: `009-add-scoring` **Date**: 16 December 2025

## Overview

This quickstart guide provides step-by-step instructions to build, test, and verify the scoring system feature.

---

## Prerequisites

- Rust 1.81+ (edition 2021)
- Cargo
- Git

---

## Build & Run

### 1. Checkout Feature Branch

```bash
git checkout 009-add-scoring
```

### 2. Build the Project

```bash
# Development build (faster compile)
cargo build

# Release build (optimized)
cargo build --release
```

### 3. Run the Game

```bash
# Development mode
cargo run

# Release mode (recommended for performance testing)
cargo run --release
```

---

## Automated Testing

### Run All Tests

```bash
cargo test
```

### Run Scoring-Specific Tests

```bash
# Score tracking and accumulation
cargo test --test scoring

# Score display UI
cargo test --test score_display

# Integration tests
cargo test --test integration_transition
```

### Run with Detailed Output

```bash
cargo test -- --nocapture
```

### Expected Test Coverage

- Score initialization (starts at 0)
- Point award on brick destruction (all brick types 10-57)
- Random scoring for Question brick (range 25-300)
- Milestone detection at 5000-point intervals
- Milestone ball spawning
- Score persistence across levels
- Score reset on game restart
- UI display updates

---

## Manual Verification

### Test Scenario 1: Score Initialization

**Goal**: Verify score starts at 0 and displays correctly.

1. Launch game: `cargo run --release`
2. **Observe**: Score display shows "0" in top-right corner
3. **Expected**: Score is visible, formatted as integer, no visual glitches

✅ **Pass Criteria**: Score display shows "0" at game start.

---

### Test Scenario 2: Points Awarded on Brick Destruction

**Goal**: Verify points accumulate when bricks are destroyed.

1. Launch game and start level 1
2. Destroy a Simple Stone brick (index 20, worth 25 points)
3. **Observe**: Score display updates to "25"
4. Destroy another Simple Stone brick
5. **Observe**: Score display updates to "50"

✅ **Pass Criteria**:

- Score increases by correct amount (25 points per Simple Stone)
- Display updates immediately (<1 frame delay)
- No flickering or rendering issues

---

### Test Scenario 3: Different Brick Values

**Goal**: Verify various brick types award correct points.

1. Destroy bricks of different types:
   - Simple Stone (20): 25 points
   - Multi-hit brick (10): 50 points
   - Apple brick (30): 300 points
2. **Observe**: Score increases by documented amount for each brick
3. **Verify**: Check docs/bricks.md for expected values

✅ **Pass Criteria**: Each brick awards points matching its documented value in docs/bricks.md.

---

### Test Scenario 4: Milestone Ball Award (5000 Points)

**Goal**: Verify extra ball/life is awarded at 5000-point threshold.

**Setup**: Use console command or modified level with many high-value bricks to reach 5000 points quickly.

1. Start game with score at 4980 points (destroy bricks to reach this)
2. Destroy one more brick worth ≥20 points (crosses 5000 threshold)
3. **Observe**:
   - Score updates to ≥5000
   - Lives counter increases by 1 (e.g., "♥ ♥ ♥" → "♥ ♥ ♥ ♥")
   - Extra ball/life is now available
4. Continue to 10,000 points
5. **Observe**: Lives counter increases again (second bonus ball awarded)

✅ **Pass Criteria**:

- Lives counter increments within 1 frame of crossing milestone
- Multiple milestones work (5000, 10000, 15000, etc.)
- Lives display shows correct total

---

### Test Scenario 5: Score Persistence Across Levels

**Goal**: Verify score carries forward between levels.

1. Start game, accumulate 500 points in level 1
2. Complete level 1 (destroy all destructible bricks)
3. **Observe**: Score remains 500 at level 2 start
4. Destroy brick in level 2
5. **Observe**: Score increases from 500 (not from 0)

✅ **Pass Criteria**: Score persists across level transitions, cumulative throughout game session.

---

### Test Scenario 6: Score Reset on Game Restart

**Goal**: Verify score resets when starting a new game.

1. Play game, accumulate any non-zero score
2. Lose all lives → "Game Over" screen
3. Press key to restart game
4. **Observe**: Score resets to "0"

✅ **Pass Criteria**: Score is 0 at start of new game session.

---

### Test Scenario 7: Question Brick Random Scoring

**Goal**: Verify Question brick (index 53) awards random points in range 25-300.

1. Find or create a level with Question bricks
2. Destroy multiple Question bricks (10+)
3. **Record**: Point awards for each (e.g., 127, 254, 89, etc.)
4. **Verify**: All values are between 25 and 300 (inclusive)
5. **Verify**: Values are different (not all the same)

✅ **Pass Criteria**:

- All Question brick scores in [25, 300] range
- Values vary (randomness working)

---

## Performance Verification

### Frame Rate Check

**Goal**: Verify scoring system maintains 60 FPS.

1. Run in release mode: `cargo run --release`
2. Enable FPS counter (if available) or use external tool
3. Play normally, destroying many bricks rapidly
4. **Observe**: FPS remains ≥60

✅ **Pass Criteria**: Game maintains 60 FPS during normal and intense gameplay.

---

### Score Display Update Latency

**Goal**: Verify score updates within 16ms (one frame at 60 FPS).

1. Play game in release mode
2. Destroy bricks while watching score display
3. **Observe**: Score changes appear instantaneous

✅ **Pass Criteria**: No perceptible delay between brick destruction and score update.

---

## Code Quality Checks

### Formatting

```bash
cargo fmt --all -- --check
```

✅ **Pass Criteria**: No formatting issues reported.

---

### Linting

```bash
cargo clippy --all-targets --all-features
```

✅ **Pass Criteria**: No warnings or errors from Clippy.

---

### Bevy-Specific Linting

```bash
bevy lint
```

✅ **Pass Criteria**: No Bevy-specific warnings.

---

## Troubleshooting

### Score Not Displaying

**Symptom**: Score display is invisible or missing.

**Checks**:

1. Verify `ScoreDisplayUi` component spawned at startup
2. Check Orbitron font loaded correctly
3. Verify UI node position (should be top-right corner)
4. Check z-index/layer ordering

**Debug**: Add log statement in `spawn_score_display` system.

---

### Points Not Accumulating

**Symptom**: Score stays at 0 despite destroying bricks.

**Checks**:

1. Verify `BrickDestroyed` events are being emitted (add log in brick destruction system)
2. Verify `award_points_system` is processing events (add log in system)
3. Check system ordering (scoring systems run after brick destruction)

**Debug**:

```bash
RUST_LOG=brkrs::systems::scoring=debug cargo run
```

---

### Milestone Ball Not Spawning

**Symptom**: Score crosses 5000 but no ball appears.

**Checks**:

1. Verify `MilestoneReached` event emitted (add log in detect_milestone)
2. Verify `spawn_milestone_ball_system` processes event (add log)
3. Check `SpawnPoints` resource has valid ball spawn position
4. Verify ball spawning logic (similar to respawn logic)

**Debug**:

```bash
RUST_LOG=brkrs::systems::scoring=debug,brkrs::systems::respawn=debug cargo run
```

---

### Random Scores Out of Range

**Symptom**: Question brick awards <25 or >300 points.

**Checks**:

1. Verify `gen_range(25..=300)` called correctly (inclusive range)
2. Check `GlobalRng` resource available
3. Test with fixed seed for reproducibility

**Debug**: Add assertion in `brick_points` function for Question brick case.

---

## Success Criteria Reference

Quick reference to specification success criteria:

- **SC-001**: Complete level with score increasing to ≥1000 points ✓
- **SC-002**: Ball spawns within 1 second at 5000 points ✓
- **SC-003**: Score display updates within 16ms ✓
- **SC-004**: Brick points match docs/bricks.md values ✓
- **SC-005**: Score visible throughout game session ✓
- **SC-006**: Two bonus balls awarded by 10,000 points ✓
- **SC-007**: 100% of destructible bricks (10-57) award points ✓

---

## Next Steps

After verification:

1. **If all tests pass**: Feature is complete, ready for merge
2. **If tests fail**: Debug using troubleshooting section, fix issues, re-test
3. **Code review**: Request review from team member
4. **Integration**: Merge to main development branch after approval

---

## Additional Resources

- [spec.md](spec.md) - Complete feature specification
- [data-model.md](data-model.md) - Data structures and relationships
- [contracts/events.md](contracts/events.md) - Event message contracts
- [docs/bricks.md](../../docs/bricks.md) - Brick point value reference

---

## Support

For issues or questions:

- Check test output for specific failures
- Review implementation in `src/systems/scoring.rs` and `src/ui/score_display.rs`
- Consult existing similar systems (lives counter, respawn) for patterns
