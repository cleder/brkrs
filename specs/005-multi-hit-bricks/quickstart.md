# Quickstart: Multi-Hit Bricks

**Feature**: 005-multi-hit-bricks
**Date**: 2025-11-29

## Prerequisites

- Rust 1.81+ (via rustup)
- Bevy 0.17 development environment
- Repository cloned and dependencies installed

## Quick Test

### 1. Run Existing Tests

```bash
cd /home/christian/devel/bevy/brkrs
cargo test
```

### 2. Create Test Level with Multi-Hit Bricks

Create `assets/levels/test_multi_hit.ron`:

```ron
LevelDefinition(
    number: 997,
    matrix: [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 13, 13, 13, 0, 12, 12, 12, 0, 11, 11, 11, 0, 10, 10, 10, 0, 0, 0],
        [0, 0, 13, 13, 13, 0, 12, 12, 12, 0, 11, 11, 11, 0, 10, 10, 10, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]
)
```

### 3. Run Game with Test Level

```bash
BK_LEVEL=997 cargo run
```

### 4. Test Multi-Hit Behavior

1. Move paddle to hit ball toward bricks
2. Observe:
   - Index 13 bricks (4 hits): Should require 4 hits + 1 final hit
   - Index 12 bricks (3 hits): Should require 3 hits + 1 final hit
   - Index 11 bricks (2 hits): Should require 2 hits + 1 final hit
   - Index 10 bricks (1 hit): Should require 1 hit + 1 final hit
3. Level should complete only when ALL bricks are destroyed

## Development Workflow

### Running Tests

```bash
# All tests
cargo test

# Multi-hit specific tests (after implementation)
cargo test multi_hit

# With logging
RUST_LOG=debug cargo test multi_hit -- --nocapture
```

### Code Quality Checks

```bash
cargo fmt --all
cargo clippy --all-targets --all-features
bevy lint
```

### Building for WASM

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Key Files to Modify

| File | Purpose |
|------|---------|
| `src/lib.rs` | Modify `mark_brick_on_ball_collision` for multi-hit logic |
| `src/level_format/mod.rs` | Add multi-hit brick constants |
| `src/systems/mod.rs` | Export new multi_hit module |
| `src/systems/multi_hit.rs` | NEW: Brick type watcher system |
| `assets/textures/manifest.ron` | Add type_variants for indices 10-13 |
| `tests/multi_hit_bricks.rs` | NEW: Integration tests |

## Expected Behavior After Implementation

1. **Visual**: Each hit on a multi-hit brick changes its appearance
2. **Gameplay**: Bricks transition 13→12→11→10→20→destroyed
3. **Level Completion**: Only after ALL destructible bricks are gone
4. **Audio** (if implemented): Sound 29 plays on multi-hit collision
5. **Scoring** (if implemented): 50 pts per hit, 25 pts on final destroy

## Troubleshooting

### Brick Not Transitioning

- Check `BrickTypeId` is being mutated in collision system
- Verify brick index is in range 10-13
- Check console for collision events being triggered

### Material Not Updating

- Ensure `TypeVariantRegistry` has entries for indices 10-13
- Check `Changed<BrickTypeId>` system is registered
- Verify texture assets are loaded

### Level Not Completing

- Confirm all multi-hit bricks have `CountsTowardsCompletion`
- Check that index 20 (simple stone) still has the component after transition
- Press K to test destroy-all-bricks behavior
