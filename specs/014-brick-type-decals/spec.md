
# Feature Specification: Brick Type Decals

**Feature Branch**: `014-brick-type-decals` **Created**: 2025-12-23 **Status**: Draft **Input**: User description: "For an easy recognition of the brick type, a visual hint should be displayed, centred on the top side of the brick.
The decals should support a normal map/bump map so that they can be embossed or engraved.
The bevy example experiments/bevy-examples/parallax_mapping.rs for User description: "For an easy recognition of the brick type, a visual hint should be displayed, centred on the top side of the brick.
See `<attachments>` above for file contents.
You may not need to search or read the file again.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Recognize Brick Type Visually (Priority: P1)

As a player, I want to easily recognize the type of each brick by a visual hint displayed on the brick, so I can plan my actions accordingly.

**Why this priority**: Immediate recognition of brick types is essential for gameplay strategy and user satisfaction.

**Independent Test**: Can be fully tested by displaying all brick types in a test level and verifying that each has a distinct, visible decal centered on the top side.

**Acceptance Scenarios**:

1. **Given** a level with multiple brick types, **When** the level is loaded, **Then** each brick displays a visible, type-specific decal centered on its top side.
2. **Given** a player viewing a brick from any angle, **When** the player looks at the top of the brick, **Then** the decal remains clearly visible and not obscured by other elements.

---

### User Story 2 - Decal Embossing/Engraving (Priority: P2)

As a player, I want the brick decals to appear embossed or engraved, so that the visual hint is more tactile and visually appealing.

**Why this priority**: Enhanced visual feedback increases immersion and makes brick types more memorable.

**Independent Test**: Can be tested by inspecting bricks in a test level and confirming that decals have a 3D embossed or engraved appearance using normal/bump mapping.

**Acceptance Scenarios**:

1. **Given** a brick with a decal, **When** lighting conditions change, **Then** the decal's embossed or engraved effect is visible due to normal/bump mapping.
2. **Given** a brick with a decal, **When** the player moves around the brick, **Then** the 3D effect of the decal remains consistent from different viewing angles.

---

### Edge Cases

- What happens if a brick type does not have an assigned decal? (Should display a default or blank decal)
- How does the system handle missing or corrupt normal/bump map assets? (Should fall back to a flat decal or log a warning)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST display a visual decal centered on the top side of every brick, indicating its type.
- **FR-002**: System MUST support normal or bump mapping for decals to allow embossed or engraved effects.
- **FR-003**: System MUST ensure decals are clearly visible and not obscured by other game elements or UI overlays.
- **FR-004**: System MUST provide a fallback for bricks without assigned decals or missing normal maps.
- **FR-005**: System MUST allow for easy addition of new brick types and corresponding decals.

### Key Entities

- **Brick**: Represents a destructible or indestructible object in the game, with a type attribute and a visual representation.
- **Decal**: A visual marker associated with a brick type, supporting normal/bump mapping for 3D effects.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of brick types in the game have a visible, type-specific decal centered on the top side in all test levels.
- **SC-002**: 95% of user testers can correctly identify brick types by their decals without external reference.
- **SC-003**: Decals with normal/bump mapping are rendered with a clear 3D effect under all standard lighting conditions.
- **SC-004**: No more than 1% of play sessions report missing or incorrect decals due to asset or mapping errors.

## Assumptions

- All brick types are known at design time, but the system should allow for future extensibility.
- Decal assets and normal/bump maps will be provided in compatible formats.
- The rendering engine supports normal/bump mapping as demonstrated in the referenced Bevy example.
