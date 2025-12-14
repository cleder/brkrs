# Data Model: Ball Lives Counter

**Feature**: [spec.md](spec.md) **Created**: 2025-12-14

## Entities & State

### LivesState (Resource)

**Purpose**: The canonical remaining-lives state for the current play session.

**Fields**:

- `lives_remaining` (integer, non-negative): number of remaining balls/lives.
- `on_last_life` (boolean): whether the player is currently on the last life (optional; keep only if used).

**Validation rules**:

- `lives_remaining` MUST never be negative.
- Decrement MUST clamp at 0.

**State transitions**:

- Session start → `lives_remaining = 3`
- On `LifeLostEvent` when `lives_remaining > 0` → `lives_remaining = lives_remaining - 1`
- On `LifeLostEvent` when `lives_remaining == 0` → no change

### LifeLostEvent (Message)

**Purpose**: Domain signal that a life was lost and lives should be decremented.

**Key fields** (conceptual):

- Lost ball identifier
- Loss cause
- Respawn spawn transform(s)

### GameOverRequested (Message)

**Purpose**: Domain signal that remaining lives are exhausted and the game-over UX should be displayed.

**Fields**:

- `remaining_lives` (integer): expected to be 0.

### UI Markers (Components)

**Lives Counter UI (Component marker)**

- Marks the UI entity that displays remaining lives.

**Game Over Overlay UI (Component marker)**

- Marks the UI entity that displays the game-over message.

## Relationships

- `LivesState` is updated in response to `LifeLostEvent`.
- When `LivesState.lives_remaining` becomes 0 due to a `LifeLostEvent`, `GameOverRequested` is emitted.
- UI systems read `LivesState` to render the lives counter and read/track `GameOverRequested` to display the game-over overlay.
