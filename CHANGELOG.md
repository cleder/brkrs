# Changelog

All notable changes to this project are documented here.

## Unreleased

- Feature: Add "indestructible bricks" tile type (index 90). These bricks do not count towards level completion.
- Migration: Add `tools/migrate-level-indices` CLI and `scripts/migrate-assets.sh` wrapper to migrate existing level assets (3 -> 20) safely with backups.
- Runtime: Add `SIMPLE_BRICK = 20` and `INDESTRUCTIBLE_BRICK = 90` constants in `src/level_format/mod.rs` and update loader/ui to use constants.
- Tests: Add migration parity tests, runtime tests (level definition / completion), editor palette tests, and a profiling smoke test.
- CI: Add a `migration_tests` job that runs migration parity tests when PRs touch `assets/levels/`.
- Docs: Add `assets/levels/README.md` and `specs/001-indestructible-bricks` docs (quickstart, perf, tasks).

## Notes for Maintainers

- When landing a PR that modifies `assets/levels/` or changes the canonical simple brick, run `scripts/migrate-assets.sh` locally or in the landing workflow. This will update files with a safe backup and tests will validate parity.
