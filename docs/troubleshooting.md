# Troubleshooting

Common issues and their solutions when running brkrs.

## Build Issues

### "linker not found" or missing system libraries

**Linux**: Install the required development packages:

```bash
# Ubuntu/Debian
sudo apt install build-essential pkg-config libasound2-dev libudev-dev

# Fedora
sudo dnf install gcc-c++ alsa-lib-devel systemd-devel
```

**macOS**: Install Xcode Command Line Tools:

```bash
xcode-select --install
```

**Windows**: Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with "Desktop development with C++".

### Compilation takes too long

The first build compiles all dependencies (Bevy, Rapier, etc.) which takes several minutes.
This is normal.

**Tips to speed up development builds**:

- Use `cargo run` (debug mode) during development — faster compilation
- Use `cargo run --release` only for final testing — slower compilation but better performance
- Enable dynamic linking for Bevy in development (see `Cargo.toml` for feature flags)

### Out of memory during compilation

Bevy is a large framework.
If you run out of RAM during compilation:

1. Close other applications
2. Reduce parallel compilation jobs:

   ```bash
   CARGO_BUILD_JOBS=2 cargo build
   ```

3. Add swap space (Linux)

## Runtime Issues

### Black screen or no window appears

**Check your graphics drivers**:

- brkrs requires OpenGL 3.3+ or Vulkan support
- Update your GPU drivers to the latest version
- On Linux, ensure you have the appropriate Mesa or proprietary drivers

**Try a different renderer** (Bevy environment variable):

```bash
WGPU_BACKEND=gl cargo run  # Force OpenGL
WGPU_BACKEND=vulkan cargo run  # Force Vulkan
```

### Game runs slowly or stutters

1. **Use release mode**: `cargo run --release`
2. **Check VSync**: The game targets 60 FPS with VSync enabled
3. **Integrated GPU**: If you have both integrated and discrete GPUs, ensure the game uses the discrete GPU

### Audio issues (no sound or crackling)

**Linux**: Ensure ALSA or PulseAudio is configured correctly:

```bash
# Check audio devices
aplay -l
```

**All platforms**: Audio is optional — the game runs without sound.

### Physics issues (ball doesn't bounce, no collisions)

**Enable physics debug rendering:**

```rust
// In src/lib.rs, temporarily uncomment this line in the app setup:
app.add_plugins(RapierDebugRenderPlugin::default());
```

This shows collision shapes, velocities, and contact points.

**Check physics configuration:**

- Verify `BallPhysicsConfig`, `PaddlePhysicsConfig`, and `BrickPhysicsConfig` have valid values
- Run `cargo run` and check for validation error messages at startup

### Paddle pushes "Ghost" objects or Hazards

If the paddle (Kinematic Character Controller) pushes objects that should be triggers/sensors (like Hazards), even if `SolverGroups` are set correctly:

1. **Check `CollisionGroups`**: The dynamic object must have a specific Group (e.g., `Group::GROUP_2`) assigned via `CollisionGroups`.
2. **Check Controller Filters**: The `KinematicCharacterController` has its own `filter_groups` property.
   It must be explicitly configured to ignore the dynamic object's group (e.g., `filter_groups: Some(CollisionGroups::new(Group::GROUP_1, Group::ALL ^ Group::GROUP_2))`).

Without this, the character controller's internal shape cast will detect the object as a default obstacle and resolve collision by pushing it.

- Common issues: `restitution > 2.0`, negative friction, or infinite damping values

**Collision event debugging:**

- Ensure both colliding entities have `ActiveEvents::COLLISION_EVENTS`
- Check that entities have appropriate `RigidBody` components (`Dynamic`, `Fixed`, etc.)
- Use logging in collision systems to verify events are being generated

**Ball physics problems:**

- Ball needs `Velocity::zero()` component at spawn (not just `RigidBody::Dynamic`)
- Check that `LockedAxes::TRANSLATION_LOCKED_Y` allows XZ movement
- Verify gravity settings in level files or `RapierConfiguration`

## WASM/Web Issues

### Web version doesn't load

- Use a modern browser (Chrome 80+, Firefox 75+, Safari 14+, Edge 80+)
- Ensure WebGL 2.0 is enabled in your browser
- Check the browser console (F12) for error messages

### Textures don't load in WASM build

```{important}
This is the most common WASM issue. Bevy's WASM asset loader requires explicit
metadata files for all assets loaded via HTTP.
```

**Symptoms**:

- Browser console shows 404 errors for `.meta` files (e.g., `brick_base.png.meta not found`)
- Textures appear black/missing
- Console shows "Failed to load asset" messages

**Solution**: Create `.meta` files for all PNG textures:

```bash
# Generate meta files for all textures
find assets/textures -name "*.png" -type f | while read png; do
  cat > "${png}.meta" << 'EOF'
(
    asset: Load(
        loader: "bevy_image::image_loader::ImageLoader",
        settings: (
            format: FromExtension,
            is_srgb: true,
            sampler: Default,
            asset_usage: 1,
        ),
    ),
)
EOF
done
```

**Deploy checklist**:

- [ ] Copy both `.png` and `.png.meta` files to web server
- [ ] Preserve directory structure
- [ ] Clear browser cache (Ctrl+Shift+R)
- [ ] Check browser console for 404 errors

### "Failed to deserialize meta" errors

**Symptoms**:

- Console shows: `Failed to deserialize meta for asset textures/...`
- SpannedError with code like `ExpectedString` or missing field names

**Cause**: Incorrect `.meta` file format or syntax error in RON file.

**Solution**: Verify `.meta` file format matches exactly:

```rust
(
    asset: Load(
        loader: "bevy_image::image_loader::ImageLoader",
        settings: (
            format: FromExtension,
            is_srgb: true,
            sampler: Default,
            asset_usage: 1,
        ),
    ),
)
```

**Common mistakes**:

- Missing commas or parentheses
- Incorrect `asset_usage` format (use simple `1`, not struct syntax)
- Wrong loader name (must be exact: `"bevy_image::image_loader::ImageLoader"`)

### Levels don't work in WASM

**Symptoms**:

- Only level 1-2 available
- Can't progress to higher levels
- Console shows level loading errors

**Cause**: Levels must be embedded at compile time for WASM (no filesystem access).

**Solution**: Update `embedded_level_str()` in `src/level_loader.rs`:

```rust
pub fn embedded_level_str(path: &str) -> Option<&'static str> {
    match path {
        "assets/levels/level_001.ron" => Some(include_str!("../assets/levels/level_001.ron")),
        "assets/levels/level_002.ron" => Some(include_str!("../assets/levels/level_002.ron")),
        // Add entries for all levels...
        _ => None,
    }
}
```

Then rebuild the WASM binary to bake in the new levels.

### Performance is poor in the browser

WASM builds are slower than native builds.
For best web performance:

- Use Chrome or Firefox (best WASM support)
- Close other browser tabs
- Disable browser extensions that might interfere
- Note: Large WASM binaries (>50MB) may have slow initial load times

## Level Loading Issues

### "Failed to load level" or missing levels

Ensure the `assets/levels/` directory exists and contains valid `.ron` files:

```bash
ls assets/levels/
# Should show: level_001.ron, level_002.ron, etc.
```

### Level doesn't render correctly

Check the level file format:

- Matrix must be exactly 20×20
- Cell values: 0=empty, 2=paddle, 1=ball, 3=brick
- See the {doc}`asset-format` guide for details

## Still stuck?

If none of the above solutions work:

1. **Search existing issues**: [GitHub Issues](https://github.com/cleder/brkrs/issues)
2. **Open a new issue** with:
   - Your operating system and version
   - Rust version (`rustc --version`)
   - Full error message or description of the problem
   - Steps to reproduce
