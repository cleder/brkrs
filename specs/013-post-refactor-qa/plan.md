# Implementation Plan: Post-Refactor QA & Sanitation

**Branch**: `013-post-refactor-qa` | **Date**: 2025-12-20 | **Spec**: [specs/013-post-refactor-qa/spec.md](specs/013-post-refactor-qa/spec.md)
**Input**: Feature specification from `/specs/013-post-refactor-qa/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

This plan addresses technical debt and quality assurance following a major refactor.
It involves three main tasks:

1. **Test Integrity Audit**: Identifying and removing/fixing "fake tests" (comment-only or no-op assertions) to ensure the test suite provides real confidence.
2. **Constitution Compliance**: Auditing the codebase for strict adherence to Bevy 0.17 mandates and prohibitions (e.g., query safety, change detection, correct API usage).
3. **Code Review Fixes**: Restricting visibility of specific constants and enforcing deterministic execution order for startup systems.

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0 **Storage**: N/A (In-memory ECS state only) **Testing**: `cargo test` (standard Rust testing framework) **Target Platform**: Linux (Native) + WASM **Project Type**: Game (Bevy ECS) **Performance Goals**: 60 FPS **Constraints**: Strict adherence to `constitution.md` mandates.

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
- Message vs Event usage follows Bevy 0.17 APIs (`MessageWriter/Reader` vs observers).
- Assets are loaded once and handles are stored in Resources (no repeated `asset_server.load()` in loops).
- Hierarchies use `ChildOf::parent()` and `add_children()`/`remove::<Children>()` patterns.

**Compliance Status**:

- **TDD**: This plan *is* the QA pass, so "tests" here are largely the audit scripts/checks themselves.
  However, for the code review fixes (startup order), we will verify behavior.
  For the "fake tests", the "test" is the removal of the bad tests and ensuring the suite still passes.
- **Bevy 0.17**: The core purpose of this plan is to ENFORCE these mandates.
  The implementation phase will actively fix any violations found.

## Project Structure

### Documentation (this feature)

```text
specs/013-post-refactor-qa/
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
├── lib.rs               # Startup system ordering changes, constant visibility changes
├── systems/
│   └── spawning.rs      # Startup system definitions
tests/
├── change_detection.rs  # Target for "fake test" cleanup
└── ...                  # Other test files to be scanned
```
