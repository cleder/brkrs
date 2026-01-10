# Research: Merkaba Rotor Brick

## Decisions

- Decision: Spawn Delay = 0.5s
  - Rationale: Quick, perceivable delay that heightens urgency without feeling unfair.
  - Alternatives considered: 1.0s (balanced but slower), 1.5s (too long), 2.0s (predictable but sluggish).

- Decision: Minimum Z-Speed = 3.0 u/s (forward motion on XZ plane)
  - Rationale: Ensures consistent horizontal traversal; prevents stalling after vertical bounces.
  - Alternatives considered: 2.0 (too slow), 5.0 (more challenging), 8.0 (too fast).

- Decision: Spawn Location = Brick position at destruction
  - Rationale: Spatially intuitive; players anticipate hazard emerging from destroyed rotor brick.
  - Alternatives considered: Center field (predictable), random (unpredictable), top edge (consistent but arbitrary).

- Decision: Rotor Brick Visual Distinction = Unique texture pattern
  - Rationale: Clear identification; requires texture asset selection but improves UX.
  - Alternatives considered: Unique color (simpler), pulsing animation (complex), icon overlay (extra rendering).

- Decision: Audio Sourcing = Placeholder/synthesized sounds
  - Rationale: Fast implementation; allows later upgrade to production assets.
  - Alternatives considered: Existing game audio (may not match), new custom sounds (higher cost).

## Event System Architecture

- Messages (Buffered) for spawn requests: `SpawnMerkabaMessage` carries brick world position and initial direction seed; processed by a spawn system that applies a 0.5s timer before entity creation.
- Observers/Events (Immediate) for:
  - Collision-triggered audio (wall/brick/paddle): distinct SFX per surface.
  - Paddle contact → life loss: triggers ball and merkaba despawn; stops helicopter loop.
  - Helicopter loop management: start when merkaba_count > 0; stop when returns to 0.

## Physics & Movement

- Initial angle: horizontal ±20° variance.
- Maintain min z-speed: clamp/boost z-component to ≥ 3.0 u/s (forward motion, with lateral X drift limited to half of Z speed).
- Constrain to gaming plane: z fixed (or tightly limited); rotation around z-axis.
- Bounces: Restitution tuned to feel consistent with balls; bricks not damaged by merkaba.

## Testing Strategy (TDD)

- Failing tests first for message emission, delayed spawn timing, min z-speed (forward motion), bounce responses, goal despawn, paddle contact consequences, audio triggers and loop lifecycle.
- Integration tests in `tests/` exercising ECS flows and resources.

## Assets & Resources

- Texture: choose or create placeholder texture for rotor brick distinct pattern.
- Audio: synthesized/placeholder SFX for collisions; loop for helicopter blades.
  Store handles in resources; start/stop via counts.
