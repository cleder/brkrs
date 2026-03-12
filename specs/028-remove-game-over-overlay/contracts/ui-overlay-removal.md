# Contract: Legacy Game-Over Overlay Removal

**Feature**: 028-remove-game-over-overlay **Date**: 2026-03-10

## Purpose

Define behavioral contracts for removing legacy gameplay game-over overlay behavior while preserving non-overlay game-over flow.

## Contract C1: Legacy overlay spawn contract (removed)

**Before**:

- `spawn_game_over_overlay` consumed `MessageReader<GameOverRequested>` and could spawn `GameOverOverlay` UI entity.

**After**:

- No system in gameplay UI path may spawn `GameOverOverlay`.
- `UiPlugin` no longer registers legacy overlay spawning system.

**Verification**:

- Integration tests assert `Query<With<GameOverOverlay>>` count is always `0` in active gameplay and restart flows.

## Contract C2: Message/Event separation

**Requirement**:

- Keep `GameOverRequested` as buffered `Message` in respawn/life-loss domain.
- Do not repurpose it as observer/event trigger in this feature.

**Rationale**:

- Overlay removal is presentation cleanup, not a messaging architecture rewrite.

## Contract C3: Pause system decoupling

**Requirement**:

- Pause overlay behavior MUST NOT depend on presence/absence of `GameOverOverlay` marker.

**Verification**:

- Pause tests assert expected pause overlay behavior independently from legacy overlay entities.

## Contract C4: Restart regression guard

**Requirement**:

- After reaching zero lives and starting a new game, gameplay frames must not show legacy overlay.

**Verification**:

- Acceptance test runs restart flow and checks zero overlay entities for >=10 update frames.

## Contract C5: No replacement overlay

**Requirement**:

- Feature MUST NOT introduce any new game-over gameplay overlay entity/system.

**Verification**:

- Code review and tests confirm no replacement overlay spawn path is added in active gameplay systems.

## Verification Checklist (C1-C5)

- [ ] C1 verified: no `GameOverOverlay` spawn system remains registered in gameplay UI paths.
- [ ] C2 verified: `GameOverRequested` remains a buffered `Message` flow and is not converted to observer/event triggers.
- [ ] C3 verified: pause behavior no longer depends on `GameOverOverlay` marker presence.
- [ ] C4 verified: restart regression tests assert zero legacy overlay entities across >=10 updates.
- [ ] C5 verified: no replacement game-over overlay entity/system introduced in active gameplay.
- [ ] Hierarchy safety verified: touched UI paths use relationship-safe APIs and do not manually mutate `Parent`/`Children`.
