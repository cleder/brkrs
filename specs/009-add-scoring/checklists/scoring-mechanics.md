# Scoring Mechanics & Data Model Checklist: Add Scoring System

**Purpose**: Validate completeness, clarity, and consistency of scoring mechanics and data model requirements before planning phase **Created**: 16 December 2025 **Feature**: [spec.md](../spec.md) **Depth**: Standard (Core requirement quality checks) **Audience**: Author (Pre-Planning validation)

## Requirement Completeness

- [x] CHK001 - Are score initialization requirements defined for all game start conditions? [Completeness, Spec §FR-001]
- [x] CHK002 - Are score accumulation rules specified for all destructible brick types (indices 10-57)? [Completeness, Spec §FR-003, §FR-008]
- [x] CHK003 - Are requirements defined for score behavior during level transitions (advance/return)? [Gap, related to Assumptions]
- [x] CHK004 - Are brick point value mapping requirements traceable to the external docs/bricks.md source? [Completeness, Spec §FR-008]
- [x] CHK005 - Are requirements specified for score state persistence across the game session? [Completeness, Clarification 2025-12-16]
- [x] CHK006 - Are milestone detection requirements defined for all multiples of 5000 points? [Completeness, Spec §FR-005]
- [x] CHK007 - Are requirements specified for special scoring bricks (Question, Extra Ball, Magnet)? [Completeness, Spec §FR-009, Assumptions]

## Requirement Clarity

- [x] CHK008 - Is "game session" clearly defined to distinguish it from individual levels? [Clarity, Assumptions]
- [x] CHK009 - Is the random score range for Question brick (25-300) unambiguously specified with distribution type? [Clarity, Spec §FR-009, Clarification 2025-12-16]
- [x] CHK010 - Is "synchronous (immediate)" score accumulation timing quantified with measurable criteria? [Clarity, Assumptions]
- [x] CHK011 - Are the exact conditions triggering ball spawns at score milestones explicitly defined? [Clarity, Spec §FR-005]
- [x] CHK012 - Is "clearly visible location" for score display quantified with specific UI positioning requirements? [Ambiguity, Spec §FR-006]
- [x] CHK013 - Is "real-time" score display update quantified beyond the 16ms success criterion? [Clarity, Spec §FR-007, §SC-003]

## Requirement Consistency

- [x] CHK014 - Are score persistence requirements consistent between Clarifications and Assumptions sections? [Consistency, Clarifications vs Assumptions]
- [x] CHK015 - Do brick point value requirements align between FR-003, FR-008, and SC-004? [Consistency, Spec §FR-003/008, §SC-004]
- [x] CHK016 - Are milestone ball spawn requirements consistent between FR-005, User Story 3, and SC-002/006? [Consistency, Spec §FR-005, §US3, §SC-002/006]
- [x] CHK017 - Are special effect independence requirements (Clarification Q4) reflected consistently in FR-003? [Consistency, Clarification 2025-12-16, Spec §FR-003]
- [x] CHK018 - Do exclusion statements for multiplier bricks align across FR-004, Clarifications, and Assumptions? [Consistency, Spec §FR-004, Clarifications, Assumptions]

## Data Model Quality

- [x] CHK019 - Are all Score entity attributes (initial value, data type, range constraints) explicitly specified? [Completeness, Spec §Key Entities]
- [x] CHK020 - Is the relationship between Score and Ball Lives milestone mechanics clearly defined? [Clarity, Spec §Key Entities]
- [x] CHK021 - Are brick-to-point-value mappings specified with sufficient detail for implementation? [Completeness, Spec §Key Entities, §FR-008]
- [x] CHK022 - Is the Score Multiplier entity necessary given multipliers are out of scope (FR-004)? [Conflict, Spec §Key Entities vs §FR-004]
- [x] CHK023 - Are data type constraints defined for score values (e.g., integer, maximum value)? [Gap, Spec §Key Entities]

## Acceptance Criteria Quality

- [x] CHK024 - Can "score increases from 0 to at least 1000 points" (SC-001) be objectively verified? [Measurability, Spec §SC-001]
- [x] CHK025 - Is the 1-second ball spawn timing (SC-002) testable with clear pass/fail criteria? [Measurability, Spec §SC-002]
- [x] CHK026 - Can "100% of destructible bricks award points" (SC-007) be verified without implementation? [Measurability, Spec §SC-007]
- [x] CHK027 - Are acceptance criteria defined for score persistence across level boundaries? [Gap, related to Clarification 2025-12-16]
- [x] CHK028 - Are acceptance criteria specified for Question brick random score distribution? [Gap, Spec §FR-009]

## Scenario Coverage

- [x] CHK029 - Are requirements defined for score behavior at exactly 5000 points (boundary condition)? [Coverage, Edge Cases]
- [x] CHK030 - Are requirements specified for rapid successive brick destructions? [Coverage, Edge Cases]
- [x] CHK031 - Are requirements defined for concurrent milestone triggering (e.g., reaching 5000 points while completing level)? [Coverage, Edge Cases]
- [x] CHK032 - Are requirements specified for score overflow scenarios (maximum value)? [Gap, Edge Case]
- [x] CHK033 - Are requirements defined for zero-score game scenarios (player destroys no bricks)? [Coverage, Edge Case]
- [x] CHK034 - Are requirements specified for Question brick score randomization edge cases (min/max values)? [Coverage, Spec §FR-009]

## Dependencies & Assumptions

- [x] CHK035 - Is the dependency on docs/bricks.md as the source of truth validated and documented? [Dependency, Spec §FR-008]
- [x] CHK036 - Is the assumption that Extra Ball brick (41) uses a different mechanism validated? [Assumption, Assumptions section]
- [x] CHK037 - Are assumptions about synchronous score accumulation validated against game architecture? [Assumption, Assumptions section]
- [x] CHK038 - Is the assumption that Magnet bricks have no score value explicitly confirmed? [Assumption, Assumptions section]
- [x] CHK039 - Are integration points with existing ball spawning/lives systems documented as dependencies? [Gap, related to §FR-005]

## Ambiguities & Conflicts

- [x] CHK040 - Is the inclusion of Score Multiplier entity (Key Entities) conflicting with multipliers being out of scope (FR-004)? [Conflict, Spec §Key Entities vs §FR-004]
- [x] CHK041 - Are "points only accrue from destroyed bricks, not spawned entities" requirements formalized? [Ambiguity, Edge Cases]
- [x] CHK042 - Is the behavior at level transition bricks (50, 54) "award points before advancing" specified with timing details? [Ambiguity, Assumptions]
- [x] CHK043 - Are requirements clear on whether ball spawn happens before or after score display update at milestones? [Ambiguity, related to §FR-005]

## Notes

- Focus: Scoring mechanics and data model requirement quality validation
- Depth: Standard (completeness, clarity, consistency, measurability)
- Use: Author pre-planning phase validation
- Items marked [Gap] indicate missing requirements that should be added
- Items marked [Ambiguity] or [Conflict] require clarification or resolution before planning
