:orphan:
# Direction Brick Coordinate System

## Overview

Direction bricks (43-48, 52) apply impulses to the ball in specific directions based on Bevy's coordinate system.

## Bevy Coordinate System

- **X-axis**: Positive direction is forward (down the play field toward the goal)
- **Y-axis**: Positive direction is up (unused for horizontal movement)
- **Z-axis**: Positive direction is left, negative direction is right

## Direction Brick Mappings

All impulses have magnitude **5.0** units/second, except brick 52 (random) which ranges 5.0-15.0.

| Brick | Direction | Impulse Vector | Bevy Meaning |
|-------|-----------|---|---|
| 43 | Down | (+5.0, 0, 0) | Move down (toward goal) along +X |
| 44 | Left | (0, 0, +5.0) | Move left along +Z |
| 45 | Right | (0, 0, -5.0) | Move right along -Z |
| 46 | Up | (-5.0, 0, 0) | Move up (away from goal) along -X |
| 47 | Up-Right | (-5.0, 0, -5.0) | Move up and right (diagonal) |
| 48 | Up-Left | (-5.0, 0, +5.0) | Move up and left (diagonal) |
| 52 | Random | RNG(θ) at r∈[5,15] | Random direction in XZ plane |

## Random Brick Formula (Brick 52)

```rust
let magnitude = rng.random_range(5.0..15.0);  // [5.0, 15.0)
let angle = rng.random_range(0.0..TAU);       // [0, 2π)
impulse = Vec3::new(
    magnitude * angle.cos(),
    0.0,
    magnitude * angle.sin()
)
```

- Y component is always zero (no vertical impulse)
- Direction is uniformly random across 0-2π radians
- Magnitude varies from 5.0 to 15.0 units/sec

## Visual Layout (Top-Down View)

```text
                    [46] Up (-X)
                      ↑
                      |
      [48] Up-Left ↖   |   ↗ [47] Up-Right
              ↖    \  |  /    ↗
               \    \ | /    /
[44] Left ←─────────► ◯ ◄─────────→ [45] Right
               /    / | \    \
              ↙    /  |  \    ↘
      [44] Left  ↙    |    ↘ [45] Right
                      |
                      ↓
                   [43] Down (+X)
```

## Gameplay Context

When a ball collides with a direction brick:

1. The collision is detected by `mark_brick_on_ball_collision()`
2. A `DirectionBrickEffect` event is triggered with the impulse vector
3. The `apply_direction_brick_effects()` observer applies the impulse to the ball's `ExternalImpulse` component
4. The physics engine integrates the impulse in the next frame

## See Also

- [src/signals.rs](../src/signals.rs) - `DirectionBrickEffect` event definition
- [src/systems/brick_effects.rs](../src/systems/brick_effects.rs) - Observer implementation
- [tests/direction_bricks.rs](../tests/direction_bricks.rs) - Unit tests for impulse vectors
