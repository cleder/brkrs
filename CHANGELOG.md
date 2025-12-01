# Changelog

All notable changes to this project are documented here.

## [0.0.1] - 2025-11-29

### Added

- Feature: Add "indestructible bricks" tile type (index 90). These bricks do not count towards level completion.
- Migration: Add `tools/migrate-level-indices` CLI and `scripts/migrate-assets.sh` wrapper to migrate existing level assets (3 -> 20) safely with backups.
- Runtime: Add `SIMPLE_BRICK = 20` and `INDESTRUCTIBLE_BRICK = 90` constants in `src/level_format/mod.rs` and update loader/ui to use constants.
- Tests: Add migration parity tests, runtime tests (level definition / completion), editor palette tests, and a profiling smoke test.
- CI: Add a `migration_tests` job that runs migration parity tests when PRs touch `assets/levels/`.
- Docs: Add `assets/levels/README.md` and `specs/001-indestructible-bricks` docs (quickstart, perf, tasks).

### Features

- Multi-hit bricks (indices 10-13) that require multiple hits to destroy
- Pause system with UI overlay and window mode management
- Level transition system with growth animation and fade overlay
- Ball respawn system with gravity configuration and paddle integration
- Textured visuals with per-level texture overrides and fallback handling
- Camera shake effect and velocity limiting
- 20x20 grid format for levels

### Infrastructure

- Sphinx documentation with Read the Docs integration
- GitHub Actions CI/CD with caching and WASM deployment
- GitHub Codespaces prebuild configuration

