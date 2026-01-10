# Gameplay Requirements Quality Checklist: Merkaba Rotor Brick

**Purpose**: Validate requirement quality for gameplay, physics, audio, and event architecture **Created**: 2026-01-07 **Feature**: [specs/018-merkaba-rotor-brick/spec.md](specs/018-merkaba-rotor-brick/spec.md)

## Requirement Completeness

- [x] CHK001 Are audio requirements defined for all collision surfaces (wall, brick, paddle)? [Completeness, Spec §US2/FR-017..FR-019]
- [x] CHK002 Is helicopter blade loop behavior specified for start/stop conditions when merkabas exist? [Completeness, Spec §US2/FR-020..FR-021]
- [x] CHK003 Are spawn timing and location requirements fully defined (delay and position)? [Completeness, Spec §Clarifications/FR-003, §US1]
- [x] CHK004 Is minimum y-speed requirement defined with a numeric threshold and enforcement context? [Completeness, Spec §FR-007]
- [x] CHK005 Are paddle-contact consequences specified for life, balls, and merkabas? [Completeness, Spec §US3/FR-012..FR-014, FR-022]
- [x] CHK006 Is rotor brick visual distinction defined with a specific method? [Completeness, Spec §SC-001]

## Requirement Clarity

- [x] CHK007 Is the term “gaming plane” clarified sufficiently for implementers (z constraint magnitude/approach)? [Clarity, Spec §US2/FR-008]
- [x] CHK008 Is “distinct collision sound” defined in terms of differentiation (unique per surface)? [Clarity, Spec §US2/FR-017..FR-019]
- [x] CHK009 Is the helicopter loop sound’s behavior under pause/mute states explicitly defined? [Clarity, Spec §Edge Cases, §Assumptions]
- [x] CHK010 Are angle variance and initial direction clearly quantified (±20° around horizontal y)? [Clarity, Spec §US1/FR-006]
- [x] CHK011 Is the rotation behavior specified (continuous around z-axis) without ambiguity of rate? [Ambiguity, Spec §US1/FR-005]

## Requirement Consistency

- [x] CHK012 Do spawn delay (0.5s) and message-based async architecture align consistently across US1 and FR-003? [Consistency, Spec §US1/FR-003]
- [x] CHK013 Do audio requirements respect global audio settings as stated in assumptions? [Consistency, Spec §Assumptions/Audio]
- [x] CHK014 Do paddle-contact outcomes (life loss + despawns) consistently stop the helicopter loop? [Consistency, Spec §US3/FR-021]

## Acceptance Criteria Quality

- [x] CHK015 Are acceptance scenarios independently testable for rotor brick → message → delayed spawn? [Acceptance, Spec §US1]
- [x] CHK016 Are audio acceptance scenarios measurable (sound emits within reasonable frame timing)? [Acceptance, Spec §US2]
- [x] CHK017 Are min y-speed acceptance checks actionable (post-collision or periodic enforcement)? [Acceptance, Spec §US2/FR-007]
- [x] CHK018 Are paddle-contact scenarios complete (life decrement, balls and merkabas despawn, loop stops)? [Acceptance, Spec §US3]

## Scenario Coverage

- [x] CHK019 Are primary, alternate, and exception flows covered (normal movement, collisions, goal despawn, paddle contact)? [Coverage, Spec §US1..US3]
- [x] CHK020 Is multi-merkaba coexistence addressed, including audio loop behavior? [Coverage, Spec §US2/FR-015, FR-020..FR-021]
- [x] CHK021 Are pause-state behaviors covered for timers and audio loops? [Coverage, Spec §Edge Cases]

## Edge Case Coverage

- [x] CHK022 Are overlapping spawns at occupied positions addressed with physics resolution? [Edge Case, Spec §Edge Cases]
- [x] CHK023 Are simultaneous collisions producing overlapping sounds considered? [Edge Case, Spec §Edge Cases]
- [x] CHK024 Are level transitions canceling pending spawn timers documented? [Edge Case, Spec §Edge Cases]

## Non-Functional Requirements

- [x] CHK025 Are performance targets measurable (30+ FPS under 5 merkabas) and aligned with constitution 60 FPS goal? [Performance, Spec §SC-003/SC-006]
- [x] CHK026 Is audio asset strategy documented (placeholder/synthesized) with upgrade path implied? [NFR, Spec §Clarifications/Assumptions]
- [x] CHK027 Is event system rationale documented (Messages vs Observers) per constitution? [NFR, Spec §US1/Bevy Requirement]

## Dependencies & Assumptions

- [x] CHK028 Are audio and texture asset availability assumptions explicitly documented? [Assumptions, Spec §Assumptions]
- [x] CHK029 Is level data format support (brick index 36) documented as an assumption? [Assumptions, Spec §Assumptions]
- [x] CHK030 Is pause behavior affecting timers and audio loops documented? [Assumptions, Spec §Assumptions]

## Ambiguities & Conflicts

- [x] CHK031 Is rotation rate around z-axis specified or intentionally left flexible? [Ambiguity, Spec §FR-005]
- [x] CHK032 Is “gaming plane” constraint tolerance defined (exact z or band)? [Ambiguity, Spec §FR-008]
- [x] CHK033 Are audio differentiation rules (e.g., distinct samples per surface) confirmed? [Ambiguity, Spec §FR-017..FR-019]

## Traceability & IDs

- [x] CHK034 Is a requirement & acceptance criteria ID scheme established linking US/FR/SC to tests? [Traceability]
- [x] CHK035 Do ≥80% of checklist items include a traceability reference or Gap/Ambiguity marker? [Traceability]

## Notes

- Each `/speckit.checklist` run creates a new checklist file.
- This checklist tests requirement quality, not implementation behavior.
- Mark items with findings inline and add references to updated spec sections.
