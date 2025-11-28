# Quickstart: Implementing Indestructible Bricks (LevelDefinition)

This quickstart walks through the main implementation tasks and how to verify them locally.

1. Ensure you are on the feature branch:

```bash
git checkout 001-indestructible-bricks
```

1. Run migration tooling (once implemented) against repository assets to update index `3` → `20`:

```bash
# hypothetical helper script
./tools/migrate-level-indices --backup --from 3 --to 20 assets/levels/*.ron
```

1. Run unit tests and integration tests:

```bash
cargo test --all
# or run a smaller focused test set
cargo test --tests level_definition
```

1. Validate behavior manually in the game:

   - Load a level with `20` tiles (simple bricks) and `90` tiles (indestructible). Confirm:

     - Simple bricks break and decrement the level completion counter.
     - Indestructible bricks remain and do not count toward completion.

1. Update docs and LevelDefinition samples in `assets/levels/` and commit them.
