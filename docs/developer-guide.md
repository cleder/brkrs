# Developer Guide

This guide helps you set up a development environment, understand the codebase, and contribute to brkrs.

## Prerequisites

Before you start developing, ensure you have:

- **Rust toolchain** (1.81+) via [rustup](https://rustup.rs/)
- **Git** for version control
- **A code editor** (VS Code with rust-analyzer recommended)

See {doc}`quickstart` for platform-specific dependencies.

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

Fast compilation, includes debug assertions. Use for day-to-day development.

### Release build

```bash
cargo run --release
```

Optimized build with better performance. Use for testing gameplay feel.

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

## Code quality checks

Before submitting a PR, run all quality checks:

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

All checks must pass for PR approval.

## Adding content

### Adding a new level

1. Create a new RON file in `assets/levels/`:

   ```bash
   cp assets/levels/level_001.ron assets/levels/level_003.ron
   ```

2. Edit the file with your level design (see {doc}`asset-format` for details)

3. Test locally:

   ```bash
   BK_LEVEL=3 cargo run
   ```

4. Run migration checks:

   ```bash
   ./scripts/migrate-assets.sh --check assets/levels/level_003.ron
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
|--------|---------|
| Physics (Rapier3D) | Collision detection, physics simulation |
| Level Loader | Parse RON files, spawn entities |
| Pause System | Game state management |
| Respawn | Ball respawn after falling |

See {doc}`architecture` for a detailed breakdown.

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

Assets support hot reloading in debug builds. Edit a level file and see changes immediately.

## Getting help

- **Issues**: [GitHub Issues](https://github.com/cleder/brkrs/issues)
- **Documentation**: This site
- **Code**: Read the source — it's well-documented
