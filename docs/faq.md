# Frequently Asked Questions

Common questions about playing and developing brkrs.

## Gameplay

### Why does my paddle not respond to mouse input?

The game might be paused.
Press **ESC** to toggle pause, or click to resume.
If running in WASM (browser), ensure the game canvas has focus by clicking on it.

### How do I rotate the paddle?

Use your **mouse scroll wheel** during gameplay to rotate the paddle.
This allows you to angle the ball in different directions.

### Why is the ball moving slowly / strangely?

Each level can define custom gravity settings.
Some levels use gravity to create unique ball physics.
Check the level file's `gravity` field if editing levels.

### Can I use a gamepad or touch controls?

Not currently. brkrs is designed for keyboard and mouse only.
See the {doc} `quickstart` for the full control scheme.

## Building & Running

### The game won't compile. What should I check?

1. Ensure you have Rust 1.81+ installed: `rustc --version`
2. Run `cargo clean` and try again
3. Check the {doc}`troubleshooting` guide for common build issues

### How do I run the game in release mode?

```bash
cargo run --release
```

Release mode provides better performance but slower compile times.

### How do I build for WebAssembly?

See the WASM section in {doc} `developer-guide`.
You'll need the `wasm32-unknown-unknown` target and a local server to test.

## Level Design

### How do I create a new level?

Levels are RON files in `assets/levels/`.
Copy an existing level and modify:

1. Update the `number` field to a unique level number
2. Edit the 20×20 `matrix` grid:
   - `0` = empty
   - `1` = paddle spawn
   - `2` = ball spawn
   - `3` = brick

See {doc}`asset-format` for the complete level format specification.

### Can I add custom brick types?

The brick system is extensible but currently only standard bricks (`3`) are implemented.
Indestructible bricks and other types are planned features.

### What happens if I forget to place a paddle or ball?

The game uses fallback spawn positions.
Your level will load, but the paddle and ball will appear at default locations.

## Development

### How do I run the tests?

```bash
cargo test
```

For verbose output with test names:

```bash
cargo test -- --nocapture
```

### What's the project structure?

See {doc} `architecture` for the ECS design and module organization.
Key modules:

- `src/level_loader.rs` — Level parsing and entity spawning
- `src/pause.rs` — Pause state machine
- `src/systems/` — Game systems (respawn, textures, etc.)

### How do I contribute?

Read the {doc} `contributing` guide for the PR workflow, code style requirements, and commit message conventions.

## Documentation

### How do I build the docs locally?

```bash
cd docs
pip install -r requirements.txt
make html
```

Open `_build/html/index.html` in your browser.

### Where is the API documentation?

The Rust API docs are generated with `cargo doc`.
See {doc} `api-reference` for links to the embedded rustdoc.
