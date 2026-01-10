# Quickstart: Gravity Switching Bricks

**Feature**: 020-gravity-bricks | **Date**: 2026-01-10 | **Phase**: 1 Design

## Feature at a Glance

Gravity bricks (indices 21-25) change the ball's physics gravity when destroyed.
This enables dynamic, challenging level design.

| Brick | Index | Effect | Score |
|-------|-------|--------|-------|
| Zero Gravity | 21 | Ball floats (gravity = 0) | 125 pts |
| 2G (Moon) | 22 | Light gravity like the Moon | 75 pts |
| 10G (Earth) | 23 | Normal Earth-like gravity | 125 pts |
| 20G (High) | 24 | Heavy, crushing gravity | 150 pts |
| Queer Gravity | 25 | Random directional gravity | 250 pts |

---

## Integration Points

### 1. Level Metadata (RON File)

To define a level's default gravity:

```ron
// assets/levels/level_1.ron
(
    name: "Level 1: Earth Gravity",
    bricks: [
        // ... your brick map ...
    ],
    default_gravity: Some((0.0, 10.0, 0.0)),  // ← Optional: Earth gravity
    // ... other fields ...
)
```

**If omitted**: Defaults to zero gravity `(0.0, 0.0, 0.0)`.

---

### 2. Create Gravity Bricks in Level Map

Place gravity bricks in your level's brick grid.
The map parser automatically detects indices 21-25 and creates entities with the `GravityBrick` component.

```rust
// In level map (inside bricks array):
[
    20, 20, 20,  // Regular stones
    21, 22, 23,  // Gravity bricks: Zero, 2G, 10G
    90, 90, 20,  // Solid + regular stone
    // ...
]
```

---

### 3. System Registration

The feature requires three systems to be registered in your Bevy app:

```rust
use bevy::prelude::*;
use brkrs::systems::gravity::*;  // Import gravity systems

fn main() {
    App::new()
        // ... existing plugins ...
        .add_systems(
            Update,
            (
                gravity_configuration_loader_system,  // Load level default gravity
                brick_destruction_gravity_handler,    // Detect and broadcast gravity changes
            ),
        )
        .add_systems(
            PhysicsUpdate,  // or appropriate schedule
            gravity_application_system,  // Apply gravity changes to physics
        )
        .add_systems(
            PostUpdate,
            gravity_reset_on_life_loss_system,  // Reset gravity on ball loss
        )
        .run();
}
```

---

### 4. Message System Setup

Register the `GravityChanged` message type:

```rust
use brkrs::systems::gravity::GravityChanged;

app.register_message::<GravityChanged>();
```

---

## Component Definitions

### GravityBrick Component

Automatically attached to brick entities with indices 21-25:

```rust
#[derive(Component)]
pub struct GravityBrick {
    pub index: u32,      // 21-25
    pub gravity: Vec3,   // (x, y, z) acceleration vector
}
```

### GravityConfiguration Resource

Singleton resource tracking gravity state:

```rust
#[derive(Resource)]
pub struct GravityConfiguration {
    pub current: Vec3,        // Currently applied gravity
    pub level_default: Vec3,  // Default gravity for reset
}
```

---

## Message Flow

**When a gravity brick is destroyed:**

1. `brick_destruction_gravity_handler` detects the destruction
2. Reads the brick's `GravityBrick` component
3. For Queer Gravity (25): generates random gravity via RNG
4. Writes `GravityChanged` message with the new gravity vector
5. `gravity_application_system` reads the message
6. Updates `GravityConfiguration::current`
7. Physics system applies the new gravity in the next frame

**When a ball is lost:**

1. `ball_lives_system` detects the loss
2. `gravity_reset_on_life_loss_system` resets gravity
3. Sets `GravityConfiguration::current = GravityConfiguration::level_default`
4. Next ball spawn uses the default gravity

---

## Physics Implementation Details

### Ball Physics Bodies

The ball is a Rapier 3D `RigidBody` with a `Velocity` component:

```rust
#[derive(Component)]
pub struct Ball { /* ... */ }

// Ball entity has:
// - RigidBody (Dynamic)
// - Velocity { linear, angular }
// - Collider (for ball shape)
```

### Applying Gravity

Gravity is applied via Rapier's external forces or `GravityScale` property:

```rust
// Option 1: Modify GravityScale (Rapier-native)
gravity_scale *= (gravity_vector / physics_default_gravity);

// Option 2: Apply direct force (more control)
commands.entity(ball).insert(ExternalForce::at_center(gravity_vector * mass));
```

**Implementation note**: Check `src/physics_config.rs` for existing gravity handling pattern.

---

## Test Coverage

Before implementation, write tests for:

### 1. Gravity Application Tests

```rust
#[test]
fn test_zero_gravity_stops_falling() {
    // Setup: Create ball with velocity, apply zero gravity
    // Assert: Ball maintains vertical velocity (no acceleration)
}

#[test]
fn test_earth_gravity_applies_force() {
    // Setup: Create ball at zero velocity, apply 10G gravity
    // Assert: Ball accelerates downward at 10 units/frame
}

#[test]
fn test_queer_gravity_within_ranges() {
    // Setup: Destroy Queer Gravity brick 5 times
    // Assert: All gravity values within specified ranges
}
```

### 2. Gravity Reset Tests

```rust
#[test]
fn test_gravity_resets_on_ball_loss() {
    // Setup: Change gravity to 20G, then lose ball
    // Assert: Gravity resets to level default before next ball
}

#[test]
fn test_default_gravity_zero_when_undefined() {
    // Setup: Load level without default_gravity field
    // Assert: GravityConfiguration::level_default == Vec3::ZERO
}
```

### 3. Message Flow Tests

```rust
#[test]
fn test_gravity_message_buffering() {
    // Setup: Destroy two gravity bricks in same frame
    // Assert: Both messages queued and processed in order
}

#[test]
fn test_last_gravity_wins() {
    // Setup: Destroy three gravity bricks (21, 24, 22) in sequence
    // Assert: Final gravity is from brick 22
}
```

### 4. Scoring Tests

```rust
#[test]
fn test_gravity_brick_scores() {
    // Assert: Brick 21 = 125 pts, 22 = 75 pts, etc.
}
```

---

## Common Patterns

### Accessing Current Gravity

```rust
// In a system that needs current gravity
fn my_system(gravity_config: Res<GravityConfiguration>) {
    let current_gravity = gravity_config.current;
    // Use gravity for calculations
}
```

### Writing a Gravity Change (for testing)

```rust
// In a system or test
fn trigger_gravity_change(mut gravity_writer: MessageWriter<GravityChanged>) {
    let new_gravity = Vec3::new(0.0, 20.0, 0.0);
    gravity_writer.send(GravityChanged { gravity: new_gravity });
}
```

### Detecting Gravity Changes (for logging/UI)

```rust
// In a system that monitors gravity updates
fn log_gravity_changes(mut gravity_reader: MessageReader<GravityChanged>) {
    for msg in gravity_reader.read() {
        println!("Gravity changed to: {:?}", msg.gravity);
    }
}
```

---

## Expected Performance Impact

- **CPU**: Minimal (message processing is O(n) where n = gravity bricks destroyed per frame, typically 0-2)
- **Memory**: One `GravityConfiguration` resource (32 bytes) + one `GravityBrick` component per gravity brick entity (16-20 bytes)
- **Physics**: No additional per-frame cost (gravity already computed by Rapier; just updating magnitude)

**Frame rate**: Should maintain 60 FPS with no performance degradation.

---

## Debugging Tips

### Gravity Not Changing?

1. ✅ Check level has gravity bricks (indices 21-25)
2. ✅ Verify brick destruction is detected (check brick destruction logs)
3. ✅ Ensure `gravity_application_system` is registered in correct schedule
4. ✅ Check `GravityChanged` message is registered

### Ball Floating in Wrong Direction?

1. ✅ Verify gravity vector sign (negative Y = down in Bevy)
2. ✅ Check physics schedule order (gravity should apply before physics simulate)
3. ✅ Inspect `GravityConfiguration::current` value in debugger

### Gravity Not Resetting After Ball Loss?

1. ✅ Verify ball loss detection system is running
2. ✅ Check `gravity_reset_on_life_loss_system` is registered
3. ✅ Inspect `GravityConfiguration::level_default` is set from level metadata

---

## Next Steps

1. **Write Tests**: Implement all test scenarios from "Test Coverage" section
2. **Implement Systems**: Code the three gravity systems
3. **Integration**: Test gravity mechanics with existing brick destruction and physics
4. **Level Design**: Create levels using gravity bricks to test gameplay
5. **Performance**: Profile and validate 60 FPS target

---

## Feature Flags & Configuration

No feature flags required.
Gravity mechanics are always enabled if systems are registered.

Optional: Add a `GravityBricksConfig` resource for tuning if needed:

```rust
#[derive(Resource)]
pub struct GravityBricksConfig {
    pub enable_gravity_bricks: bool,
    pub queer_gravity_rng_seed: Option<u64>,  // For reproducible testing
}
```

---

## References

- **Bevy 0.17.3 Documentation**: <https://docs.rs/bevy/0.17.3/>
- **Rapier 3D Integration**: Check `src/physics_config.rs` for gravity implementation
- **Message System**: See constitution at `.specify/memory/constitution.md`
- **Level Format**: See `src/level_format/` for RON parsing
- **Brick System**: See `src/systems/brick_destruction.rs` for existing patterns
