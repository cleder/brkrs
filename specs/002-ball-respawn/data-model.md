# Data Model: Ball & Paddle Respawn + Level Flow

## Asset: LevelDefinition (RON)

| Field | Type | Source | Description | Validation |
|-------|------|--------|-------------|------------|
| `grid` | `[[i32; 22]; 22]` | Existing level files | Encodes entities by numeric token (1=paddle, 2=ball, 10+ bricks). | Must remain 22×22; at least one paddle and one ball symbol present. |
| `gravity` | `Option<[f32; 3]>` | **New** per-level override | Custom gravity vector applied when the level loads. | Defaults to global gravity when `None`. Non-zero vector magnitude <= 50 to avoid instability. |
| `respawn_overrides` | `Option<RespawnConfig>` | **New** optional block | Allows overriding default respawn delay, paddle scale, freeze duration. | When provided, all durations must be >= 0. |
| `level_id` | `String` | Existing metadata | Unique human-readable ID (e.g., "level_001"). | Must match file stem. |
| `display_name` | `String` | Existing metadata | Player-facing label. | Non-empty. |

## Struct: RespawnConfig

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `ball_spawn` | `Option<Vec3>` | Explicit ball spawn; overrides grid lookup "2". | `None` → use grid derived spawn. |
| `paddle_spawn` | `Option<Vec3>` | Explicit paddle spawn; overrides grid lookup "1". | `None`. |
| `respawn_delay_ms` | `u64` | Delay between life loss and respawn. | 1000 ms. |
| `paddle_growth_ms` | `u64` | Duration of ease-out tween. | 2000 ms. |
| `freeze_ball_ms` | `u64` | How long the ball stays frozen. | `paddle_growth_ms`. |

## Component: SpawnPoint

Represents the initial transform derived from the level grid or overrides.

| Field | Type | Description |
|-------|------|-------------|
| `entity_type` | `SpawnPointKind` (`Paddle`/`Ball`) | Distinguishes ball vs paddle spawn usage. |
| `translation` | `Vec3` | Absolute world position used during respawn. |
| `facing` | `Quat` | Orientation (for paddle alignment). |

Validation: Each level must yield exactly one `SpawnPoint` for paddle and ball. Stored as components on the respective entities for rapid respawn without re-reading assets.

## Resource: LevelOverrides

Caches per-level settings derived from the asset.

| Field | Type | Notes |
|-------|------|-------|
| `gravity` | `Option<Vec3>` | When `Some`, overrides Rapier gravity when the level is active. |
| `respawn_config` | `RespawnConfig` | Fully resolved config (with defaults). |
| `level_id` | `String` | Keeps association for logging + telemetry. |

State Transition: Updated whenever a new level loads. Consumers watch for `Changed<LevelOverrides>` to adjust physics or timers.

## Resource: RespawnSchedule

| Field | Type | Description |
|-------|------|-------------|
| `pending` | `Option<RespawnRequest>` | Set when a life is lost; cleared once respawn executes. |
| `timer` | `Timer` | Counts down from `respawn_delay_ms`. |
| `queued_at` | `Instant` | Diagnostic timestamp for profiling repeated losses. |

`RespawnRequest` contains entity IDs to respawn, spawn points, and overrides for velocity reset. Ensures only one respawn executes at a time.

## Component: PaddleGrowth

| Field | Type | Description |
|-------|------|-------------|
| `tween` | `Animator<Transform>` handle | Manages scale animation via bevy_tweening. |
| `state` | `PaddleGrowthState` (`Idle`, `Growing`, `Complete`) | Governs when paddle can move normally. |

Business Rule: Ball receives `BallFrozen` marker while state != `Complete`.

## Resource: LevelProgress

| Field | Type | Description |
|-------|------|-------------|
| `current_level` | `usize` | Index into level manifest. |
| `lives` | `u32` | Remaining lives; decremented on respawn; hitting zero triggers GameOver state. |
| `cleared_bricks` | `u32` | Count maintained to detect completion. |
| `total_bricks` | `u32` | Set on level load for completion threshold. |
| `last_respawn_at` | `Instant` | Used to throttle repeated respawns + debugging. |

Transitions: `LevelProgress` resets counters when a new level loads, but `lives` persists unless level restart occurs.

## Resource: FadeOverlayState

| Field | Type | Description |
|-------|------|-------------|
| `entity` | `Entity` | Overlay quad (spawned once). |
| `phase` | `OverlayPhase` (`Hidden`, `FadingOut`, `FadingIn`) | Controls tween direction. |
| `timer` | `Timer` | Shared timer for fade durations (<500 ms per user requirement). |

## Events

| Event | Payload | Purpose |
|-------|---------|---------|
| `LifeLostEvent` | `Entity` (ball), `Entity` (paddle) | Emitted on lower-goal collision to begin respawn. |
| `RespawnQueuedEvent` | `RespawnRequest` | Signals scheduling happened—useful for UI or logging. |
| `RespawnCompleteEvent` | `Entity` (ball) | Unfreezes ball + re-enables input. |
| `LevelAdvanceEvent` | `usize` next level index | Blocks gameplay while fade overlay runs, then loads next level. |
| `LevelRestartRequested` | `()` | Sent by input system when `R` is pressed; merges into same level-flow path as completion. |

These events ensure modular boundaries between detection, animation, and loader subsystems.
