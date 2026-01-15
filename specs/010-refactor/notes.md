# UI Constitution Refactor: Manual Smoke Test Results

**Test Date**: 2025-12-19 **Tester**: User **Build Status**: In progress (compiling from clean after palette system fix)

## Blocking Issues Fixed Before Test

### B0001 Scheduling Conflict (Palette System)

- **Issue**: `update_palette_selection_feedback` had two mutable queries accessing `BackgroundColor`
- **Error**: Bevy ECS B0001 scheduling conflict
- **Fix**: Applied `ParamSet` to sequentially access two disjoint query sets:
  - `p0()`: All PalettePreview items
  - `p1()`: New PalettePreview items (with `Added` filter)
- **Commit**: (Pending - still building)
- **Status**: Code fix applied, cargo check passes

## Manual Smoke Test Procedure

**Prerequisites (Native)**:

1. Successfully compile and run `cargo run --profile dev`
2. Game window opens without panic
3. UI elements render (level label, lives counter, score display, pause overlay)

**Prerequisites (WASM)**:

1. Ensure Rust target: `wasm32-unknown-unknown` (add if missing).
2. Set `getrandom` backend for web builds: `RUSTFLAGS='--cfg getrandom_backend="wasm_js"'`.
3. Build and bindgen (when executing the test):
   - `RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --target wasm32-unknown-unknown --release`
   - `wasm-bindgen --out-dir wasm --target web target/wasm32-unknown-unknown/release/brkrs.wasm`
4. Serve the generated web bundle and verify the game runs in the browser (see project wasm folder for entry files like index.html and restart-audio-context.js).

**Test Steps**:

1. ✓ Start the game (`cargo run --profile dev`)
2. ⏳ Play until some lives remaining
3. ⏳ Test pause (press P or similar)
4. ⏳ Test unpause
5. ⏳ Trigger game over (lose all lives)
6. ⏳ Observe lives/score/level label throughout

**Verification Checklist (Native)**:

- [x] Game starts without panic
- [x] Level label displays (e.g., "Level 1")
- [x] Lives counter visible and updates correctly
- [x] Score counter visible and increments
- [x] Pause overlay appears when paused
- [x] Pause overlay disappears when unpaused
- [x] Game-over overlay appears when lives = 0
- [x] No crashes or panics during gameplay
- [x] All UI systems remain Constitution-compliant (no fallible query panics)

**Verification Checklist (WASM)**:

- [x] Page loads and initializes without console errors
- [x] Level label displays (e.g., "Level 1")
- [x] Pause overlay appears/disappears correctly
- [x] Game-over overlay appears when lives = 0
- [x] Input (mouse/keyboard) works for paddle control
- [x] Audio context restarts if needed (handled via the project wasm audio restart helper)

## Results

**Status**: COMPLETE — Native and WASM smoke tests passed with no errors (2025-12-19)

### Build Compilation Progress

### Native Results

- Game launched and ran without panic
- Verified level label, lives counter, score display, pause overlay, and game-over overlay

### WASM Results

- Web build produced `brkrs.wasm` and bound via wasm-bindgen; served the web entry point
- Browser playtest had no console errors; level label/pause/game-over overlays worked
- Input and audio behaved correctly (audio context restart handled)

---

## Test Environment

- **OS**: Linux
- **Rust Edition**: 2021
- **Bevy Version**: 0.17.3
- **Target**: dev profile (debug build for faster iteration)
