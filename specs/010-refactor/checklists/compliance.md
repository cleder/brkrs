# Compliance Checklist: UI Constitution Refactor

**Purpose**: Unit tests for the *requirements/spec* quality of the UI Constitution compliance refactor **Created**: 2025-12-19 **Feature**: [specs/010-refactor/spec.md](../spec.md)

**Note**: This checklist evaluates whether the requirements are complete, unambiguous, measurable, and ready for implementation (not whether the code already complies).

## Requirement Completeness

- [X] CHK001 Are all deliverables explicitly defined (audit artifact, refactoring plan artifact, code refactor outcome)? [Completeness, Spec §User Story 1, Spec §User Story 2, Spec §FR-001–FR-006]
- [X] CHK002 Is the audited code scope precisely bounded (directories/files) and are exclusions explicit? [Completeness, Spec §Clarifications, Spec §Assumptions]
- [X] CHK003 Is the definition of “compliance” explicitly tied to the Constitution text and scoped to specific rule sets? [Clarity, Spec §Clarifications, Spec §FR-011]
- [X] CHK004 Are the required contents of the compliance audit report specified (fields, granularity, traceability expectations)? [Completeness, Spec §User Story 1, Spec §FR-003]
- [X] CHK005 Are the required contents of the refactoring plan specified (grouping rules, separation of complex changes, rule citations)? [Completeness, Spec §FR-004–FR-005]
- [X] CHK006 Are “minimal supporting edits outside `src/ui`” bounded with concrete examples and a definition of “minimal”? [Clarity, Spec §Clarifications, Spec §FR-010]
- [X] CHK007 Is the “no player-facing UI changes” constraint translated into explicit requirements for what must not change? [Completeness, Spec §Intro, Spec §FR-007]
- [X] CHK008 Are acceptance scenarios defined for each functional requirement, or is the mapping between FRs and scenarios made explicit? [Traceability, Spec §FR-001–FR-011, Spec §Acceptance Scenarios]

## Requirement Clarity

- [X] CHK009 Is “documented rationale” for scoped-out findings defined (where stored, minimum content, approval expectations)? [Clarity, Spec §User Story 2, Spec §SC-001]
- [X] CHK010 Is “performance regressions attributable to per-frame UI updates” defined with objective criteria (what counts as per-frame work, what threshold indicates regression)? [Ambiguity, Spec §FR-008, Spec §SC-003]
- [X] CHK011 Is “UI-relevant game state does not change” defined (which resources/components constitute UI source data)? [Clarity, Spec §User Story 2, Spec §SC-003]
- [X] CHK012 Are “optional UI inputs missing” scenarios enumerated beyond examples (which missing inputs are expected vs exceptional)? [Coverage, Spec §User Story 3]
- [X] CHK013 Are terms “legacy code”, “anti-pattern”, and “best practice mandate” aligned to Constitution language (Mandates/Prohibitions)? [Consistency, Spec §Input, Spec §Clarifications]

## Requirement Consistency

- [X] CHK014 Do scope statements remain consistent across sections (Intro, Assumptions, FR-010) with no contradictions? [Consistency, Spec §Intro, Spec §Assumptions, Spec §FR-010]
- [X] CHK015 Is “strictly against mandates/prohibitions” consistent with “Section VIII + other MUST/NEVER rules” (no hidden expansions)? [Consistency, Spec §Clarifications, Spec §FR-011]
- [X] CHK016 Do success criteria align with functional requirements (each SC supports one or more FRs, and no SC implies out-of-scope work)? [Consistency, Spec §Success Criteria, Spec §FR-001–FR-011]

## Acceptance Criteria Quality

- [X] CHK017 Can SC-001 (“100% of identified findings resolved or scoped out”) be objectively measured (what counts as “identified”, who identifies, and by when)? [Measurability, Spec §SC-001, Spec §FR-002]
- [X] CHK018 Is SC-003 (“not performed every frame”) measurable (how to detect/measure, acceptable instrumentation)? [Measurability, Spec §SC-003, Spec §FR-008]
- [X] CHK019 Is SC-004 (“maintainer can complete plan without inference”) operationalized into a measurable review rubric (e.g., required fields per task)? [Measurability, Spec §SC-004, Spec §FR-004–FR-005]
- [X] CHK020 Are acceptance scenarios written at a level that is testable without depending on undocumented internal details? [Clarity, Spec §Acceptance Scenarios]

## Scenario Coverage

- [X] CHK021 Are scenarios included for “multiple matches” query situations (duplicate UI entities) with explicit expected behavior (skip, warn, pick first)? [Coverage, Spec §Edge Cases]
- [X] CHK022 Are scenarios included for missing fonts/assets on WASM vs native, including expected fallback behavior? [Coverage, Spec §Edge Cases]
- [X] CHK023 Are scenarios included for rapid state transitions (pause/unpause/restart) specifying expected UI entity lifecycle outcomes? [Coverage, Spec §Edge Cases]
- [X] CHK024 Are scenarios included for “audit-only” completion (User Story 1) that don’t require refactor completion to be considered successful? [Coverage, Spec §User Story 1]

## Edge Case Coverage

- [X] CHK025 Are requirements defined for the case where the Constitution changes during the refactor (rule versioning / point-in-time reference)? [Gap]
- [X] CHK026 Are requirements defined for ambiguous Constitution interpretations (tie-breaker: which section wins if rules appear to conflict)? [Gap]
- [X] CHK027 Are requirements defined for partial compliance (some violations fixed, others deferred) including how deferrals are tracked? [Completeness, Spec §SC-001]

## Non-Functional Requirements

- [X] CHK028 Are performance expectations quantified beyond “no regressions” (e.g., max frequency of UI updates, constraints on allocations)? [Ambiguity, Spec §FR-008, Spec §SC-003]
- [X] CHK029 Are cross-platform considerations explicitly called out for this refactor (native vs WASM parity expectations for UI behavior)? [Coverage, Spec §Assumptions, Spec §Dependencies]

## Dependencies & Assumptions

- [X] CHK030 Are dependencies actionable and testable (what constitutes maintainer approval of the red commit; what evidence is required)? [Clarity, Spec §Dependencies, Spec §FR-009]
- [X] CHK031 Is it explicit whether existing tests are sufficient to prove “no behavior change”, and if not, which UI behaviors require new tests? [Gap, Spec §Assumptions, Spec §FR-007]

## Ambiguities & Conflicts

- [X] CHK032 Is it explicitly stated where the compliance audit and refactoring plan will live (paths, naming, review expectations), to avoid inconsistent documentation? [Gap, Spec §FR-001, Spec §FR-004]
- [X] CHK033 Is the “minimal supporting edits outside `src/ui`” constraint protected against scope creep (explicit non-goals/examples of disallowed changes)? [Gap, Spec §FR-010]

## Notes

- Defaults used: Depth = Standard; Audience = Reviewer (PR); Focus = Requirements quality for compliance audit + refactor scope.
- Each checklist item is intentionally phrased as a requirement-quality question, not an implementation test.
