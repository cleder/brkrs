# Quickstart: Extra Ball Brick (Brick 41)

## Implementation Summary

Brick 41 (Extra Ball) is now fully implemented with the following behavior:

- **Durability**: 1 hit (single-hit destruction)
- **Score**: 0 points (no combo/multiplier effects)
- **Life Award**: +1 life on destruction, clamped to MAX_LIVES (5)
- **Audio**: Unique destruction sound (`Brick41ExtraLife`) with fallback to generic `BrickDestroy` if asset missing
- **Multi-ball Safety**: Single life award per brick even with simultaneous hits

## Key Implementation Details

1. **Brick Type Constant**: `EXTRA_LIFE_BRICK = 41` defined in `src/level_format/mod.rs`
2. **Life Award Flow**:
   - `mark_brick_on_ball_collision` writes `LifeAwardMessage { delta: 1 }` on brick 41 hit
   - Per-frame `HashSet<Entity>` prevents double-awards from multi-ball
   - `apply_life_awards` consumes messages and clamps to [0, MAX_LIVES]
3. **Audio Flow**:
   - `consume_brick_destroyed_messages` detects brick type 41
   - Plays `SoundType::Brick41ExtraLife` if handle exists, else `SoundType::BrickDestroy` with warning log
4. **Score Isolation**: `brick_points()` returns 0 for brick 41; score system ignores it

## Testing

Run the test suite to verify brick 41 behavior:

```bash
# All brick 41 tests (life gain + audio)
cargo test --test extra_ball_brick --test extra_ball_brick_audio

# Life gain tests only
cargo test --test extra_ball_brick

# Audio tests only
cargo test --test extra_ball_brick_audio

# Full suite with linters
cargo test
cargo fmt --all
cargo clippy --all-targets --all-features
bevy lint
```

## Usage in Levels

Add brick 41 to level matrices in `assets/levels/*.ron`:

```ron
// Example level with brick 41
(
    id: "test_extra_life",
    name: "Extra Life Test",
    brick_matrix: [
        [41, 41, 41],  // Row of extra-life bricks
        [10, 20, 30],  // Regular bricks
    ],
    ball_spawn: (x: 0.0, y: 1.0, z: 0.0),
    paddle_spawn: (x: 0.0, y: 0.5, z: 0.0),
)
```

## WASM Compatibility

Brick 41 is WASM-safe:

- No platform-specific panics (optional parameters in systems)
- Audio fallback handles missing assets gracefully
- Life clamping uses safe arithmetic (no overflow)
- Message-driven architecture (no observers)

Build and test WASM:

```bash
# Build WASM target
cargo build --target wasm32-unknown-unknown --release

# Run in browser (requires wasm-server-runner or similar)
cargo run --target wasm32-unknown-unknown --release
```

## Architecture Notes

- **Message-Event Separation**: Uses `LifeAwardMessage` and `BrickDestroyed` messages (no Observers per constitution)
- **Bevy 0.17 Compliance**: Fallible systems, filtered queries, `Changed<T>` for reactive patterns, asset handle reuse
- **Performance**: No per-frame work; asset handles loaded once at startup via `AudioAssets` resource
- **Multi-ball Deduplication**: `HashSet<Entity>` in collision system tracks processed bricks per frame

## Troubleshooting

**Issue**: Multi-ball awards multiple lives **Solution**: Deduplication via `HashSet<Entity>` ensures single award per brick per frame

**Issue**: Missing audio handle causes panic **Solution**: Fallback to `BrickDestroy` sound with warning log (check `config/audio.ron` for asset path)

**Issue**: Lives exceed maximum **Solution**: `apply_life_awards` clamps to MAX_LIVES (5); check logs for corrupted state warnings

**Issue**: Score increments on brick 41 hit **Solution**: Verify `brick_points()` returns 0 for brick 41; regression tests cover this
