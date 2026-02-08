# Quickstart: Direction Bricks Development

**Purpose**: Get direction bricks feature working in 5 phases **Target Audience**: Developers implementing direction bricks **Estimated Time**: 4-6 hours (TDD phase 1-3)

## Prerequisites

- Rust 1.81, Bevy 0.17.3, bevy_rapier3d 0.32.0 installed
- Project builds and tests pass: `cargo test`
- Familiarity with ECS systems, Observers, and Bevy messaging
- Understand existing brick destruction flow (see `src/systems/brick_effects.rs`)

## Phase 0: Set Up Feature Branch

```bash
# Already on branch 026-direction-bricks (verify)
git branch | grep 026-direction-bricks

# Pull latest changes
git pull origin develop
```

## Phase 1: Write Failing Tests (Red)

**Goal**: Create test files with all acceptance scenarios as failing tests

**File**: `tests/direction_bricks.rs`

```bash
# Create test file with skeleton
touch tests/direction_bricks.rs
```

**Minimal Test Structure** (reference [data-model.md](data-model.md) for expected values):

```rust
#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use brkrs::*;

    #[test]
    fn test_brick_43_down_impulse() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins)
            .add_plugins(BrkrsPlugins); // Your plugin list

        // Spawn ball with velocity (3.0, 2.0, 0.0)
        let ball = app.world_mut().spawn((
            Ball,
            LinearVelocity::new(Vec3::new(3.0, 2.0, 0.0)),
        )).id();

        // Trigger direction brick 43 effect
        app.world_mut().trigger(DirectionBrickEffect {
            ball_entity: ball,
            brick_type: 43,
            brick_position: Vec3::ZERO,
            velocity_before: Vec3::new(3.0, 2.0, 0.0),
        });

        // Update world to run observer
        app.update();

        // Verify velocity is (3.0, -3.0, 0.0)
        let velocity = app.world().get::<LinearVelocity>(ball).unwrap();
        assert!((velocity.linvel.x - 3.0).abs() < 0.001);
        assert!((velocity.linvel.y - (-3.0)).abs() < 0.001);
        assert!((velocity.linvel.z - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_brick_44_left_impulse() {
        // Similar structure for brick 44
        // Expected: (3.0, 2.0, 0.0) → (-2.0, 2.0, 0.0)
    }

    // ... tests for bricks 45, 46, 47, 48

    #[test]
    fn test_brick_52_randomizer_magnitude() {
        // Test magnitude is in 5.0..=15.0 range
        // Run multiple times to verify randomness
    }

    #[test]
    fn test_velocity_persists_10_frames() {
        // Spawn ball, apply impulse, run 10+ app.update() cycles
        // Verify velocity doesn't reset or get overwritten
    }
}
```

**Run Tests** (should fail):

```bash
cargo test direction_bricks -- --nocapture
# All tests should FAIL (red phase)
```

**Commit Red Phase**:

```bash
git add tests/direction_bricks.rs
git commit -m "tests: Add failing tests for direction bricks (red phase)"
```

## Phase 2: Implement Observer System (Green)

**Goal**: Make tests pass by implementing direction brick observer

**File**: `src/systems/brick_effects.rs` (new file)

**Minimal Implementation**:

```rust
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Define trigger type
#[derive(Event)]
pub struct DirectionBrickEffect {
    pub ball_entity: Entity,
    pub brick_type: u32,
    pub brick_position: Vec3,
    pub velocity_before: Vec3,
}

// Observer system
pub fn apply_direction_brick_effects(
    trigger: Trigger<DirectionBrickEffect>,
    mut query: Query<&mut LinearVelocity, With<Ball>>,
) {
    let ball_entity = trigger.event().ball_entity;
    let brick_type = trigger.event().brick_type;

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
            52 => apply_randomizer(&mut velocity),
            _ => {}
        }
    }
}

fn apply_randomizer(velocity: &mut LinearVelocity) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let magnitude = rng.gen_range(5.0..=15.0);
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);

    velocity.linvel.x = magnitude * angle.cos();
    velocity.linvel.y = magnitude * angle.sin();
    // Z unchanged
}
```

**Register Observer in Plugin**:

```rust
// In your plugin (src/lib.rs or main.rs)
app.add_observer(apply_direction_brick_effects);
```

**Run Tests** (should pass):

```bash
cargo test direction_bricks -- --nocapture
# All tests should PASS (green phase)
```

**Commit Green Phase**:

```bash
git add src/systems/brick_effects.rs
git commit -m "feat: Implement direction brick observer system (green phase)"
```

## Phase 3: Add Tracing & Integrate Scoring (Refactor)

**Goal**: Add observability and scoring integration

**Update Observer**:

```rust
pub fn apply_direction_brick_effects(
    trigger: Trigger<DirectionBrickEffect>,
    mut query: Query<&mut LinearVelocity, With<Ball>>,
) {
    let event = trigger.event();
    let ball_entity = event.ball_entity;
    let brick_type = event.brick_type;
    let velocity_before = event.velocity_before;

    if let Ok(mut velocity) = query.get_mut(ball_entity) {
        // [Apply impulse as before]
        match brick_type {
            // ... same as above
        }

        // Emit tracing span
        let points = match brick_type {
            43..=46 => 75,
            47..=48 => 100,
            52 => 125,
            _ => 0,
        };

        tracing::info_span!(
            "direction_brick_effect",
            ?brick_type,
            ?ball_entity,
            velocity_before = ?velocity_before,
            velocity_after = ?velocity.linvel,
            points = points,
        ).in_scope(|| {
            tracing::info!("Direction brick impulse applied");
        });
    }
}
```

**Scoring Integration** (in existing `src/systems/scoring.rs`):

Add to brick type match:

```rust
match brick_type {
    43 | 44 | 45 | 46 => 75,
    47 | 48 => 100,
    52 => 125,
    // ... existing types
    _ => 0,
}
```

**Run Tests & Full Test Suite**:

```bash
cargo test
# All tests should pass, including multi-frame persistence
```

**Commit Refactor Phase**:

```bash
git add src/systems/brick_effects.rs src/systems/scoring.rs
git commit -m "refactor: Add tracing instrumentation and scoring integration"
```

## Phase 4: Create Test Levels

**Goal**: Add test levels with direction bricks

**File**: `assets/levels/test_direction_bricks.ron`

```ron
(
  background: "background.png",
  bricks: [
    // Cardinal impulses
    (position: (1.0, 5.0, 10.0), brick_type: 43),
    (position: (3.0, 5.0, 10.0), brick_type: 44),
    (position: (5.0, 5.0, 10.0), brick_type: 45),
    (position: (7.0, 5.0, 10.0), brick_type: 46),
    // Diagonals
    (position: (2.0, 5.0, 12.0), brick_type: 47),
    (position: (6.0, 5.0, 12.0), brick_type: 48),
    // Randomizer
    (position: (4.0, 5.0, 14.0), brick_type: 52),
  ],
)
```

**Verify Level Loads**:

```bash
# Run game and load level
cargo run -- --level test_direction_bricks
```

## Phase 5: Acceptance Testing

**Goal**: Validate all acceptance scenarios manually and programmatically

**Checklist**:

- [ ] Brick 43 decreases Y-velocity (ball bounces down)
- [ ] Brick 44 decreases X-velocity (ball moves left)
- [ ] Brick 45 increases X-velocity (ball moves right)
- [ ] Brick 46 increases Y-velocity (ball bounces up)
- [ ] Brick 47 increases both X and Y (diagonal up-right)
- [ ] Brick 48 decreases X, increases Y (diagonal up-left)
- [ ] Brick 52 produces random velocity (different each time)
- [ ] Velocity changes persist 10+ frames (test passes)
- [ ] Multiple bricks in sequence stack correctly (test passes)
- [ ] Scoring awards correct points (75, 100, 125)
- [ ] Tracing spans appear in test output
- [ ] No regression in existing brick types (all tests pass)

**Run Full Test Suite**:

```bash
cargo test
cargo test -- --nocapture direction_bricks  # With logging
```

## File Structure Reference

**After completion**:

```text
specs/026-direction-bricks/
├── spec.md              # Feature specification
├── plan.md              # Implementation plan
├── data-model.md        # Data model & examples
├── quickstart.md        # This file
└── ...

src/systems/
├── brick_effects.rs     # NEW: Direction brick observer
├── scoring.rs           # MODIFIED: Add direction brick scoring
└── ...

tests/
├── direction_bricks.rs  # NEW: Direction brick tests
└── ...

assets/levels/
└── test_direction_bricks.ron  # NEW: Test level
```

## Troubleshooting

**Tests fail with "DirectionBrickEffect not found"**:

- Ensure `DirectionBrickEffect` is defined as `#[derive(Event)]`
- Verify observer is registered: `app.add_observer(apply_direction_brick_effects)`

**Velocity not changing**:

- Verify `LinearVelocity` component exists on ball entity
- Check brick type matches (43-48 or 52)
- Verify observer is receiving trigger event (check tracing output)

**Randomizer always produces same value**:

- Expected if using seeded RNG in tests; use thread RNG for variety
- Tests should accept any magnitude in 5.0..=15.0 range

**Scoring not updating**:

- Ensure `BrickDestroyed` message is emitted alongside direction brick effect
- Verify scoring system reads correct brick type in match statement

## Next Steps

1. Implement Phase 1-3 (TDD: Red → Green → Refactor)
2. Create test levels (Phase 4)
3. Run full acceptance test suite (Phase 5)
4. Submit for review: `git push origin 026-direction-bricks`
5. Create pull request linking to spec and plan documents

**Support**: See [plan.md](plan.md) for detailed technical context and [data-model.md](data-model.md) for expected behavior.
