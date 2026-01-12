# Implementation Plan: Ball Lives Counter

**Branch**: `001-ball-lives` | **Date**: 2025-12-14 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-ball-lives/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add a limited “balls/lives” counter: start at 3, decrement on each `LifeLostEvent`, show remaining balls on-screen, and display a “Game over” message when the last ball is lost.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1 **Storage**: N/A (in-memory ECS state only) **Testing**: `cargo test` **Target Platform**: Native (Linux/Windows/macOS) + WASM **Project Type**: single **Performance Goals**: 60 FPS **Constraints**: Avoid per-frame allocations in UI updates; keep systems event-driven where practical **Scale/Scope**: Single-player, one lives counter + one game-over message

**Resolved Decisions**

- Decrement lives while processing `LifeLostEvent` during respawn scheduling so each loss gets a correct post-decrement count.
- Use existing `LivesState` as the canonical remaining-lives source of truth.
- Use existing `GameOverRequested` message as the game-over intent signal.
- Add UI modules under `src/ui/` and wire their systems from `src/lib.rs` (consistent with existing UI wiring).
- Ensure the “GAME OVER” message remains visible even if pause is toggled (avoid ambiguous overlay stacking).

## Constitution Check

*GATE: Must pass before Phase 0 research.*
                                                                                                                                                                                                                                                                                                                         *Re-check after Phase 1 design.*

- ECS-first: PASS (lives state stored as ECS resources; logic in systems)
- Physics-driven gameplay: PASS (feature is UI/state; does not bypass physics for gameplay)
- Modular feature design: PASS (encapsulated systems/components; event-driven via existing messages)
- Performance-first: PASS (event-driven updates; bounded UI)
- Cross-platform: PASS (Bevy UI; no platform-specific APIs)
- Rustdoc requirements: PASS for new public APIs (ensure new public items are documented)

## Project Structure

### Documentation (this feature)

```text
specs/001-ball-lives/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
src/
├── lib.rs
├── main.rs
├── pause.rs
├── level_loader.rs
├── systems/
│   ├── respawn.rs
│   └── ...
└── ui/
  ├── mod.rs
  ├── pause_overlay.rs
  └── palette.rs

tests/
├── paddle_shrink.rs
├── respawn_timer.rs
├── respawn_visual.rs
└── ...
```

**Structure Decision**: Single Bevy game crate.
Lives logic will live in `src/systems/respawn.rs` alongside existing `LifeLostEvent` and `GameOverRequested` messages.
UI will be added under `src/ui/` and wired in `src/lib.rs` or via a small plugin (decision documented in Phase 0 research).

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations.

## Phase 0: Outline & Research

**Goal**: Resolve the open questions in Technical Context and converge on a minimal design that matches repo conventions.

**Research Tasks**

- Research where `LifeLostEvent` is emitted/consumed and how lives are currently modeled.
- Research UI patterns for overlays and HUD text in this repo.
- Research how game-over currently signals intent (existing message types) and how it should be surfaced as UI.

**Output**: [research.md](research.md) with all decisions finalized.

## Phase 1: Design & Contracts

**Prerequisite**: `research.md` completed with no remaining NEEDS CLARIFICATION.

**Design Artifacts**

- [data-model.md](data-model.md): define resources/messages/components for lives count and game-over UI state.
- [contracts/events.openapi.yaml](contracts/events.openapi.yaml): message contracts (payload schemas) for the feature-facing signals.
- [quickstart.md](quickstart.md): instructions to run and verify the feature manually and via tests.

**Post-Design Constitution Re-check**

- ECS-first: PASS
- Physics-driven gameplay: PASS
- Modular feature design: PASS
- Performance-first: PASS
- Cross-platform: PASS
- Rustdoc requirements: PASS (ensure new public items are documented)

**Agent Context Update**

- Ran `/home/christian/devel/bevy/brkrs/.specify/scripts/bash/update-agent-context.sh copilot` and updated `.github/copilot-instructions.md`.

## Phase 2: Implementation Planning

**Implementation Steps (high level)**

1. Update lives tracking to decrement on each `LifeLostEvent` and clamp at 0.
2. Ensure game-over is requested on last life loss and is stable under repeated loss events.
3. Add UI: on-screen lives counter (always visible during gameplay).
4. Add UI: “Game over” message when lives reach 0.
5. Add/adjust tests for lives decrement, clamp behavior, and game-over request.

**Validation Commands**

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features`
- `cargo test`
- `bevy lint`
