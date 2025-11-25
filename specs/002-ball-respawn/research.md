# Research Findings: Ball Respawn System

## Research 1: Lower-goal detection with Rapier sensors

- **Decision**: Attach a Rapier sensor collider (`Sensor` flag + `ActiveEvents::COLLISION_EVENTS`) to the lower boundary entity and listen for `CollisionEvent::Started` pairs of (`Ball`, `LowerGoal`). On detection, emit a `LifeLostEvent` and despawn the ball in the physics schedule.
- **Rationale**: Sensors keep physics-driven gameplay intact (Principle II) while avoiding unintended impulses. Events decouple detection from respawn orchestration so the respawn system can live in its own system set and remain testable.
- **Alternatives considered**: (1) Polling the ball `Transform` y-value every frame—breaks physics-first rule and risks tunneling. (2) Using `IntersectionEvent`s via queries—adds manual broad-phase logic without benefit over sensors already supported by Rapier.

## Research 2: Respawn delay tracking via global Time resource

- **Decision**: Store a `RespawnSchedule` resource containing `Option<RespawnRequest>` plus a Bevy `Timer` constructed from `Time::delta_seconds()` each frame. Systems tick the timer using the global `Time` resource so both native and WASM builds share deterministic 1-second delays.
- **Rationale**: Using `Time` avoids per-entity timers and honors the clarification that the delay must be sourced from Bevy time. A resource keeps the scheduling logic single-owner and simplifies serialization for tests.
- **Alternatives considered**: (1) Using Rapier's physics timestep—ties respawn cadence to physics frequency and complicates pausing. (2) Spawning ad-hoc `Timer` components on entities—harder to coordinate across multiple balls and wastes memory.

## Research 3: Maintaining stationary ball & disabled controls

- **Decision**: Introduce `BallFrozen` and `InputLocked` marker components. While present, the movement/input systems early-return and Rapier velocity integration zeroes out linear velocity. Removal happens once the respawn timer completes and the player explicitly launches the ball.
- **Rationale**: Components make the lock state visible to ECS queries and satisfy clarifications about stationary ball + disabled paddle controls. They also prevent accidental movement from other systems (e.g., power-ups) because those systems can filter on the markers.
- **Alternatives considered**: (1) Relying solely on timers without explicit markers—other systems could still mutate velocity or accept input. (2) Pausing the entire physics world—overkill and would stop bricks or other moving pieces.

## Research 4: Event contract with lives/game-over system

- **Decision**: The respawn feature emits `LifeLostEvent` with metadata (ball entity, cause) and waits for a `LivesState` resource or `GameOverEvent` to indicate whether respawn should proceed. When zero lives remain, the respawn system aborts scheduling and emits `GameOverRequested` for UI/state machines.
- **Rationale**: Events keep modules independent (Principle III) and align with requirement FR-009/FR-010. The respawn feature can be developed/tested before the lives UI exists by mocking the event stream.
- **Alternatives considered**: (1) Embedding a lives counter directly inside the respawn system—violates modularity and complicates future HUD integration. (2) Polling global state every frame—less explicit and harder to test.

## Research 5: Multi-ball safety and fallback spawn positions

- **Decision**: Cache spawn transforms in `SpawnPoints` resource keyed by `BallId`. When multiple balls exist, only the despawned ball gets a respawn entry. If the level grid lacks `1` or `2`, use a constant fallback at the board center with configurable offsets logged as warnings.
- **Rationale**: Matches FR-008 and ensures multi-ball power-ups don't all reset due to one loss. Centralizing spawn data avoids re-reading level assets during play and keeps per-ball state light-weight.
- **Alternatives considered**: (1) Respawning all balls together—breaks multi-ball requirement. (2) Re-parsing the level matrix on every respawn—unnecessary IO and potential stutter, especially on WASM.
