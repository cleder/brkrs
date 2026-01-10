# Implementation Plan: Cheat Mode Safeguards

**Branch**: `001-cheat-mode-safeguards` | **Date**: 2025-12-17 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-cheat-mode-safeguards/spec.md`

**Note**: This plan is filled per `/speckit.plan` workflow.

## Summary

Enable a gated cheat mode toggled via 'g' that resets score to 0 on enter/exit, shows a clear image indicator (asset `assets/textures/default/cheat-mode-128.png`, rendered ~48×48 px, fixed lower-right, skip spawn if asset unavailable), and restricts level control keys (R/N/P) to cheat mode only, with blocked inputs ignored outside gameplay and a soft beep for feedback.

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0 (physics baseline), tracing 0.1 **Storage**: N/A (in-memory ECS state only) **Testing**: cargo test, cargo clippy --all-targets --all-features, cargo fmt --all, manual gameplay checks; bevy lint
**Target Platform**: Native (Linux/macOS/Windows) + WASM | **Project Type**: Game (single project)
**Performance Goals**: Maintain 60 FPS; indicator appears within 100 ms; input gating adds negligible overhead; no accidental level changes **Constraints**: ECS-first systems; avoid blocking main thread; indicator must not obscure gameplay; cross-platform compatibility; audio feedback minimal and non-intrusive **Scale/Scope**: Single-player session; small number of UI nodes and input events per frame

## Constitution Check

*GATE: Must pass before Phase 0 research.*
     *Re-check after Phase 1 design.*

- ECS-first: Plan uses Bevy systems, components, events; no global mutable state outside ECS — PASS
- Physics-driven gameplay: No physics changes; ensure no manual transform hacks — PASS (not impacted)
- Modular feature design: Implement as feature plugin/system sets; level control gating isolated — PASS
- Performance-first (60 FPS): Lightweight UI + input checks; no heavy loops — PASS
- Cross-platform: No platform-specific APIs; works on native + WASM — PASS
- Rustdoc documentation: Public APIs to be documented; enforce during implementation — PASS

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── systems/           # gameplay systems (inputs, gating, toggles)
├── ui/                # UI indicator components/systems
├── level_format/      # existing level assets wiring
└── lib.rs, main.rs    # app entry, plugin wiring

tests/
├── integration/       # gameplay/integration tests (existing)
└── unit/              # unit tests for systems/components
```

**Structure Decision**: Use the existing single-project layout under `src/`, adding systems for input gating, cheat mode state, and UI indicator within existing modules; tests live under `tests/` following current convention.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |

## Phase 0: Outline & Research

- Unknowns/clarifications: Observability approach for cheat toggles and blocked inputs (decide tracing signals)
- research.md tasks (completed):
  - Decide logging/tracing signals for cheat toggles and blocked level-control inputs
  - Confirm input gating pattern for R/N/P tied to cheat-mode state
  - Confirm UI indicator rendering approach consistent with Bevy UI best practices
- Output: [research.md](research.md)

## Phase 1: Design & Contracts

- data-model.md: Components/resources/events for CheatModeState, Score interactions, LevelControl events; state transitions for activation/deactivation (completed)
- contracts/: No external API; document as empty/README describing local-only input handling (completed)
- quickstart.md: How to run, toggle cheat mode, verify indicator, and test gating; commands for fmt/clippy/test (completed)
- Outputs: [data-model.md](data-model.md), [contracts/README.md](contracts/README.md), [quickstart.md](quickstart.md)

## Phase 1: Agent Context Update

- Run `.specify/scripts/bash/update-agent-context.sh copilot` and ensure new tech/feature notes recorded; preserve manual additions between markers.
