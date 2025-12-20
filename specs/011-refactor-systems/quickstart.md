# Quickstart: Refactor Systems for Constitution Compliance

## Prerequisites

- Rust 1.81 toolchain via rustup
- Branch: `011-refactor-systems`

## Tests-First Workflow

1. Create failing tests (red) for each FR (see spec):
   - Fallible systems behavior
   - Message/Event separation
   - No tuple `.chain()` usage (verify order via observable state)
   - Change-driven visuals and overlays
   - Required component presence on marker-only spawns
   - Asset handle reuse
2. Commit failing tests.
3. Request review/approval of tests.
4. Implement refactors until tests pass (green).

## Commands

Run tests:

```bash
cargo test
```

Lint/format:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -D warnings
```

WASM build check (optional):

```bash
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' \
  cargo build --target wasm32-unknown-unknown --no-default-features
```

## Notes

- Prefer Messages for gameplay-domain signals and observers for engine events.
- Avoid per-frame work; add `Changed<T>`/`RemovedComponents<T>` filters.
- Replace tuple `.chain()` with System Set ordering via `.configure_sets()` and `.after()`.
