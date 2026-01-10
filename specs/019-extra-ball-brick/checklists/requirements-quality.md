# Requirements Quality Checklist: Extra Ball Brick (Brick 41)

**Purpose**: Unit-test the quality of the requirements for brick 41 extra life + unique sound behavior.
**Created**: 2026-01-10 **Feature**: [specs/019-extra-ball-brick/spec.md](specs/019-extra-ball-brick/spec.md)

## Requirement Completeness

- [x] CHK001 Are asset/level metadata requirements explicit for brick id 41 (durability, score 0, sound handle) across spec and data model? [Completeness, Spec §FR-001; Data Model §Brick 41]
- [x] CHK002 Are life-cap configuration requirements referenced wherever life gain is mentioned to ensure clamping behavior is fully specified? [Completeness, Spec §User Story 1; Assumptions]
- [x] CHK003 Is the fallback sound behavior fully specified for missing/failed asset load, including which generic sound to use? [Completeness, Spec §Edge Cases; Spec §FR-004]

## Requirement Clarity

- [x] CHK004 Is "unique destruction sound" defined with an identifiable asset key/handle and channel so implementers cannot confuse it with other brick sounds? [Clarity, Spec §FR-004; Data Model §Messages]
- [x] CHK005 Is the life award path clearly tied to the standard brick-hit pipeline, including when the award is emitted relative to despawn? [Clarity, Spec §User Story 1; Contracts §Destruction Flow]
- [x] CHK006 Is "0 points" unambiguous for combos/multipliers (no multiplier trigger, no combo increment)? [Clarity, Spec §FR-003]

## Requirement Consistency

- [x] CHK007 Do User Story 1 acceptance scenarios and FR-002/FR-003 align on awarding +1 life with zero score impact? [Consistency, Spec §User Story 1; Spec §FR-002; Spec §FR-003]
- [x] CHK008 Is Message usage for life/audio consistent across FR-002/FR-005 and the contracts file (no observer usage creep)? [Consistency, Spec §FR-002; Spec §FR-005; Contracts §Life Award Message]
- [x] CHK009 Do coordinate system notes in data model match the constitution’s XZ-plane guidance to avoid directional ambiguity? [Consistency, Data Model §Coordinate System Notes; Constitution §VIII]

## Acceptance Criteria Quality

- [x] CHK010 Are success criteria measurable for sound playback (e.g., "plays once" and "no wrong sound") with traceable checks? [Acceptance Criteria, Spec §SC-003]
- [x] CHK011 Are success criteria measurable for life gain timing within one tick and clamp behavior? [Acceptance Criteria, Spec §SC-001]
- [x] CHK012 Is there an acceptance criterion or scenario validating score remains unchanged including multipliers/combos? [Acceptance Criteria, Spec §SC-002; Spec §FR-003]

## Scenario Coverage

- [x] CHK013 Do scenarios cover multi-ball simultaneous hits and ensure single award/despawn? [Coverage, Spec §Edge Cases; Spec §User Story 1]
- [x] CHK014 Are scenarios defined for audio fallback when the dedicated asset is missing or fails to load? [Coverage, Spec §Edge Cases; Spec §FR-004]
- [x] CHK015 Is WASM/audio platform variability covered or explicitly out-of-scope? [Coverage, Gap]

## Edge Case Coverage

- [x] CHK016 Are life-cap-at-max and at-minimum (e.g., life = 0) behaviors both specified for award and UI feedback? [Edge Case, Spec §Edge Cases; Gap]
- [x] CHK017 Is retry or duplicate-message handling defined if the life or audio message queue is saturated or delayed? [Edge Case, Gap]

## Non-Functional Requirements

- [x] CHK018 Are performance expectations (no extra per-frame cost, asset reuse) documented for this brick’s logic? [Non-Functional, Plan §Summary; Plan §Technical Context]
- [x] CHK019 Are observability/logging expectations for life awards and sound fallback specified? [Non-Functional, Gap]

## Dependencies & Assumptions

- [x] CHK020 Are dependencies on existing audio bus/channel definitions and life-counter consumers captured and versioned? [Dependency, Contracts §Audio Trigger Message; Data Model §Messages]
- [x] CHK021 Are assumptions about existing max-life configuration and generic brick sound availability documented and validated? [Assumption, Spec §Assumptions; Spec §FR-004]

## Ambiguities & Conflicts

- [x] CHK022 Is the term "unique" sound free of conflict with other bricks that may also claim uniqueness (naming/handle collision)? [Ambiguity, Spec §FR-004; Gap]
- [x] CHK023 Is the message-event separation requirement (FR-005) free of conflict with any observer-based audio patterns elsewhere in the codebase? [Conflict, Spec §FR-005; Plan §Constitution Check]

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Items are numbered sequentially for easy reference
