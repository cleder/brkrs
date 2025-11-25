# Research Findings: Ball & Paddle Respawn + Level Flow

## Research 1: Rapier Lower-Goal Detection & Respawn Scheduling

- **Decision**: Use Rapier sensor colliders on lower goals with `ActiveEvents::COLLISION_EVENTS`, listen for `CollisionEvent::Started` where one entity carries `LowerGoal` marker and the other `Ball` marker, then emit a `LifeLostEvent` that kicks off respawn scheduling in a fixed update set.
- **Rationale**: Sensors avoid physical response while still emitting reliable collision events. Events decouple detection from respawn logic, keeping systems pure and testable, and aligns with ECS-first requirement.
- **Alternatives considered**: (1) Polling ball `Transform` y-position each frame—simpler but bypasses physics-driven gameplay. (2) Using Rapier intersection queries manually—more code and still requires event wiring. Sensors + collision events are idiomatic and cheap.

## Research 2: Paddle Growth Animation & Ball Freeze Timing

- **Decision**: Add `bevy_tweening` to drive a scaled `Transform` tween on the paddle (ease-out cubic, 0.0→1.0 over 2s) while toggling a `BallFrozen` component so Rapier velocity stays zero until tween completion.
- **Rationale**: `bevy_tweening` supplies battle-tested easing curves, handles elapsed time in both native + WASM, and integrates with ECS components. A `BallFrozen` marker keep systems simple: physics step checks for the component to skip velocity integration.
- **Alternatives considered**: (1) Manual timers per paddle with custom interpolation—duplicated code and more error-prone, especially across multiple paddles or future power-ups. (2) Shader-only scaling—would still require component state to lock ball physics, providing no added value.

## Research 3: Level-Specific Gravity Overrides

- **Decision**: Extend `LevelDefinition` with `gravity: Option<Vec3>` and `respawn_overrides: Option<RespawnConfig>`; when loading a level, update Rapier's `GravityScale` resource (or apply per-body gravity via `ExternalForce`) and cache overrides on a `LevelOverrides` resource.
- **Rationale**: Keeping overrides in level data keeps gameplay deterministic per level and avoids hardcoding per-level constants. Option type maintains backward compatibility with existing RON files.
- **Alternatives considered**: (1) Global config file keyed by level—extra IO and risk of divergence from level assets. (2) Per-entity gravity components—complicates ball/paddle setup and duplicates data already inherent to the level definition.

## Research 4: Level Advance & Restart Flow with Fade Overlay

- **Decision**: Represent progress in a `LevelProgress` resource (current level index, cleared bricks, lives). Level completion triggers a `LevelAdvanceEvent`, after which a fade overlay entity (quad + unlit material) tweens alpha (fade out → fade in) while a timer delays `LevelLoader::load` invocation. Keyboard input system listens for `KeyCode::R` to emit `LevelRestartRequested` that reuses the same overlay pipeline.
- **Rationale**: Central resource keeps transitions deterministic and easier to test. Overlay entity reused for both advance and restart, ensuring animation consistency. Event-driven restart avoids direct coupling between input and loader.
- **Alternatives considered**: (1) Immediately load next level without delay—does not satisfy user requirement for fade/anticipation. (2) Scene reload via commands—heavyweight and would respawn entire world instead of targeted entities, hurting performance.

## Research 5: Game Progress Persistence Within Session

- **Decision**: Track lives, score, and last respawn timestamp inside `LevelProgress` + `RespawnCounters` resources, persisting through level transitions but resetting when the player restarts via menu; expose debugging info through Bevy `info!` logs to verify state.
- **Rationale**: Resources remain alive across levels without storing data externally, aligning with ECS-first design. Logging progress aids manual verification until automated tests cover flows.
- **Alternatives considered**: (1) Storing progress inside individual entities (paddle/ball) leads to state loss when entities despawn. (2) Writing to disk between levels—overkill for single-session progress and would slow transitions.
