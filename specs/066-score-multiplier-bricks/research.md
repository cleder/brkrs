# Phase 0 Research: Score Multiplier Bricks

## Decision 1: Use a dedicated score multiplier resource in ECS state

- Decision: Track active multiplier as a runtime ECS resource (`1x` default; values `1,2,3,4`).
- Rationale: Multiplier is global scoring state and should persist across frames and level transitions until explicit reset triggers.
- Alternatives considered:
  - Derive multiplier on demand from recent brick history: rejected due to unnecessary event-history coupling and race ambiguity.
  - Store multiplier on ball entities: rejected because multiplier is global and must remain consistent in multi-ball play.

## Decision 2: Apply multiplier only to brick-destruction score awards

- Decision: Scope multiplier scaling strictly to brick-destruction score awards.
- Rationale: Matches clarified product behavior and avoids side effects on non-brick score sources.
- Alternatives considered:
  - Apply to all score sources: rejected as out of scope and behaviorally surprising.
  - Apply to brick points plus bonus sources: rejected due to ambiguous boundary and unnecessary coupling.

## Decision 3: Activation timing is forward-only

- Decision: The multiplier brick that activates a multiplier is scored at normal base value; multiplier applies from subsequent brick hits.
- Rationale: Eliminates same-hit ordering ambiguity and keeps score determinism for tests.
- Alternatives considered:
  - Apply multiplier to activating brick hit: rejected due to circular timing ambiguity.
  - Zero-point multiplier bricks: rejected because docs define base score.

## Decision 4: Reset only on actual life decrement

- Decision: Reset multiplier to `1x` only when player life counter decreases.
- Rationale: Aligns with clarified requirement and prevents unintended reset when extra balls despawn in multi-ball scenarios.
- Alternatives considered:
  - Reset on any ball despawn: rejected as too broad and gameplay-breaking for multi-ball.
  - Reset on both ball despawn and life decrement: rejected for same reason.

## Decision 5: Persist through level transitions unless reset trigger occurs

- Decision: Do not reset multiplier on level transitions unless life decrement or explicit replacement multiplier hit occurs.
- Rationale: Matches clarified feature behavior and simplifies player mental model.
- Alternatives considered:
  - Reset on level load: rejected as contrary to clarified requirement.

## Decision 6: Message-driven scoring/life integration

- Decision: Use existing buffered Messages flow for score/life events; avoid introducing observer-only reset logic.
- Rationale: Matches constitution guidance for buffered gameplay streams and keeps frame-order behavior predictable.
- Alternatives considered:
  - Observer-triggered reset path: rejected due to mixed-pattern complexity and weaker determinism.

## Decision 7: Show multiplier indicator only for active multipliers above 1x

- Decision: Render a score-adjacent UI indicator beneath the score display for `x2`, `x3`, and `x4`, and hide it when multiplier is `1x`.
- Rationale: Gives players clear feedback about active scoring state while keeping the neutral state visually uncluttered.
- Alternatives considered:
  - Always show `x1`: rejected as unnecessary noise.
  - No indicator: rejected because active multiplier state would be hidden from the player.

## Decision 8: Drive UI updates from multiplier-state changes only

- Decision: Update multiplier indicator text/visibility only when multiplier state changes.
- Rationale: Matches Bevy change-detection guidance and avoids unnecessary per-frame UI writes.
- Alternatives considered:
  - Rewrite indicator every frame: rejected due to avoidable UI churn and constitution guidance.
