# Research: Indestructible bricks (LevelDefinition)

**Purpose**: Resolve technical unknowns from the feature spec and define the implementation approach.

## Decision 1 — Index mapping and migration

- Decision: Repository assets under `assets/levels/` will be automatically migrated from tile index `3` to `20` at feature landing. The LevelDefinition parser will accept `3` during an immediate compatibility window but the canonical mapping going forward will be `20` for the simple destructible brick. Index `90` is reserved for indestructible bricks.
- Rationale: Automatic migration ensures shipped/packaged levels continue to behave consistently after the index remap and reduces manual work for maintainers. A short compatibility window allows external user levels to remain playable while providing time and tooling to migrate them.
- Alternatives considered:
  - Runtime-only compatibility (treat `3` as `20` at runtime): simpler but leaves repository assets inconsistent and can cause surprises for designers writing new levels.
  - Manual update / documentation only: least intrusive but shifts maintenance burden and increases chance of regressions in packaged levels.

## Decision 2 — Migration tool behavior and scope

- Decision: The migration tooling will operate only on repository-level assets (files under `assets/levels/`). The tool will:
  - Create a `.bak` backup for each file changed (e.g., `level_001.ron.bak`).
  - Replace tile value tokens representing simple bricks (`3`) with `20` when used in a LevelDefinition context.
  - Preserve formatting and comments where possible.
- Rationale: Scoped migrations minimize risk (no surprises for external user-provided or 3rd-party levels) while ensuring packaged levels used during development and testing are consistent.

## Decision 3 — Tests and acceptance criteria

- Decision: Add unit tests for parser behaviour and integration/regression tests for a small sample of migrated levels.
- Rationale: Guarantees consistent runtime behaviour after migration and detects inadvertent changes to layout or semantics. Tests will also verify that indestructible bricks (index `90`) do not decrement the level completion counter and that simple bricks (`20`) behave like prior `3` bricks.

## Implementation notes / next steps

- Implement a small migration script (Rust or a repository helper script) and a test harness that runs the migration against `assets/levels/*` and asserts behavior parity.
- Add parsing unit tests in `src/` for LevelDefinition values 3, 20 and 90 to cover legacy support, new canonical mapping, and indestructible behaviour.
