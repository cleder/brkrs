# Quickstart: Score Multiplier Bricks

## 1. Preconditions

- Be on branch `066-score-multiplier-bricks`.
- Use Rust toolchain compatible with project (`rustup` managed).

## 2. Red Phase (TDD First)

1. Add integration tests for all P1 scenarios:
   - Activation forward-only behavior (activating brick scored at normal value)
   - Replacement by newer multiplier brick
   - Reset only on life decrement
   - No reset on non-life ball despawn
   - Persistence across 10+ frames
   - Persistence across level transition without life decrement
2. Add UI-focused tests for multiplier indicator behavior:
   - `x2`/`x3`/`x4` rendered beneath score indicator when active
   - indicator hidden at `1x`
   - no unnecessary UI rewrites when multiplier state is unchanged
3. Commit failing tests (red proof commit).

## 3. Implementation Phase

1. Add/update multiplier state resource in scoring flow.
2. Wire multiplier activation on brick indices `26..=29`.
3. Apply multiplier only to brick-destruction score awards.
4. Reset multiplier on life decrement path only.
5. Ensure level transition does not implicitly reset multiplier.
6. Extend score UI to render/hide multiplier indicator beneath the score indicator.
7. Gate multiplier indicator updates on multiplier-state changes.

## 4. Green Phase Validation

Run:

```bash
cargo test
cargo fmt --all
cargo clippy --all-targets --all-features
bevy lint
cargo build --target wasm32-unknown-unknown
```

## 5. Focused Regression Checks

- Existing scoring behavior unchanged at `1x`.
- Multi-ball gameplay does not produce spurious resets.
- No observer-only shortcut introduced for scoring/life reset logic.
- Multiplier indicator shows correct `x2`/`x3`/`x4` text and is hidden at `1x`.
