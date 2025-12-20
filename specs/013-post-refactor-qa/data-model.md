# Data Model: Post-Refactor QA & Sanitation

**Feature**: Post-Refactor QA & Sanitation **Status**: N/A (Maintenance Task)

## Entities

No new entities are being created.
This task involves cleaning up existing code and tests.

## API Contracts

No new APIs are being created.
Existing public constants in `lib.rs` will have their visibility restricted to `pub(crate)`.

### Modified Constants (Visibility Change)

- `BALL_RADIUS`: `pub` -> `pub(crate)`
- `PADDLE_RADIUS`: `pub` -> `pub(crate)`
- `PADDLE_HEIGHT`: `pub` -> `pub(crate)`
- `PLANE_H`: `pub` -> `pub(crate)`
- `PLANE_W`: `pub` -> `pub(crate)`

## Startup System Ordering

The startup systems in `lib.rs` will be explicitly ordered:

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
