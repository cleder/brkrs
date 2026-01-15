# Research: Post-Refactor QA & Sanitation

**Feature**: Post-Refactor QA & Sanitation **Status**: Complete

## Decisions

### 1. Test Integrity Audit Strategy

- **Decision**: We will manually review and remove/rewrite the "fake tests" identified in `tests/change_detection.rs`.
- **Rationale**: These tests were likely placeholders or documentation-as-code that provide no runtime value.
  Removing them reduces noise.
- **Alternatives**:
  - *Keep them*: Misleading.
  - *Automate removal*: Too risky for a small number of files; manual review is safer.

### 2. Constant Visibility

- **Decision**:
  - `BALL_RADIUS`, `PADDLE_RADIUS`, `PADDLE_HEIGHT`: Keep `pub` but consider `pub(crate)` if only used in crate.
    However, they are used in `level_loader.rs` and `systems/respawn.rs`, which are modules.
    `lib.rs` defines them.
  - `PLANE_H`, `PLANE_W`: Used in `systems/spawning.rs`, `systems/grid_debug.rs`, `ui/palette.rs`, `level_loader.rs`.
  - **Refinement**: We will change `pub` to `pub(crate)` for all of them in `lib.rs` to restrict them to the crate, preventing external API leakage while allowing internal usage.
- **Rationale**: They are widely used across the crate but don't need to be part of the public library API.

### 3. Startup System Ordering

- **Decision**: Create a `StartupSet` (e.g., `Initialization`) or use `.chain()` in `lib.rs`.
- **Rationale**: `spawn_camera`, `spawn_ground_plane`, `spawn_light` are currently unordered.
  Chaining them or putting them in a set ensures deterministic execution.
- **Implementation**:

    ```rust
    app.add_systems(
        Startup,
        (
            setup,
            spawn_border,
            systems::grid_debug::spawn_grid_overlay,
            systems::spawning::spawn_camera,
            systems::spawning::spawn_ground_plane,
            systems::spawning::spawn_light,
        ).chain()
    );
    ```

    Or better, explicit ordering if dependencies exist.
    Since they are independent spawners, `.chain()` is sufficient to ensure a consistent order (even if the specific order doesn't matter for logic, it matters for determinism).

## Unknowns & Clarifications

- **Resolved**: No major unknowns.
  The task is well-defined maintenance.
