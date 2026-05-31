# Contract: Collision Feedback Observer

## Purpose

Define the internal observer contract used to trigger immediate collision visual feedback.

## Event Type

`CollisionFeedbackTriggered` (observer/event-style signal)

## Producer Responsibilities

Producers (collision handling systems) MUST emit one trigger per qualifying collision for:

- Ball vs Wall
- Ball vs Paddle
- Ball vs Brick

Producer MUST NOT emit triggers when gameplay is paused or in non-playing states.

## Payload Schema

```text
ball_entity: Entity
target_entity: Entity
target_kind: Wall | Paddle | Brick
contact_point: Vec3
brick_destroyed_on_impact: bool
```

## Consumer Responsibilities

Consumer (collision feedback system) MUST:

- Spawn one effect instance per trigger.
- Spawn at exact `contact_point`.
- Spawn 8-16 particles.
- Assign lifetime in [0.20, 0.35] seconds.
- Despawn effect when lifetime expires.

Consumer MUST NOT:

- Merge multiple triggers into one effect.
- Apply per-frame cap, queue, or deferred replay behavior.

## Invariants

- One qualifying collision maps to one spawned feedback effect.
- Effects suppressed during pause are dropped permanently (no replay).
- Brick-destruction collisions still produce effect before cleanup completes.

## Verification Mapping

- FR-011: one-per-collision, no cap/queue
- FR-012: lifetime window
- FR-013: particle count window
- FR-014: pause suppression + no replay
- FR-015: exact contact-point spawn
- FR-016: brick-destruction still spawns
