# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: [e.g., Python 3.11, Swift 5.9, Rust 1.75 or NEEDS CLARIFICATION] **Primary Dependencies**: [e.g., FastAPI, UIKit, LLVM or NEEDS CLARIFICATION] **Storage**: [if applicable, e.g., PostgreSQL, CoreData, files or N/A] **Testing**: [e.g., pytest, XCTest, cargo test or NEEDS CLARIFICATION] **Target Platform**: [e.g., Linux server, iOS 15+, WASM or NEEDS CLARIFICATION] **Project Type**: [single/web/mobile - determines source structure] **Performance Goals**: [domain-specific, e.g., 1000 req/s, 10k lines/sec, 60 fps or NEEDS CLARIFICATION] **Constraints**: [domain-specific, e.g., <200ms p95, <100MB memory, offline-capable or NEEDS CLARIFICATION] **Scale/Scope**: [domain-specific, e.g., 10k users, 1M LOC, 50 screens or NEEDS CLARIFICATION]

## Constitution Check

*GATE: Must pass before Phase 0 research.*
            *Re-check after Phase 1 design.*

This check MUST verify compliance with the constitution, including **Test-Driven Development (TDD)** gates:

- Tests are defined and committed prior to implementation efforts for each story/feature.
- A proof-of-failure commit (tests that FAIL) MUST exist in the branch history prior to implementation commits.
- Tests MUST be reviewed and approved by the feature owner or requestor before implementation begins.

This check MUST also verify compliance with **Bevy 0.17 mandates & prohibitions** (if the feature touches ECS, rendering, assets, or scheduling):

- Systems are fallible (`Result`) and do not panic on query outcomes (`?`, no `.unwrap()` on queries).
- Queries use `With<T>`/`Without<T>` filters and `Changed<T>` where appropriate (especially UI).
- **Message-Event Separation**: Verify correct use of `MessageWriter/Reader` for buffered, frame-agnostic streams and observers/`Trigger<T>` for immediate, reactive logic (e.g., UI/sound triggers).
- Assets are loaded once and handles are stored in Resources (no repeated `asset_server.load()` in loops).
- Hierarchies use `ChildOf::parent()` and `add_children()`/`remove::<Children>()` patterns.

[Gates determined based on constitution file]

**Constitution Compliance for this Feature:**

- All tasks and tests are defined before implementation (see tasks.md Phase 1 and 3).
- Failing test commits are required and tracked (see T007, T012, T013, T014, T015).
- All ECS systems use fallible patterns, no panicking queries, and correct Bevy 0.17 event/message separation (see spec.md and tasks.md for BallWallHit event and observer system).
- Asset loading, hierarchy, and system set organization follow Bevy 0.17 mandates.
- Manual and integration tests are included for both native and WASM targets.

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

tests/ ios/ or android/

```text
src/
├── signals.rs
├── lib.rs
├── main.rs
├── systems/
│   ├── physics.rs
│   └── audio.rs
tests/
├── integration/
│   └── wall_audio.rs
```

**Structure Decision**: Single project, with src/ and tests/ as above.
No web or mobile subprojects.

## Complexity Tracking

No constitution violations or extra complexity for this feature.
