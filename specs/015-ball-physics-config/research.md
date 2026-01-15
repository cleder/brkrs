# Phase 0: Research

## Unknowns/Clarifications

- Best practice for centralizing physics config in Bevy ECS (resource vs. component vs. module constant)
- How to ensure all ball, paddle, and brick spawns use the centralized config (pattern enforcement)
- How to document and enforce non-hot-reloadable, source-only config
- How to structure tests for config usage and absence of hardcoded values

## Research Tasks

1. Research best practices for centralizing physics config in Bevy ECS (resource/component/module constant)
2. Find patterns for enforcing config usage in all relevant spawns (ball, paddle, brick)
3. Document how to make config non-hot-reloadable and source-only
4. Identify test strategies for verifying config usage and absence of hardcoded values

## Consolidated Findings

- **Decision**: Use a dedicated Rust struct (e.g., `BallPhysicsConfig`, `PaddlePhysicsConfig`, `BrickPhysicsConfig`) defined in source, registered as a Bevy resource for each entity type.
  This allows ECS access and compile-time enforcement, but is not hot-reloadable.
- **Rationale**: Resource pattern is idiomatic in Bevy for global config, and source-only struct ensures no runtime reload.
  Using separate resources for each entity type keeps config clear and maintainable.
- **Alternatives considered**: Module-level constants (less flexible, harder to test/change), config files (would enable hot-reload, which is explicitly prohibited), component-based config (not needed for global/static values).

- **Decision**: All spawn systems (ball, paddle, brick) must query the relevant config resource and apply its values to the collider/rigidbody.
  Add a lint/test to check for hardcoded values in spawn logic.
- **Rationale**: This ensures all entities use the centralized config and prevents drift.
  Lint/test enforces compliance.
- **Alternatives considered**: Manual code review (less reliable), macros (overkill for this use case).

- **Decision**: Document in code and in the spec that config is source-only and not hot-reloadable.
  Add a test to ensure no config file is loaded for these values.
- **Rationale**: Prevents accidental introduction of runtime config or hot-reload.
- **Alternatives considered**: None (requirement is explicit).

- **Decision**: Use static analysis and integration tests to verify all entities use the config and no hardcoded values remain.
- **Rationale**: Ensures long-term maintainability and testability.
- **Alternatives considered**: Manual review only (less robust).
