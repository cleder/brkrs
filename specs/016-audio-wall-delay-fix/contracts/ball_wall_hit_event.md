# Contract: BallWallHit Event

## Purpose

Defines the structure and contract for the BallWallHit event used to trigger wall hit audio feedback.

## Event Structure

- `ball: Entity` (required) — The ball entity that hit the wall.
- `wall: Entity` (required) — The wall entity that was hit.
- `impulse: Vec3` (optional) — The collision impulse, may be used for audio intensity.
- `timestamp: f64` (optional) — Time of collision, for latency measurement.

## Emission Rules

- Emitted only for ball-wall collisions (not paddle, brick, etc.).
- Emitted once per collision, even if multiple occur in the same frame.
- Must be processed by the audio system within the same frame if possible.

## Consumption Rules

- Audio system must attempt to play wall hit sound for every event, subject to concurrency limits.
- If concurrency limit is reached, event is logged/skipped.

## Testability

- Event emission and consumption must be testable via integration tests.
- Latency from event emission to audio playback must be measurable (<50ms in 99% of cases).

---

This contract ensures the BallWallHit event is well-defined, testable, and compliant with feature requirements.
