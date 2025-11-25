# Implementation Plan: Ball & Paddle Respawn + Level Flow

**Branch**: `002-ball-respawn` | **Date**: 2025-11-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-ball-respawn/spec.md` plus user-requested extensions (per-level gravity, paddle growth animation, fade overlays, level restart/advance flow).

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a resilient respawn + level-flow pipeline: detect when the ball exits via the lower goal, despawn ball and paddle, queue a one-second respawn delay, then recreate both entities at the matrix-defined start positions while freezing ball physics until a two-second paddle growth tween completes. Extend level definitions with optional gravity overrides so each level can tune Rapier gravity, temporarily zeroing gravity during respawn animations. Add progress tracking (lives left, cleared bricks, current level), a fade overlay that ramps opacity while transitioning between levels, and a manual restart shortcut via the `R` key. All behavior remains ECS- and physics-driven: dedicated plugins for respawn + level advancement, systems reading/writing Bevy resources/events, and level loader refactors to surface spawn points + overrides for every level.

## Technical Context

**Language/Version**: Rust 1.81 (Rust 2021 edition managed via rustup)

**Primary Dependencies**:

- Bevy 0.16 (ECS, renderer, state machine, timers)
- bevy_rapier3d 0.31 (physics integration + collision events for life loss)
- Serde 1.0 + RON 0.8 (level metadata, respawn/gravity overrides)
- bevy_tweening 0.14 (planned) for paddle scaling + fade overlay easings

**Storage**: File-based RON under `assets/levels/`; runtime-only ECS resources for respawn schedule and progress (no persistent storage)

**Testing**: `cargo test`, `cargo clippy --all-targets --all-features`, manual gameplay verification native + WASM (focus on repeated respawns, level restart/advance)

**Target Platform**: Native desktop (Linux/Windows/macOS) plus WASM builds served through wasm-bindgen

**Project Type**: Single Bevy crate with modular plugins

**Performance Goals**: Sustain 60 FPS on native + WASM; respawn/overlay systems must add <2 ms/frame; gravity overrides settle within one frame; no allocation spikes during life-loss loops

**Constraints**: ECS-first state (no global singletons), physics-driven movement (Rapier handles ball/paddle transforms outside of respawn reposition), 1 second respawn delay, 2 second paddle growth (ease-out cubic), overlay alpha ramp <500 ms, deterministic timers for cross-platform parity

**Scale/Scope**: Touches `main.rs`, `level_loader.rs`, new `components/`, `plugins/respawn.rs`, `plugins/level_flow.rs`, physics resources, two sample level files; supports 77 total levels via schema updates

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Evidence/Notes |
|-----------|--------|----------------|
| **I. ECS-First Architecture** | ✅ PASS | Respawn + transitions implemented as Bevy systems (no global state); spawn metadata stored in components/resources; timers via `ResMut<Timer>` or `Time::<Fixed>` handlers. |
| **II. Physics-Driven Gameplay** | ✅ PASS | Life loss triggered via Rapier sensor events; paddle/ball transforms updated through Rapier bodies (except controlled respawn set); gravity overrides applied through Rapier `GravityScale`. |
| **III. Modular Feature Design** | ✅ PASS | Feature isolated into `RespawnPlugin` and `LevelFlowPlugin` with explicit events/resources; system sets toggled via states. |
| **IV. Performance-First Implementation** | ✅ PASS | Systems scheduled in fixed timestep set, data-oriented queries, re-use overlay mesh/material; plan mandates profiling repeated respawns (>100 cycles). |
| **V. Cross-Platform Compatibility** | ✅ PASS | All dependencies supported on WASM; timers/input rely on Bevy abstractions; fade overlay uses standard material/shader path that works on web. |

**Overall Assessment**: ✅ **APPROVED** – Plan follows every constitutional principle; no waivers required.

## Project Structure

### Documentation (this feature)

```text
specs/002-ball-respawn/
├── plan.md              # This file (/speckit.plan output)
├── research.md          # Phase 0 research findings
├── data-model.md        # Phase 1 entity/resource definitions
├── quickstart.md        # Phase 1 getting-started + verification steps
├── contracts/
│   └── gameplay.yaml    # OpenAPI contracts for respawn/level control
└── tasks.md             # Phase 2 execution plan (via /speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── main.rs                  # Registers plugins, states, and schedule ordering
├── level_loader.rs          # RON parsing, spawn/despawn (extend for overrides)
├── components/
│   ├── ball.rs              # Ball marker + SpawnPoint component
│   ├── paddle.rs            # Paddle marker + PaddleGrowth state
│   └── progress.rs          # NEW: LevelProgress & RespawnCounters resources
├── plugins/
│   ├── respawn.rs           # NEW: RespawnPlugin wiring detection → spawn
│   └── level_flow.rs        # NEW: LevelFlowPlugin for fade + restart/advance
├── systems/
│   ├── grid_debug.rs        # Existing debug overlay
│   ├── respawn.rs           # NEW: respawn queue execute + ball freeze/unfreeze
│   ├── gravity.rs           # NEW: apply per-level gravity overrides
│   └── level_transition.rs  # NEW: fade overlay, next-level spawn, restart key
├── events.rs                # NEW: LifeLostEvent, RespawnQueued, LevelAdvance
└── resources/
    └── respawn.rs          # NEW: RespawnSchedule, PaddleGrowthTimers

tests/
├── integration/
│   └── respawn_flow.rs      # Future integration tests for respawn pipeline
└── unit/
    └── level_overrides.rs   # Future unit tests for level gravity parsing
```

**Structure Decision**: Single Bevy crate (Option 1). Expanding `components/`, `plugins/`, and `systems/` keeps respawn + level-flow logic modular and testable. Documentation remains co-located under `specs/002-ball-respawn/` for feature-specific artifacts.

## Constitution Check (Post-Design)

| Principle | Status | Evidence/Notes |
|-----------|--------|----------------|
| **I. ECS-First Architecture** | ✅ PASS | Data model locks critical state (SpawnPoint components, LevelProgress resource) so systems stay declarative; contracts + quickstart reinforce ECS ownership of gameplay data. |
| **II. Physics-Driven Gameplay** | ✅ PASS | Research decisions confirm Rapier collisions + gravity overrides drive mechanics; no manual transform hacks beyond controlled respawn set. |
| **III. Modular Feature Design** | ✅ PASS | Separate plugins + documented events keep respawn, gravity, fade overlay isolated; contracts describe clear boundaries. |
| **IV. Performance-First Implementation** | ✅ PASS | Respawn schedule + fade overlay reuse shared timers/materials; plan mandates profiling repeated respawns; data model avoids per-entity allocations. |
| **V. Cross-Platform Compatibility** | ✅ PASS | Quickstart includes WASM checks; animations rely on `bevy_tweening` which supports web; no platform-specific APIs introduced. |

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| *None* | N/A | N/A |
