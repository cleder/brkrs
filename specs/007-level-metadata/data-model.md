# Data Model: Level Metadata

**Feature**: 007-level-metadata  
**Date**: 2025-12-06  
**Purpose**: Define data structures and validation rules for level metadata fields

## Core Entities

### LevelDefinition (Enhanced)

**Purpose**: RON-serializable structure representing a game level with optional metadata

**Location**: `src/level_loader.rs`

**Fields**:

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `number` | `u32` | Yes | N/A | Level index/identifier |
| `gravity` | `Option<(f32, f32, f32)>` | No | `None` | Optional gravity override (x, y, z) |
| `matrix` | `Vec<Vec<u8>>` | Yes | N/A | Tile grid (20x20) |
| `description` | `Option<String>` | No | `None` | **NEW**: Level design documentation |
| `author` | `Option<String>` | No | `None` | **NEW**: Contributor attribution |
| `presentation` | `Option<LevelTextureSet>` | No | `None` | Texture configuration (feature-gated) |

**New Field Details**:

- **`description`**:
  - Purpose: Document what makes the level unique, challenging, or engaging
  - Format: Free-form text, supports multi-line with RON raw strings `r#"..."#`
  - Constraints: None enforced (Git/PR review prevents abuse)
  - Use cases: Design notes, difficulty indicators, gameplay hints

- **`author`**:
  - Purpose: Contributor attribution and contact information
  - Format: Plain string ("Name") or markdown link ("[Name](url)")
  - Constraints: None enforced (validation is linter/editor concern)
  - Use cases: Credit contributors, enable feedback channels

**Validation Rules**:

1. **Structural**:
   - Both new fields use `#[serde(default)]` for backward compatibility
   - Empty strings treated as equivalent to `None` in helper methods
   - RON parser handles escaping for special characters automatically

2. **Semantic** (not enforced at runtime):
   - Description should explain design intent, not implementation
   - Author should be single line (though multi-line not prevented)
   - Markdown links should follow `[Text](url)` format

3. **Backward Compatibility**:
   - Existing level files without metadata fields deserialize successfully
   - New fields default to `None` when omitted
   - No breaking changes to existing deserialization logic

## State Transitions

**Not Applicable**: Metadata fields are read-only documentation, no runtime state transitions.

Level files are loaded once per level start/switch.
Metadata is not modified during gameplay.

## Helper Functions

### extract_author_name

**Purpose**: Extract display name from author field (handle both plain text and markdown links)

**Signature**:

```rust
pub fn extract_author_name(author: &str) -> &str
```

**Behavior**:

| Input | Output | Reason |
|-------|--------|--------|
| `"Jane Smith"` | `"Jane Smith"` | Plain text passthrough |
| `"[Jane Smith](mailto:jane@example.com)"` | `"Jane Smith"` | Extract from markdown link |
| `"[](url)"` | `""` | Empty brackets edge case |
| `"[[Name]](url)"` | `[Name]` | Nested brackets (acceptable) |
| `"Not a link"` | `"Not a link"` | Fallback to original |

**Algorithm**:

1. Trim whitespace
2. Check if starts with `[`
3. Find `](` position
4. Extract substring between `[` and `](`
5. If pattern not found, return original string

**Error Handling**: No errors - always returns valid string slice

### LevelDefinition helper methods

**Purpose**: Convenience methods for checking metadata presence

```rust
impl LevelDefinition {
    /// Returns true if description is present and non-empty
    pub fn has_description(&self) -> bool {
        self.description.as_ref().map_or(false, |s| !s.trim().is_empty())
    }

    /// Returns true if author is present and non-empty
    pub fn has_author(&self) -> bool {
        self.author.as_ref().map_or(false, |s| !s.trim().is_empty())
    }

    /// Get author display name (extract from markdown if needed)
    pub fn author_name(&self) -> Option<&str> {
        self.author.as_ref()
            .filter(|s| !s.trim().is_empty())
            .map(|s| extract_author_name(s))
    }
}
```

## Relationships

```text
LevelDefinition (RON file)
    ↓ deserialized by
CurrentLevel (Resource)
    ↓ consumed by
level loader systems (ECS)
    ↓ spawn entities
Game world (bricks, paddle, ball)
```

**Metadata Flow**:

1. Level file loaded from `assets/levels/level_NNN.ron`
2. Deserialized into `LevelDefinition` struct
3. Metadata fields available for logging, debugging, documentation
4. **Not used during gameplay** - documentation only

## File Format Examples

### Minimal Level (Backward Compatible)

```ron
LevelDefinition(
  number: 1,
  matrix: [
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    // ... 19 more rows
  ],
)
```

### Level with Description Only

```ron
LevelDefinition(
  number: 42,
  description: Some("Classic corridor layout testing precision"),
  matrix: [
    // ... matrix data
  ],
)
```

### Level with Full Metadata

```ron
LevelDefinition(
  number: 100,
  gravity: Some((0.0, -9.81, 0.0)),
  description: Some(r#"
    "The Gauntlet" - Expert level challenge

    Features:
    - Narrow passages requiring perfect angles
    - Indestructible brick maze (type 90)
    - High-value multi-hit targets (types 10-13)

    Design Notes:
    Originally designed for tournament play.
    Average completion time: 5-7 minutes.
  "#),
  author: Some("[Jane Smith](mailto:jane@example.com)"),
  matrix: [
    // ... matrix data
  ],
)
```

### Level with Plain Author

```ron
LevelDefinition(
  number: 50,
  description: Some("Beginner-friendly introduction level"),
  author: Some("Community Contributors"),
  matrix: [
    // ... matrix data
  ],
)
```

## Migration Considerations

**Existing Levels**: No changes required.
All existing `level_*.ron` files load unchanged with metadata defaulting to `None`.

**Migration Tool**: Removed - no longer needed.

**Adding Metadata**: Level designers can add metadata fields to existing levels incrementally via:

1. Manual editing (recommended for small number of files)
2. Scripted updates (for bulk metadata addition)

## Testing Strategy

**Unit Tests** (`tests/level_definition.rs`):

- ✅ Deserialize level with description only
- ✅ Deserialize level with author only
- ✅ Deserialize level with both fields
- ✅ Deserialize level without metadata (backward compat)
- ✅ Empty string handling
- ✅ Multi-line description with raw strings
- ✅ Author name extraction (plain text)
- ✅ Author name extraction (markdown link)
- ✅ Author name extraction (edge cases)

**Integration Tests**:

- ✅ Load existing levels without breaking
- ✅ Load new levels with metadata
- ✅ Migration tool preserves metadata

**Documentation Tests**:

- ✅ Verify `assets/levels/README.md` examples are valid RON
- ✅ Verify `docs/asset-format.md` builds correctly with Sphinx
- ✅ Verify `docs/developer-guide.md` examples compile

## Documentation Requirements

**Technical Documentation** (`assets/levels/README.md`):

- RON syntax examples with description and author fields
- Multi-line description examples using raw strings
- Markdown link format examples for author field
- When to use metadata (design guidelines)

**User-Facing Documentation** (`docs/asset-format.md`):

- High-level explanation of level metadata purpose
- Examples showing common use cases
- Best practices for writing descriptions
- Attribution guidelines

**Developer Documentation** (`docs/developer-guide.md`):

- Complete level creation workflow with metadata
- Tips for good descriptions
- Attribution best practices
- Integration with existing level editing workflow

**API Documentation** (rustdoc):

- `LevelDefinition` struct field documentation
- `extract_author_name()` function documentation
- Helper method documentation (`has_description()`, etc.)

## Open Questions

**None** - all design decisions resolved in research.md.

## Summary

Simple, backward-compatible extension to `LevelDefinition`:

- Two optional string fields (`description`, `author`)
- Zero runtime overhead when fields omitted
- No new dependencies
- Minimal code changes (~10 lines struct definition, ~20 lines helper functions)
- Comprehensive testing ensures no regressions
- Documentation updates in three locations (assets/levels/, docs/, rustdoc)
