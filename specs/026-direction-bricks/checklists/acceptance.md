# Acceptance Criteria Checklist: Direction Bricks

**Purpose**: Validate that all acceptance scenarios in the feature specification are concrete, measurable, and testable **Created**: 2026-02-01 **Feature**: [spec.md](../spec.md)

**Note**: This checklist tests the QUALITY of acceptance scenarios themselves (testability, clarity, measurability), not the implementation.
Each item asks "Can this scenario be objectively verified with tests?"

## Acceptance Scenario Testability

- [x] CHK001 - Are all single-direction brick scenarios (43-46) specified with measurable initial velocity and expected final velocity values? [Clarity, Spec §User Story 1]
- [x] CHK002 - Can brick 43 (Down) scenario be tested by asserting Y-velocity decreased by exactly 5.0 units/sec? [Measurability, Spec §US1-Acceptance-1]
- [x] CHK003 - Can brick 44 (Left) scenario be tested by asserting X-velocity decreased by exactly 5.0 units/sec? [Measurability, Spec §US1-Acceptance-2]
- [x] CHK004 - Can brick 45 (Right) scenario be tested by asserting X-velocity increased by exactly 5.0 units/sec? [Measurability, Spec §US1-Acceptance-3]
- [x] CHK005 - Can brick 46 (Up) scenario be tested by asserting Y-velocity increased by exactly 5.0 units/sec? [Measurability, Spec §US1-Acceptance-4]
- [x] CHK006 - Does the additive impulse scenario (brick 43 on -3.0 Y-velocity → -8.0) provide concrete numbers for verification? [Clarity, Spec §US1-Acceptance-5]
- [x] CHK007 - Does the multi-frame stacking scenario specify how many consecutive bricks and what constitutes "correct stacking"? [Clarity, Gap, Spec §US1-Acceptance-6]
- [x] CHK008 - Are diagonal brick scenarios (47-48) specified with measurable changes in both axes simultaneously? [Measurability, Spec §User Story 2]
- [x] CHK009 - Does brick 47 (Up-Right) scenario verify both Y +5.0 AND X +5.0 as separate assertions? [Clarity, Spec §US2-Acceptance-1]
- [x] CHK010 - Does brick 48 (Up-Left) scenario specify the correct signs (Y +5.0, X -5.0)? [Clarity, Spec §US2-Acceptance-2]
- [x] CHK011 - Does the diagonal additive scenario provide concrete before/after coordinates (2.0, 2.0, 0) → (7.0, 7.0, 0)? [Measurability, Spec §US2-Acceptance-3]

## Randomization Scenario Coverage

- [x] CHK012 - Does brick 52 scenario specify what "replaced" means operationally (previous velocity discarded, not modified)? [Clarity, Spec §US3-Acceptance-4]
- [x] CHK013 - Does the randomization scenario define "statistically different" with measurable criteria (e.g., at least X different directions in Y trials)? [Measurability, Gap, Spec §US3-Acceptance-2]
- [x] CHK014 - Are the bounds for random magnitude (5.0-15.0 units/sec) explicitly included in the testable scenario? [Completeness, Spec §US3-Acceptance-3]
- [x] CHK015 - Does the directional distribution scenario verify "uniformly distributed across all 360 degrees"? [Clarity, Spec §US3-Acceptance-3]
- [x] CHK016 - Is the randomization scenario independent of ball velocity (does not rely on additive behavior like cardinal bricks)? [Consistency, Spec §US3-Acceptance-1]

## Scoring Scenario Coverage

- [x] CHK017 - Are all point values explicit for each brick type: 75 (43-46), 100 (47-48), 125 (52)? [Completeness, Spec §User Story 4]
- [x] CHK018 - Does each scoring scenario test exactly one brick type without mixing velocity changes? [Independence, Spec §US4]
- [x] CHK019 - Are scoring scenarios written as independent tests from physics scenarios (can be tested without direction change)? [Testability, Spec §US4]
- [x] CHK020 - Does scoring cover all 7 brick types (43, 44, 45, 46, 47, 48, 52) with scenarios? [Completeness, Spec §US4]

## Edge Case Testability

- [x] CHK021 - Does the stationary ball edge case specify how to create/detect a ball at velocity ≈ 0? [Clarity, Gap, Spec §Edge Cases]
- [x] CHK022 - Is "rapid succession" edge case quantified (same frame, adjacent frames, or any rapid interval)? [Clarity, Ambiguity, Spec §Edge Cases]
- [x] CHK023 - Does the Z-velocity independence edge case explicitly verify that Z is unchanged after destruction? [Measurability, Spec §Edge Cases]
- [x] CHK024 - Can Z-velocity preservation be tested by asserting `ball.linear_velocity.z == initial_z` before and after? [Testability, Spec §Edge Cases]

## Multi-Frame Persistence Testing

- [x] CHK025 - Do acceptance scenarios specify multi-frame persistence checks (minimum 10 update cycles per mandate)? [Completeness, Gap, Spec §Multi-Frame Persistence Requirement]
- [x] CHK026 - Is there a scenario that verifies direction brick effects persist without being overwritten by cleanup/reset systems? [Completeness, Gap, Spec §Multi-Frame Persistence Requirement]
- [x] CHK027 - Are multi-frame scenarios written to catch initialization systems that reset velocity unconditionally? [Testability, Gap, Spec §Multi-Frame Persistence Requirement]

## Tracing/Observability Testability

- [x] CHK028 - Does the tracing scenario specify which span fields must be captured (brick ID, before velocity, after velocity, points)? [Clarity, Spec §US1-Acceptance-7]
- [x] CHK029 - Can tracing output be asserted in tests (does scenario include verification of span context)? [Measurability, Gap, Spec §FR-011]
- [x] CHK030 - Is the tracing scenario testable without relying on external logging infrastructure? [Testability, Gap, Spec §FR-011]

## Scenario Consistency & Completeness

- [x] CHK031 - Are all 7 brick types covered in acceptance scenarios? [Completeness, Spec §All User Stories]
- [x] CHK032 - Do scenarios consistently use the same coordinate terminology (X/Y/Z, not "left/right/up/down")? [Consistency, Spec §All Scenarios]
- [x] CHK033 - Do physics scenarios (velocity changes) and scoring scenarios remain independent in test structure? [Separation of Concerns, Spec §US1-4]
- [x] CHK034 - Are there any acceptance scenarios for the Observers/Trigger pattern choice or just physics behavior? [Gap, Spec §User Scenarios & Testing]
- [x] CHK035 - Is there a scenario validating that brick 52 randomization uses direct generation (not clamping)? [Completeness, Spec §Clarifications]

## Measurability & Objective Verification

- [x] CHK036 - Can every scenario in User Story 1-2 be verified using `assert_eq!` or similar direct comparison? [Measurability, Spec §US1-2]
- [x] CHK037 - Can every scenario in User Story 4 (scoring) be verified using `assert_eq!` on score values? [Measurability, Spec §US4]
- [x] CHK038 - Can the randomization scenarios be verified statistically (distribution check) rather than single-point assertions? [Measurability, Gap, Spec §US3]
- [x] CHK039 - Are floating-point comparisons addressed (velocity values are floats; tolerance specified)? [Clarity, Gap, Spec §All Physics Scenarios]
- [x] CHK040 - Is there guidance on acceptable tolerance for velocity comparisons (e.g., ≈0.01 units/sec)? [Clarity, Gap, Spec §Requirements]

## Clarification Integration

- [x] CHK041 - Do scenarios reflect the instantaneous impulse decision (applied once per destruction, not per-frame)? [Consistency, Spec §Clarifications]
- [x] CHK042 - Do scenarios reflect the Observer pattern choice (not Message-based messages)? [Consistency, Spec §Clarifications, Spec §Bevy 0.17 Requirement]
- [x] CHK043 - Do scenarios confirm no validation/clamping of velocity (physics system owns bounds)? [Consistency, Spec §Clarifications]
- [x] CHK044 - Do scenarios use direct RNG generation (5.0-15.0 magnitude) per clarification Q5? [Consistency, Spec §Clarifications]

## Notes

- **Checklist Focus**: This validates scenario TESTABILITY and CLARITY, not implementation correctness
- **TDD Requirement**: All items marked ✅ indicate acceptance scenarios are ready for test-first development
- **Acceptance Criteria**: Scenarios should be committed as failing tests before implementation begins
- **Tolerance Guidance** (CHK039): Consider adding float comparison tolerance to Requirements section if not present
- **Multi-Frame Gap** (CHK025-027): Feature specification should add multi-frame persistence scenarios per 020-gravity-bricks retrospective mandate
- **Items to Verify Before Implementation**:
  - CHK039, CHK040: Address floating-point tolerance in test guidance
  - CHK025-027: Add explicit multi-frame persistence scenarios
  - CHK007: Clarify "stacking" test with specific example (e.g., 3 consecutive bricks, expected velocity progression)
  - CHK013: Define statistical threshold for randomization "variability"
