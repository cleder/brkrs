# Developer Guide

This guide helps you set up a development environment, understand the codebase, and contribute to brkrs.

## Prerequisites

Before you start developing, ensure you have:

- **Rust toolchain** (1.81+) via [rustup](https://rustup.rs/)
- **Git** for version control
- **A code editor** (VS Code with rust-analyzer recommended)

```{note}
See {doc}`quickstart` for platform-specific dependencies like build-essential on Linux
or Xcode Command Line Tools on macOS.
```

## SpecKit Quickstart

Read the official 📌 [SpecKit quickstart documentation](https://github.github.io/spec-kit/quickstart.html) ‼️

### Start using slash commands with your AI agent

The constitution ⚖️ is already established ✅.
See `.specify/memory/constitution.md` for the non-negotiable rules, including strict **TDD-first** and **Bevy 0.17 mandates & prohibitions**.

1. `/speckit.specify` - Create baseline specification. 👈 Describe in detail **what** feature you want to implement, NOT *how* (implementation details)
   - `/speckit.clarify` (optional) - Ask structured questions to de-risk ⚠️ ambiguous areas before planning (run before `/speckit.plan` if used)
2. `/speckit.plan` - Create implementation plan
   - `/speckit.checklist` (optional) - Generate quality checklists 📋 to validate requirements completeness, clarity, and consistency (after `/speckit.plan`)
3. `/speckit.tasks` - Generate actionable tasks 📝
   - `/speckit.analyze` (optional) - Cross-artifact consistency & alignment 🔎 report (after `/speckit.tasks`, before `/speckit.implement`)
4. `/speckit.implement` - Execute implementation 🔧

## Repository structure

```text
brkrs/
├── src/                    # Rust source code
│   ├── main.rs             # Application entry point
│   ├── lib.rs              # Library exports
│   ├── level_loader.rs     # Level file parsing and loading
│   ├── pause.rs            # Pause system implementation
│   ├── level_format/       # Level format definitions
│   ├── systems/            # Bevy ECS systems
│   │   ├── grid_debug.rs   # Debug grid visualization
│   │   ├── level_switch.rs # Level transition logic
│   │   ├── multi_hit.rs    # Multi-hit brick events and systems
│   │   ├── respawn.rs      # Ball respawn system
│   │   ├── spawning.rs     # Entity spawning (camera, light, ground)
│   │   └── textures/       # Texture loading systems
│   └── ui/                 # User interface components
│       ├── palette.rs      # Color palette
│       └── pause_overlay.rs # Pause menu overlay
├── assets/                 # Game assets
│   ├── levels/             # Level definition files (RON)
│   └── textures/           # Texture assets
├── tests/                  # Integration tests
├── docs/                   # Documentation (this site)
├── specs/                  # Feature specifications
├── scripts/                # Build and utility scripts
├── tools/                  # Development tools
└── wasm/                   # WASM build configuration
```

## Building and running

### Development build

```bash
cargo run
```

Fast compilation, includes debug assertions.
Use for day-to-day development.

### Release build

```bash
cargo run --release
```

Optimized build with better performance.
Use for testing gameplay feel.

### Running a specific level

```bash
BK_LEVEL=997 cargo run --release
```

### Coordinate System & Level Grid

```{seealso}
See {doc}`architecture` (Physics Architecture → Coordinate System) for the complete coordinate system reference, including Bevy's conventions vs. gameplay directions.
```

**Grid size**: `GRID_HEIGHT = 20`, `GRID_WIDTH = 20`; plane: `PLANE_H = 30.0` (X span), `PLANE_W = 40.0` (Z span); cell sizes: `CELL_HEIGHT = 1.5` (X), `CELL_WIDTH = 2.0` (Z).

**Mapping from RON matrix indices `(row, col)` to world space:**

- `x = -PLANE_H / 2.0 + (row + 0.5) * CELL_HEIGHT`
- `z =  PLANE_W / 2.0 - (col + 0.5) * CELL_WIDTH`
- `y = 2.0` (brick/merkaba height plane)

**Camera and axes**: Camera is top-down at `(0, 37, 0)` looking at the origin:

- **X axis** = left/right on screen (lateral movement)
- **Z axis** = up/down on screen (forward/backward from gameplay perspective)
- **Y axis** = height (locked for gameplay entities via `LockedAxes::TRANSLATION_LOCKED_Y`)

**Important**: Gameplay "forward" (+Z toward goal) differs from Bevy's `Transform::forward()` (-Z into screen).
Physics code uses direct axis manipulation (`velocity.linvel.z`), not Transform API semantics.

**Transform best practices**:

- Do not set `GlobalTransform` manually on spawn; set `Transform` only and let Bevy propagate.
- Avoid clamping Z for grid entities—columns legitimately vary along Z.

## Plugin Architecture

Brkrs uses a **plugin-based architecture** to organize systems and features, following Bevy's best practices for modularity and reusability.

### What is a Plugin?

A plugin is a self-contained struct that implements `Plugin` and registers related systems, resources, and schedules in the `build()` method.
Plugins enable developers to:

- Group related functionality together
- Avoid tight coupling between features
- Enable/disable features easily
- Test features independently

### Core Plugins

The main application (`src/lib.rs::run()`) registers these plugins:

| Plugin | Feature | Location |
|--------|---------|----------|
| `LevelSwitchPlugin` | Level transitions and progression | `src/systems/level_switch.rs` |
| `LevelLoaderPlugin` | Level loading and entity spawning | `src/level_loader.rs` |
| `RespawnPlugin` | Ball respawn mechanics | `src/systems/respawn.rs` |
| `PausePlugin` | Pause state and overlay UI | `src/pause.rs` |
| `AudioPlugin` | Sound effects and audio events | `src/systems/audio.rs` |
| `PaddleSizePlugin` | Paddle resize powerup effects | `src/systems/paddle_size.rs` |
| `CheatModePlugin` | Developer/testing cheat mode | `src/systems/cheat_mode.rs` |
| `TextureManifestPlugin` | Texture loading and overrides (optional, feature-gated) | `src/systems/textures/` |
| `FontsPlugin` | Font loading (desktop & WASM) | `src/ui/fonts.rs` |
| `UiPlugin` | UI systems (score, lives, overlays, palette) | `src/ui/mod.rs` |

### How to Create a New Plugin

**Example: Simple feature plugin**

```rust
pub struct MyFeaturePlugin;

impl Plugin for MyFeaturePlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<MyFeatureState>();

        // Register systems
        app.add_systems(
            Update,
            (
                my_input_system,
                my_logic_system.after(my_input_system),
                my_render_system.after(my_logic_system),
            )
        );
    }
}
```

Then register in `src/lib.rs`:

```rust
app.add_plugins(MyFeaturePlugin);
```

### Plugin Best Practices

- **Self-contained**: Each plugin should have minimal external dependencies
- **Resource naming**: Use unique resource names to avoid collisions
- **System ordering**: Use `after()` and `before()` to specify execution order
- **Error handling**: Follow Bevy 0.17 fallible systems pattern (return `()`, log errors)
- **Documentation**: Add module-level rustdoc explaining the plugin's purpose

## Running tests

### All tests

```bash
cargo test
```

### Specific test

```bash
cargo test test_name
```

### With output

```bash
cargo test -- --nocapture
```

### Single-threaded (for env var tests)

```bash
cargo test -- --test-threads=1
```

```{note}
Tests that use environment variables (like `BK_LEVEL`) can conflict when run
in parallel. Use `--test-threads=1` if you see flaky test failures.
```

### TDD workflow (required)

All implementation work follows strict Test-Driven Development (TDD):

1. Write unit/integration tests first.
2. Get tests validated/approved (by the feature owner/requestor).
3. Confirm tests fail (red).
4. Only then implement until tests pass (green).

### Testing physics features

When testing physics-related code, consider these patterns:

**Physics config validation:**

```rust
#[test]
fn ball_physics_config_validates_correctly() {
    let config = BallPhysicsConfig {
        restitution: 0.9,
        friction: 0.1,
        linear_damping: 0.5,
        angular_damping: 0.5,
    };
    assert!(config.validate().is_ok());
}
```

**Collision event testing:**

```rust
#[test]
fn ball_wall_collision_emits_event() {
    let mut app = App::new();
    // Set up physics world with ball and wall entities
    // Trigger collision
    // Assert BallWallHit event is emitted
}
```

**Integration testing:**

Use `tests/integration/` for full physics simulation tests.
These require:

- Proper Bevy app setup with physics plugins
- Entity spawning with correct components
- Frame stepping to process physics

## Code quality checks

Before submitting a PR, run all quality checks:

```{warning}
All checks must pass before your PR can be merged. The CI will run these
automatically, but running them locally saves time.
```

```bash
# Format code
cargo fmt --all

# Run lints
cargo clippy --all-targets --all-features

# Bevy-specific lints
bevy lint

# Run tests
cargo test
```

## Cheat Mode (developer/testing)

Cheat Mode is a testing/developer feature that allows quick exploration and debugging of levels and mechanics.

### How to toggle

- Press `G` during active gameplay to toggle Cheat Mode on or off.

### Behavior

- When Cheat Mode is toggled (either on or off), the player's current **score** is reset to `0`.
- When Cheat Mode is enabled, a persistent image indicator appears in the lower-right corner of the screen (asset: `assets/textures/default/cheat-mode-128.png`) so the player knows the session is in cheat mode.
- Level-control keys (`R` = restart level, `N` = next level, `P` = previous level, `K` = destroy all bricks) and debug tools (`Space` = wireframe) are gated to Cheat Mode: they only execute when Cheat Mode is active.
  If they are pressed while Cheat Mode is inactive, a short soft UI beep plays and the action is ignored.
- If Cheat Mode is toggled while a **Game Over** overlay is active (i.e., the player has 0 lives), Cheat Mode activation will set `LivesState.lives_remaining` to `3` and remove the Game Over overlay so the player can resume play.
  Note: toggling Cheat Mode does **not** reload or reset the current level — gameplay resumes in-place with the level state unchanged.

### Notes & Testing

- Use Cheat Mode for rapid iteration or to explore levels without the normal gating of level-control keys.
- The feature is intended for debugging and testing; enable it intentionally — the UI indicates when it's active.
- Unit and integration tests for Cheat Mode are in `tests/cheat_mode.rs` and `tests/restart_cheat.rs`.

## Adding content

### Adding a new level

1. Create a new RON file in `assets/levels/`:

   ```bash
   cp assets/levels/level_001.ron assets/levels/level_003.ron
   ```

2. Edit the file with your level design and optional metadata:

   ```rust
   LevelDefinition(
     number: 3,
     description: Some(r#"
       Beginner tutorial level.

       Teaches basic paddle control and ball bouncing.
       Features a simple brick pattern for practice.
     "#),
     author: Some("[Jane Smith](mailto:jane@example.com)"),
     matrix: [
       // ... your level design
     ],
   )
   ```

   The `description` and `author` fields are optional but recommended for:
   - **Description**: Document design intent, gameplay mechanics, or technical notes
   - **Author**: Credit contributors with plain text or Markdown links

3. Test locally:

   ```bash
   BK_LEVEL=3 cargo run
   ```

### Adding textures

1. Place texture files in `assets/textures/`
2. Update the texture manifest in `assets/textures/manifest.ron`
3. See `assets/textures/README.md` for naming conventions

## Architecture overview

brkrs follows Bevy's Entity-Component-System (ECS) architecture:

- **Entities**: Game objects (paddle, ball, bricks, walls)
- **Components**: Data attached to entities (position, velocity, brick type)
- **Systems**: Logic that operates on components (physics, rendering, input)

Key systems:

| System | Purpose |
|--------|----------|
| Physics (Rapier3D) | Collision detection, physics simulation |
| Level Loader | Parse RON files, spawn entities |
| Pause System | Game state management |
| Respawn | Ball respawn after falling |
| Multi-Hit | Brick damage states, material transitions |
| Textures | Asset loading and material management |

### Physics Configuration System

brkrs uses a centralized physics configuration system to ensure consistent physics behavior across all entities.
Instead of hardcoding physics values in spawn functions, all physics properties are defined in dedicated resource structs.

**Core Resources:**

- `BallPhysicsConfig` — Controls ball bounciness, friction, and damping
- `PaddlePhysicsConfig` — Controls paddle physics properties
- `BrickPhysicsConfig` — Controls brick collision properties

**Usage in Spawn Systems:**

```rust
fn spawn_ball(
    mut commands: Commands,
    ball_config: Res<BallPhysicsConfig>, // Inject the config
) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        Restitution::coefficient(ball_config.restitution), // Use config values
        Friction::coefficient(ball_config.friction),
        Damping {
            linear_damping: ball_config.linear_damping,
            angular_damping: ball_config.angular_damping,
        },
    ));
}
```

**Validation:** All config structs provide a `validate()` method that checks for reasonable physics values and prevents runtime errors.

**Tuning:** Modify the config resources in `src/physics_config.rs` to adjust gameplay feel.
Changes apply to all newly spawned entities.

### Collision Events

Collision detection in brkrs uses Rapier3D's event system.
For collision events to be generated, **both colliding entities must have `ActiveEvents::COLLISION_EVENTS`**.

```rust
// Both entities need this for collision events
ActiveEvents::COLLISION_EVENTS
```

**Collision Event Flow:**

1. Entities with `ActiveEvents::COLLISION_EVENTS` generate `CollisionEvent`s when they collide
2. Systems read `CollisionEvent`s via `MessageReader<CollisionEvent>`
3. Events trigger game logic (audio, scoring, destruction)

**Common Issues:**

- **No collision events?**
  Check that both entities have `ActiveEvents::COLLISION_EVENTS`
- **Missing RigidBody?**
  Only entities with physics bodies can generate collision events
- **Timing issues?**
  Collision events are processed in the physics update loop

### Multi-Hit Bricks

Multi-hit bricks (indices 10-13) require multiple ball collisions to destroy.
Each hit transitions the brick to a lower index until it becomes a simple stone (index 20), which can then be destroyed.

**Lifecycle**: `13 → 12 → 11 → 10 → 20 → destroyed`

The `MultiHitBrickHit` event is emitted on each transition, allowing systems to react for audio feedback or scoring:

```rust
use brkrs::systems::multi_hit::MultiHitBrickHit;

fn on_brick_hit(trigger: On<MultiHitBrickHit>) {
    let event = trigger.event();
    info!("Brick hit: {} → {}", event.previous_type, event.new_type);
    // Play sound, update score, etc.
}
```

brkrs uses two distinct signalling patterns.
They are not interchangeable: brkrs uses two distinct signalling patterns, which are **not interchangeable**:

### Messages vs Observers (Bevy 0.17+)

See the constitution's "Bevy 0.17 Event, Message, and Observer Clarification" for the full authoritative explanation.

- **Messages** (`#[derive(Message)]`) are for double-buffered, frame-agnostic data streams (e.g., scoring, telemetry).
  Produced via `MessageWriter`, consumed via `MessageReader`.
  Use for batchable or delayed work, not for immediate side-effects.
- **Observers** (with `#[derive(Event)]`, `On<T>`, `Trigger<T>`, or observer systems) are for immediate or next-frame reactions (e.g., UI, sound, spawning).
  Use for real-time, reactive logic that needs full system access and instant feedback.

**Key rules:**

- Use Messages for batchable, cross-frame work; Observers for instant, reactive logic.
- Never create observer systems that listen to Messages; only Events/Triggers are valid for observers.
- Always justify your choice in specs/plans (see constitution for rationale and examples).
>
> - **Never** create observer systems that listen to Messages; only Events/Triggers are valid for observers.

#### Events (immediate, observer pattern)

Use the observer pattern for any logic that must react immediately (e.g., play a sound, update UI):

```rust
#[derive(Event)]
pub struct MyEvent { /* fields */ }

pub fn my_observer(trigger: On<MyEvent>) {
   let event = trigger.event();
   // Handle event (immediate side-effect)
}

// In app setup:
app.add_observer(my_observer);
```

#### Messages (buffered, frame-agnostic)

For buffered, frame-agnostic data, use `MessageReader`/ `MessageWriter`.
**Do not perform immediate side-effects in the same system that writes messages.**

```rust
use bevy::ecs::message::{Message, MessageReader, MessageWriter};
use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy)]
pub struct BrickDestroyed {
   pub brick_entity: Entity,
   pub brick_type: u8,
}

fn award_points(
   mut destroyed: MessageReader<BrickDestroyed>,
) -> Result<(), ()> {
   for msg in destroyed.read() {
      debug!(?msg, "Award points");
      // Do NOT trigger sounds or UI here; use an Event for that.
   }
   Ok(())
}
```

See {doc}`architecture` for a detailed breakdown.

Note: The audio observers for multi-hit brick events have been centralized in the audio system.
If you are following older documentation that references a placeholder observer in `systems::multi_hit`, see `systems::audio::on_multi_hit_brick_sound` for the current implementation.

## Development workflow

1. **Create a feature branch**:

   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make changes** and test locally

3. **Run quality checks** (format, lint, test)

4. **Commit with descriptive message**:

   ```bash
   git commit -m "feat: add new brick type with special behavior"
   ```

5. **Push and open a PR**

See {doc}`contributing` for the full contribution workflow.

## Common development tasks

### Debugging physics

```{tip}
Physics debug rendering is invaluable for understanding collision issues.
Enable it when ball behavior seems unexpected.
```

Enable physics debug rendering:

```rust
// In app/plugin setup
app.add_plugins(RapierDebugRenderPlugin::default());
```

### Inspecting entities

Use Bevy's built-in inspector or add logging:

```rust
fn debug_system(query: Query<(Entity, &Transform), With<Brick>>) {
    for (entity, transform) in query.iter() {
        info!("Brick {:?} at {:?}", entity, transform.translation);
    }
}
```

### Hot reloading assets

```{important}
Hot reloading only works in debug builds. Release builds bake assets
at compile time for performance.
```

Assets support hot reloading in debug builds.
Edit a level file and see changes immediately.

### Writing doctests

```{note}
Bevy-dependent doctests often fail in CI due to shared library loading issues.
Use `no_run` to compile-check without executing:

\`\`\`rust,no_run
use bevy::prelude::*;
// Your example code
\`\`\`
```

### Working with Events and Observers

Bevy 0.17 uses the **observer pattern** for `Event` types.
For buffered communication between systems, use `Message` types.

Events are structs that derive `Event`:

```rust
#[derive(Event)]
pub struct MyEvent {
    pub data: String,
}
```

Create observers to react to events:

```rust
fn my_observer(trigger: On<MyEvent>) {
    let event = trigger.event();
    // Handle the event
}
```

Register observers in your plugin:

```rust
app.add_observer(my_observer);
```

Events can be emitted from systems:

```rust
commands.trigger(MyEvent { data: "hello".to_string() });
```

See the multi-hit brick system for a complete example.

## Building for WASM

```{warning}
WASM builds have different asset loading requirements than desktop builds.
Assets should be embedded at compile time.
```

### Building the WASM binary

```bash
# Install target if needed
rustup target add wasm32-unknown-unknown

# Build release WASM
cargo build --target wasm32-unknown-unknown --release

# Generate JS bindings
wasm-bindgen --out-dir wasm --target web \
  target/wasm32-unknown-unknown/release/brkrs.wasm
```

### Platform differences

| Feature | Desktop | WASM |
|---------|---------|------|
| Asset Loading | Synchronous from filesystem | Asynchronous via HTTP |
| Level Loading | Read from `assets/levels/*.ron` | Embedded at compile time |
| Font Loading | Startup schedule | Deferred to Update schedule |
| Binary Size | ~20MB (debug) | ~88MB (includes embedded levels) |

### Debugging WASM builds

```{tip}
Use browser DevTools (F12) to inspect console errors.
```

**Common issues**:

1. **Levels don't load**: Not embedded → Update `embedded_level_str()` in `level_loader.rs`
2. **Performance issues**: Large binary size → Consider on-demand HTTP fetching for levels

- **Issues**: [GitHub Issues](https://github.com/cleder/brkrs/issues)
- **Documentation**: This site and the {doc}`api-reference`
- **Code**: Read the source — it's well-documented
