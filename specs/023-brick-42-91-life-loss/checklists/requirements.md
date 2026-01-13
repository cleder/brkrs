# Specification Quality Checklist: Brick Types 42 & 91 — Paddle Life Loss

**Feature**: 023-brick-42-91-life-loss **Purpose**: Validate specification completeness, clarity, consistency, and readiness for implementation **Created**: 2026-01-13 **Last Updated**: 2026-01-13 **Status**: READY FOR IMPLEMENTATION

## Overview

This checklist validates the quality of requirements in the specification document using the "Unit Tests for English" methodology.
Each item tests whether the specification is well-written, complete, unambiguous, and ready for implementation—NOT whether the code works.

---

## Requirement Completeness

Items test whether all necessary requirements are documented.

- [ ] CHK001 - Are collision handling requirements specified for both ball-brick AND paddle-brick interactions? [Spec §Req]
- [ ] CHK002 - Are scoring rules defined for type 42 including exact point value (90) and trigger condition (ball collision)? [Spec §FR-001]
- [ ] CHK003 - Are indestructibility rules explicitly stated for type 91 (ball collision MUST NOT destroy)? [Spec §FR-004]
- [ ] CHK004 - Is life-loss behavior defined for both brick types separately (types 42 and 91 both cause loss on paddle contact)? [Spec §FR-002, FR-003]
- [ ] CHK005 - Is the multi-contact paddle policy explicitly documented (one life lost per frame max)? [Spec §FR-009, Clarifications §Q1]
- [ ] CHK006 - Are level completion rules specified for type 42 (must count) and type 91 (must not count)? [Spec §FR-006, FR-007]
- [ ] CHK007 - Is the integration with existing lives flow documented (LifeLostEvent, respawn sequence)? [Spec §FR-008, Assumptions]
- [ ] CHK008 - Are frame-boundary behaviors defined (what happens when paddle/ball contacts occur at frame start/end)? [Plan §Execution Order]
- [ ] CHK009 - Is the Message vs Observer event pattern choice justified? [Plan §Architecture §Design Decision, Bevy 0.17 REQUIREMENT]
- [ ] CHK010 - Are component markers (BrickTypeId, CountsTowardsCompletion) mapped to both types? [Data Model §Entity & Component Model]
- [ ] CHK011 - Is hierarchy safety specified (despawn patterns, parent-child relationships)? [Bevy 0.17 REQUIREMENT]
- [ ] CHK012 - Is the respawn/spawn transform requirement documented for life loss events? [Data Model §LifeLostEvent, Plan §Phase 1 Task 2]
- [ ] CHK013 - Are audio/visual feedback requirements specified for paddle collision or life loss? [Spec §User Stories]
- [ ] CHK014 - Is the fallback behavior specified when ball entity is not found (for LifeLostEvent)? [Plan §Phase 1]
- [ ] CHK015 - Are texture/material requirements specified for type 91 visual representation? [Plan §Phase 3]

---

## Requirement Clarity

Items test whether requirements are specific, unambiguous, and measurable.

- [ ] CHK016 - Is "paddle collision" quantified (any point of contact, minimum duration, enter/exit distinction)? [Spec §User Stories]
- [ ] CHK017 - Is "life lost" quantified to exactly -1 (not variable or conditional decrement)? [Spec §FR-002, FR-003]
- [ ] CHK018 - Is "indestructible" defined as "ball collision does not trigger destruction or points award"? [Spec §FR-004, FR-005]
- [ ] CHK019 - Is "counts toward completion" defined as "must have CountsTowardsCompletion marker and query includes it"? [Data Model]
- [ ] CHK020 - Is "at most one life per frame" defined to apply ONLY to paddle collisions (not ball→lower goal)? [Spec §FR-009]
- [ ] CHK021 - Is the scoring value "90 points" tied to type ID 42 with no ambiguity about conditions? [Spec §FR-001]
- [ ] CHK022 - Is the paddle-life-loss rule specified as applying to BOTH types 42 and 91 equally? [Spec §FR-002, FR-003]
- [ ] CHK023 - Is the timing of life-loss event emission relative to ball/paddle state machine defined? [Plan §Phase 1]
- [ ] CHK024 - Is the frame-flag reset mechanism explicitly described (when, how often, which system)? [Plan §Phase 1 Task 3]
- [ ] CHK025 - Is the expected output of "ball-brick collision" explicitly stated (destruction OR no destruction, event emitted OR not)? [Data Model §BrickDestroyed]

---

## Requirement Consistency

Items test whether requirements align without conflicts.

- [ ] CHK026 - Are paddle-collision requirements for types 42 and 91 consistent (both cause exactly 1 life loss)? [Spec §FR-002, FR-003]
- [ ] CHK027 - Do ball-collision requirements for type 42 (destroy + score) align with existing brick destruction flow? [Data Model §System Execution Order]
- [ ] CHK028 - Do ball-collision requirements for type 91 (no destroy + no score) align with indestructible brick pattern (type 90)? [Plan §Phase 0]
- [ ] CHK029 - Is the multi-contact policy consistent with the single-loss-per-frame rule across all scenarios? [Spec §FR-009]
- [ ] CHK030 - Do completion requirements for type 42 (must count) and type 91 (must not count) align with the marker component pattern? [Data Model §Level Completion Integration]
- [ ] CHK031 - Is the LifeLostEvent payload consistent with existing ball→lower-goal life loss? [Data Model §LifeLostEvent]
- [ ] CHK032 - Do phase dependencies (Phase 0 → Phase 1 → Phase 2 → Phase 3) eliminate circular or conflicting changes? [Plan §Phases]
- [ ] CHK033 - Is the Local<bool> flag behavior consistent with standard Bevy system patterns? [Plan §Architecture §Design Decision]
- [ ] CHK034 - Are texture/material references for type 91 consistent with indestructible brick visual style (reference type 90)? [Plan §Phase 3]

---

## Acceptance Criteria Quality

Items test whether success criteria are measurable and testable.

- [ ] CHK035 - Is SC-001 testable (destroy type 42, verify score += 90 and entity gone)? [Spec §Success Criteria]
- [ ] CHK036 - Is SC-002 testable (paddle contact, verify lives -= 1 and respawn initiated)? [Spec §Success Criteria]
- [ ] CHK037 - Is SC-003 testable (destroy all type 42, verify level complete with type 91 present)? [Spec §Success Criteria]
- [ ] CHK038 - Is SC-004 testable (run 10 frames, verify score/lives persist without regression)? [Spec §Success Criteria, MULTI-FRAME PERSISTENCE REQUIREMENT]
- [ ] CHK039 - Are all acceptance scenarios tied to measurable assertions (not subjective "feels right")? [Spec §User Stories]
- [ ] CHK040 - Is the "95% of test runs" in SC-002 a valid proxy for "correct decrement" behavior? [Spec §Success Criteria]

---

## Scenario Coverage

Items test whether all flows, cases, and conditions are addressed.

- [ ] CHK041 - Are primary flow requirements covered (ball hits 42, paddle hits 42, paddle hits 91)? [Spec §User Stories 1-3]
- [ ] CHK042 - Are alternate flow requirements covered (multiple balls, multiple bricks, overlapping bricks)? [Spec §Edge Cases, Plan §Testing]
- [ ] CHK043 - Are exception/error flow requirements covered (missing ball entity, invalid brick type)? [Plan §Risks & Mitigations]
- [ ] CHK044 - Are recovery flow requirements covered (life loss → respawn sequence → continue play)? [Spec §Assumptions]
- [ ] CHK045 - Are non-happy-path scenarios covered (boundary conditions, extreme values)? [Spec §Edge Cases]
- [ ] CHK046 - Are frame-boundary scenarios covered (collision at frame 0, frame N, frame N-1→N transition)? [Plan §Phase 1 Task 3]
- [ ] CHK047 - Is the zero-state scenario covered (level with only indestructible bricks → immediate completion)? [Spec §Edge Cases, User Story 3]
- [ ] CHK048 - Are state-transition scenarios covered (spawned → hit → destroyed/life loss → respawn)? [Data Model §State Transitions]

---

## Edge Case Coverage

Items test whether boundary conditions and uncommon scenarios are specified.

- [ ] CHK049 - Is the overlapping-brick scenario specified (paddle contacts both type 42 and 91 simultaneously)? [Spec §Edge Cases, FR-009]
- [ ] CHK050 - Is the rapid-succession scenario specified (paddle hits same brick twice within 2 frames)? [Spec §Edge Cases, FR-009]
- [ ] CHK051 - Is the multi-ball scenario specified (two balls destroy two type 42 bricks simultaneously)? [Spec §Edge Cases]
- [ ] CHK052 - Is the level-start-overlap scenario specified (paddle spawns overlapping a brick)? [Spec §Edge Cases]
- [ ] CHK053 - Is the physics-disabled scenario handled (what if rapier3d collision fails to fire)? [Plan §Risks & Mitigations]
- [ ] CHK054 - Is the missing-ball scenario handled (level with bricks but no ball entity)? [Plan §Phase 1 Task 2]
- [ ] CHK055 - Is the score-overflow scenario handled (90-point awards near u32::MAX)? [Spec §Success Criteria, Scoring Contract]
- [ ] CHK056 - Is the frame-rate variance scenario covered (60 FPS vs 30 FPS; multi-contact still = 1 loss)? [Spec §FR-009]

---

## Non-Functional Requirements

Items test whether quality attributes are specified.

- [ ] CHK057 - Is performance requirement specified (paddle collision detection MUST NOT cause latency spikes)? [Spec §Requirements, Non-Functional]
- [ ] CHK058 - Is reliability requirement specified (life loss MUST NOT cause game crash or data loss)? [Spec §Assumptions, Bevy 0.17]
- [ ] CHK059 - Is accessibility requirement specified (visual/audio feedback for paddle collision and life loss)? [Spec §User Scenarios]
- [ ] CHK060 - Is consistency requirement specified (same paddle collision behavior for all brick types)? [Spec §FR-002, FR-003]
- [ ] CHK061 - Is timing requirement specified (life loss event emission MUST occur within same frame as collision detection)? [Plan §Phase 1, System Execution Order]
- [ ] CHK062 - Is memory requirement specified (Local<bool> flag uses negligible memory)? [Plan §Architecture §Design Decision]

---

## Dependencies & Assumptions

Items test whether all assumptions are documented and validated.

- [ ] CHK063 - Is the existing lives system dependency documented (LifeLostEvent message, LivesState resource)? [Spec §Assumptions]
- [ ] CHK064 - Is the existing scoring system dependency documented (BrickDestroyed message, brick_points function)? [Spec §Assumptions]
- [ ] CHK065 - Is the existing level completion dependency documented (CountsTowardsCompletion query)? [Data Model §Level Completion Integration]
- [ ] CHK066 - Is the Bevy 0.17 event system choice (Messages over Observers) justified? [Plan §Design Decision, Bevy 0.17 REQUIREMENT]
- [ ] CHK067 - Is the assumption that type 42 already has a 90-point mapping validated? [Spec §Assumptions, Plan §Phase 0]
- [ ] CHK068 - Is the assumption that respawn flow handles LifeLostEvent correctly from any source documented? [Spec §Assumptions]
- [ ] CHK069 - Is the assumption that paddle entity always exists in playable levels stated? [Plan §Phase 1 Task 2]
- [ ] CHK070 - Are external dependencies (bevy_rapier3d collision events, physics config) documented? [Plan §Dependencies & Integration]

---

## Ambiguities & Conflicts Resolution

Items test whether ambiguities discovered during planning are resolved.

- [ ] CHK071 - Was the multi-contact life-loss policy ambiguity resolved? (Yes → Clarifications §Q1: "one life per frame") [Spec §Clarifications]
- [ ] CHK072 - Is type 91 integration with existing indestructible brick type 90 specified (separate or unified)? [Plan §Phase 0]
- [ ] CHK073 - Is the paddle-collision cause enum value specified (LifeLossCause::LowerGoal reused vs new PaddleHazard)? [Data Model §LifeLostEvent]
- [ ] CHK074 - Is the ball-entity fallback behavior specified when multiple balls exist (first ball, specific ball, random)? [Plan §Phase 1 Task 2]
- [ ] CHK075 - Is the visual/audio feedback requirement for paddle collisions within scope or deferred? [Spec §User Scenarios, Edge Cases]

---

## Traceability & Documentation

Items test whether requirements are linked to implementation and are easily referenced.

- [ ] CHK076 - Does each FR (FR-001 through FR-009) have at least one acceptance scenario tied to it? [Spec §Requirements §Functional, User Scenarios]
- [ ] CHK077 - Does each user story reference one or more success criteria? [Spec §User Stories, Success Criteria]
- [ ] CHK078 - Does the plan include explicit task count and per-phase breakdown for estimation? [Plan §Phases]
- [ ] CHK079 - Are all test cases explicitly named and linked to acceptance scenarios? [Plan §Phase 4, Data Model §Testing Anchors]
- [ ] CHK080 - Is there a cross-reference mapping between spec, plan, and data model documents? [Documents structure]

---

## Implementation Readiness

Items test whether the specification is ready to be handed off to implementation.

- [ ] CHK081 - Are all [NEEDS CLARIFICATION] markers resolved in the spec? [Spec §Clarifications]
- [ ] CHK082 - Is the plan broken into actionable phases with clear success criteria? [Plan §Phases]
- [ ] CHK083 - Are all files and functions to be modified explicitly listed? [Plan §Dependencies & Integration]
- [ ] CHK084 - Is the test suite defined with minimum 6 tests specified? [Plan §Phase 4, Data Model §Testing Anchors]
- [ ] CHK085 - Are pre-implementation validation steps (clippy, fmt, tests) specified? [Quickstart §Before Submitting]
- [ ] CHK086 - Are code examples (skeleton patterns) provided for reference? [Quickstart §Reference Implementation Patterns]
- [ ] CHK087 - Is there a manual test procedure for each user story? [Quickstart §Testing the Feature]
- [ ] CHK088 - Are build/run commands specified for local verification? [Quickstart §Testing]

---

## Summary

**Total Checklist Items**: 88

**Completion Status**:

- [ ] All 88 items MUST be checked before marking specification as "Ready for Implementation"
- [ ] Any unchecked item indicates a gap in requirements quality

**Recommendation**: When used in tandem with the specification (spec.md), plan (plan.md), and data model (data-model.md), this checklist validates that:

1. Requirements are **complete** (all necessary items specified)
2. Requirements are **clear** (unambiguous, measurable, testable)
3. Requirements are **consistent** (no conflicting rules)
4. Requirements are **well-scoped** (in-scope items defined; out-of-scope items deferred)
5. Implementation is **feasible** (tasks, phases, and test cases defined)
6. Handoff is **safe** (documentation sufficient for implementation team)

---

## How to Use This Checklist

**For Specification Authors**:

- Review each item against spec.md, plan.md, and data-model.md
- Uncheck items that reveal gaps → update spec documents
- Check items only when the corresponding requirement is unambiguous and complete

**For Implementation Team**:

- Before starting code, verify ALL items are checked
- Use checked items as confirmation that requirements are ready
- Reference this checklist during code review to ensure implementation matches validated requirements

**For QA/Testers**:

- Use success criteria from checked items to build test cases
- Cross-reference acceptance scenarios to test execution plans
