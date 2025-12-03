# Data Model: Ball Respawn System

## Assets

### LevelDefinition (RON)

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| `grid` | `[[i32; 22]; 22]` | Encodes spawn markers (`1` = paddle, `2` = ball) plus bricks. | Must remain 22×22 with at least one `1` and one `2`. |
| `level_id` | `String` | Matches file stem (e.g., `level_001`). | Non-empty, unique per level. |

During load, the grid is converted into cached spawn transforms instead of being re-read on each respawn.

## Core Components

### `Ball`

- Marker attached to every ball entity.
- Paired with Rapier components (`Velocity`, `RigidBody`, `Collider`).
- Validation: every active ball must also carry `RespawnHandle` (see below) so the scheduler knows how to rebuild it.

### `Paddle`

- Marker for the controllable paddle entity.
- Has `Velocity`, `Transform`, and input-driven components.
- Stores `InitialTransform` component with the spawn transform resolved from the matrix.

### `RespawnHandle`

| Field | Type | Description |
|-------|------|-------------|
| `spawn_point` | `Vec3` | Cached world position derived from the level matrix (`1` or `2`). |
| `rotation` | `Quat` | Orientation to restore. |
| `entity_kind` | `RespawnEntityKind` (`Ball`/`Paddle`) | Distinguishes which entity this handle restores |

Attached to both paddle and ball so systems can reposition without re-reading assets.

### `BallFrozen`

- Marker applied to a ball while respawn delay is active.
- Systems that impart velocity must check `Without<BallFrozen>` before applying impulses.

### `InputLocked`

- Marker applied to the paddle entity during respawn delay.
- The paddle control system exits early when this marker is present, fulfilling FR-012.

### `LowerGoal`

- Marker on the lower-boundary sensor collider.
- Used alongside Rapier collision events to detect ball loss.

## Resources

### `SpawnPoints`

| Field | Type | Description |
|-------|------|-------------|
| `paddle` | `Vec3` | Spawn derived from matrix value `1`. |
| `ball` | `Vec3` | Spawn derived from matrix value `2`. |
| `fallback_center` | `Vec3` | Used when grid markers are missing; defaults to board center. |

Provides instant lookup for respawn scheduling and is rebuilt on level load.

### `RespawnSchedule`

| Field | Type | Description |
|-------|------|-------------|
| `pending` | `Option<RespawnRequest>` | Contains entity IDs to respawn plus spawn transforms. |
| `timer` | `Timer` | Configured for 1 second using Bevy `Time` resource. |
| `last_loss` | `Duration` | Tracking consecutive life losses (for SC-002 testing). |

`RespawnRequest` fields: `ball_entity`, `paddle_entity`, `ball_spawn`, `paddle_spawn`, `remaining_lives` (mirrors latest `LivesState`).

### `LivesState`

- Shared resource updated by the separate lives system.
- Fields: `lives_remaining: u8`, `on_last_life: bool`.
- The respawn system reads this resource when handling `LifeLostEvent`; it never mutates it directly.

## Events

| Event | Payload | Emitted By | Consumed By |
|-------|---------|------------|-------------|
| `LifeLostEvent` | `{ ball: Entity, cause: LifeLossCause }` | Lower-goal collision system | Lives system (to decrement), respawn scheduler |
| `GameOverRequested` | `{ remaining_lives: u8 }` | Lives system when counter hits zero | Game state machine (to enter GameOver screen) |
| `RespawnScheduled` | `{ ball: Entity, paddle: Entity, completes_at: f64 }` | Respawn scheduler when timer starts | UI feedback / debugging |
| `RespawnCompleted` | `{ ball: Entity }` | Respawn system after transforms/velocities reset | Input system to re-enable launch |

Event-driven flow keeps the respawn feature modular and satisfies FR-009/FR-010.

## State Transitions

1. `CollisionEvent` (Ball + LowerGoal) ⇒ emit `LifeLostEvent` and remove ball entity.
2. Lives system decrements `LivesState`; when lives > 0 it echoes `LifeLostAck` (implicit via resource) so respawn scheduler enqueues `RespawnRequest` and starts `RespawnSchedule.timer`.
3. While the timer is active, `BallFrozen` and `InputLocked` markers remain attached.
4. When `timer.finished()` is true, respawn system respawns ball + paddle at cached transforms, keeps velocity zero, and removes the lock markers once the player relaunches the ball.
5. If `LivesState.lives_remaining == 0`, the scheduler aborts and emits `GameOverRequested` instead of respawning.

## Validation Rules

- Each level load must yield exactly one paddle and one ball spawn; missing markers trigger warnings and fallback center positions per FR-008.
- Only one `RespawnRequest` may be active at a time.
  A second `LifeLostEvent` while `pending.is_some()` should be queued for after the current respawn completes to satisfy multi-loss handling.
- Systems manipulating paddle/ball transforms must honor `BallFrozen`/`InputLocked` markers to keep initial velocity zero until the player launches.
