# Quickstart: Pause and Resume System

**Feature**: `004-pause-system` **Date**: 2025-11-28 **Branch**: `004-pause-system`

## Prerequisites

- Rust 1.81 toolchain via `rustup` (matches repo toolchain file)
- Bevy 0.16 and bevy_rapier3d 0.31 (already in `Cargo.toml`)
- Assets present under `assets/levels/` (for gameplay testing)
- Optional: WASM target for browser testing (`rustup target add wasm32-unknown-unknown`)

---

## Build & Test Commands

```bash
# Run unit + integration tests
cargo test

# Targeted pause system tests
cargo test pause

# Lint for regressions
cargo clippy --all-targets --all-features

# Validate Bevy schedule layout
bevy lint

# Launch game with dynamic linking for faster reloads
cargo run --features bevy/dynamic_linking

# (Optional) WASM build to ensure platform compatibility
cargo build --target wasm32-unknown-unknown --release
```

---

## Running the Game

### Native Execution

**Development (fast iteration)**:

```bash
cargo run --features bevy/dynamic_linking
```

**Release build**:

```bash
cargo run --release
```

**Window Behavior**:

- Game launches in borderless fullscreen mode (native platforms)
- Press **ESC** during gameplay to pause (switches to windowed mode)
- **Click anywhere** on screen to resume (switches back to fullscreen)
- Press **Q** to quit

### WASM Execution

1. **Build WASM target**:

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. **Start local web server**:

   ```bash
   # Using Python
   python3 -m http.server 8080

   # Or using basic-http-server
   cargo install basic-http-server
   basic-http-server .
   ```

3. **Navigate to** `http://localhost:8080/wasm/index.html`

**WASM Behavior**:

- Window mode switching is **not supported** on WASM (browser security)
- Pause/resume functionality works identically (ESC to pause, click to resume)
- Use browser's native fullscreen (F11 or fullscreen button) if desired

---

## Testing

### Manual Testing Scenarios

#### User Story 1: Pause Game (P1)

1. **Test pause activation**:

   ```bash
   cargo run
   ```

   - Start gameplay (ball moving, paddle active)
   - Press **ESC**
   - **Verify**: Physics frozen (ball stationary), overlay visible ("PAUSED\nClick to Resume")
   - **Native**: Verify window switches to windowed mode
   - **WASM**: Verify window mode unchanged

2. **Test pause ignore when already paused**:

   - With game paused, press **ESC** again
   - **Verify**: No state change, overlay remains visible

3. **Test pause blocked during level transition**:

   - Clear all bricks (press **K** for testing)
   - Immediately press **ESC** during fade transition
   - **Verify**: Pause request ignored until transition completes

#### User Story 2: Resume Game (P1)

1. **Test resume activation**:

   - Pause game (**ESC**)
   - Click anywhere on screen
   - **Verify**: Overlay disappears, physics resumes from exact state, ball continues movement
   - **Native**: Verify window returns to fullscreen

2. **Test resume with click on overlay text**:

   - Pause game
   - Click directly on "PAUSED" text
   - **Verify**: Resume works (click anywhere includes text)

3. **Test click ignored when not paused**:

   - With game active, click screen multiple times
   - **Verify**: No pause overlay, gameplay continues normally

#### User Story 3: Window Mode Switching (P2 - Native Only)

1. **Test fullscreen → windowed → fullscreen**:

   - Launch game (starts fullscreen)
   - Verify fullscreen mode visually
   - Press **ESC** to pause
   - **Verify**: Window switches to windowed mode
   - Click to resume
   - **Verify**: Window switches back to fullscreen

2. **Test windowed → windowed (no change)**:

   - Manually switch to windowed mode before pausing
   - Press **ESC** to pause
   - **Verify**: Window remains windowed (no unintended fullscreen switch)
   - Click to resume
   - **Verify**: Window remains windowed

3. **Test manual window mode change during pause**:

   - Pause game (ESC)
   - Manually toggle window mode (Alt+Enter or OS controls)
   - Click to resume
   - **Verify**: Window respects manual change (does not force back to captured mode)

#### Edge Case Testing

1. **Rapid ESC key presses**:

   - Spam **ESC** key rapidly (10+ presses per second)
   - **Verify**: No stuttering, no crashes, debouncing works

2. **Pause/resume cycles**:

   - Perform 10 consecutive pause/resume cycles
   - **Verify**: No state corruption, physics consistent, window mode stable

3. **State preservation**:

   - Pause game with ball at specific position
   - Wait 10 seconds (paused)
   - Resume game
   - **Verify**: Ball continues from exact pre-pause position (zero drift)

---

## Performance Validation

### Frame Rate Tests

```bash
# Run game with FPS counter (if available)
cargo run --release --features fps_counter
```

**Targets**:

- Pause activation latency: <16ms (1 frame at 60 FPS)
- Resume activation latency: <16ms
- Window mode switching: <100ms (native only)
- 60 FPS maintained during pause/resume transitions

### Platform Compatibility

**Native (Linux/Windows/macOS)**:

```bash
cargo test
cargo run --release
```

- Verify window mode switching works
- Verify ESC/click input handling
- Verify physics freeze/resume

**WASM**:

```bash
cargo build --target wasm32-unknown-unknown --release
# Run in browser via local server
```

- Verify pause/resume without window mode switching
- Verify ESC/click input in browser
- Verify physics freeze/resume
- Verify no console errors related to window mode

---

## Debugging

### Common Issues

#### Issue: ESC key not pausing game

**Solution**:

- Verify window has focus (click on game window)
- Check if level transition is active (pause blocked during transitions)
- Check console logs for input events

#### Issue: Click not resuming game

**Solution**:

- Verify click is on game window (not outside)
- Check if game is actually paused (overlay visible?)
- Verify mouse input not captured by other system

#### Issue: Window mode not switching (Native)

**Solution**:

- Verify running native build (not WASM)
- Check if display supports fullscreen mode
- Look for window mode errors in console logs
- Test on different monitor if multi-monitor setup

#### Issue: Physics not freezing on pause

**Solution**:

- Check `RapierConfiguration::physics_pipeline_active` value
- Verify physics systems not running with custom logic bypassing pause
- Check for external forces being applied outside pause control

### Debug Logging

Enable structured logging:

```bash
RUST_LOG=info cargo run
```

Expected log output:

```text
[INFO] Pause activated: ESC pressed
[INFO] Window mode: BorderlessFullscreen -> Windowed
[INFO] Physics pipeline: active=false
[INFO] Pause overlay spawned
...
[INFO] Resume activated: Mouse click
[INFO] Window mode: Windowed -> BorderlessFullscreen
[INFO] Physics pipeline: active=true
[INFO] Pause overlay despawned
```

---

## Integration with Existing Systems

### Level Switching

- **L key**: Switch to next level (works while active, blocked while paused)
- **R key**: Restart current level (works while active, blocked while paused)
- **K key**: Destroy all bricks (works while active, blocked while paused)

### Respawn System

- Ball respawn sequence: Pause blocked during respawn delay
- Paddle growth animation: Pause blocked during growth

### Debug Systems

- **Space bar**: Toggle wireframe mode (works while paused)
- Grid overlay: Visible with wireframe (unaffected by pause)

---

## Known Limitations

1. **WASM Window Mode**: Window mode switching not supported on WASM due to browser security.
   Users must use browser's native fullscreen (F11).

2. **Gamepad/Touch Input**: Out of scope for this feature (FR-015).
   ESC key and mouse click only.

3. **Audio Pause**: Audio systems not affected by pause.
   If audio is implemented in future, separate audio pause handling required.

4. **Pause Menu**: Current design shows simple text overlay.
   Interactive menu (resume/restart/quit buttons) is future enhancement.

---

## Next Steps

### Phase 2: Implementation Tasks

See `tasks.md` (generated by `/speckit.tasks` command) for:

- File creation checklist (`src/pause.rs`, `src/ui/pause_overlay.rs`)
- System implementation order
- Test file setup
- Integration with existing code
- Performance profiling

### Post-Implementation

1. **Validation**: Run full test suite + manual testing scenarios above
2. **Performance**: Measure pause/resume latency, verify 60 FPS target
3. **Cross-Platform**: Test native + WASM builds
4. **Code Review**: Verify adherence to constitution principles
5. **Documentation**: Update README with pause controls

---

## Support

For issues or questions, refer to:

- Feature Specification: [spec.md](spec.md)
- Data Model: [data-model.md](data-model.md)
- Research: [research.md](research.md)
- Bevy documentation: <https://bevyengine.org/learn/>
- Rapier3D documentation: <https://rapier.rs/docs/>
- Project constitution: `.specify/memory/constitution.md`
