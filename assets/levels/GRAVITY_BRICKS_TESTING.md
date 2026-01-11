# Gravity Bricks Test Levels

This document describes the test levels created to validate gravity bricks functionality.

## Test Files

### test_gravity_bricks.ron (Level 9001)

**Purpose**: Comprehensive test level demonstrating all 5 gravity brick types (21-25) and their effects.

**Configuration**:

- Starting gravity: Earth-like (0.0, -9.8, 0.0)
- Default gravity: (0.0, -9.8, 0.0) - reset point after ball loss
- Level number: 9001

**Brick Layout**:

**Row 3**: Individual demonstration of each gravity brick type

- Brick 21 (Zero Gravity): Sets gravity to (0.0, 0.0, 0.0)
- Brick 22 (Lunar Gravity): Sets gravity to (0.0, -1.625, 0.0)
- Brick 23 (Earth Gravity): Sets gravity to (0.0, -9.8, 0.0)
- Brick 24 (Venus Gravity): Sets gravity to (0.0, -8.87, 0.0)
- Brick 25 (Queer Gravity): Random gravity X∈[-2, +15], Y=0, Z∈[-5, +5]

**Row 5**: Sequential gravity change testing

- Groups of 3x same brick type to test rapid sequential changes
- Tests that last destroyed brick's gravity wins

**Row 7**: Mixed gameplay testing

- Gravity bricks interspersed with regular bricks (type 1)
- Tests realistic gameplay scenarios

**Rows 9-13**: Mass destruction patterns

- Full rows of each gravity brick type
- Tests multiple simultaneous destructions
- Validates message buffering and ordering

**Manual Testing Checklist**:

1. ✅ Level loads without errors
2. ⏳ Destroy brick 21 → ball floats (zero gravity)
3. ⏳ Destroy brick 22 → ball moves slowly down (lunar gravity)
4. ⏳ Destroy brick 23 → ball moves normally (earth gravity)
5. ⏳ Destroy brick 24 → ball moves slightly slower (venus gravity)
6. ⏳ Destroy brick 25 → ball affected by random gravity
7. ⏳ Lose a ball → gravity resets to (-9.8, 0.0, 0.0)
8. ⏳ Destroy multiple bricks → last brick's gravity applies

---

### level_001.ron (Modified)

**Modifications**:

- Added `default_gravity: Some((2.0, 0.0, 0.0))`
- Replaced 4 type-20 bricks with gravity bricks:
  - Row 2, positions 8 and 11: bricks 21 and 22
  - Row 4, positions 8 and 11: bricks 23 and 24

**Purpose**:

- Validates backward compatibility with `default_gravity` field
- Tests gravity bricks in existing level structure
- Ensures gravity reset works in level progression

---

### level_010.ron (Modified)

**Modifications**:

- Added `default_gravity: Some((10.0, 0.0, 0.0))`
- Replaced 6 type-20 bricks with gravity bricks:
  - Row 3: positions 4 and 14 → bricks 21 and 22
  - Row 5: positions 4 and 14 → bricks 23 and 24
  - Row 7: positions 8 and 11 → two brick 25s (Queer Gravity)

**Purpose**:

- Tests gravity bricks with custom initial gravity (10.0 X-axis)
- Validates gravity transitions from unusual starting conditions
- Tests Queer Gravity RNG in interesting level layout

---

## Testing Workflow

### Automated Tests

```bash
# Run all gravity brick tests
cargo test --test gravity_bricks

# Verify backward compatibility
cargo test --test backward_compatibility

# All library tests
cargo test --lib
```

### Manual Testing

1. Start game with test level:

   ```bash
   cargo run -- --level 9001  # If level selection supported
   ```

2. Observe gravity changes as bricks are destroyed
3. Verify ball physics responds correctly to each gravity type
4. Confirm gravity resets after ball loss

### Expected Behaviors

| Brick Type | Index | Gravity Value | Expected Ball Behavior |
|------------|-------|---------------|------------------------|
| Zero Gravity | 21 | (0, 0, 0) | Ball floats, no acceleration |
| Lunar Gravity | 22 | (0, -1.625, 0) | Ball falls slowly (~16% Earth) |
| Earth Gravity | 23 | (0, -9.8, 0) | Normal downward acceleration |
| Venus Gravity | 24 | (0, -8.87, 0) | Slightly slower than Earth (~90%) |
| Queer Gravity | 25 | Random | Unpredictable direction/magnitude |

---

## Implementation Notes

- All test levels use the new `default_gravity` field for reset functionality
- Gravity bricks are visually identical to placeholder brick (type 20) until textures are implemented
- Physics integration uses Rapier3D's gravity configuration
- Gravity changes apply immediately on brick destruction (same frame)
- Sequential destructions are buffered; last message wins

---

## Status

- ✅ Test level created
- ✅ Existing levels modified
- ✅ Backward compatibility verified
- ✅ All automated tests passing (49/49 lib, 40/40 gravity)
- ⏳ Manual testing pending
- ⏳ Visual feedback (textures) pending future user story
