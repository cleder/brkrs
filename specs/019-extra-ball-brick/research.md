# Research: Extra Ball Brick (Brick 41)

## Decision: Event transport for life and audio

- **Decision**: Use Messages for life-award and audio trigger; no observers.
- **Rationale**: Life updates and audio playback can tolerate next-schedule delivery, benefit from batching and cursored reads, and align with existing score/life pipelines.
  Avoids ordering hazards between multiple balls and keeps message-event separation clear.
- **Alternatives considered**: Observers (rejected: ordering/duplication risk across simultaneous hits; not needed for immediate transform work).
  Direct resource mutation (rejected: bypasses constitution’s message separation, harder to test).

## Decision: Life clamping and idempotency

- **Decision**: Award exactly +1 life, clamped to configured max; guard with single-destruction path so multiple balls cannot double-grant.
- **Rationale**: Matches spec, keeps lives within design cap, prevents duplication from simultaneous collisions; leverages existing brick despawn flow.
- **Alternatives considered**: Unclamped increment (rejected: violates life cap).
  Per-ball gating (rejected: more state, redundant once brick despawns on first hit).

## Decision: Unique destruction sound handling

- **Decision**: Load one dedicated audio asset handle for brick 41 at startup/config; on destroy, enqueue audio Message referencing that handle; fallback to generic brick destruction sound if dedicated asset missing.
- **Rationale**: Asset reuse prevents repeated loads; audio via Messages stays consistent with other bricks; fallback preserves gameplay and feedback if asset unavailable.
- **Alternatives considered**: Observer-triggered immediate audio (rejected: unnecessary immediacy, adds ordering complexity).
  Hard-fail on missing asset (rejected: degrades gameplay and violates graceful handling).
