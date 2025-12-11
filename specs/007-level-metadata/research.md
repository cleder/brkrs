# Research: Level Metadata (Description and Author)

**Feature**: 007-level-metadata  
**Date**: 2025-12-06  
**Purpose**: Document technical decisions for adding description and author fields to LevelDefinition

## Overview

This feature adds two optional metadata fields to the `LevelDefinition` struct for documentation purposes.
Research focuses on:

1. Serde/RON best practices for optional fields
2. Backward compatibility strategies
3. String parsing for Markdown link extraction

## Decision 1: Optional Field Implementation

**Context**: Need to add `description` and `author` fields while maintaining backward compatibility with existing level files.

**Decision**: Use `Option<String>` with `#[serde(default)]` attribute

**Rationale**:

- `Option<String>` naturally represents optional fields in Rust
- `#[serde(default)]` provides automatic backward compatibility - omitted fields deserialize as `None`
- No custom deserialization logic required
- Zero performance overhead when fields are `None`
- Standard Rust/serde pattern used throughout the project

**Alternatives Considered**:

- `String` with empty string default: Rejected because `Option` better expresses "not provided" vs "empty"
- Custom deserializer: Rejected as over-engineering - serde default behavior sufficient
- Separate `LevelMetadata` struct: Rejected as unnecessary indirection for 2 fields

**Implementation**:

```rust
#[derive(Deserialize, Debug, Clone)]
pub struct LevelDefinition {
    pub number: u32,
    pub gravity: Option<(f32, f32, f32)>,
    pub matrix: Vec<Vec<u8>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    // ... existing fields
}
```

**References**:

- Existing `gravity` field already uses `Option` pattern successfully
- Serde documentation: <https://serde.rs/attr-default.html>

## Decision 2: Author Name Extraction from Markdown Links

**Context**: Support both plain names ("Jane Smith") and Markdown links ("[Jane Smith](mailto:jane@example.com)") in author field.

**Decision**: Simple regex-based extraction with fallback to raw string

**Rationale**:

- Markdown link format is `[Display Text](url)` - simple to parse
- Edge cases (nested brackets, multiple links) should preserve raw string rather than fail
- Extraction logic isolated to helper function for testability
- No external Markdown parsing dependencies needed for this simple case

**Alternatives Considered**:

- Full Markdown parser (e.g., pulldown-cmark): Rejected as overkill for single link extraction
- Manual character-by-character parsing: Rejected as regex is clearer and sufficient
- Store both name and URL separately: Rejected as spec only requires name extraction

**Implementation**:

```rust
/// Extract author name from plain string or markdown link format
///
/// Supports:
/// - Plain text: "Jane Smith" -> "Jane Smith"
/// - Markdown link: "[Jane Smith](mailto:jane@example.com)" -> "Jane Smith"
/// - Malformed markdown: returns original string
pub fn extract_author_name(author: &str) -> &str {
    // Pattern: [text](url)
    if let Some(captures) = AUTHOR_REGEX.captures(author) {
        captures.get(1).map_or(author, |m| m.as_str())
    } else {
        author
    }
}

lazy_static! {
    static ref AUTHOR_REGEX: Regex = Regex::new(r"^\[([^\]]+)\]\([^\)]+\)$").unwrap();
}
```

**Edge Case Handling**:

- Empty brackets `[](url)`: Returns empty string (acceptable)
- Nested brackets `[[Name]](url)`: Returns `[Name]` (acceptable)
- Multiple links: Returns original string (fallback behavior)
- URL with closing paren: Returns original string (fallback behavior)

**Note**: Actually, for simplicity and zero dependencies, we can use string manipulation instead of regex:

```rust
pub fn extract_author_name(author: &str) -> &str {
    let trimmed = author.trim();
    if trimmed.starts_with('[') {
        if let Some(end_bracket) = trimmed.find("](") {
            return trimmed[1..end_bracket].trim();
        }
    }
    trimmed
}
```

**Final Decision**: Use string manipulation approach (no regex dependency needed)

**Testing Strategy**:

- Unit tests for plain strings, Markdown links, edge cases
- Integration tests with level files containing both formats

## Decision 3: Validation and Constraints

**Context**: Determine if description/author fields need length limits or content validation.

**Decision**: No validation enforced at deserialization time

**Rationale**:

- Fields are documentation-only, not displayed in UI
- Git repository is the natural constraint (large files flagged in PR review)
- RON parser already handles escaping for special characters
- Overly long descriptions are a code review issue, not a runtime error
- Validation would complicate backward compatibility testing

**Alternatives Considered**:

- Length limits (e.g., 1000 chars): Rejected as arbitrary and unnecessary
- Content validation (no special chars): Rejected as RON handles escaping
- Required field validation: Rejected as contradicts "optional" requirement

**Constraints**:

- Description can be multi-line (RON raw strings `r#"..."#` supported)
- Author should be single line (but not enforced - validation is editor/linter concern)
- Empty strings treated as `None` equivalent (implementation choice)

## Decision 4: Documentation Examples

**Context**: Need clear examples for level designers to understand how to use new fields.

**Decision**: Update `assets/levels/README.md` with comprehensive examples

**Example Level File**:

```ron
LevelDefinition(
  number: 42,
  gravity: Some((0.0, -9.81, 0.0)),
  description: Some(r#"
    Classic maze layout with narrow corridors.
    Tests player precision and patience.
    Indestructible bricks form the outer walls.
  "#),
  author: Some("[Jane Smith](mailto:jane@example.com)"),
  matrix: [
    // ... level data
  ],
)
```

**Documentation Sections to Add**:

1. Field descriptions (what each field is for)
2. Format examples (plain text, Markdown links, multi-line)
3. When to use description (design intent, difficulty notes)
4. Author format options (name only vs. contact info)
5. Migration guide (adding fields to existing levels)

## Decision 5: Migration Tool Impact

**Context**: Migration tool (`tools/migrate-level-indices`) was removed as no longer needed.

**Decision**: No migration tool updates required

**Rationale**:

- Migration tool was removed in a previous refactor
- No tool exists to update for new fields
- Metadata fields use `#[serde(default)]` so existing levels work unchanged
- Existing fields (number, gravity) are already preserved correctly

**Future Enhancement**: If migration tool needs to modify metadata fields, update its `LevelDefinition` struct to match main definition.

**Testing**:

- Run migration tool on level files with metadata fields
- Verify metadata preserved after migration

## Decision 6: Empty String Handling

**Context**: Decide how to handle empty strings vs. `None` for optional fields.

**Decision**: Treat empty strings as equivalent to `None` in helper functions

**Rationale**:

- User might write `description: Some("")` by mistake
- Better UX to treat empty as "not provided"
- Consistent with Rust conventions (empty collections often treated as "nothing")

**Implementation**:

```rust
impl LevelDefinition {
    pub fn has_description(&self) -> bool {
        self.description.as_ref().map_or(false, |s| !s.trim().is_empty())
    }

    pub fn has_author(&self) -> bool {
        self.author.as_ref().map_or(false, |s| !s.trim().is_empty())
    }
}
```

## Research Summary

All technical unknowns resolved:

1. ✅ Optional fields: Use `Option<String>` with `#[serde(default)]`
2. ✅ Markdown parsing: Simple string manipulation, no external dependencies
3. ✅ Validation: None required - documentation and code review sufficient
4. ✅ Examples: Comprehensive README.md updates planned
5. ✅ Migration: No changes needed initially
6. ✅ Empty strings: Treat as equivalent to `None`

**Zero new dependencies required** - uses existing serde/ron capabilities.

**Backward compatibility confirmed** - existing level files load unchanged with new fields defaulting to `None`.

**Performance impact**: Negligible - two optional string fields parsed once per level load.

Ready to proceed to Phase 1 (data model and contracts).
