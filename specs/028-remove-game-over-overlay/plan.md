# Implementation Plan: Remove Legacy Game Over Overlay

**Branch**: `028-remove-game-over-overlay` | **Date**: 2026-03-10 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/028-remove-game-over-overlay/spec.md`

## Summary

Remove legacy gameplay `GameOverOverlay` behavior so losing all lives and starting a new game never shows the old full-screen overlay again.
Keep existing non-overlay game-over handling (state transitions and state-driven UI in `GameState::GameOver`) unchanged.

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1 **Storage**: N/A (in-memory ECS state) **Testing**: `cargo test` integration tests (TDD-first red/green workflow) **Target Platform**: Native (Linux/Windows/macOS) and WASM **Project Type**: Single-project Bevy game (`src/` + `tests/`) **Performance Goals**: Maintain 60 FPS; zero additional per-frame UI work introduced **Constraints**: No replacement game-over overlay UI, no panicking queries, preserve Bevy 0.17 message/event separation **Scale/Scope**: Remove one legacy UI path, decouple dependent systems/tests, and add restart regression coverage

## Constitution Check

*GATE: Must pass before Phase 0 research.*
*Re-check after Phase 1 design.*

### TDD Compliance

- [x] Tests-first implementation required for this feature.
- [x] Failing (red) test commit required before implementation commits.
- [x] Test approval gate documented before code changes.
- [x] Acceptance tests include restart regression path and repeated-cycle checks.

### Bevy 0.17 Event/Message Strategy

- [x] **System chosen**: Keep buffered messages (`MessageWriter`/`MessageReader`) for `GameOverRequested` in respawn/life-loss flow.
- [x] **Why**: This feature removes a UI consumer only; it does not require immediate observer-based reactions.
- [x] **Message-Event Separation**: No conversion between `#[derive(Message)]` and observer `Trigger<T>` patterns in this feature.

### Bevy 0.17 ECS and UI Safety

- [x] No panicking query patterns introduced.
- [x] Query filters remain specific (`With<T>` marker checks in tests and touched systems).
- [x] No manual hierarchy mutation (`Children`/`Parent`) introduced.
- [x] No repeated asset loading introduced.
- [x] Change-driven UI behavior preserved for touched update systems.

### Coordinate/Physics Guidance

- Not applicable: feature does not alter movement axes, velocity semantics, camera orientation, or locked axes.

### Initialization Idempotence and Multi-Frame Persistence

- Initialization idempotence not directly impacted (no new loaders/initializers added).
- Multi-frame persistence checks are still included for regression confidence: tests assert no legacy overlay reappears over >=10 frames after restart.

### Gates Status

- [x] Pre-research constitution gate: PASS
- [x] Post-design constitution gate: PASS

## Project Structure

### Documentation (this feature)

```text
specs/028-remove-game-over-overlay/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── ui-overlay-removal.md
└── tasks.md
```

### Source Code Impact (repository root)

```text
src/
├── ui/
│   ├── mod.rs                       # remove legacy module wiring
│   ├── game_over_overlay.rs         # remove legacy overlay implementation (or make unreachable)
│   └── pause_overlay.rs             # remove GameOverOverlay dependency/query gate
├── systems/
│   ├── cheat_mode.rs                # remove overlay-despawn coupling
│   └── respawn.rs                   # preserve buffered GameOverRequested message flow checks
└── game_state.rs                    # unchanged non-overlay game-over state flow (verify no regressions)

tests/
├── ui_overlays.rs                   # adapt tests away from legacy overlay precedence assumptions
├── cheat_mode.rs                    # remove/replace overlay-specific assertions
└── [new or existing restart regression test file]  # verify no legacy overlay after game-over -> restart
```

**Structure Decision**: Single-project Bevy structure is retained.
This feature is a targeted UI-behavior removal with integration-test updates in existing `tests/` layout.

## Phase 0: Research & Unknown Resolution

Status: Complete.
See [research.md](research.md).

Resolved topics:

1. Exact removal scope and what remains in game-over flow.
2. Message architecture choice (`Message` remains for `GameOverRequested`).
3. Coupled subsystems requiring decoupling (`pause_overlay`, `cheat_mode`, tests).
4. Regression test depth and multi-frame coverage requirements.

## Phase 1: Design & Contracts

Status: Complete.
Artifacts produced:

- [data-model.md](data-model.md)
- [contracts/ui-overlay-removal.md](contracts/ui-overlay-removal.md)
- [quickstart.md](quickstart.md)

Design highlights:

- Canonical invariant: gameplay/restart flows contain zero legacy `GameOverOverlay` entities.
- Legacy overlay system wiring is removed from UI plugin.
- Non-overlay `GameState::GameOver` path remains the source of truth.
- Pause and cheat systems are decoupled from removed marker dependencies.

## Phase 2: Implementation Planning (for `/speckit.tasks`)

### Phase 2.1 Red Tests First

1. Add failing acceptance tests for `lose all lives -> start new game -> no legacy overlay`.
2. Add repeated-cycle regression test (>=10 cycles) checking zero legacy overlay entities during gameplay.
3. Update overlay-coupled tests (`tests/ui_overlays.rs`, `tests/cheat_mode.rs`) to new expected behavior.

### Phase 2.2 Implementation

1. Remove legacy overlay module registration in `src/ui/mod.rs`.
2. Remove or retire `src/ui/game_over_overlay.rs` and related imports.
3. Remove `GameOverOverlay` query coupling from `src/ui/pause_overlay.rs`.
4. Remove overlay cleanup coupling from `src/systems/cheat_mode.rs` while keeping required cheat behavior.
5. Ensure compile-clean imports and module exports after removal.

### Phase 2.3 Green + Quality Gates

1. Run `cargo test` and confirm updated acceptance tests pass.
2. Run `cargo fmt --all`.
3. Run `cargo clippy --all-targets --all-features`.
4. Run `bevy lint`.
5. Run `cargo check --target wasm32-unknown-unknown`.
6. Validate no replacement gameplay overlay was introduced.

## Complexity Tracking

No constitutional violations or complexity exceptions required.
