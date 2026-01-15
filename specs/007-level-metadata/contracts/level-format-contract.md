# API Contracts: Level Metadata

**Feature**: 007-level-metadata  
**Date**: 2025-12-06

## Overview

This feature does not introduce REST APIs or network interfaces.
Instead, it defines **data contracts** for the RON file format and Rust public API.

## Data Contract 1: LevelDefinition RON Format

**Contract Type**: File format schema  
**Location**: `assets/levels/*.ron`  
**Version**: 1.0 (extends existing format)

### Schema

```ron
LevelDefinition(
  number: u32,                           // Required: Level identifier
  gravity: Option<(f32, f32, f32)>,      // Optional: Gravity override (x, y, z)
  description: Option<String>,            // Optional: NEW - Design documentation
  author: Option<String>,                 // Optional: NEW - Contributor attribution
  matrix: Vec<Vec<u8>>,                   // Required: 20x20 tile grid
  presentation: Option<LevelTextureSet>,  // Optional: Texture config (feature-gated)
)
```

### Field Specifications

#### `description: Option<String>` (NEW)

**Type**: Optional string  
**Format**: Free-form text, supports multi-line with RON raw strings  
**Purpose**: Document level design intent, unique features, gameplay characteristics  
**Constraints**: None enforced (Git/PR review prevents abuse)  
**Default**: `None` (when omitted)

**Examples**:

```ron
// Single line
description: Some("Beginner-friendly tutorial level")

// Multi-line with raw string
description: Some(r#"
  Expert challenge level featuring:
  - Narrow corridors
  - Indestructible maze (type 90)
  - Multi-hit targets (types 10-13)
"#)

// Omitted (backward compatible)
// No description field needed
```

#### `author: Option<String>` (NEW)

**Type**: Optional string  
**Format**: Plain text name OR Markdown link `[Name](url)`  
**Purpose**: Contributor attribution and contact information  
**Constraints**: None enforced  
**Default**: `None` (when omitted)

**Examples**:

```ron
// Plain text
author: Some("Jane Smith")

// With email (markdown link)
author: Some("[Jane Smith](mailto:jane@example.com)")

// With URL
author: Some("[Community Team](https://example.com/team)")

// Omitted (backward compatible)
// No author field needed
```

### Backward Compatibility

**Guarantee**: All existing level files without `description` or `author` fields deserialize successfully.

**Mechanism**: `#[serde(default)]` attribute causes omitted fields to default to `None`.

**Breaking Changes**: None.
This is a purely additive change.

## API Contract 2: LevelDefinition Rust API

**Contract Type**: Public Rust struct and helper functions  
**Location**: `src/level_loader.rs`  
**Version**: 1.0 (extends existing API)

### Public Struct

```rust
#[derive(Deserialize, Debug, Clone)]
pub struct LevelDefinition {
    pub number: u32,
    pub gravity: Option<(f32, f32, f32)>,
    pub matrix: Vec<Vec<u8>>,

    // NEW FIELDS
    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub author: Option<String>,

    #[cfg(feature = "texture_manifest")]
    #[serde(default)]
    pub presentation: Option<LevelTextureSet>,
}
```

**Stability**: Public API - changes must maintain backward compatibility

### Public Helper Functions

#### extract_author_name

```rust
/// Extract display name from author field
///
/// Handles both plain text and markdown link formats:
/// - "Jane Smith" → "Jane Smith"
/// - "[Jane Smith](mailto:jane@example.com)" → "Jane Smith"
///
/// # Arguments
/// * `author` - Author string in plain or markdown format
///
/// # Returns
/// Display name as string slice (never fails)
pub fn extract_author_name(author: &str) -> &str
```

**Guarantees**:

- Always returns valid string slice
- Never panics
- Original string returned if not valid Markdown link

**Contract**: This is a pure function (no side effects, deterministic)

#### LevelDefinition::has_description

```rust
impl LevelDefinition {
    /// Check if level has a non-empty description
    ///
    /// Returns true if `description` field is `Some(non_empty_string)`
    pub fn has_description(&self) -> bool
}
```

**Guarantees**:

- Returns `false` if field is `None`
- Returns `false` if field is `Some("")` or `Some("   ")`
- Returns `true` if field has non-whitespace content

#### LevelDefinition::has_author

```rust
impl LevelDefinition {
    /// Check if level has a non-empty author
    ///
    /// Returns true if `author` field is `Some(non_empty_string)`
    pub fn has_author(&self) -> bool
}
```

**Guarantees**: Same as `has_description`

#### LevelDefinition::author_name

```rust
impl LevelDefinition {
    /// Get author display name (extract from markdown if needed)
    ///
    /// Returns `None` if author field is empty/absent.
    /// Automatically extracts name from markdown links.
    pub fn author_name(&self) -> Option<&str>
}
```

**Guarantees**:

- Returns `None` if author field is `None` or empty
- Returns extracted name if Markdown link format
- Returns plain text if not Markdown format

## Contract 3: Documentation Format

**Contract Type**: Documentation standard  
**Location**: `assets/levels/README.md`  
**Version**: 1.0 (extends existing documentation)

### Required Documentation Sections

1. **Field Descriptions**: Explain purpose of `description` and `author`
2. **Format Examples**: Show plain text, Markdown, multi-line usage
3. **Use Cases**: When to add metadata, what makes good descriptions
4. **Migration Guide**: How to add fields to existing levels

### Example Level Template

```ron
LevelDefinition(
  number: 1,
  gravity: Some((0.0, -9.81, 0.0)),  // Optional
  description: Some(r#"
    [Explain what makes this level unique]
    [Note any difficulty considerations]
    [Document design intent]
  "#),
  author: Some("[Your Name](mailto:your@email.com)"),
  matrix: [
    // 20 rows of 20 values each
  ],
)
```

## Deprecation Policy

**None**: This is a new feature with no deprecated elements.

**Future Considerations**:

- If internal representation changes, maintain RON format compatibility
- Consider versioned level format if breaking changes needed in future

## Versioning

**Level Format Version**: 1.0 (implicit - no version field yet)  
**Rust API Version**: Follows crate semver (currently 0.0.1)

**Compatibility Promise**:

- Minor version bumps: Additive changes only (new optional fields)
- Major version bumps: Breaking changes (required fields, format changes)

## Testing Contracts

**Contract**: All tests in `tests/level_definition.rs` verify these contracts

**Test Coverage**:

- ✅ Backward compatibility (old files load)
- ✅ New field deserialization
- ✅ Empty string handling
- ✅ Markdown extraction
- ✅ Helper function behavior

## Summary

**No network APIs** - this is a data format extension.

**Two main contracts**:

1. **RON file format**: Additive, backward-compatible schema
2. **Rust public API**: New struct fields and helper functions

**Stability guarantees**:

- Existing levels load unchanged
- New helper functions are pure (no side effects)
- Empty/missing fields handled gracefully
