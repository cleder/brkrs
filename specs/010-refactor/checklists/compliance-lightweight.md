# Compliance Checklist (Lightweight PR Review): UI Constitution Refactor

**Purpose**: Lightweight “unit tests for requirements” before implementation work begins **Created**: 2025-12-19 **Feature**: [specs/010-refactor/spec.md](../spec.md)

**Profile**:

- **Depth**: Lightweight
- **Audience**: Reviewer (PR)
- **Focus**: Ensure requirements are clear enough to implement the audit + refactor plan without scope creep

**Notes on decisions**:

- Performance language may remain qualitative for this feature (no hard thresholds required).
- No Constitution version pin is required.

## Requirement Completeness

- [X] CHK001 Are deliverables explicitly defined (audit artifact + refactoring plan artifact + code compliance outcome)? [Completeness, Spec §User Story 1, Spec §User Story 2]
- [X] CHK002 Is the audited code scope precisely bounded (primary: `src/ui`) and are the allowed “minimal supporting edits” outside scope stated? [Completeness, Spec §Clarifications, Spec §FR-010]
- [X] CHK003 Is “compliance” explicitly tied to (a) Section VIII and (b) other applicable Constitution MUST/NEVER rules? [Clarity, Spec §Clarifications, Spec §FR-011]
- [X] CHK004 Are the required audit report fields specified (file, rule citation, explanation) so reviewers can confirm findings quickly? [Completeness, Spec §User Story 1]
- [X] CHK005 Are the required refactoring plan qualities specified (group small fixes, isolate complex logic, cite rules per task)? [Completeness, Spec §FR-004–FR-005]

## Requirement Clarity

- [X] CHK006 Is “minimal supporting edits” constrained enough to prevent scope creep (examples of allowed edits + a clear non-goal statement)? [Clarity, Spec §FR-010]
- [X] CHK007 Is “no player-facing UI changes” expressed in a way that can be reviewed (which UI surfaces are included: score/lives/pause/cheat indicator/level label/game over)? [Clarity, Spec §Intro, Spec §User Story 3]
- [X] CHK008 Is the expectation for “scoped-out findings” clear (where rationale is recorded, and who approves)? [Clarity, Spec §User Story 2, Spec §SC-001]

## Consistency

- [X] CHK009 Do scope statements match across Intro/Assumptions/FR-010 (no contradictions)? [Consistency, Spec §Intro, Spec §Assumptions, Spec §FR-010]
- [X] CHK010 Do success criteria align with requirements without implying extra work (e.g., quantified perf instrumentation)? [Consistency, Spec §Success Criteria, Spec §FR-008]

## Acceptance Criteria Quality

- [X] CHK011 Are the acceptance scenarios sufficient to validate the audit artifact quality (traceability + confirmability)? [Measurability, Spec §User Story 1]
- [X] CHK012 Are the acceptance scenarios sufficient to validate that refactoring preserves player-facing UI behavior? [Coverage, Spec §User Story 3]
- [X] CHK013 Is “UI should not do per-frame work when data doesn’t change” stated clearly enough to guide implementation (even if measured qualitatively)? [Clarity, Spec §User Story 2, Spec §SC-003]

## Scenario & Edge Case Coverage

- [X] CHK014 Are requirements present for duplicate UI entities (“multiple matches”)—do we skip, warn, or define deterministic selection? [Gap, Spec §Edge Cases]
- [X] CHK015 Are requirements present for missing fonts/assets during early WASM frames (fallback expectations)? [Gap, Spec §Edge Cases]
- [X] CHK016 Are requirements present for rapid pause/unpause/restart transitions (expected UI lifecycle outcomes)? [Gap, Spec §Edge Cases]

## Dependencies & Assumptions

- [X] CHK017 Is the TDD approval gate described with enough detail for reviewers to enforce (what proof constitutes “red commit”)? [Clarity, Spec §FR-009, Spec §Dependencies]
- [X] CHK018 Is it explicit whether new tests are required for UI areas without existing coverage (e.g., cheat indicator)? [Gap, Spec §Assumptions]

## Notes

- This checklist is intentionally lightweight; deeper measurability/performance quantification checks are out of scope by decision.
