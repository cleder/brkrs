# Requirements Quality Checklist: Ball, Paddle, Brick Physics Config

**Purpose**: Validate the quality, clarity, and completeness of requirements for the centralized physics config feature.
**Created**: 2025-12-25 **Feature**: [specs/015-ball-physics-config/spec.md](../spec.md)

## Requirement Completeness

- [ ] CHK001 Are all physics properties (restitution, friction, damping, etc.) for balls, paddles, and bricks explicitly listed in the requirements? [Completeness, Spec §Functional Requirements]
- [ ] CHK002 Are requirements defined for all entities with physics properties, not just balls? [Coverage, Spec §FR-007]
- [ ] CHK003 Are requirements for config registration, usage, and enforcement present for each entity type? [Completeness, Spec §Functional Requirements]
- [ ] CHK004 Are requirements for error handling (missing/invalid config) explicitly stated? [Completeness, Spec §FR-006]

## Requirement Clarity

- [ ] CHK005 Is it clear that all config must be encoded in source and not hot-reloadable? [Clarity, Spec §FR-008, Edge Cases]
- [ ] CHK006 Are the boundaries of what is and is not configurable (e.g., no runtime mutation) unambiguous? [Clarity, Spec §Edge Cases]
- [ ] CHK007 Is the documentation requirement for config location and usage explicit? [Clarity, Spec §FR-004]

## Requirement Consistency

- [ ] CHK008 Are requirements for all entity types (ball, paddle, brick) consistent in structure and enforcement? [Consistency, Spec §FR-007]
- [ ] CHK009 Are error handling requirements consistent across all config types? [Consistency, Spec §FR-006]

## Acceptance Criteria Quality

- [ ] CHK010 Are all success criteria measurable and technology-agnostic? [Acceptance Criteria, Spec §Success Criteria]
- [ ] CHK011 Are acceptance scenarios defined for both correct config usage and error cases? [Acceptance Criteria, Spec §User Scenarios, Edge Cases]

## Scenario & Edge Case Coverage

- [ ] CHK012 Are edge cases for missing/invalid config and non-hot-reloadability addressed? [Edge Case, Spec §Edge Cases]
- [ ] CHK013 Are requirements defined for all spawn paths (multiple ways to spawn entities)? [Coverage, Spec §User Scenarios]

## Non-Functional Requirements

- [ ] CHK014 Are non-functional requirements (performance, maintainability, testability) specified or referenced? [Non-Functional, Spec §Functional Requirements, Success Criteria]

## Dependencies & Assumptions

- [ ] CHK015 Are all dependencies (e.g., Bevy ECS, in-memory state) and assumptions (no config files, no runtime mutation) documented? [Dependencies, Spec §Technical Context, Data Model]

## Ambiguities & Conflicts

- [ ] CHK016 Are there any ambiguous or conflicting requirements regarding config mutability, error handling, or entity coverage? [Ambiguity, Spec §Edge Cases, Functional Requirements]

---

*Each item should be checked before implementation or review.*
   *If any are incomplete, update the spec or plan before proceeding.*
