# Feature Specification: Level Metadata (Description and Author)

**Feature Branch**: `007-level-metadata`  
**Created**: 2025-12-06  
**Status**: Draft  
**Input**: User description: "I want to include a description in the Level Files to encourage designers to document what is unique, engaging, etc.
A description of the file format is in `assets/levels/README.md` The description is for documentation purposes only and does not need to be displayed in the game itself.
Add an author to the level file, so contributors can take credit for their work.
The author can be a plain string (name) or a Markdown link `[Your Name](mailto:your@email.com)`, in that case only the name should be extracted."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Level Designer Documents Level Intent (Priority: P1)

A level designer creates or edits a level file and adds a description explaining what makes this level unique, challenging, or interesting.
This documentation helps other designers, contributors, and maintainers understand the design intent without needing to analyze the tile matrix.

**Why this priority**: Core value proposition - enables designers to communicate design intent, which is the primary goal of adding metadata fields.

**Independent Test**: Can be fully tested by creating a level file with a description field, loading it successfully without runtime errors, and verifying the description persists when the file is read back from disk.

**Acceptance Scenarios**:

1. **Given** a level designer is creating a new level file, **When** they add a `description` field with text explaining the level's unique features, **Then** the file parses correctly and the description is preserved
2. **Given** an existing level file without a description, **When** the designer adds a description field, **Then** the level continues to load and function normally in the game
3. **Given** a level file with a multi-line description, **When** the file is parsed, **Then** the description text preserves line breaks and formatting

---

### User Story 2 - Contributor Takes Credit for Work (Priority: P1)

A level designer or contributor adds their name and optionally contact information to a level file using the author field.
This attribution allows them to receive recognition for their creative work and enables others to contact them with questions or feedback.

**Why this priority**: Essential for contributor recognition and motivation - equally important to description field for establishing ownership and encouraging contributions.

**Independent Test**: Can be fully tested by adding an author field (plain string or Markdown link format) to a level file, verifying it parses correctly, and confirming the extracted name is available for display or documentation purposes.

**Acceptance Scenarios**:

1. **Given** a level designer creates a new level, **When** they add an `author` field with their name as a plain string (e.g., "Jane Smith"), **Then** the file parses correctly and the author name is preserved
2. **Given** a contributor wants to provide contact information, **When** they add an author field with Markdown link format `[Jane Smith](mailto:jane@example.com)`, **Then** the system extracts "Jane Smith" as the author name
3. **Given** an existing level file without an author field, **When** the file is loaded, **Then** the system treats author as optional and loads the level successfully

---

### User Story 3 - Maintainer Reviews Level Documentation (Priority: P2)

A project maintainer or reviewer examines level files in the repository and reads the description and author fields to understand design decisions, assess quality, and identify who to contact with questions.

**Why this priority**: Secondary use case that benefits from P1 functionality - adds value but isn't required for basic feature operation.

**Independent Test**: Can be tested by opening level files in a text editor and confirming that description and author fields are human-readable and provide useful context.

**Acceptance Scenarios**:

1. **Given** a maintainer reviews a pull request with new level files, **When** they open the RON files, **Then** they can read the description and author fields without special tooling
2. **Given** multiple level files in the repository, **When** a maintainer wants to find levels by a specific author, **Then** they can search for the author name in level files

---

### User Story 4 - Migration of Existing Levels (Priority: P3)

Existing level files without description or author fields continue to work without modification.
Designers can optionally add these fields to existing levels over time without breaking changes.

**Why this priority**: Backward compatibility requirement - important for adoption but doesn't provide new functionality.

**Independent Test**: Can be tested by loading existing level files (without new metadata fields) and verifying they work identically to before the feature was added.

**Acceptance Scenarios**:

1. **Given** an existing level file without `description` or `author` fields, **When** the system loads the level, **Then** it functions normally and uses default/empty values for missing fields
2. **Given** a level file with only a `description` field (no author), **When** loaded, **Then** the system accepts the partial metadata
3. **Given** a level file with only an `author` field (no description), **When** loaded, **Then** the system accepts the partial metadata

---

### Edge Cases

- What happens when the description field contains special characters (quotes, newlines, RON syntax characters)?
- How does the system handle an author field with malformed Markdown link syntax?
- What happens when description is an empty string versus field being omitted entirely?
- How does the system handle extremely long descriptions (e.g., 1000+ characters)?
- What happens when author field contains multiple Markdown links or nested brackets?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Level files MUST support an optional `description` field that accepts multi-line text explaining the level's unique features, design intent, or gameplay characteristics
- **FR-002**: Level files MUST support an optional `author` field that accepts either a plain string name or Markdown link format `[Name](mailto:email)` or `[Name](url)`
- **FR-003**: The system MUST extract only the name portion from Markdown-formatted author fields (e.g., `[Jane Smith](mailto:jane@example.com)` extracts "Jane Smith")
- **FR-004**: The system MUST treat both `description` and `author` fields as optional - level files without these fields must load successfully
- **FR-005**: The system MUST preserve description text formatting including line breaks, spaces, and common punctuation when parsing RON files
- **FR-006**: Existing level files without metadata fields MUST continue to load and function identically to current behavior (backward compatibility)
- **FR-007**: The description field MUST be for documentation purposes only and does not need to be displayed during gameplay
- **FR-008**: The system MUST handle empty string values for description and author fields the same as omitted fields
- **FR-009**: Level file documentation MUST be updated to include examples and guidance for using description and author fields

### Key Entities

- **LevelDefinition**: The RON structure representing a game level, enhanced with optional `description: Option<String>` and `author: Option<String>` fields
  - Core attributes: number (u32), gravity (optional vector), matrix (tile grid)
  - New attributes: description (optional string for design documentation), author (optional string for contributor attribution)
  - Relationships: Loaded by level loader system, referenced by level advancement logic

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of existing level files load successfully without modification after feature implementation
- **SC-002**: Level designers can add description and author fields to new or existing levels in under 30 seconds
- **SC-003**: All Markdown-formatted author fields correctly extract the name portion without the email/URL
- **SC-004**: Level files with metadata fields parse and load with the same performance characteristics as files without metadata (no measurable degradation)
- **SC-005**: The repository README or level documentation includes clear examples of using description and author fields within 1 day of feature completion
