# UX Requirements Quality Checklist

Created: 2026-01-11 Feature: specs/021-gravity-bricks/spec.md Plan: specs/021-gravity-bricks/plan.md Purpose: Validate the quality (completeness, clarity, consistency, measurability, coverage) of UX-related requirements for the Gravity Indicator UI.

## Requirement Completeness

- [ ] CHK001 Are UI placement offsets (12px) and anchoring rules fully specified across window modes? [Completeness, Spec §FR-001; Spec §P2]
- [ ] CHK002 Are initial spawn conditions specified (GravityConfiguration + textures loaded) with failure handling? [Completeness, Spec §FR-001; Spec §FR-006]
- [ ] CHK003 Are all recognized gravity levels documented (0/2/10/20) and unknown behavior defined? [Completeness, Spec §FR-003]
- [ ] CHK004 Are mixed-axis selection rules (highest magnitude wins) specified for X/Z? [Completeness, Spec §FR-004]
- [ ] CHK005 Are pause and life-loss scenarios covered for indicator stability and reset behavior? [Completeness, Spec §P3; Spec §FR-005]
- [ ] CHK006 Are multi-frame persistence requirements defined (≥10 frames) after change events? [Completeness, Spec §MULTI-FRAME; Spec §FR-008]
- [ ] CHK007 Are asset paths and availability requirements documented, including non-fatal failure behavior? [Completeness, Spec §FR-006]
- [ ] CHK008 Are developer indicator coexistence rules documented (opposite corners, no overlap)? [Completeness, Spec §P2]

## Requirement Clarity

- [ ] CHK009 Is the ±0.5 tolerance explicitly defined with rounding semantics and axis scope (X/Z only, Y ignored)? [Clarity, Spec §FR-002; Spec §Edge Cases]
- [ ] CHK010 Is "update within one frame" quantified with scheduling assumptions (e.g., response to Changed<GravityConfiguration>)? [Clarity, Spec §FR-005]
- [ ] CHK011 Is "visible over overlays" defined for game-over and pause layers (z-order / UI layer assumptions)? [Clarity, Spec §FR-007]
- [ ] CHK012 Are asset failure logs specified as non-fatal with explicit skip behavior (no placeholder rendering)? [Clarity, Spec §FR-006]
- [ ] CHK013 Is the bottom-left position defined as absolute offsets from the window edge (not layout-relative)? [Clarity, Spec §FR-001]

## Requirement Consistency

- [ ] CHK014 Do mapping rules and edge cases align (threshold examples match the ±0.5 tolerance statement)? [Consistency, Spec §FR-002; Spec §Edge Cases]
- [ ] CHK015 Do spawn/update rules align with change detection (no per-frame updates implied elsewhere)? [Consistency, Spec §FR-005; Spec §BEVY 0.17]
- [ ] CHK016 Do developer indicator coexistence rules conflict with any other UI placement requirements? [Consistency, Spec §P2]
- [ ] CHK017 Do success criteria (SC-001..004) align with functional requirements (FR-001..008) without contradictions? [Consistency, Spec §Success Criteria; Spec §Functional Requirements]

## Acceptance Criteria Quality

- [ ] CHK018 Are success criteria measurable with objective checks (frame update latency, tolerance correctness, visibility)? [Acceptance Criteria, Spec §SC-001..SC-004]
- [ ] CHK019 Are acceptance scenarios testable independently and mapped directly to user stories? [Acceptance Criteria, Spec §User Stories]
- [ ] CHK020 Is there an ID scheme (FR/SC/US) enabling traceability between requirements and tests? [Traceability]

## Scenario Coverage

- [ ] CHK021 Are primary scenarios covered (spawn on first valid frame, updates on gravity changes)? [Coverage, Spec §US1; Spec §FR-001; Spec §FR-005]
- [ ] CHK022 Are alternate scenarios covered (mixed X/Z axes, sequential changes within a frame)? [Coverage, Spec §US1; Spec §Edge Cases]
- [ ] CHK023 Are exception scenarios covered (asset missing, unrecognized gravity values)? [Coverage, Spec §FR-006; Spec §FR-003]
- [ ] CHK024 Are recovery scenarios covered (life loss reset, level transition updates)? [Coverage, Spec §US3; Spec §Edge Cases]

## Edge Case Coverage

- [ ] CHK025 Are threshold boundaries (e.g., 1.6–2.4, 9.6–10.4) explicitly documented and consistent with tolerance rules? [Edge Case, Spec §Edge Cases]
- [ ] CHK026 Are multiple changes per frame resolved to last-write-wins with clarity on ordering guarantees? [Edge Case, Spec §US1; Spec §Edge Cases]
- [ ] CHK027 Is behavior defined when textures are not yet loaded at spawn time (deferred spawn strategy)? [Edge Case, Spec §Clarifications; Spec §FR-001]

## Non-Functional Requirements

- [ ] CHK028 Are performance goals documented (update within one frame, minimal overhead, WASM compatibility)? [Non-Functional, Spec §Technical Context; Spec §SC-001]
- [ ] CHK029 Are accessibility considerations documented (indicator visibility, color/contrast of icons across backgrounds)? [Gap]
- [ ] CHK030 Are DPI scaling and resolution independence specified for icon rendering? [Gap]

## Dependencies & Assumptions

- [ ] CHK031 Are dependencies on `GravityConfiguration` and texture assets explicitly documented with load-order assumptions? [Dependencies, Spec §FR-001; Spec §FR-006]
- [ ] CHK032 Is the assumption that Y-axis is ignored validated and documented with rationale? [Assumption, Spec §FR-002]
- [ ] CHK033 Are scheduling assumptions (Changed<Resource>, Update schedule) documented for measurability? [Assumption, Spec §FR-005]

## Ambiguities & Conflicts

- [ ] CHK034 Is the term "visible over overlays" free of ambiguity (define layering/z-index requirements)? [Ambiguity, Spec §FR-007]
- [ ] CHK035 Do any requirements conflict with Bevy 0.17 mandates (e.g., per-frame UI updates)? [Conflict, Spec §BEVY 0.17]
- [ ] CHK036 Is an explicit requirement/test ID scheme established to reduce cross-document ambiguity? [Traceability, Gap]
