# Tasks: Post-Refactor QA & Sanitation

**Feature Branch**: `013-post-refactor-qa` **Status**: Draft

## Phase 1: Setup

- [X] T001 Verify feature branch `013-post-refactor-qa` is active

## Phase 2: Foundational

- [X] T002 Run full test suite to establish baseline `cargo test`

## Phase 3: User Story 1 - Test Integrity Audit

**Goal**: Remove "fake tests" that provide false confidence.
**Independent Test**: `cargo test` passes after removal.

- [X] T003 [US1] Scan `tests/` for comment-only tests or no-op assertions (manual or grep)
- [X] T004 [US1] Remove/Fix fake tests in `tests/change_detection.rs`
- [X] T005 [US1] Verify `cargo test` passes after cleanup

## Phase 4: User Story 2 - Constitution Compliance Sweep

**Goal**: Ensure strict adherence to Bevy 0.17 mandates.
**Independent Test**: Manual audit confirms compliance.

- [X] T006 [US2] Audit codebase for `#[require]` usage on components
- [X] T007 [US2] Audit codebase for `Changed<T>` usage in reactive systems
- [X] T008 [US2] Audit codebase for `ChildOf::parent()` usage (hierarchy)
- [X] T009 [US2] Audit codebase for panicking queries (`.unwrap()`, `expect()`)
- [X] T010 [US2] Audit codebase for `assert!(true)` or similar no-ops
- [X] T011 [US2] Fix any violations found during audit

## Phase 5: User Story 3 - Code Review Fixes

**Goal**: Address specific code review feedback (visibility, ordering).
**Independent Test**: Code compiles, startup is deterministic.

- [X] T012 [P] [US3] Change `BALL_RADIUS` to `pub(crate)` in `src/lib.rs`
- [X] T013 [P] [US3] Change `PADDLE_RADIUS` to `pub(crate)` in `src/lib.rs`
- [X] T014 [P] [US3] Change `PADDLE_HEIGHT` to `pub(crate)` in `src/lib.rs`
- [X] T015 [P] [US3] Change `PLANE_H` to `pub(crate)` in `src/lib.rs`
- [X] T016 [P] [US3] Change `PLANE_W` to `pub(crate)` in `src/lib.rs`
- [X] T017 [US3] Chain startup systems in `src/lib.rs` for deterministic order
- [ ] T018 [US3] Verify startup order by running app

## Final Phase: Polish

- [X] T019 Run `cargo clippy --all-targets --all-features`
- [X] T020 Run `cargo test` to ensure no regressions
- [X] T021 Run `bevy lint` (if available/configured)

## Dependencies

- US1, US2, US3 are independent and can be done in parallel.
- T012-T016 can be done in parallel.

## Implementation Strategy

- Start with US1 (Test Integrity) to clean up noise.
- Then do US3 (Code Review Fixes) as they are concrete and low-risk.
- Finally do US2 (Compliance Sweep) as it requires more deep inspection.
