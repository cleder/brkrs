# Phase 0 Research: Collision Particle Feedback

## Decision 1: Use Observers for Immediate VFX Triggers

- Decision: Trigger collision feedback via Bevy observer events (`commands.trigger(...)` + `On<T>` handlers) rather than buffered messages.
- Rationale: The feature requires same-frame visual response on wall/paddle/brick impact.
  Observer semantics align with the constitution's immediate-reactivity guidance.
- Alternatives considered:
  - `MessageWriter/MessageReader`: Rejected because buffered delivery can defer handling to later schedule steps and is intended for frame-agnostic streams.

## Decision 2: Implement ECS-Managed Short-Lived Effect Entities (No New External Particle Crate)

- Decision: Represent each collision burst as ECS effect entities/components with deterministic lifetime cleanup (0.20-0.35s).
- Rationale: Keeps architecture consistent with existing ECS patterns, avoids introducing additional runtime dependencies, and allows straightforward integration with pause/state run conditions.
- Alternatives considered:
  - Introduce a third-party particle framework immediately: Rejected for this feature scope because current requirements can be met with lightweight ECS-driven visuals and existing Bevy rendering.

## Decision 3: Resolve Contact Point from Collision Context, Then Spawn Exactly There

- Decision: Use collision context to derive a world-space contact point and spawn feedback at that exact point; include a deterministic fallback path only if contact manifold data is unavailable.
- Rationale: Spec mandates exact collision contact-point spawning (including brick-destruction-on-impact path).
  Positioning at entity centers would violate clarified requirements.
- Alternatives considered:
  - Spawn at collider/entity center: Rejected as visually inaccurate and contrary to FR-015/FR-016.
  - Spawn at ball transform only: Rejected as inaccurate for wall and edge contacts.

## Decision 4: Preserve Clarified Burst and Pause Semantics Exactly

- Decision: Emit one effect per qualifying collision with no per-frame cap, but suppress all new effect creation while paused/non-playing and never replay suppressed effects after resume.
- Rationale: Directly encodes approved clarifications and avoids hidden buffering behavior.
- Alternatives considered:
  - Per-frame cap with drop/queue: Rejected by clarification (explicitly no cap, no queue replay).
  - Replay backlog after pause: Rejected by clarification.

## Decision 5: Reuse Existing Collision Entry Points Instead of New Collision Pipeline

- Decision: Integrate feedback trigger hooks into existing collision handling paths in `src/lib.rs` (`detect_ball_wall_collisions`, `mark_brick_on_ball_collision`, `read_character_controller_collisions`).
- Rationale: Current systems already classify wall/paddle/brick interactions and drive related audio/score/gameplay effects; extending them minimizes regression risk.
- Alternatives considered:
  - Build a parallel collision interpretation system: Rejected due to duplicate logic and ordering risk.
