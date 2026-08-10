# Research: Sea Mine Brick

## Decisions

- Decision: Sea mine brick index is 31.
  - Rationale: The repository's brick documentation already identifies the sea-mine enemy as index 31, so the feature should align with the existing numeric mapping instead of inventing a new one.
  - Alternatives considered: A new unused index in the 30s or 40s; rejected because it would duplicate an existing documented identity and complicate level authoring.

- Decision: Use `bevy_hanabi` 0.17.0 for explosion particles.
  - Rationale: The crate's 0.17 release track matches Bevy 0.17, and this feature will therefore require raising the feature toolchain baseline to Rust 1.89 because Hanabi 0.17.0 does not support Rust 1.81.
  - Alternatives considered: A custom mesh-based burst or a bespoke particle system; rejected because Hanabi already provides a standard particle pipeline and avoids writing and maintaining a one-off effect system.

- Decision: Raise the feature toolchain baseline to Rust 1.89.
  - Rationale: This resolves the compatibility mismatch between the repository's previous Rust 1.81 baseline and `bevy_hanabi` 0.17.0.
  - Alternatives considered: Staying on Rust 1.81 with Hanabi 0.17.0; rejected because it is not buildable.
    Replacing Hanabi; rejected because the feature explicitly requires Hanabi-based explosion particles.

- Decision: Sea mine launch motion is arbitrary in the XZ plane with minimum linear speed 3.0 u/s and minimum angular spin 180 deg/s.
  - Rationale: This preserves the requested merkaba-like motion while keeping the hazard mobile and testable.
  - Alternatives considered: Fixed launch direction, non-zero-only thresholds, or a lower floor; rejected because those options either made the behavior predictable in a bad way or allowed stall states.

- Decision: Gameplay state uses Messages; particle burst uses an Observer.
  - Rationale: Spawn, detonation, destruction, and life-loss are buffered gameplay state changes and need deterministic ordering.
    The Hanabi burst is immediate presentation and fits observer-style reactive logic.
  - Alternatives considered: A single observer-only pipeline for all behavior; rejected because it would blur message/event boundaries and make the gameplay path harder to test.

- Decision: The explosion radius is 30 world units and the mine is destroyed after the detonation resolution completes.
  - Rationale: This matches the spec and keeps the damage area readable while preventing repeat detonations.
  - Alternatives considered: Larger radius or persistent hazard after detonation; rejected because it would change the requested gameplay balance.

## Alternatives Considered

- Custom particle burst system
  - Rejected because `bevy_hanabi` already matches the rendering requirement and keeps the visual effect isolated from gameplay logic.

- New sea mine brick index outside the documented 31 entry
  - Rejected because the repo's brick documentation already reserves index 31 for the sea mine enemy, and the feature should follow the established mapping.
