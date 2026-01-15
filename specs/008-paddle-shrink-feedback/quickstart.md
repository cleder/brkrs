# Quickstart: Paddle Shrink Visual Feedback

**Feature**: 008-paddle-shrink-feedback **Date**: 2025-12-12

## Overview

This guide helps developers and testers quickly verify the paddle shrink visual feedback feature.
Follow these steps to observe the feature in action and confirm it meets requirements.

## Prerequisites

- Rust toolchain (1.81+)
- Cargo
- Git (on branch `008-paddle-shrink-feedback`)

## Quick Build & Run

```bash
# From repository root
cargo run --release

# For WASM testing
cargo build --release --target wasm32-unknown-unknown
# Serve with your preferred static server
```

## Manual Verification

### 1. Basic Shrink Behavior

**Goal**: Verify paddle shrinks when ball is lost

**Steps**:

1. Launch the game: `cargo run`
2. Start any level (press Space to begin)
3. Intentionally let the ball fall past the paddle into the lower goal
4. Observe the paddle

**Expected Behavior**:

- ✅ Paddle immediately begins shrinking smoothly
- ✅ Paddle shrinks from full size to very small (near-invisible at 0.01 scale)
- ✅ Shrink animation takes approximately 1 second
- ✅ Paddle remains visible throughout (does not fade or despawn)

**Visual Reference**:

```text
Frame 0:  ▬▬▬▬▬▬▬▬  (Full size, ball just lost)
Frame 15: ▬▬▬▬▬▬    (75% size)
Frame 30: ▬▬▬▬      (50% size)
Frame 45: ▬▬        (25% size)
Frame 60: ▬         (Near-zero size, barely visible)
```

---

### 2. Timing Integration

**Goal**: Verify shrink runs concurrently with respawn delay

**Steps**:

1. Lose the ball (as above)
2. Watch for:
   - Black fadeout overlay appearing
   - Paddle shrinking simultaneously
   - Both completing around the same time
3. Observe paddle regrowth after 1-second delay

**Expected Behavior**:

- ✅ Fadeout overlay and paddle shrink start together
- ✅ Both animations run for approximately 1 second
- ✅ Paddle reaches minimum size when overlay is at peak opacity
- ✅ After 1-second delay, paddle respawns and grows back to full size
- ✅ Total time from loss to gameplay resumption: ~3 seconds (1s shrink/delay + 2s regrowth)

**Timing Diagram**:

```text
0.0s: Ball lost → Shrink starts, overlay starts
0.5s: Paddle at 50% size, overlay at peak opacity
1.0s: Shrink complete, overlay fading out
1.0s: Respawn executor runs, paddle regrowth starts
3.0s: Regrowth complete, controls restored
```

---

### 3. Input Locking

**Goal**: Verify player controls remain locked during shrink

**Steps**:

1. Lose the ball
2. While paddle is shrinking, try to move mouse/scroll
3. Observe paddle behavior

**Expected Behavior**:

- ✅ Paddle does not respond to input during shrink animation
- ✅ Paddle does not respond to input during 1-second delay after shrink
- ✅ Paddle does not respond to input during regrowth (existing behavior)
- ✅ Controls restore only after regrowth completes

---

### 4. Edge Case: Shrink During Level Transition

**Goal**: Verify shrink interrupts level transition growth gracefully

**Steps**:

1. Complete a level (destroy all bricks)
2. During the paddle's growth animation in the next level, intentionally lose the ball quickly
3. Observe the transition

**Expected Behavior**:

- ✅ Paddle stops growing immediately when ball is lost
- ✅ Paddle begins shrinking from its current (partial growth) size
- ✅ Shrink animation completes smoothly
- ✅ Normal respawn sequence continues afterward

**Visual**: Paddle might be at 50% size (mid-growth) → shrinks from 50% to 1%

---

### 5. Edge Case: Rapid Consecutive Losses

**Goal**: Verify multiple quick ball losses handle shrink correctly

**Steps**:

1. Use debug mode or cheat to spawn multiple balls (if available)
2. Lose balls in rapid succession (< 1 second apart)
3. Observe paddle behavior

**Expected Behavior**:

- ✅ First loss: Paddle shrinks
- ✅ Second loss (during shrink): Paddle continues current shrink, respawn queued
- ✅ Third loss (during regrowth): Paddle stops growing, shrinks again
- ✅ Each queued respawn processes sequentially with full shrink-delay-regrowth cycle

---

### 6. Edge Case: Game Over During Shrink

**Goal**: Verify shrink completes when running out of lives

**Steps**:

1. Play until one life remains
2. Lose the ball
3. Observe paddle behavior

**Expected Behavior**:

- ✅ Paddle shrinks normally
- ✅ Shrink animation completes
- ✅ Game over sequence activates (no regrowth)
- ✅ No errors or visual glitches

---

## Automated Testing

### Run Integration Tests

```bash
# Run all tests
cargo test

# Run paddle shrink specific tests
cargo test paddle_shrink

# Run with output
cargo test paddle_shrink -- --nocapture
```

### Key Test Cases

1. **test_paddle_shrinks_on_ball_loss**: Verifies component added and scale changes
2. **test_shrink_duration_matches_respawn_delay**: Verifies timing correctness
3. **test_shrink_concurrent_with_fadeout**: Verifies parallel execution
4. **test_shrink_interrupted_by_regrowth**: Verifies edge case handling
5. **test_rapid_consecutive_losses**: Verifies queue handling

---

## Performance Verification

### Frame Rate Monitoring

**Steps**:

1. Run with Bevy diagnostics: `cargo run --features bevy/trace_chrome`
2. Lose ball repeatedly (10+ times)
3. Check FPS remains stable (60 FPS target)

**Expected**:

- ✅ No FPS drops during shrink animation
- ✅ No FPS drops during concurrent shrink + fadeout
- ✅ Consistent performance across multiple cycles

### WASM Performance

**Steps**:

1. Build WASM: `cargo build --release --target wasm32-unknown-unknown`
2. Serve with `python -m http.server` or similar
3. Open in browser (Chrome/Firefox)
4. Lose ball repeatedly, monitor DevTools Performance tab

**Expected**:

- ✅ Smooth animation at 60 FPS (or stable 30 FPS on lower-end devices)
- ✅ No JavaScript errors in console
- ✅ No WASM panics

---

## Debug Tools

### Enable Bevy Inspector

```bash
# Add bevy-inspector-egui to dev dependencies if needed
cargo run --features bevy_inspector_egui
```

**Usage**:

- Open inspector UI (press F1 or configured hotkey)
- Find paddle entity in hierarchy
- Verify `PaddleGrowing` component appears/disappears correctly
- Check `Transform.scale` values update smoothly

### Logging

```bash
# Enable debug logging
RUST_LOG=brkrs=debug cargo run
```

**Look For**:

- "Paddle shrink triggered" log on ball loss
- Timer tick logs showing progress
- Component add/remove logs

---

## Troubleshooting

### Issue: Paddle doesn't shrink

**Possible Causes**:

1. `apply_paddle_shrink` system not running → Check system registration
2. `PaddleGrowing` not added → Check event handling
3. `update_paddle_growth` not animating → Check system schedule

**Debug**:

```bash
RUST_LOG=brkrs::systems::respawn=trace cargo run
```

### Issue: Shrink too fast/slow

**Possible Causes**:

1. Timer duration mismatch → Verify `respawn_schedule.timer.duration()`
2. Easing curve incorrect → Check cubic calculation

**Fix**: Adjust `PADDLE_GROWTH_DURATION` constant or shrink timer initialization

### Issue: Visual popping during transitions

**Possible Causes**:

1. Scale interpolation not smooth → Check lerp logic
2. Component replacement timing → Verify respawn executor behavior

**Debug**: Enable frame-by-frame stepping with debugger, inspect `Transform.scale` each frame

---

## Acceptance Criteria Checklist

Before marking this feature complete, verify:

- [ ] Paddle shrinks smoothly on ball loss (FR-001, FR-002)
- [ ] Shrink reaches minimum scale of 0.01 (FR-003)
- [ ] Shrink runs concurrently with respawn delay (FR-004)
- [ ] Paddle remains at minimum scale until respawn (FR-005)
- [ ] Input locked during shrink (FR-006)
- [ ] Paddle visible throughout shrink (FR-007)
- [ ] Integrates with respawn system without breaking existing behavior (FR-008)
- [ ] Smooth cubic easing applied (FR-009)
- [ ] Duration matches fadeout overlay (FR-010)
- [ ] Handles interruption during growth (FR-011)
- [ ] Works with queued respawns (FR-012)
- [ ] Only affects associated paddle (FR-013)

---

## Next Steps

After manual verification:

1. Run full test suite: `cargo test`
2. Build for WASM and test in browser
3. Run `cargo fmt` and `cargo clippy`
4. Run `bevy lint` for Bevy-specific linting
5. Update CHANGELOG.md with feature summary
6. Create pull request with test results

---

## References

- Feature Spec: `specs/008-paddle-shrink-feedback/spec.md`
- Data Model: `specs/008-paddle-shrink-feedback/data-model.md`
- Implementation Plan: `specs/008-paddle-shrink-feedback/plan.md`
- Code: `src/systems/respawn.rs` (shrink system), `src/lib.rs` (component)
