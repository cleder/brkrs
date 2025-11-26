# Implementation Plan: Ball Respawn System

**Branch**: `002-ball-respawn` | **Date**: 2025-11-25 | **Spec**: `specs/002-ball-respawn/spec.md`
**Input**: Feature specification for continuous ball+paddle respawn after life loss.

## Summary

Implement a physics-driven respawn loop so any ball that falls below the lower boundary triggers a `LifeLostEvent`, queues a one-second delay using Bevy's global `Time` resource, and respawns the ball/paddle at their matrix-defined transforms with zero velocity. The respawn feature must emit events for the lives/game-over system, freeze controls until the player relaunches the ball, and handle repeated losses or multi-ball scenarios without stalling gameplay.

## Technical Context

**Language/Version**: Rust 1.81 (Rust 2021 edition via rustup)
**Primary Dependencies**: Bevy 0.16 (ECS, scheduling, `Time`), bevy_rapier3d 0.31 (physics + collision sensors), serde/ron (level matrix assets)
**Storage**: File-based RON assets under `assets/levels/`; no runtime persistence
**Testing**: `cargo test`, Bevy system unit tests, targeted WASM smoke build for timer parity
**Target Platform**: Native desktop (Linux/macOS/Windows) plus WASM (Chrome/Firefox)
**Project Type**: Single Bevy game workspace (`src/` + `assets/`)
**Performance Goals**: Maintain 60 FPS and avoid respawn-induced GC spikes; respawn delay timing tolerance ±16 ms
**Constraints**: ECS-first systems, physics-driven detection (Rapier sensors), 1 s delay sourced from Bevy `Time`, ball remains stationary until player launch, paddle controls disabled during delay, respawn positions always derived from matrix markers or fallback center
**Scale/Scope**: Single-player brick breaker with ≤4 balls simultaneously; dozens of entities per level; respawn flow must succeed for 100 consecutive losses (SC-002)

## Constitution Check

| Principle | Gate Assessment |
|-----------|-----------------|
| ECS-First | Respawn logic implemented as system sets operating on components/events → **PASS** |
| Physics-Driven Gameplay | Lower-goal detection relies on Rapier sensors, not manual transforms → **PASS** |
| Modular Feature Design | Communication occurs via `LifeLostEvent`, `RespawnScheduled`, `GameOverRequested`; lives system stays decoupled → **PASS** |
| Performance-First | Minimal allocations (spawn data cached), timer per feature, 60 FPS target tracked in SC-002 → **PASS** |
| Cross-Platform Compatibility | Uses Bevy `Time` instead of platform clocks; no native-only APIs → **PASS** |

Re-check after design: still PASS (no new violations introduced).

## Project Structure

### Documentation (this feature)

```text
specs/002-ball-respawn/
├── plan.md          # This file
├── research.md      # Phase 0 findings
├── data-model.md    # Entities, resources, events
├── quickstart.md    # Build/test/verification guide
├── contracts/       # Conceptual event contract (gameplay.yaml)
├── spec.md          # Feature spec with clarifications
└── tasks.md         # Produced by /speckit.tasks (future)
```

### Source Code (repository root)

```text
src/
├── main.rs                # App entry; schedules plugins/system sets
├── level_loader.rs        # Level matrix parsing into spawn resources
└── systems/
   ├── mod.rs             # System set registration
   └── grid_debug.rs      # Existing helpers (extend with respawn module)

assets/
└── levels/
   ├── level_001.ron
   └── level_002.ron

tests/
└── (integration tests live under src/tests modules via `cfg(test)`)
```

**Structure Decision**: Keep single Bevy game crate. Add a new `systems/respawn.rs` module registered in `systems/mod.rs`; reuse existing asset loader and resources under `src/` rather than introducing new crates.

## Implementation Phases

### Phase 0 – Research & Unknowns Resolution

Key findings recorded in `research.md`:

1. **Lower-goal detection** – Use Rapier sensors + `CollisionEvent::Started` to emit `LifeLostEvent` (ensures physics compliance).
2. **Timer source** – Manage a `RespawnSchedule` resource ticking a Bevy `Timer` via global `Time` for deterministic 1 s delays across native/WASM.
3. **Freeze/lock markers** – `BallFrozen` and `InputLocked` components shield other systems from mutating state until relaunch.
4. **Lives/game-over handshake** – Respawn emits `LifeLostEvent`, waits on `LivesState`, aborts when zero lives remain, emits `GameOverRequested` instead of respawn.
5. **Multi-ball safety** – Cache spawn transforms per entity via `RespawnHandle` and only respawn the lost ball; log and fallback to board center when markers missing.

All technical unknowns resolved; no pending research blockers.

### Phase 1 – Design, Data Model & Contracts

- **Resources & components** (see `data-model.md`):

  - `SpawnPoints` (paddle, ball, fallback) built during level load.
  - `RespawnSchedule` (pending request, timer, last_loss) to coordinate delay.
  - Components: `RespawnHandle`, `BallFrozen`, `InputLocked`, `LowerGoal` markers.
  - Events: `LifeLostEvent`, `RespawnScheduled`, `RespawnCompleted`, `GameOverRequested`.

- **Systems layout**:

  - `detect_ball_loss_system` (Rapier event reader) → emit `LifeLostEvent` & despawn ball.
  - `lives_ack_system` (existing or stub) updates `LivesState` and optionally triggers `GameOverRequested`.
  - `respawn_scheduler_system` consumes `LifeLostEvent`, reads `LivesState`, populates `RespawnSchedule`, adds `BallFrozen`/`InputLocked` markers, zeroes velocities.
  - `respawn_executor_system` ticks timer, respawns transforms when finished, emits `RespawnCompleted` and removes lock markers.
  - `ball_launch_system` listens for player launch input to remove `BallFrozen` and apply initial velocity.

- **Contracts**: `contracts/gameplay.yaml` documents conceptual HTTP endpoints mirroring these events for automation/telemetry (life loss, respawn completion, game over).

- **Agent context**: `.specify/scripts/bash/update-agent-context.sh copilot` executed to ensure Copilot instructions include Bevy/Rapier respawn constraints (see repo logs for confirmation).

### Phase 2 – Implementation & Testing Plan

- **Code tasks**:

  - Add new respawn module with system set ordering: detection (Physics schedule) → scheduling (PostUpdate) → execution (FixedUpdate) → input unlock (Update).
  - Extend `level_loader.rs` to populate `SpawnPoints` and attach `RespawnHandle` components.
  - Introduce event structs/resources in `src/` plus tests validating serialization/defaults.

- **Testing**:

  - Unit tests for `SpawnPoints` extraction (missing markers fallback) and `RespawnSchedule` timer logic using Bevy `App` + `Time` stepping.
  - Integration/system tests ensuring `LifeLostEvent` triggers respawn exactly once per loss and respects `GameOverRequested` short-circuit.
  - Manual quickstart steps in `quickstart.md` (native + WASM) verifying stationary ball + control lock + repeated respawns.

- **Observability**:

  - Add `info!` logs when respawn schedules/executes and when game over cancels respawn for easier QA.

- **Risks & mitigations**:

  - **Timer drift**: rely on `Time` resource and deterministic tests; add asserts in tests for ±16 ms tolerance.
  - **Multi-ball race**: ensure `RespawnSchedule` queues per entity; consider future extension to a Vec but start with single pending + warning if already active.

## Complexity Tracking

No constitution violations; table not required.
