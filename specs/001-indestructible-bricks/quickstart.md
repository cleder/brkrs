# Quickstart: Implementing Indestructible Bricks (LevelDefinition)

This quickstart walks through the main implementation tasks and how to verify them locally.

1. Ensure you are on the feature branch:

```bash
git checkout 001-indestructible-bricks
```

1. Run migration tooling (once implemented) against repository assets to update index `3` → `20`:

```bash
# Build the migration CLI (one-time)
cd tools/migrate-level-indices && cargo build

# Option A (recommended) — use the repo wrapper which checks the tool is built and
# writes backups when --backup is provided:
./scripts/migrate-assets.sh --backup --from 3 --to 20 assets/levels/*.ron

# Option B — run the tool directly with cargo if you prefer not to use the wrapper:
cargo run --manifest-path tools/migrate-level-indices/Cargo.toml -- --backup --from 3 --to 20 assets/levels/*.ron
```

1. Run unit tests and integration tests:

```bash
# full test-suite
cargo test --all
# focused tests (example): run the level_definition and indestructible_visual tests
cargo test --test level_definition
cargo test --test indestructible_visual

# Migration parity test (new): verifies that the migration tool converts 3→20 and preserves layout
cargo test --test migration_parity
```

1. Validate behavior manually in the game:

   - Load a level with `20` tiles (simple bricks) and `90` tiles (indestructible). Confirm:

     - Simple bricks break and decrement the level completion counter.
     - Indestructible bricks remain and do not count toward completion.

1. Update docs and LevelDefinition samples in `assets/levels/` and commit them.

CI notes

- This repo's CI includes a `migration_tests` job which only runs when files under `assets/levels/` are changed on PRs.
  It builds `tools/migrate-level-indices` and executes `cargo test --test migration_parity` to prevent accidental content drift.
- To validate the same checks locally run the migration-parity test shown above and push
  your changes to confirm CI picks them up.

Special test level: `assets/levels/test_mixed_indestructible.ron` — used by unit/integration tests to validate that clearing destructible bricks completes the level while indestructible bricks remain.
