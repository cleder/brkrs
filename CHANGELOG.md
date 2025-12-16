# Changelog

All notable changes to this project are documented here.

## Unreleased

### Changed

- Add compatibility alias `LevelAdvanceSet = LevelAdvanceSystems` (deprecated) to avoid breaking external callers after renaming the system set.
- Made core event types (`WallHit`, `BrickHit`, `BallHit`) public to allow external systems (e.g., the audio plugin) to observe them; review for API compatibility.

### Added

- **Scoring system** (`009-add-scoring`): Players accumulate points by destroying bricks, with values ranging from 25-300 points based on brick type (documented in `docs/bricks.md`). The score persists across level transitions within a game session and displays in real-time in the top-right corner. Every 5000 points, players earn a bonus life. Special mechanics include random scoring for Question bricks (25-300 points) and zero points for effect-only bricks (Extra Ball, Magnet). Implemented with ECS resources (`ScoreState`), message-based events (`BrickDestroyed`, `MilestoneReached`), and change-detection optimized UI updates.
- Paddle shrink visual feedback (`008-paddle-shrink-feedback`): When a player loses their last ball, the paddle immediately shrinks from full size to nearly invisible (scale 0.01) over 1 second, providing instant visual feedback while running concurrently with the respawn delay. Smooth animation uses cubic easing interpolation and integrates seamlessly with the existing respawn system and fadeout overlay.
- Level metadata (`007-level-metadata`): optional `description` and `author` fields in `LevelDefinition` for level design documentation and contributor attribution. Supports plain text and markdown link formats for authors. Fully backward compatible with existing level files.

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

