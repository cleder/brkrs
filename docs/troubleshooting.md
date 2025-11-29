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

The first build compiles all dependencies (Bevy, Rapier, etc.) which takes several minutes. This is normal.

**Tips to speed up development builds**:

- Use `cargo run` (debug mode) during development — faster compilation
- Use `cargo run --release` only for final testing — slower compilation but better performance
- Enable dynamic linking for Bevy in development (see `Cargo.toml` for feature flags)

### Out of memory during compilation

Bevy is a large framework. If you run out of RAM during compilation:

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

## WASM/Web Issues

### Web version doesn't load

- Use a modern browser (Chrome 80+, Firefox 75+, Safari 14+, Edge 80+)
- Ensure WebGL 2.0 is enabled in your browser
- Check the browser console (F12) for error messages

### Performance is poor in the browser

WASM builds are slower than native builds. For best web performance:

- Use Chrome or Firefox (best WASM support)
- Close other browser tabs
- Disable browser extensions that might interfere

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
- Cell values: 0=empty, 1=paddle, 2=ball, 3=brick
- See the {doc}`asset-format` guide for details

## Still stuck?

If none of the above solutions work:

1. **Search existing issues**: [GitHub Issues](https://github.com/cleder/brkrs/issues)
2. **Open a new issue** with:
   - Your operating system and version
   - Rust version (`rustc --version`)
   - Full error message or description of the problem
   - Steps to reproduce
