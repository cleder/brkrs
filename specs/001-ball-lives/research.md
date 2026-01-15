# Phase 0 Research: Ball Lives Counter

**Feature**: [spec.md](spec.md) **Plan**: [plan.md](plan.md) **Created**: 2025-12-14

## Findings & Decisions

### Decision 1: Decrement lives in the respawn scheduling system

**Decision**: Decrement the remaining lives count while processing `LifeLostEvent` messages in the respawn scheduling stage, so each loss produces a correct “post-decrement” remaining-lives value.

**Rationale**:

- The project already centralizes loss handling in the respawn pipeline (`LifeLostEvent` → respawn scheduling / game over request).
- Handling decrement at the same point where respawn/game-over decisions are made avoids inconsistencies when multiple losses occur in a single update.
- Using a clamp-at-zero decrement prevents underflow and satisfies the “never negative” requirement.

**Alternatives considered**:

- Decrement in a separate “lives system” that runs before scheduling.
  - Rejected because per-event remaining-lives values can become ambiguous if multiple loss events occur in the same frame unless additional per-event bookkeeping is added.

### Decision 2: Represent lives as a resource (single source of truth)

**Decision**: Use the existing `LivesState` resource as the canonical remaining-lives state for the current play session.

**Rationale**:

- The repo already defines and initializes `LivesState`.
- A resource is a natural fit for single-player, global session state.

**Alternatives considered**:

- Store remaining lives on a dedicated “player” entity.
  - Rejected as unnecessary complexity for a single-player game with no existing player entity abstraction.

### Decision 3: Emit game-over intent via the existing message type

**Decision**: Use the existing `GameOverRequested` message as the domain signal for “show Game Over UI”.

**Rationale**:

- The respawn pipeline already defines `GameOverRequested`.
- Keeping it event-driven preserves modularity and allows other systems to react in the future.

**Alternatives considered**:

- Add a new “GameOverEvent” message.
  - Rejected because it duplicates an existing intent message.

### Decision 4: Add UI using the existing “overlay + marker component” pattern

**Decision**: Implement two UI elements:

- A HUD lives counter (marker component, spawned once, updated when lives change)
- A full-screen “GAME OVER” overlay (marker component, spawned when game over is requested)

**Rationale**:

- Matches existing UI modules (pause overlay uses marker + spawn/despawn on state).
- Keeps UI concerns isolated under `src/ui/`.

**Alternatives considered**:

- Render lives using debug overlay or 3D text.
  - Rejected because existing UI already uses Bevy UI text and this is a HUD-like element.

### Decision 5: Wiring approach (consistent with existing patterns)

**Decision**: Wire the lives counter + game-over overlay systems similarly to existing UI systems:

- Implement UI modules under `src/ui/`.
- Register systems from `src/lib.rs` (as done for the palette UI), or via a small plugin if that improves ordering/encapsulation.

**Rationale**:

- The repository already uses both approaches; direct wiring in `lib.rs` is acceptable for UI.

### Decision 6: Pause vs Game Over overlay interaction

**Decision**: Ensure that when the game is over, the “GAME OVER” message is visible even if pause is toggled; avoid ambiguous overlay stacking.

**Rationale**:

- The requirement is explicit about showing game over when lives are exhausted.
- The simplest approach is to gate pause overlay spawning when game-over is active, or to ensure game-over overlay is spawned independently and can coexist.

**Alternatives considered**:

- Add complex overlay z-order management.
  - Rejected as unnecessary for this feature.

## Completed Unknowns (from plan)

- Where to decrement lives: decrement during loss processing in the respawn scheduling logic.
- UI wiring: add UI modules and wire from `lib.rs` (or a tiny plugin) following existing UI patterns.
- Pause interaction: ensure game-over message remains visible and not overridden by pause overlay.
