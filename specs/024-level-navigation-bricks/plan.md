# Implementation Plan: [FEATURE]

**Branch**: `024-level-navigation-bricks` | **Date**: 2026-01-31 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement two navigation bricks that enable player-controlled level progression:

- Brick 50 (Level Up): Advances to the next level when destroyed by ball collision
- Brick 54 (Level Down): Returns to the previous level when destroyed by ball collision

Both bricks are destructible, award 0 points (utility bricks), and trigger unique audio feedback.
On level boundaries (final/first level), bricks destroy with no transition: brick 50 shows victory screen on final level, brick 54 has no effect on level 1.
Level transitions clear all active game elements (balls, powerups, effects) and reset to target level's default state.

## Technical Context

<!-- **Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1 **Storage**: RON files in `assets/levels/` directory; in-memory ECS state only (no persistent storage) **Testing**: cargo test (integration tests + unit tests with TDD-first requirement) **Target Platform**: Native (Linux/Windows/macOS) + WASM (web) **Project Type**: Single Rust game project **Performance Goals**: 60 FPS (game-specific; matches existing constitution requirement) **Constraints**: Cross-platform (native + WASM); physics-driven gameplay; level state must persist across multiple frames **Scale/Scope**: 2 new brick types (50, 54); extend existing level transition system; minimal scope (single feature)

## Constitution Check

*GATE: Must pass before Phase 0 research.*
Re-check after Phase 1 design.

This check MUST verify compliance with the constitution, including **Test-Driven Development (TDD)** gates:

- Tests are defined and committed prior to implementation efforts for each story/feature.
- A proof-of-failure commit (tests that FAIL) MUST exist in the branch history prior to implementation commits.
- Tests MUST be reviewed and approved by the feature owner or requestor before implementation begins.

### TDD Gates

✅ **Tests First**: Acceptance tests MUST be written and committed before any implementation ✅ **Proof of Failure**: A failing-test commit (red) MUST exist in branch history before implementation ✅ **Test Approval**: Tests MUST be reviewed and approved by feature owner before implementation begins ✅ **Multi-Frame Persistence**: Tests MUST verify level state persists across minimum 10 frames after transition

### Bevy 0.17 Event System Compliance

✅ **Event System Choice**: Use **Messages** (buffered events) for level transition signals
  - **Justification**: Level transitions are cross-system, frame-agnostic state changes that require coordination across multiple systems (brick destruction → level loader → UI update).
    Messages provide the batching and decoupling needed for this multi-step workflow.
  - **Audio triggers**: Use Messages for brick destruction sounds (existing `BrickDestroyed` message system)
  - **Victory screen**: Use observer or command-based approach for UI spawning (immediate, reactive)

### Coordinate System Guidance

✅ **Not Applicable**: Navigation bricks do not involve spatial movement or physics velocity changes.
They trigger level state transitions only.

### Initialization System Idempotence

⚠️ **Requires Attention**: The level loading system (`load_level`, `force_load_level_from_path`) runs in Update schedule and must not overwrite runtime state unconditionally.
  - **Current Pattern**: Existing level loader uses `CurrentLevel` resource to track level number; level transitions happen via `LevelAdvanceState` and `LevelSwitchRequested` messages
  - **Navigation Brick Integration**: New navigation bricks will emit level transition messages that flow through existing `process_level_switch_requests` system
  - **Idempotence Check**: Level loading only occurs when `LevelSwitchRequested` message is emitted or `LevelAdvanceState.active` flag is set; not executed every frame unconditionally ✅

### Multi-Frame Persistence Testing

✅ **Required**: Tests MUST verify that after a level transition triggered by brick 50 or 54:
  - New level number persists across 10+ `app.update()` cycles
  - All systems that write to `CurrentLevel` resource are included in test setup
  - No initialization systems overwrite the transitioned level state

### Query Safety & ECS Patterns

✅ **Systems are fallible**: No `.unwrap()` on query results; use `?` operator and early returns ✅ **Queries use filters**: `With<Brick>`, `With<BrickTypeId>` filters for specificity ✅ **Change detection**: Not applicable for brick collision (event-driven), but UI updates use `Changed<CurrentLevel>` ✅ **Message-Event Separation**:
  - `BrickDestroyed` messages (existing) for audio/scoring integration
  - New level transition triggers via `LevelSwitchRequested` messages (existing system)
  - No observer pattern needed for this feature
✅ **Asset loading**: Audio assets already loaded once in startup; brick materials loaded via existing texture system ✅ **Hierarchy safety**: Not applicable (no parent-child relationships modified)

### Summary

**Status**: ✅ **PASS** - All gates satisfied

**Key Decisions**:
1. Use existing `LevelSwitchRequested` message system for level transitions (Messages, not Observers)
2. Reuse existing brick destruction flow (collision → mark for despawn → emit `BrickDestroyed`)
3. Add boundary condition logic to level switch system (victory screen on final level, no-op on level 1)
4. Multi-frame persistence tests required for level state transitions
5. No new initialization systems needed; leverage existing idempotent level loading

## Project Structure

### Documentation (this feature)

`` `text specs/[###-feature]/ ├── plan.md # This file (/speckit.plan command output) ├── research.md # Phase 0 output (/speckit.plan command) ├── data-model.md        # Phase 1 output (/speckit.plan command) ├── quickstart.md        # Phase 1 output (/speckit.plan command) ├── contracts/ # Phase 1 output (/speckit.plan command) └── tasks.md # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan) ` ``

### Source Code (repository root)

```text src/ ├── lib.rs # Main game systems (brick collision, despawn) ├── level_loader.rs            # Level loading, advancement, transitions ├── level_format/ # Level file parsing and validation │ └── mod.rs                # Brick type constants (add BRICK_50, BRICK_54) ├── systems/ │ ├── level_switch.rs # Level switching logic (extend for boundary conditions) │ ├── scoring.rs            # Brick point values (already handles 0-point bricks) │ └── audio.rs # Audio triggers (extend for brick 50/54 sounds) └── signals.rs # Message definitions (BrickDestroyed, LevelSwitchRequested)

tests/ ├── brick_50_level_up.rs # TDD tests for brick 50 (Level Up) ├── brick_54_level_down.rs # TDD tests for brick 54 (Level Down) └── level_navigation_audio.rs # TDD tests for unique audio feedback

assets/ └── levels/ ├── level_001.ron # Potentially include test bricks ├── level_002.ron # Test level progression └── level_XXX.ron # Final level (for victory screen testing) ```

**Structure Decision**: [Document the selected structure and reference the real **Structure Decision**: Single Rust project (Option 1).
This feature extends existing systems in `src/lib.rs`, `src/level_loader.rs`, and `src/systems/level_switch.rs`.
New brick types (50 and 54) follow the same pattern as existing special bricks (41 Extra Ball, 42/91 Hazard, 57 Paddle-Destroyable): identified by `BrickTypeId` component, handled in ball-brick collision system, trigger messages for downstream effects.
