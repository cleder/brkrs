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

Read through the official [SpecKit quickstart documentation](https://github.github.io/spec-kit/quickstart.html).
You will need to follow the whole process (with the exception of `/speckit.constitution`, the constitution exist).

### Start using slash commands with your AI agent

1. `/speckit.constitution` - Establish project principles (already done, but can be used to amend the constition)
2. `/speckit.specify` - Create baseline specification
3. `/speckit.plan` - Create implementation plan
4. `/speckit.tasks` - Generate actionable tasks
5. `/speckit.implement` - Execute implementation

### Optional commands that you can use for your specs (improve quality & confidence)

- `/speckit.clarify` (optional) - Ask structured questions to de-risk ambiguous areas before planning (run before `/speckit.plan` if used)
- `/speckit.analyze` (optional) - Cross-artifact consistency & alignment report (after `/speckit.tasks`, before `/speckit.implement`)
- `/speckit.checklist` (optional) - Generate quality checklists to validate requirements completeness, clarity, and consistency (after
  `/speckit.plan`)

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
- Level-control keys (R = respawn, N = next level, P = previous level) are gated to Cheat Mode: they only execute when Cheat Mode is active.
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

### Event handling with observers

Bevy 0.17 uses the **observer pattern** for custom events:

```rust
#[derive(Event)]
pub struct MyEvent { /* fields */ }

pub fn my_observer(trigger: On<MyEvent>) {
    let event = trigger.event();
    // Handle event
}

// In app setup:
app.add_observer(my_observer);
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
// In your system
commands.spawn(RapierDebugRenderPlugin::default());
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

Bevy 0.17 uses the **observer pattern** for custom events.
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
