# Quickstart: Brkrs Complete Game

**Feature**: 001-complete-game **Created**: 2025-10-31 **Purpose**: Build, run, and test the Brkrs game

## Prerequisites

- **Rust**: 1.75 or later (2021 edition)
- **System Dependencies**:
  - Linux: `sudo apt install libasound2-dev libudev-dev`
  - macOS: None (provided by Xcode)
  - Windows: None (provided by MSVC)
- **Optional**: `wasm-pack` for WASM builds: `cargo install wasm-pack`

## Initial Setup

1. **Clone the repository** (if not already done):

   ```bash
   git clone <repository-url>
   cd brkrs
   ```

2. **Checkout the feature branch**:

   ```bash
   git checkout 001-complete-game
   ```

3. **Verify Rust version**:

   ```bash
   rustc --version  # Should be 1.75+
   ```

## Building

### Native Build (Debug)

Fast compilation for development:

```bash
cargo build
```

Build artifacts: `target/debug/brkrs`

**Build Time**: ~30s incremental, ~5min clean build (with opt-level=3 for deps)

### Native Build (Release)

Optimized for performance:

```bash
cargo build --release
```

Build artifacts: `target/release/brkrs`

**Build Time**: ~10min (aggressive optimization)

### WASM Build

Build for web deployment:

```bash
cargo build --release --target wasm32-unknown-unknown
```

Build artifacts: `target/wasm32-unknown-unknown/release/brkrs.wasm`

**Optional Size Optimization**:

```bash
wasm-opt -Oz -o brkrs_optimized.wasm \
    target/wasm32-unknown-unknown/release/brkrs.wasm
```

## Running

### Native Execution

Run directly (debug build with fast deps):

```bash
cargo run
```

Or run the compiled binary:

```bash
./target/debug/brkrs        # Debug
./target/release/brkrs      # Release
```

**Window Behavior**: The game launches in borderless fullscreen mode by default for an immersive experience.
Press Escape to unlock the cursor and access window controls.
Press Alt+Enter to toggle between windowed and fullscreen modes (native platforms).

### WASM Execution

1. **Start a local web server**:

   ```bash
   # Using Python
   python3 -m http.server 8080

   # Or using a simple HTTP server
   cargo install basic-http-server
   basic-http-server .
   ```

2. **Navigate to** `http://localhost:8080/wasm/index.html`

## Controls

### In-Game

- **Mouse Movement**: Move paddle in X and Z directions
- **Mouse Wheel**: Rotate paddle
- **Left Click**: Lock cursor for gameplay
- **Escape**: Unlock cursor / Pause game
- **Space**: Toggle wireframe mode (native only)

### Development

- **Space**: Toggle wireframe mode (native only) - Also shows 22x22 debug grid overlay
- **F3**: Toggle FPS counter (if implemented)
- **F5**: Reload current level
- **F12**: Toggle debug physics rendering

**Debug Grid Overlay**: When wireframe mode is enabled (Space key on native platforms), a 22x22 grid overlay becomes visible.
This grid aligns with the game's logical grid cells and helps with:

- Verifying brick alignment
- Debugging paddle/ball positioning
- Understanding coordinate mapping
- Visual confirmation of play area boundaries

The grid is automatically hidden when wireframe mode is disabled.
Note: WASM builds do not support wireframe mode, so the grid overlay remains hidden.

## Testing

### Manual Testing

1. **Basic Gameplay** (User Story 1):

   ```bash
   cargo run
   ```

- Move mouse → paddle moves
- Scroll wheel → paddle rotates
- Ball bounces off paddle/walls
- Ball destroys bricks
- Ball moving past lower boundary → life lost

1. **Game States** (User Story 2):

- Start game from menu
- Press Escape → pause menu
- Resume game
- Lose all lives → game over screen
- Complete level → level transition

1. **Brick Types** (User Story 3):

- Hit multi-hit brick multiple times
- Observe different brick behaviors
- Verify brick destruction

1. **Cross-Platform** (User Story 5):

   ```bash
   # Native
   cargo run

   # WASM
   cargo build --target wasm32-unknown-unknown
   # Then open in browser
   ```

### Performance Testing

Verify 60 FPS target:

```bash
cargo run --release
# Monitor FPS display or use system tools
```

**Profiling** (if performance issues):

```bash
cargo install cargo-flamegraph
cargo flamegraph --root
# Generates flamegraph.svg
```

## Development Workflow

### Incremental Development

1. **Make changes** in `src/`
2. **Run**:

   ```bash
   cargo run
   ```

3. **Observe behavior** in game
4. **Iterate**

**Fast Recompilation**: Debug build with `opt-level=3` for dependencies provides good runtime performance with fast compile times.

### Code Formatting

Format before commits:

```bash
cargo fmt
```

### Linting

Check for common issues:

```bash
cargo clippy -- -D warnings
```

### Documentation

Generate and view docs:

```bash
cargo doc --open
```

## Troubleshooting

### Compilation Errors

**Error**: "failed to run custom build command for `alsa-sys`"

**Solution**: Install ALSA development libraries (Linux only):

```bash
sudo apt install libasound2-dev
```

**Error**: "linker `cc` not found"

**Solution**: Install C compiler:

```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora
sudo dnf install gcc
```

### Runtime Errors

**Issue**: Black screen / no rendering

**Solution**: Update graphics drivers, verify GPU supports Vulkan/Metal/DX12

**Issue**: Low FPS

**Solutions**:

- Use release build: `cargo run --release`
- Check system resources (CPU/GPU usage)
- Verify no background processes interfering

**Issue**: Mouse not captured

**Solution**: Click in game window, ensure window has focus

### WASM Issues

**Issue**: "Module not found"

**Solution**: Ensure serving from correct directory with `wasm/` folder

**Issue**: Very slow loading

**Solution**: Use `wasm-opt` to compress binary

**Issue**: Audio not working

**Solution**: User interaction required before audio starts (browser security)

## File Locations

### Source Code

- `src/main.rs` - Application entry point
- `src/components/` - ECS component definitions
- `src/systems/` - Game logic systems
- `src/events/` - Custom events
- `src/plugins/` - Feature modules

### Assets

- `assets/levels/` - Level definition files (RON format)
- `assets/models/` - 3D models (GLBB format)
- `assets/textures/` - Texture images

### Build Outputs

- `target/debug/` - Debug builds
- `target/release/` - Release builds
- `target/wasm32-unknown-unknown/` - WASM builds

## Configuration

### Physics Tuning

Edit `src/resources/game_config.rs` to adjust:

- Ball speed
- Paddle sensitivity
- Steering strength ("english")
- Gravity direction/magnitude

### Level Editing

Modify RON files in `assets/levels/`:

```ron
// assets/levels/level_001.ron
Level(
    number: 1,
    grid: Grid(width: 22, height: 22),
    bricks: [
        Brick(pos: (0, 0), type: Standard),
        Brick(pos: (1, 0), type: MultiHit(durability: 2)),
        // Add more bricks...
    ],
)
```

## Next Steps

1. Read `data-model.md` for ECS component structure
2. Review `contracts/events.md` for event patterns
3. See `plan.md` for full architecture overview
4. Start implementing User Story 1 (basic gameplay)

## Performance Targets

- **Native**: 60 FPS on modern desktop (last 5 years)
- **WASM**: 60 FPS on moderate hardware (Chrome/Firefox)
- **Input Latency**: <100ms mouse → paddle movement
- **Build Time**: <30s incremental compilation

## Support

For issues or questions, refer to:

- Bevy documentation: <https://bevyengine.org/learn/>
- Rapier3D documentation: <https://rapier.rs/docs/>
- Project constitution: `.specify/memory/constitution.md`
