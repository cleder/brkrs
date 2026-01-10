# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: [e.g., Python 3.11, Swift 5.9, Rust 1.75 or NEEDS CLARIFICATION]
**Primary Dependencies**: [e.g., FastAPI, UIKit, LLVM or NEEDS CLARIFICATION]
**Storage**: [if applicable, e.g., PostgreSQL, CoreData, files or N/A]
**Testing**: [e.g., pytest, XCTest, cargo test or NEEDS CLARIFICATION]
**Target Platform**: [e.g., Linux server, iOS 15+, WASM or NEEDS CLARIFICATION]
**Project Type**: [single/web/mobile - determines source structure]
**Performance Goals**: [domain-specific, e.g., 1000 req/s, 10k lines/sec, 60 fps or NEEDS CLARIFICATION]
**Constraints**: [domain-specific, e.g., <200ms p95, <100MB memory, offline-capable or NEEDS CLARIFICATION]
**Scale/Scope**: [domain-specific, e.g., 10k users, 1M LOC, 50 screens or NEEDS CLARIFICATION]

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

This check MUST verify compliance with the constitution, including **Test-Driven Development (TDD)** gates:

- Tests are defined and committed prior to implementation efforts for each story/feature.
- A proof-of-failure commit (tests that FAIL) MUST exist in the branch history prior to implementation commits.
- Tests MUST be reviewed and approved by the feature owner or requestor before implementation begins.

This check MUST also verify compliance with **Bevy 0.17 mandates & prohibitions** (if the feature touches ECS, rendering, assets, or scheduling):

- **Bevy Event System Guidance:**
  - For any feature using events, messages, or observers, the plan MUST explicitly state which system is used (Messages vs Observers) and why, referencing the constitution's "Bevy 0.17 Event, Message, and Observer Clarification" section.
  - Justify the choice (e.g., "Messages for batchable, cross-frame work; Observers for immediate, reactive logic").

- **Coordinate System Guidance (if feature involves spatial movement/physics):**
  - Plan MUST specify which axes are used for movement (XZ plane for horizontal, Y for vertical, etc.).
  - Clarify whether directional terms (forward/backward) refer to Bevy's Transform API (-Z forward), gameplay perspective, or direct axis references.
  - Document any `LockedAxes` constraints and their relationship to camera orientation.

- Systems are fallible (`Result`) and do not panic on query outcomes (`?`, no `.unwrap()` on queries).
- Queries use `With<T>`/`Without<T>` filters and `Changed<T>` where appropriate (especially UI).
- **Message-Event Separation**: Verify correct use of `MessageWriter/Reader` for buffered, frame-agnostic streams and observers/`Trigger<T>` for immediate, reactive logic (e.g., UI/sound triggers).
- Assets are loaded once and handles are stored in Resources (no repeated `asset_server.load()` in loops).
- Hierarchies use `ChildOf::parent()` and `add_children()`/`remove::<Children>()` patterns.

[Gates determined based on constitution file]

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

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
