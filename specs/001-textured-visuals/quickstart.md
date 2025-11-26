# Quickstart: Textured Visuals Overhaul

## Prerequisites

- Rust 1.81 toolchain installed via `rustup` (`rust-toolchain.toml` enforces this).
- Bevy dynamic linking optional but recommended for faster `cargo run` reloads: enable via `BEVY_DYNAMIC_PLUGIN=1` or `cargo run --features bevy/dynamic_linking`.
- Texture manifest present at `assets/textures/manifest.ron` plus referenced PNG/KTX assets under `assets/textures/`.
- Level definitions under `assets/levels/` (at least `level_001.ron` + `level_002.ron`) so the level switch shortcut can wrap.
- Optional: WASM target for browser parity (`rustup target add wasm32-unknown-unknown`).

## Build & Test Commands

```bash
# Fast dev loop with structured logs
RUST_LOG=info cargo run --features bevy/dynamic_linking

# Asset + manifest parsing tests (add more granular filters as modules land)
cargo test texture_manifest

# Level switching regression (ensures KeyCode::L wraps safely)
cargo test level_switcher

# Full suite + lints before sharing builds
cargo test
cargo clippy --all-targets --all-features
bevy lint

# (Optional) Prove WASM build still functions with textured assets
cargo build --target wasm32-unknown-unknown --release
```

## Automated Coverage

- `texture_manifest.rs` unit tests assert that every `VisualAssetProfile` entry enforces required fields, validates fallback chains, and round-trips from `manifest.ron`.
- `fallback_registry.rs` tests confirm each object class receives a canonical material within one frame when a texture fails to load and that warnings log only once per session.
- `level_switcher.rs` integration test fakes `KeyCode::L` input, ensures the next level loads (wrapping to index 0), and checks that `LevelPresentation` resources refresh overrides without leaking handles.
- `type_variants.rs` tests guarantee that updating a ball/brick type swaps the corresponding material within 0.1 seconds (assert via timer-controlled system step).

## Manual Verification

1. **Baseline textured spawn**
   - Launch the game (`cargo run`). Load level 001 and confirm ball, paddle, bricks, sidewalls, and backdrop are textured on the first frame.
   - Temporarily rename a texture referenced in `manifest.ron`; restart and verify the entity uses the fallback material while a single warning appears in logs.
2. **Type-driven visuals**
   - Use an in-game debug command (or scripted test) to flip ball type ids; watch the material swap without popping. Repeat with at least two brick types in level 002.
3. **Per-level overrides**
   - Edit `assets/levels/level_002.ron` to reference a distinct `LevelTextureSet`. Reload via `cargo run` and ensure only that level’s ground/background change.
4. **Level switch shortcut**
   - Press **L** repeatedly during play. Each press should load the next level within two seconds and reapply the correct texture manifest entries, wrapping after the last level.
5. **WASM sanity**
   - Serve the `wasm/` folder (e.g., `python -m http.server`), open in Chrome/Firefox, and confirm textures load along with the **L** shortcut.

## Troubleshooting

- **Blank/white meshes**: Most likely `manifest.ron` failed to parse. Run `cargo test texture_manifest` and inspect logs for Serde errors.
- **Repeated fallback warnings**: Ensure `FallbackRegistry::log_once` is used when reporting missing assets; duplicate logs indicate the registry resource was not initialized before spawns.
- **Level switch ignores input**: Verify the `LevelSwitchPlugin` system is added to the appropriate schedule and that `KeyCode::L` is not consumed by another input handler.
- **WASM build missing textures**: Confirm assets are copied to the web build output (`wasm/run.sh` or bespoke pipeline) and that texture formats are browser-compatible (PNG/KTX2 only).
