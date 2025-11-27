# Phase 5 Testing Guide: Level Transition Sequence Control

## Objective

Verify that level transitions now show bricks BEFORE ball physics activate, fixing the issue where the ball would fall through an empty field during level transitions.

## Prerequisites

- ✅ Phase 3 complete (20x20 grid format)
- ✅ Phase 4 complete (grid overlay verified)
- ✅ Phase 5 implementation complete (brick spawning before delay)
- ✅ Compilation verified (`cargo check` passes)
- ✅ All tests pass (`cargo test` passes)

## Implementation Changes

### What Changed

1. **Fade sequence**: Fade OUT on current level → spawn new level at peak → fade IN on new level
2. **Brick spawning timing**: Bricks spawn at PEAK of fade (when screen is fully black)
3. **Ball physics**: Ball spawns with `GravityScale(0.0)` and `BallFrozen` marker
4. **Physics activation**: Ball physics activate AFTER paddle growth completes
5. **Paddle growth**: Paddle grows from tiny (0.01 scale) to full size (1.0 scale)

### System Execution Order

1. `advance_level_when_cleared()` - Detects cleared level, despawns paddle/ball, starts 1s fade-out
2. `handle_level_advance_delay()` - At peak of fade (1s later), spawns bricks + tiny paddle + frozen ball
3. `finalize_level_advance()` - After paddle growth (1s), unfreezes ball and activates physics

**Total transition time**: ~2 seconds (1s fade-out + 1s fade-in during paddle growth)

## Testing Procedure

### T049: Verify Bricks Appear During Fade-In

**Steps:**

1. Run the game: `cargo run`
2. Start playing level 1
3. Clear all bricks in level 1 (break every brick)
4. Observe the level transition to level 2

**Expected Behavior:**

- ✅ Screen fades to black on current level (old bricks still visible during fade-out)
- ✅ At peak of fade (full black), new level loads (bricks spawn but screen is black)
- ✅ Screen fades in from black, revealing new level with bricks already in place
- ✅ Bricks are visible when screen becomes visible (no empty field at any point)

**Actual Result:**

- [ ] PASS / [ ] FAIL
- Notes: **_****_****_**

---

### T050: Verify Ball Remains Stationary During Paddle Growth

**Steps:**

1. Continue observing the level 2 transition from T049
2. Watch the ball when paddle/ball spawn (after ~1 second)
3. Observe ball behavior during paddle growth animation

**Expected Behavior:**

- ✅ Ball spawns at its designated position
- ✅ Ball remains completely stationary (does not fall)
- ✅ Ball does not move during entire paddle growth phase (~1 second)
- ✅ Ball appears "frozen" in place

**Technical Details:**

- Ball has `BallFrozen` marker component
- Ball has `GravityScale(0.0)` initially
- These prevent physics from affecting the ball

**Actual Result:**

- [ ] PASS / [ ] FAIL
- Notes: **_****_****_**

---

### T051: Verify Ball Starts Falling Only After Paddle Reaches Full Size

**Steps:**

1. Continue observing from T050
2. Watch the paddle grow from tiny to full size
3. Observe when ball physics activate

**Expected Behavior:**

- ✅ Paddle grows smoothly from tiny (0.01 scale) to full (1.0 scale)
- ✅ Ball remains frozen throughout paddle growth
- ✅ Ball starts falling ONLY after paddle reaches full size
- ✅ Physics activate smoothly with no sudden jumps

**Technical Details:**

- `finalize_level_advance()` waits for `PaddleGrowing` to complete
- After growth completes:
  - Removes `BallFrozen` marker
  - Sets `GravityScale(1.0)`
  - Ball physics activate

**Actual Result:**

- [ ] PASS / [ ] FAIL
- Notes: **_****_****_**

---

### T052: Time Level Transition Duration

**Steps:**

1. Start a fresh game: `cargo run`
2. Prepare a stopwatch or timer
3. Clear all bricks in level 1
4. Start timing when the last brick is cleared
5. Stop timing when ball starts falling (physics activate)

**Expected Behavior:**

- ✅ Total transition duration ≤ 2 seconds
- ✅ Breakdown:
  - Brick spawn: immediate (0s)
  - Fade-in + delay: ~1 second
  - Paddle growth: ~1 second
  - Total: ~2 seconds

**Actual Timing:**

- Start: Last brick cleared
- Stop: Ball starts falling
- Duration: **_**__ seconds
- [ ] PASS (≤ 2.0s) / [ ] FAIL (> 2.0s)

---

## Visual Timeline

```text
Time 0.0s: Last brick cleared
         ↓
         • Paddle and ball despawn
         • Fade overlay starts (alpha 0 → 1)
         • Screen fades TO BLACK (old level still visible during fade)
         ↓
Time 1.0s: Peak of fade (alpha = 1.0, screen fully black)
         ↓
         • Bricks for level 2 spawn (not visible, screen is black)
         • Tiny paddle spawns (scale 0.01)
         • Ball spawns with BallFrozen + GravityScale(0.0)
         • Fade overlay starts fading IN (alpha 1 → 0)
         ↓
Time 2.0s: Paddle growth completes, fade-in completes
         ↓
         • Paddle reaches full size (scale 1.0)
         • BallFrozen removed from ball
         • GravityScale set to 1.0
         • Screen fully visible (alpha = 0)
         • Ball starts falling
         • GAMEPLAY READY
```

## Technical Verification

### Code Changes Summary

**File: src/level_loader.rs**

1. **advance_level_when_cleared()** (line ~730):

   ```rust
   // Despawn paddle & ball, start fade-out timer
   // Bricks spawn later at peak of fade
   level_advance.timer.reset();
   level_advance.active = true;
   ```

2. **handle_level_advance_delay()** (line ~878):

   ```rust
   // NEW: Spawn bricks at PEAK of fade (when screen is fully black)
   spawn_bricks_only(def, &mut commands, &mut meshes, &mut materials, ...);

   // THEN spawn paddle and ball
   // Ball spawns with physics disabled
   GravityScale(0.0),  // Changed from 1.0
   ```

3. **finalize_level_advance()** (line ~1018):

   ```rust
   // Activate ball physics after paddle growth completes
   for (entity, mut gravity_scale) in balls.iter_mut() {
       commands.entity(entity).remove::<crate::BallFrozen>();
       gravity_scale.0 = 1.0; // Activate gravity
   }
   ```

## Troubleshooting

### Bricks Not Visible During Transition

- **Symptom**: Empty field visible during fade-in
- **Check**: Verify `spawn_bricks_only()` called in `advance_level_when_cleared()`
- **Location**: Before `level_advance.timer.reset()`

### Ball Falls During Paddle Growth

- **Symptom**: Ball moves before paddle reaches full size
- **Check**: Verify `GravityScale(0.0)` in `handle_level_advance_delay()`
- **Check**: Verify `BallFrozen` marker present on ball

### Ball Never Unfreezes

- **Symptom**: Ball stays frozen after paddle growth
- **Check**: Verify `finalize_level_advance()` removes `BallFrozen`
- **Check**: Verify `GravityScale` set to 1.0 after growth

### Transition Takes Too Long

- **Symptom**: > 2 seconds from clear to gameplay
- **Check**: Timer duration in `LevelAdvanceState::default()` (should be 1.0s)
- **Check**: `PADDLE_GROWTH_DURATION` constant (should be 1.0s)

## Completion Criteria

Phase 5 is complete when ALL tests pass:

- ✅ T049: Bricks visible during fade-in (no empty field)
- ✅ T050: Ball stationary during paddle growth
- ✅ T051: Ball starts falling only after paddle full size
- ✅ T052: Total transition ≤ 2 seconds

After completion, update `specs/003-map-format/tasks.md`:

- Mark T031-T052 as `[x]`
- Add checkpoint note: "Phase 5 complete: Level transitions show bricks before ball physics"
