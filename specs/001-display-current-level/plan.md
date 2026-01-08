# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add a minimal, accessible HUD indicator showing the current level number during gameplay.
The HUD label appears within 1 second of level start, is localized and accessible to screen readers (announced via a polite live region).
Detailed progress metrics are intentionally omitted from the in-play HUD; final progress may be shown on pause or summary screens when objective metadata exists.

Implementation approach: add a small HUD system that listens for `LevelStarted` events and updates a localized HUD label; expose an accessible announcement hook for screen readers and make pause/summary screens request `PlayerProgress` when available.

## Technical Context

**Language/Version**: Rust 1.81 (project baseline) **Primary Dependencies**: Bevy 0.17.3 (UI), bevy_rapier3d (physics if needed), serde/ron for level metadata **Storage**: N/A (in-memory ECS state only) **Testing**: `cargo test` (unit + integration), manual gameplay tests, UI integration tests for HUD behavior **Target Platform**: Native (Linux/Windows/macOS) + WASM (web) **Project Type**: Game (ECS-driven, single repo) **Performance Goals**: Maintain 60 FPS; HUD updates must be non-blocking and cheap (no allocations in hot path) **Constraints**: Must follow ECS-first architecture and modular design (no global mutable state for HUD); accessibility APIs must work on both native and WASM targets **Scale/Scope**: Small UI feature scoped to HUD and pause/summary screens; minimal cross-system dependencies (Level metadata, optional PlayerProgress)

## Constitution Check

*GATE: Must pass before Phase 0 research.*
 *Re-check after Phase 1 design.*

- ECS-first: ✔ Feature will be implemented as Bevy systems updating components and responding to events (no global mutable state).
- Physics-driven: n/a (HUD-only change; no physics changes required).
- Modular design: ✔ HUD system will be implemented as an independently testable system set that can be enabled/disabled.
- Performance-first: ✔ HUD updates are designed to be non-allocating in hot paths and executed on the main UI system at frame-safe frequency.
- Cross-platform compatibility: ✔ Accessibility and HUD updates will be implemented with conditional paths where necessary to support WASM and native targets.

**Result**: Constitution gates satisfied; no violations detected.

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

<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: Single Rust game project.
Implement HUD systems under `src/ui/` or `src/systems/hud/`, tests under `tests/` and unit tests alongside modules.
New files to add:

```text
src/systems/hud/
├── mod.rs             # hud system set
├── level_hud.rs       # HUD update system and components
├── accessibility.rs   # helpers to announce via platform-appropriate APIs
└── tests/
    └── hud_tests.rs   # unit/integration tests for HUD behavior
```

## Phase 0 (Research) Outputs

- `research.md` — resolved HUD placement, accessibility behavior, and progress visibility policy.
- Confirmed no major constitution gates are violated.

## Phase 1 (Design) Outputs

- `data-model.md` — Level, PlayerProgress (optional), HUDConfig definitions and validation rules.
- `contracts/level-events.yaml` — simple schema for `LevelStarted`, `HUDUpdate`, and `LevelProgressSummary`.
- `quickstart.md` — steps to validate locally and manual checks.

## Next steps (Phase 2)

- Create implementation tasks (`specs/001-display-current-level/tasks.md`) and break into small tickets:
  - Add `LevelHud` components and systems in `src/systems/hud/`.
  - Add accessibility announcement helpers and tests.
  - Add integration tests that simulate `LevelStarted` and `PauseRequested`.
  - Create UI smoke tests for WASM and native builds.
- Implement feature on `001-display-current-level` branch and open PR with testing notes.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
