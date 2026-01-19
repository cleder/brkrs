# Quickstart: Level Metadata Implementation

**Feature**: 007-level-metadata  
**Estimated Time**: 2-3 hours  
**Difficulty**: Easy

## Prerequisites

- Rust 1.81+ installed
- Bevy 0.17 project cloned
- Familiarity with RON format and serde

## Overview

Add optional `description` and `author` fields to `LevelDefinition` for level documentation and contributor attribution.

## Implementation Steps

### Step 1: Update LevelDefinition Struct (10 min)

**File**: `src/level_loader.rs`

**Action**: Add two optional fields to the struct

```rust
#[derive(Deserialize, Debug, Clone)]
pub struct LevelDefinition {
    pub number: u32,
    pub gravity: Option<(f32, f32, f32)>,
    pub matrix: Vec<Vec<u8>>,

    // ADD THESE LINES:
    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub author: Option<String>,

    #[cfg(feature = "texture_manifest")]
    #[serde(default)]
    pub presentation: Option<LevelTextureSet>,
}
```

**Verification**:

```bash
cargo check
```

### Step 2: Add Helper Functions (20 min)

**File**: `src/level_loader.rs`

**Action**: Add author name extraction and convenience methods

```rust
/// Extract display name from author field (handles markdown links)
///
/// # Examples
/// ```
/// assert_eq!(extract_author_name("Jane"), "Jane");
/// assert_eq!(extract_author_name("[Jane](mailto:jane@example.com)"), "Jane");
/// ```
pub fn extract_author_name(author: &str) -> &str {
    let trimmed = author.trim();
    if trimmed.starts_with('[') {
        if let Some(end_bracket) = trimmed.find("](") {
            return trimmed[1..end_bracket].trim();
        }
    }
    trimmed
}

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

**Verification**:

```bash
cargo check
cargo doc --no-deps --open
# Verify LevelDefinition documentation appears correctly
```

### Step 3: Add Unit Tests (30 min)

**File**: `tests/level_definition.rs`

**Action**: Create comprehensive test file

```rust
use brkrs::{extract_author_name, LevelDefinition};
use ron::de::from_str;

#[test]
fn test_level_without_metadata_backward_compat() {
    let ron = r#"
        LevelDefinition(
            number: 1,
            matrix: [[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]],
        )
    "#;
    let level: LevelDefinition = from_str(ron).expect("Should deserialize");
    assert_eq!(level.number, 1);
    assert_eq!(level.description, None);
    assert_eq!(level.author, None);
}

#[test]
fn test_level_with_description_only() {
    let ron = r#"
        LevelDefinition(
            number: 2,
            description: Some("Test level"),
            matrix: [[0]],
        )
    "#;
    let level: LevelDefinition = from_str(ron).expect("Should deserialize");
    assert!(level.has_description());
    assert!(!level.has_author());
}

#[test]
fn test_level_with_author_only() {
    let ron = r#"
        LevelDefinition(
            number: 3,
            author: Some("Jane Smith"),
            matrix: [[0]],
        )
    "#;
    let level: LevelDefinition = from_str(ron).expect("Should deserialize");
    assert!(!level.has_description());
    assert!(level.has_author());
    assert_eq!(level.author_name(), Some("Jane Smith"));
}

#[test]
fn test_level_with_full_metadata() {
    let ron = r#"
        LevelDefinition(
            number: 4,
            description: Some("Expert challenge"),
            author: Some("[Jane Smith](mailto:jane@example.com)"),
            matrix: [[0]],
        )
    "#;
    let level: LevelDefinition = from_str(ron).expect("Should deserialize");
    assert!(level.has_description());
    assert!(level.has_author());
    assert_eq!(level.author_name(), Some("Jane Smith"));
}

#[test]
fn test_multiline_description() {
    let ron = r#"
        LevelDefinition(
            number: 5,
            description: Some(r#"
                Line 1
                Line 2
                Line 3
            "#),
            matrix: [[0]],
        )
    "#;
    let level: LevelDefinition = from_str(ron).expect("Should deserialize");
    assert!(level.has_description());
    let desc = level.description.as_ref().unwrap();
    assert!(desc.contains("Line 1"));
    assert!(desc.contains("Line 2"));
}

#[test]
fn test_empty_string_treated_as_none() {
    let ron = r#"
        LevelDefinition(
            number: 6,
            description: Some(""),
            author: Some("   "),
            matrix: [[0]],
        )
    "#;
    let level: LevelDefinition = from_str(ron).expect("Should deserialize");
    assert!(!level.has_description());
    assert!(!level.has_author());
}

#[test]
fn test_extract_author_plain_text() {
    assert_eq!(extract_author_name("Jane Smith"), "Jane Smith");
    assert_eq!(extract_author_name("  Jane  "), "Jane");
}

#[test]
fn test_extract_author_markdown_email() {
    assert_eq!(
        extract_author_name("[Jane Smith](mailto:jane@example.com)"),
        "Jane Smith"
    );
}

#[test]
fn test_extract_author_markdown_url() {
    assert_eq!(
        extract_author_name("[Team](https://example.com)"),
        "Team"
    );
}

#[test]
fn test_extract_author_edge_cases() {
    // Empty brackets
    assert_eq!(extract_author_name("[](url)"), "");

    // Nested brackets
    assert_eq!(extract_author_name("[[Name]](url)"), "[Name]");

    // Malformed - no closing bracket
    assert_eq!(extract_author_name("[Name"), "[Name");

    // Not markdown
    assert_eq!(extract_author_name("Just text"), "Just text");
}
```

**Verification**:

```bash
cargo test level_definition
# All tests should pass
```

### Step 4: Update Documentation (30 min)

**File**: `assets/levels/README.md`

**Action**: Add new section explaining metadata fields

Add after the "Fields" section:

````markdown
## Metadata fields (optional)

Level files support optional documentation fields for designer notes and attribution.

### Description field

Document what makes your level unique, challenging, or interesting:

```ron
LevelDefinition(
  number: 1,
  description: Some("Beginner-friendly introduction with wide corridors"),
  matrix: [ /* ... */ ],
)
```

For multi-line descriptions, use RON raw strings:

```ron
description: Some(r#"
  "The Gauntlet" - Expert challenge

  Features:
  - Narrow passages requiring precision
  - Indestructible outer walls (type 90)
  - Multi-hit targets (types 10-13)

  Average completion time: 5-7 minutes
"#)
```

### Author field

Take credit for your work!
Use your name or include contact information:

```ron
// Plain name
author: Some("Jane Smith")

// With email (markdown link format)
author: Some("[Jane Smith](mailto:jane@example.com)")

// With URL
author: Some("[Community Team](https://github.com/example/team)")
```

**Note**: When using Markdown link format `[Name](url)`, only the name is extracted and used (the URL is preserved in the file for documentation purposes).

### When to add metadata

- **Description**: Add when the level has specific design intent, unique features, or difficulty considerations
- **Author**: Add to any level you create or significantly modify
- **Both optional**: Existing levels work fine without these fields

### Example with full metadata

```ron
LevelDefinition(
  number: 42,
  gravity: Some((0.0, -12.0, 0.0)),  // Higher gravity for challenge
  description: Some(r#"
    Speed challenge: Complete as fast as possible
    Recommended for experienced players
  "#),
  author: Some("[Jane Smith](mailto:jane@example.com)"),
  matrix: [
    // ... 20x20 grid
  ],
)
```
````

**Verification**:

```bash
# Open README.md in markdown preview
# Verify examples render correctly
```

### Step 6: Add Example Level (15 min)

**File**: `assets/levels/level_999.ron` (or create new test level)

**Action**: Add metadata to an example level

```ron
LevelDefinition(
  number: 999,
  description: Some(r#"
    Example level demonstrating metadata fields.

    This level shows how to document design intent
    and provide contributor attribution.
  "#),
  author: Some("[Brkrs Contributors](https://github.com/cleder/brkrs)"),
  matrix: [
    [90,90,90,90,90,90,90,90,90,90,90,90,90,90,90,90,90,90,90,90],
    [90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,90],
    [90, 0,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20, 0,90],
    [90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,90],
    // ... 16 more rows
  ],
)
```

**Verification**:

```bash
BK_LEVEL=999 cargo run --release
# Game should load and display the level
```

### Step 7: Integration Testing (20 min)

**Action**: Verify backward compatibility with existing levels

```bash
# Run all tests
cargo test

# Run game with various levels
BK_LEVEL=1 cargo run --release   # Old level without metadata
BK_LEVEL=999 cargo run --release # New level with metadata
```

### Step 8: Update User-Facing Documentation (30 min)

**Files**: `docs/asset-format.md`, `docs/developer-guide.md`

**Action**: Update user-facing documentation with level metadata information

#### Update docs/asset-format.md

Add a new section about level metadata fields:

```markdown
## Level Metadata Fields

Level files support optional metadata for documentation and attribution:

### Description Field

Document your level's design intent, unique features, and gameplay characteristics:

\ `\`\ `ron LevelDefinition(   number: 42,   description: Some("Classic corridor layout testing precision"),   matrix: [ /* ... */ ], ) \`\ `\`

For multi-line descriptions, use RON raw strings:

\ `\`\`ron description: Some(r#" "The Gauntlet" - Expert level challenge

  Design Intent:

- Tests player precision with narrow passages
- Features indestructible maze (type 90)
- Includes high-value multi-hit targets

  Average completion: 5-7 minutes
"#) \ `\`\`

### Author Field

Take credit for your work with contributor attribution:

\ `\`\`ron // Simple name author: Some("Jane Smith")

// With contact information (Markdown link) author: Some(" [Jane Smith](mailto:jane@example.com)") author: Some(" [Team Name](https://github.com/org/repo)") \ `\`\`

**Note**: When using Markdown format `[Name](url)`, the system extracts and uses only the display name.

### When to Use Metadata

- **Description**: Add when the level has specific design goals, unique mechanics, or difficulty notes
- **Author**: Add to levels you create or significantly modify
- **Both optional**: Existing levels work perfectly without these fields
```

#### Update docs/developer-guide.md

Add a section about creating levels with metadata:

```markdown
## Creating Levels with Metadata

When creating or editing level files, consider adding metadata to document your work:

### Example: Complete Level File

\ `\`\`ron LevelDefinition( number: 100, gravity: Some((0.0, -9.81, 0.0)), description: Some(r#" Tournament-grade expert challenge.

    Features:
    - Precision-required narrow corridors
    - Strategic use of indestructible walls
    - Multi-hit target variety (types 10-13)

    Designed for competitive play.
    Estimated completion: 5-7 minutes.
"#), author: Some(" [Your Name](mailto:your@email.com)"), matrix: [ // ... 20x20 tile grid ], ) \ `\`\`

### Tips for Good Descriptions

- Explain the **design intent**, not implementation details
- Note any unique features or mechanics
- Include difficulty considerations
- Mention estimated completion time for complex levels
- Document any special brick patterns or strategies

### Attribution Best Practices

- Use your real name or handle consistently
- Include contact info (email/GitHub) if you want feedback
- Credit collaborators in multi-line descriptions
- Update author field when significantly modifying a level
```

**Verification**:

```bash
# Build documentation to verify rendering
cd docs
make html
# Or just preview markdown files in your editor
```

### Step 9: Rustdoc Documentation (15 min)

**Action**: Add rustdoc to new public functions

Already included in Step 2, verify it renders correctly:

```bash
cargo doc --no-deps --open
# Navigate to brkrs::level_loader::extract_author_name
# Navigate to brkrs::level_loader::LevelDefinition
# Verify documentation is clear and includes examples
```

## Verification Checklist

- [ ] `cargo check` passes
- [ ] `cargo test` passes (all tests)
- [ ] `cargo clippy` has no warnings
- [ ] `cargo fmt` applied
- [ ] `bevy lint` passes
- [ ] `cargo doc` generates clean documentation
- [ ] Existing levels load correctly (backward compatibility)
- [ ] New level with metadata loads correctly
- [ ] Migration tool preserves metadata fields
- [ ] `assets/levels/README.md` has clear examples
- [ ] `docs/asset-format.md` updated with metadata documentation
- [ ] `docs/developer-guide.md` includes level creation examples
- [ ] Documentation builds without errors (`cd docs && make html`)

## Common Issues

**Issue**: "field `description` not found in LevelDefinition"

- **Solution**: Ensure `#[serde(default)]` attribute is present

**Issue**: Tests fail with deserialization errors

- **Solution**: Check RON syntax (commas, brackets, field names)

**Issue**: Markdown parsing doesn't extract name

- **Solution**: Verify helper function implementation and test coverage

## Next Steps

After implementation:

1. Run full test suite: `cargo test`
2. Test in native: `cargo run --release`
3. Test in WASM: `trunk serve` (if WASM target configured)
4. Create PR with changes
5. Update CHANGELOG.md with new feature

## Estimated Timeline

- Step 1: 10 min (struct update)
- Step 2: 20 min (helper functions)
- Step 3: 30 min (unit tests)
- Step 4: 15 min (migration tool)
- Step 5: 30 min (assets/levels/README.md documentation)
- Step 6: 15 min (example level)
- Step 7: 20 min (integration testing)
- Step 8: 30 min (user-facing docs in docs/)
- Step 9: 15 min (rustdoc)

**Total**: ~3 hours for complete implementation and testing

## References

- Feature Spec: `specs/007-level-metadata/spec.md`
- Research Notes: `specs/007-level-metadata/research.md`
- Data Model: `specs/007-level-metadata/data-model.md`
- RON Documentation: <https://github.com/ron-rs/ron>
- Serde Documentation: <https://serde.rs/>
