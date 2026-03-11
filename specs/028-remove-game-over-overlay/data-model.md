# Data Model: Remove Legacy Game Over Overlay

**Feature**: 028-remove-game-over-overlay **Date**: 2026-03-10

## Overview

This feature is primarily a subtraction/change-of-behavior feature.
The data model impact is the removal of one legacy UI entity path and cleanup of references to its marker component.

## Entities and Components

### Removed legacy entity: `GameOverOverlay`

**Current role**:

- Full-screen text UI entity spawned by `spawn_game_over_overlay`.
- Marker component queried by pause and cheat subsystems.

**Planned model change**:

- Remove legacy spawning path and references so gameplay no longer creates or depends on this entity.

**Validation rule**:

- During active gameplay and restart flows, world must contain `0` entities with `GameOverOverlay` marker.

### Retained entity: State-based game-over UI roots

**Role**:

- `GameState::GameOver` UI entities in `src/systems/ui/game_over.rs` remain the canonical game-over presentation path.

**Validation rule**:

- Existing game-over state entry/exit behavior remains unchanged by this feature.

### Retained entity: `PauseOverlay`

**Role**:

- Pause overlay remains independent of removed legacy game-over overlay marker.

**Validation rule**:

- Pause overlay spawn/despawn still follows `PauseState` and no longer requires overlay marker checks.

## Messages and Resources

### Retained message: `GameOverRequested`

**Role**:

- Buffered gameplay message emitted when lives reach zero.

**Change**:

- Legacy overlay consumer is removed; producer behavior remains unchanged in this feature.

**Validation rule**:

- Life-loss/game-over logic can continue emitting the message without creating gameplay overlay UI.

### Retained resource: `LivesState`

**Role**:

- Tracks lives remaining; used by respawn/game-over logic.

**Change**:

- No schema change in this feature.

## State Transitions

No new state transitions are introduced.

- `GameState` flow remains as implemented.
- Feature only removes legacy gameplay overlay side effects.

## Invariants after implementation

- No legacy gameplay overlay appears after game-over -> restart path.
- No legacy gameplay overlay appears on first run.
- No legacy overlay entity accumulation across repeated cycles.
- No replacement overlay is introduced by this feature.
