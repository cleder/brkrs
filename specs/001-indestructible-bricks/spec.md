# Feature Specification: Indestructible bricks (LevelDefinition)

**Feature Branch**: `001-indestructible-bricks`
**Created**: 2025-11-28
**Status**: Draft
**Input**: LevelDefinition should support indestructible bricks which do not count toward level completion. The indestructible brick uses tile index `90`. We are moving the canonical simple (destructible) brick index from `3` → `20` for newly authored levels; existing files using `3` are handled via a repository migration policy (see Clarifications).

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.

  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:

- Developed independently
- Tested independently
- Deployed independently
- Demonstrated to users independently
-->

### User Story 1 - Player completes levels regardless of indestructible bricks (Priority: P1)

As a player, when all destructible bricks in a level are destroyed, the level should complete even if indestructible bricks remain on screen.

**Why this priority**: Ensures the new brick type does not prevent players from finishing a level — this is the primary user-facing behavior change.

**Independent Test**: Load a level containing a mixture of destructible and indestructible bricks, destroy all destructible bricks and verify the level completes immediately and rewards (if any) are granted.

**Acceptance Scenarios**:

1. **Given** a level with one destructible brick and multiple indestructible bricks, **When** the player destroys the destructible brick, **Then** the level is marked complete and completion logic triggers.
2. **Given** a level with zero destructible bricks (only indestructible bricks present), **When** the level starts, **Then** the level should be considered complete (or skippable) because there are no destructible bricks to clear.

---

### User Story 2 - Level designer can place indestructible bricks (Priority: P2)

As a level designer, I can place indestructible bricks in the LevelDefinition matrix using a specific tile index so that I can create unbreakable obstacles and visual elements that do not affect level completion.

**Why this priority**: Enables content creators to design levels with decorative/unbreakable areas and standard gameplay bricks without unintentionally blocking completion.

**Independent Test**: Create or edit a level matrix and insert the indestructible index (90) — verify the editor/game renders an indestructible brick at that location and that it cannot be destroyed but otherwise behaves as a colliding brick.

**Acceptance Scenarios**:

1. **Given** a level matrix containing value `90` at a position, **When** the level is loaded, **Then** an indestructible brick is spawned at that location.
2. **Given** an indestructible brick and the player makes contact, **When** the ball collides with it, **Then** the ball reacts normally (bounces) and the brick remains undamaged.

---

### User Story 3 - Update simple brick index for clear semantics (Priority: P3)

As a developer or content maintainer, the existing simple (destructible) brick index should move from `3` to `20` so that new index values (e.g., `90`) can be reserved for special brick types with clearer separation of ranges.

**Why this priority**: Changing the simple brick index is an internal housekeeping step required to avoid index conflicts and reserve index ranges for distinct brick behaviours.

**Independent Test**: Load a level that uses index `20` for simple bricks and verify they behave like the prior index `3` bricks. Also verify that any level file that still contains `3` behaves according to migration policy (see Clarifications).

**Acceptance Scenarios**:

1. **Given** a level matrix with `20` at tile positions, **When** the level is loaded, **Then** those tiles spawn as simple destructible bricks and contribute to the level completion counter.

---

[Add more user stories as needed, each with an assigned priority]

### Edge Cases

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right edge cases.
-->

- Levels authored with only indestructible bricks (no destructible bricks): The system should treat the level as already satisfied and mark it as complete on start, or present a clear message to designers during authoring.
- Levels authored before this change that encode simple bricks with index `3`: See Clarifications — the migration approach will determine runtime behaviour.
- Collision interactions at tile boundaries when indestructible bricks abut destructible bricks: verify normal physics/bounce and no accidental destruction.

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: The LevelDefinition parser MUST recognise the tile index `90` as a new brick type called "indestructible" and instantiate an indestructible brick where that value is present.
- **FR-002**: Indestructible bricks MUST not decrement or otherwise count toward the remaining-destructible-bricks counter used to determine level completion.
- **FR-003**: Indestructible bricks MUST continue to participate in normal collision behaviour (ball bounces, ball velocity may change) but MUST not be destroyed by collisions, powerups, or other in-game effects that would normally break destructible bricks.
- **FR-004**: The interpretation of the simple (destructible) brick MUST be updated so that the index `20` represents the simple brick type for all newly authored level definitions.
- **FR-005**: The system MUST provide clear, deterministic behaviour for existing levels that contain the old simple brick index `3` (see Clarifications). This behaviour must be defined and testable before implementation.

*NOTE (Assumption)*: Unless otherwise decided (see Clarifications), levels that explicitly use index `3` should remain readable but a migration plan or documentation should be provided so designers know how index `3` will be treated going forward.

*Example of marking unclear requirements:*

- **FR-006**: System MUST authenticate users via [NEEDS CLARIFICATION: auth method not specified - email/password, SSO, OAuth?]
- **FR-007**: System MUST retain user data for [NEEDS CLARIFICATION: retention period not specified]

### Key Entities *(include if feature involves data)*

- **LevelDefinition (matrix)**: A 2D matrix of integers that map to in-game tile/brick types. Key attributes: coordinates, tile index values.
- **BrickType**: Logical classification for a tile index (e.g., simple destructible, multi-hit, indestructible). Key attributes: index, durability (if applicable), contribution-to-completion (boolean).
- **LevelCompletionCounter**: Runtime counter or logic that tracks how many destructible bricks remain in the level and drives the level-complete condition.

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: For levels containing a mix of destructible and indestructible bricks, 100% of tests show that level completion triggers when all destructible bricks are destroyed while indestructible bricks remain.
- **SC-002**: Unit tests exist and pass that verify: parsing an index `90` yields an indestructible brick; parsing index `20` yields a simple destructible brick; parsing index `3` follows the project migration/policy defined in the Clarifications section.
- **SC-003**: Integration tests show that playing a level with indestructible bricks never causes an unintended failure to complete the level (0 occurrences in the test run).
- **SC-004**: The LevelDefinition format documentation is updated to include the new index mapping and the change to the simple-brick index `20` before the feature is marked complete.

## Clarifications (required)

1. **Migration policy for existing levels that use index `3`**: The project will perform an automatic migration of repository level assets (files under `assets/levels/`) converting any tile index `3` to `20` during the feature landing. Runtime support for legacy index `3` MAY be added only for external/third-party levels (see implications). The migration approach is intended to keep existing packaged levels working after the index remap without requiring manual edits by designers.

Migration details and acceptance criteria:

- The repo-level automated migration script MUST update any `.ron` or level asset files under `assets/levels` that contain the tile value `3` to `20`, preserving formatting where possible and creating a backup copy (e.g., `level_X.ron.bak`) before modification.
- The parser MUST continue to accept index `3` for the duration of an immediate compatibility window (if implemented) but the recommended canonical mapping will be `20` going forward. The migration script and README MUST document the choice and give designers guidance.
- Acceptance: After running the migration script on repo assets, no files under `assets/levels/` should contain the standalone numeric tile `3` where it previously represented a simple brick; unit tests and a regression test run must validate that updated files behave identically to pre-migration behaviour.

We implemented the repository-level migration approach for existing assets: the migration script will convert index `3` → `20` in repository-owned level files and write backups (e.g., `level_###.ron.bak`). External levels (user-supplied or third-party assets) will not be modified by default and may be handled separately with explicit migration steps.
