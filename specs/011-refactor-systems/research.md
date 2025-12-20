# Research: Refactor Systems for Constitution Compliance

Date: 2025-12-19 Branch: 011-refactor-systems Spec: specs/011-refactor-systems/spec.md

## Decisions

- Error type: Use `anyhow::Result<()>` for all systems; leverage `?` and early returns.
- Signals: Centralize `UiBeep` and `BrickDestroyed` as `#[derive(Message)]` in `crate::signals`; remove duplicates.
- Engine events: Consume `AssetEvent<Image>` via observers; do not wrap as Messages.
- Change detection: Strict change-driven updates (`Changed<T>`, `RemovedComponents<T>`, `OnAdd`); no periodic fallbacks.
- Required components: Apply `#[require(Transform, Visibility)]` to `Paddle`, `Ball`, `GridOverlay`, `Border`, `GroundPlane` now.

## Rationale

- Maintainability & Safety: Uniform error handling, single-source-of-truth signals, and elimination of panics improve reliability and reviewability.
- Performance: Change-driven updates reduce per-frame overhead; set-based scheduling increases parallelism.
- Architectural Consistency: Aligns with Constitution 1.3.0 mandates & prohibitions and Bevy 0.17 best practices.

## Alternatives Considered

- Custom error enums per module: Rejected for added boilerplate without clear benefit at this stage.
- Messages for engine events: Rejected; engine events are designed for observers.
- Periodic fallbacks for textures/materials: Rejected; hides scheduling work and violates change-driven mandate.
- Defer required components: Rejected; applying now removes redundancy and prevents future drift.
