# brkrs Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-12-19

## Active Technologies
- N/A (In-memory ECS) (012-refactor-entity-spawning)
- N/A (In-memory ECS state only) (013-post-refactor-qa)
- Rust 1.81 (edition 2021) + Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1 (015-ball-physics-config)
- N/A (in-memory ECS state only) (015-ball-physics-config)

- Rust 1.81 (edition 2021) + Bevy 0.17.3, bevy_rapier3d 0.32.0, serde/ron, tracing (011-refactor-systems)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test cargo clippy

## Code Style

Rust 1.81 (edition 2021): Follow standard conventions

## Testing & TDD

- The project mandates **Test-Driven Development** for all feature work: tests must be written and committed before implementation and a failing-test commit (red) must exist as proof before implementation begins.
- Include unit tests, integration/acceptance tests for user scenarios, and WASM-targeted tests when behavior differs on the web.
- CI pipelines MUST enforce tests and reject merges that do not comply with the tests-first proof.
## Recent Changes
- 016-audio-wall-delay-fix: Added [if applicable, e.g., PostgreSQL, CoreData, files or N/A]
- 015-ball-physics-config: Added Rust 1.81 (edition 2021) + Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1
- 013-post-refactor-qa: Added Rust 1.81 (edition 2021) + Bevy 0.17.3, bevy_rapier3d 0.32.0


<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
