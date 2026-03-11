# Research: Remove Legacy Game Over Overlay

**Feature**: 028-remove-game-over-overlay **Date**: 2026-03-10 **Purpose**: Resolve technical decisions for removing the legacy gameplay overlay while preserving non-overlay game-over flow.

## Decision 1: Overlay removal scope

**Decision**: Remove legacy `GameOverOverlay` spawning behavior from gameplay UI entirely and eliminate module-level wiring for it.

**Rationale**: The spec requires complete removal of legacy overlay behavior in all flows and explicitly forbids replacement overlay UI in this feature.

**Alternatives considered**:

- Keep module but gate behind state checks: rejected because dormant code can regress and still represents legacy behavior.
- Keep spawning but auto-despawn on restart: rejected because overlay still appears, violating FR-001/FR-002.

## Decision 2: Game-over handling path

**Decision**: Keep `GameState::GameOver` state logic and `src/systems/ui/game_over.rs` UI unchanged.

**Rationale**: The specification only removes the legacy overlay in gameplay flow; non-overlay game-over handling remains in scope and should continue to function.

**Alternatives considered**:

- Remove all game-over UI: rejected because out of scope and would alter intended game flow.
- Migrate legacy overlay text into state UI: rejected because feature forbids replacement overlay work.

## Decision 3: Event/message model after removal

**Decision**: Continue emitting `GameOverRequested` as a buffered message from respawn/lives systems; remove only the legacy UI consumer.

**Rationale**: Message producers are used by gameplay/life-loss logic and tests; decoupling producer removal from UI cleanup minimizes regression risk.

**Alternatives considered**:

- Delete `GameOverRequested` message type immediately: rejected because it likely impacts broader life-loss contracts and is not required for overlay removal.
- Convert to observers: rejected because this feature does not need immediate reactive replacement behavior.

## Decision 4: Pause and cheat interactions

**Decision**: Remove hard dependency on `GameOverOverlay` marker in pause and cheat paths.

**Rationale**: Once overlay is removed, these cross-module references become dead coupling and create compilation/test churn.

**Alternatives considered**:

- Keep marker type only for compatibility: rejected because it preserves legacy API surface without behavior.

## Decision 5: Testing strategy and regression depth

**Decision**: Add/adjust integration tests to verify zero legacy overlay entities across restart and multi-frame gameplay cycles (>=10 updates), and update prior overlay-coupled tests.

**Rationale**: Spec success criteria require regression coverage for repeated restart cycles and no visual artifact reintroduction.

**Alternatives considered**:

- Rely only on manual QA: rejected by constitution TDD-first requirements.
- Single-frame assertions only: rejected because regressions can reappear after subsequent updates.

## Bevy 0.17 best-practice notes used by this plan

- Keep buffered message usage (`MessageWriter`/`MessageReader`) for existing life-loss stream; do not introduce observer/event confusion.
- Avoid panicking queries while removing systems and tests.
- Keep UI updates change-driven where touched (no unconditional per-frame UI mutation).
- Maintain hierarchy safety by avoiding manual `Children`/`Parent` edits.

## Open clarifications

None.
All `NEEDS CLARIFICATION` items are resolved for planning.
