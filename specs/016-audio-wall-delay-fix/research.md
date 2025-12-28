# Phase 0 Research: Audio Wall Delay Fix

## Technical Context Clarifications

### Language/Version

- **Decision**: Rust 1.81 (edition 2021)
- **Rationale**: Project and constitution mandate Rust 2021; aligns with Bevy 0.17.3 and all dependencies.
- **Alternatives considered**: None (project standard).

### Primary Dependencies

- **Decision**: Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1
- **Rationale**: Required for ECS, physics, and logging; matches project baseline.
- **Alternatives considered**: None (project standard).

### Storage

- **Decision**: In-memory ECS state only
- **Rationale**: No persistent storage required for this feature; matches project baseline.
- **Alternatives considered**: N/A

### Testing

- **Decision**: cargo test, Bevy test utilities
- **Rationale**: Standard for Rust/Bevy projects; required by constitution and copilot-instructions.
- **Alternatives considered**: None

### Target Platform

- **Decision**: Linux, Windows, macOS, WASM (cross-platform)
- **Rationale**: Project targets both native and web; constitution mandates WASM support.
- **Alternatives considered**: None

### Project Type

- **Decision**: Single-project, ECS-based game
- **Rationale**: Matches project structure and constitution.
- **Alternatives considered**: None

### Performance Goals

- **Decision**: Wall hit audio must play within 50ms of collision event in 99% of cases; maintain 60 FPS overall.
- **Rationale**: Spec and constitution require low-latency audio and high frame rate.
- **Alternatives considered**: None

### Constraints

- **Decision**: <50ms audio latency, <200ms p95 for all game logic, no audio artifacts, concurrency limit for wall hit sounds.
- **Rationale**: Derived from spec and constitution.
- **Alternatives considered**: None

### Scale/Scope

- **Decision**: Single-player, real-time gameplay; no persistent user data; ECS state only.
- **Rationale**: Matches project and feature scope.
- **Alternatives considered**: None

## Best Practices for Technologies

### Rust + Bevy 0.17.3

- Use ECS for all game logic.
- Use MessageWriter/Reader for buffered events; observer systems for immediate triggers.
- Avoid panicking queries; use early returns for missing data.
- Use With<T>/Without<T> filters for queries.
- Store asset handles in resources; do not reload assets in systems.
- Use tracing for all debug/info/warn logs.
- Write tests first (TDD), confirm red, then implement.

### bevy_rapier3d 0.32.0

- Use physics-driven collision detection for all gameplay events.
- Use Rapier events to trigger game logic, not manual transform checks.
- Tune physics config for responsiveness.

### Audio

- Use Bevy's audio system; store handles in resources.
- Enforce concurrency limits to prevent artifacts.
- Log and skip audio if concurrency limit is reached.

### Testing

- Use cargo test and Bevy's test framework for all new systems.
- Write integration tests for wall collision and audio timing.

---

All technical unknowns are now resolved.
Proceed to Phase 1 design.
