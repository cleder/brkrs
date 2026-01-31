# Requirements Checklist: Level Navigation Bricks (Logic)

**Feature**: Level Navigation Bricks (Bricks 50 & 54) **Purpose**: Validate specification and planning logic before implementation begins **Created**: 2026-01-31 **Scope**: Functional requirements, logic flows, message contracts (no UI/audio/NFRs) **Style**: Concise PR aid (practical validation gates, not exhaustive)

---

## Requirement Completeness

- [ ] CHK001 - Are brick 50 and 54 type identifiers defined as distinct values? [Spec §FR-001, FR-003]
- [ ] CHK002 - Is the level transition Message type (`LevelSwitchRequested`) documented with required fields (direction/source)? [Data Model §Messages, Plan]
- [ ] CHK003 - Are the two distinct collision behaviors (next vs previous level) specified for each brick? [Spec §FR-002, FR-004]
- [ ] CHK004 - Is the final-level boundary logic (victory screen, no transition) defined for brick 50? [Spec §FR-008, Edge Cases]
- [ ] CHK005 - Is the first-level boundary logic (no transition) defined for brick 54? [Spec §FR-008, Edge Cases]
- [ ] CHK006 - Is the score value (0 points) explicitly assigned to both navigation bricks? [Spec §FR-005]
- [ ] CHK007 - Is multi-frame persistence requirement (10+ frames) specified for level state after transition? [Spec §FR-009, Multi-Frame Persistence Requirement]

## Logic Flow Clarity

- [ ] CHK008 - Is the ball-collision-to-level-transition sequence clearly ordered (collision → despawn → message emission)? [Spec §Acceptance Scenarios]
- [ ] CHK009 - Is the boundary-condition logic unambiguous (victory screen path vs. no-transition path)? [Spec §Edge Cases, FR-008]
- [ ] CHK010 - Is the "despawn and 0-points" outcome identical for both boundary cases and normal cases? [Spec §FR-005, Edge Cases]
- [ ] CHK011 - Is the level-state-persistence requirement (no initialization system overwrites) quantified as 10+ frames minimum? [Spec §Multi-Frame Persistence Requirement]
- [ ] CHK012 - Is the state-clearing behavior on transition (balls, powerups, effects) documented? [Spec §FR-010]

## Message Contract Correctness

- [ ] CHK013 - Is `LevelSwitchRequested` message structure defined with `direction: Next | Previous` field? [Data Model §Messages]
- [ ] CHK014 - Is `BrickDestroyed` message integration confirmed (reuses existing system)? [Plan §Key Decisions]
- [ ] CHK015 - Is Message-event separation justified (Messages chosen over Observers)? [Plan §Constitution Check, Bevy 0.17 Event System Compliance]
- [ ] CHK016 - Are the message producers identified (collision handler emits both messages)? [Data Model §Entities, Behavior]
- [ ] CHK017 - Are the message consumers identified (level switch system consumes `LevelSwitchRequested`)? [Plan §Key Decisions]

## Boundary Condition Logic

- [ ] CHK018 - Is "final level" defined as a specific condition (current_level == max_level)? [Spec §Edge Cases]
- [ ] CHK019 - Is the victory-screen trigger (brick 50 on final level) distinct from normal level transition? [Spec §Acceptance Scenario 3 (US1)]
- [ ] CHK020 - Is "level 1" explicitly identified as the first level boundary? [Spec §Edge Cases]
- [ ] CHK021 - Is the no-op behavior (brick 54 on level 1) non-destructive to level state? [Spec §Acceptance Scenario 2 (US2)]

## State Persistence & Idempotence

- [ ] CHK022 - Is the level loader (or transition system) idempotent when a level is already current? [Plan §Initialization System Idempotence]
- [ ] CHK023 - Is the "initialization systems do not overwrite runtime state" requirement verified in plan? [Plan §Initialization System Idempotence, Spec §FR-009]
- [ ] CHK024 - Are the systems that write to level state enumerated in plan (for test setup)? [Plan §Constitution Check, Multi-Frame Persistence]
- [ ] CHK025 - Is the 10-frame minimum persistence check integrated into test acceptance criteria? [Spec §Multi-Frame Persistence Requirement, SC-003]

## Task Decomposition

- [ ] CHK026 - Are code modification points identified (brick constants, collision handler, level switch system)? [Plan §Technical Context, Quickstart]
- [ ] CHK027 - Is the TDD gate (tests first, failing-test commit proof) confirmed in plan? [Plan §Constitution Check, TDD Gates]
- [ ] CHK028 - Are test file locations specified (`tests/brick_50_level_up.rs`, `tests/brick_54_level_down.rs`)? [Quickstart §Implementation Checklist, Plan]
- [ ] CHK029 - Is the phased approach clear (Phase 0: constants, Phase 1: tests, Phase 2: implementation)? [Quickstart §Implementation Checklist]

## Acceptance Criteria Measurability

- [ ] CHK030 - Is "advances to next level within one game tick" quantified as 100% success rate? [Spec §SC-001]
- [ ] CHK031 - Is "returns to previous level within one game tick" quantified as 100% success rate? [Spec §SC-002]
- [ ] CHK032 - Is "state persists across 10+ frames without overwrite" a testable assertion? [Spec §SC-003]
- [ ] CHK033 - Can the victory-screen condition (final level) be objectively verified in tests? [Spec §Acceptance Scenario 3 (US1)]
- [ ] CHK034 - Can the level-1 no-op behavior be objectively verified (no transition, brick destroyed)? [Spec §Acceptance Scenario 2 (US2)]

---

## Summary

**Total Items**: 34 checks **Focus**: Logic flows, message contracts, boundary conditions, state persistence, task readiness **Excluded**: UI rendering, audio asset handling, performance metrics, accessibility **Gating Criteria**: All checks should pass before implementation begins

**Next Steps**:

1. Address any failed checks by clarifying spec/plan
2. Commit all documentation (spec.md, plan.md, data-model.md, tasks.md)
3. Write and commit failing tests (RED phase)
4. Proceed with implementation (GREEN phase)
