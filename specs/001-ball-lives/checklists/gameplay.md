# Gameplay Requirements Checklist: Ball Lives Counter

**Purpose**: Unit tests for requirements writing quality (gameplay/state focus) for the Ball Lives Counter feature **Created**: 2025-12-14 **Feature**: [spec.md](../spec.md) **Audience**: PR reviewer **Depth**: Standard **Scope focus**: Gameplay/state requirements quality (Q1=B) **Notes**: UI font choice (Orbitron) is treated as a plan/tasks constraint, not a spec completeness requirement (Q3=B)

## Requirement Completeness

- [x] CHK001 Does the spec define what qualifies as a "new play session" (start trigger and reset point)? [Resolved, Spec §Assumptions]
- [x] CHK002 Does the spec define the lives reset behavior for level restart (e.g., pressing R) explicitly, or is it intentionally out of scope? [Gap, Spec §FR-001, Spec §Assumptions] Out of scope
- [x] CHK003 Does the spec define whether lives persist across level transitions (and whether that is a requirement vs an assumption)? [Assumption, Spec §Assumptions]
- [x] CHK004 Does the spec define what “LifeLostEvent occurs” means if multiple losses occur close together (e.g., duplicate event emission) and whether each event is counted? [Completeness, Spec §Edge Cases]
- [x] CHK005 Does the spec explicitly define whether the game should attempt a respawn after lives reach 0, or is gameplay considered ended immediately? [Resolved, Spec §FR-009]

## Requirement Clarity

- [x] CHK006 Is "decremented by one" unambiguous about being exactly one per event and not time-based or frame-based? [Resolved, Spec §FR-002]
- [x] CHK007 Is "no balls are left" clearly defined as "remaining balls count == 0" rather than "no ball entities exist in the world"? [Resolved, Spec §FR-006]
- [x] CHK008 Is "display a Game over message" specified with exact message text/capitalization ("Game over" vs "GAME OVER") or explicitly stated as flexible? [Resolved, Spec §FR-006]
- [x] CHK009 Is “gets displayed on the screen” specific about when it becomes visible (e.g., on gameplay start) and under what conditions it might be hidden? [Resolved, Spec §FR-004, Spec §FR-005, Spec §FR-006]
- [x] CHK010 Is "immediately after it changes" quantified with a measurable latency threshold or frame boundary expectation? [Resolved, Spec §FR-005, Spec §SC-003]

## Requirement Consistency

- [x] CHK011 Do the user stories’ acceptance scenarios align with FRs on starting lives (3) and decrement behavior? [Resolved, Spec §User Story 1, Spec §FR-001, Spec §FR-002]
- [ ] CHK012 Do the edge cases align with FR-002/FR-003 regarding duplicate events and clamping at 0? [Consistency, Spec §Edge Cases, Spec §FR-002, Spec §FR-003]
- [x] CHK013 Are the assumptions (persist across levels; reset only on new session) consistent with the user stories and acceptance scenarios? [Resolved, Spec §Assumptions, Spec §User Story 1]
- [x] CHK014 Do success criteria (SC-003 "within 1 second") align with the unquantified "immediately" language in FR-005, or does the spec need one consistent timing definition? [Resolved, Spec §FR-005, Spec §SC-003]

## Acceptance Criteria Quality

- [x] CHK015 Are all functional requirements backed by at least one acceptance scenario or measurable outcome (directly or indirectly)? [Resolved, Spec §FR-001–FR-009, Spec §User Stories, Spec §SC-001–SC-004]
- [x] CHK016 Are the acceptance scenarios written so they can be evaluated without relying on hidden implementation details (e.g., not requiring knowledge of event system internals)? [Resolved, Spec §User Scenarios & Testing]
- [x] CHK017 Is the “usability check with at least 5 participants” in SC-004 sufficiently specified (who, where, prompt wording, success measurement) to avoid subjective interpretation? [Resolved, Spec §SC-004]

## Scenario Coverage

- [x] CHK018 Does the spec cover the primary flow: start session → see 3 lives → lose lives → see count update? [Resolved, Spec §Primary Flow, Spec §User Story 1]
- [x] CHK019 Does the spec cover the terminal flow: last life loss → count reaches 0 → game over message appears and remains? [Resolved, Spec §User Story 2, Spec §FR-005–FR-007]
- [x] CHK020 Does the spec address what happens after game over (e.g., input handling, ability to restart, ability to continue watching)?
  If intentionally excluded, is that exclusion explicit?
  [Resolved, Spec §FR-009]

## Edge Case Coverage

- [x] CHK021 Is the “multiple LifeLostEvent occurrences in the same moment” edge case sufficiently detailed to avoid conflicting interpretations (e.g., count decrements once per event vs de-duplication rules)? [Resolved, Spec §Edge Cases]
- [ ] CHK022 Does the spec define behavior when LifeLostEvent occurs while already at 0 lives beyond “remain at 0” (e.g., should game-over message re-trigger, re-animate, or stay unchanged)? [Gap, Spec §User Story 3, Spec §FR-007]
- [ ] CHK023 Are boundary conditions for lives value types addressed (max lives, overflow/underflow expectations, future extensibility) or intentionally out of scope? [Gap, Spec §FR-003]

## Non-Functional Requirements

- [ ] CHK024 Does the spec define any readability requirements for the lives display (e.g., minimum font size/contrast, placement stability), or is UI styling intentionally unspecified? [Gap, Spec §FR-004]
- [ ] CHK025 Does the spec define cross-platform constraints relevant to the feature (native vs WASM) for text/overlay behavior, or explicitly state that behavior is identical? [Gap, Spec §Success Criteria]
- [x] CHK026 Does the spec state whether the game-over message should be visible regardless of pause state (or other overlays), and if so is that requirement explicit? [Resolved, Spec §FR-008]

## Dependencies & Assumptions

- [ ] CHK027 Are all assumptions (persist across levels; reset conditions; definition of LifeLostEvent) explicitly validated as acceptable product behavior, not just inferred? [Assumption, Spec §Assumptions]
- [ ] CHK028 Are dependencies on existing domain signals (LifeLostEvent, game-over signaling) documented at a requirements level (what the system relies on) without leaking implementation detail? [Dependency, Spec §Key Entities]
- [ ] CHK029 Is the Orbitron font requirement documented in a consistent “non-functional constraints” place (plan/tasks) and explicitly treated as outside the spec’s functional scope? [Consistency, Plan §Resolved Decisions, Tasks §Phase 1–2]

## Ambiguities & Conflicts

- [x] CHK030 Are key terms defined consistently: "balls", "lives", "remaining lives count", and "life lost"? [Resolved, Spec uses canonical term "lives" throughout]
- [x] CHK031 Is there any tension between "event-driven decrement" semantics and "decrement at most once" wording in edge cases that could be read as de-duplication? [Resolved, Spec §Edge Cases clarifies one decrement per event]

## Notes

- This checklist evaluates the *quality of the written requirements*, not whether the implementation works.
- Each run creates a new checklist file; do not overwrite older checklists.
