# Implementation Tasks: Level Metadata (Description and Author)

**Feature**: 007-level-metadata  
**Created**: 2025-12-06  
**Branch**: `007-level-metadata`  
**Estimated Duration**: ~3 hours total

## Overview

Add optional `description` and `author` fields to `LevelDefinition` struct for level documentation and contributor attribution.
Four independent user stories with incremental delivery strategy.

**User Stories**:

- [US1] Level Designer Documents Level Intent (P1) - Core feature
- [US2] Contributor Takes Credit for Work (P1) - Equally core
- [US3] Maintainer Reviews Level Documentation (P2) - Secondary
- [US4] Migration of Existing Levels (P3) - Backward compatibility

**MVP Scope**: Complete US1 + US2 (both P1 stories) + basic tests

---

## Phase 1: Setup & Foundational Tasks

*(Complete before any user story implementation)*

### Project Structure & Preparation

- [x] T001 Review existing `LevelDefinition` struct in `src/level_loader.rs` and document current implementation
- [x] T002 Verify backward compatibility by listing all files that deserialize `LevelDefinition`
- [x] T003 Create test file `tests/level_definition.rs` with imports and module structure
- [x] T004 Create level example file `assets/levels/level_999.ron` as test case
- [x] T005 Review RON format in `assets/levels/README.md` for context on current structure

### Dependencies & Research Confirmation

- [x] T006 Confirm serde `#[serde(default)]` behavior with existing code patterns
- [x] T007 Validate Markdown link extraction logic from `research.md` is implementable with standard Rust string methods
- [x] T008 Verify WASM compatibility of proposed string operations (no platform-specific code)

---

## Phase 2: User Story 1 - Level Designer Documents Level Intent (P1)

*Independent Test: Create level file with description field, load successfully, verify persistence*

### Data Model Implementation

- [x] T009 [US1] Add `description: Option<String>` field to `LevelDefinition` struct in `src/level_loader.rs`
- [x] T010 [US1] Add `#[serde(default)]` attribute to description field for backward compatibility
- [x] T011 [US1] Add rustdoc comment explaining description field purpose in `src/level_loader.rs`

### Helper Functions

- [x] T012 [US1] Implement `LevelDefinition::has_description()` method in `src/level_loader.rs`
- [x] T013 [US1] Add rustdoc examples for `has_description()` method

### Unit Tests

- [x] T014 [P] [US1] Add test `test_level_with_description_only()` in `tests/level_definition.rs`
- [x] T015 [P] [US1] Add test `test_multiline_description()` in `tests/level_definition.rs`
- [x] T016 [P] [US1] Add test `test_empty_description_treated_as_none()` in `tests/level_definition.rs`
- [x] T017 [P] [US1] Add test `test_description_with_special_chars()` in `tests/level_definition.rs`

### Integration Tests

- [x] T018 [US1] Run `cargo test` and verify all description tests pass
- [x] T019 [US1] Load existing level files without description field to verify backward compatibility
- [x] T020 [US1] Create level_999.ron with description and verify it loads correctly

### Documentation - Technical Reference

- [x] T021 [US1] Update `assets/levels/README.md` with "Description field" section including:
  - Single-line example
  - Multi-line raw string example
  - When to use guidance
- [x] T022 [US1] Add description field to the "Field structure" documentation in `assets/levels/README.md`

### Documentation - User-Facing

- [x] T023 [US1] Update `docs/asset-format.md` with description field section including:
  - Purpose and use cases
  - RON syntax examples
  - Best practices for descriptions

### Verification

- [x] T024 [US1] Verify `cargo check` passes with new field
- [x] T025 [US1] Verify `cargo clippy` reports no warnings for new code
- [x] T026 [US1] Verify rustdoc builds correctly: `cargo doc --no-deps --open`

---

## Phase 3: User Story 2 - Contributor Takes Credit for Work (P1)

*Independent Test: Add author field (plain and Markdown), parse correctly, extract name properly*

**Note**: Can be implemented in parallel with US1 - independent changes

### Data Model Implementation

- [x] T027 [P] [US2] Add `author: Option<String>` field to `LevelDefinition` struct in `src/level_loader.rs`
- [x] T028 [P] [US2] Add `#[serde(default)]` attribute to author field
- [x] T029 [P] [US2] Add rustdoc comment explaining author field purpose in `src/level_loader.rs`

### Helper Functions - Author Name Extraction

- [x] T030 [US2] Implement `extract_author_name()` function in `src/level_loader.rs` using string manipulation (no regex)
- [x] T031 [US2] Add comprehensive rustdoc to `extract_author_name()` with examples:
  - Plain text: "Jane Smith" → "Jane Smith"
  - Markdown: "[Jane Smith](mailto:...)" → "Jane Smith"
  - Edge cases documented
- [x] T032 [US2] Implement `LevelDefinition::has_author()` method in `src/level_loader.rs`
- [x] T033 [US2] Implement `LevelDefinition::author_name()` method in `src/level_loader.rs`
- [x] T034 [US2] Add rustdoc examples for author helper methods

### Unit Tests

- [x] T035 [P] [US2] Add test `test_level_with_author_plain_string()` in `tests/level_definition.rs`
- [x] T036 [P] [US2] Add test `test_author_markdown_email_format()` in `tests/level_definition.rs`
- [x] T037 [P] [US2] Add test `test_author_markdown_url_format()` in `tests/level_definition.rs`
- [x] T038 [P] [US2] Add test `test_extract_author_plain_text()` in `tests/level_definition.rs`
- [x] T039 [P] [US2] Add test `test_extract_author_markdown_email()` in `tests/level_definition.rs`
- [x] T040 [P] [US2] Add test `test_extract_author_markdown_url()` in `tests/level_definition.rs`
- [x] T041 [P] [US2] Add test `test_extract_author_edge_cases()` in `tests/level_definition.rs`
- [x] T042 [P] [US2] Add test `test_empty_author_treated_as_none()` in `tests/level_definition.rs`

### Integration Tests

- [x] T043 [US2] Run `cargo test` and verify all author tests pass
- [x] T044 [US2] Create test level with plain author string and verify parsing
- [x] T045 [US2] Create test level with markdown author link and verify name extraction
- [x] T046 [US2] Verify backward compatibility - load levels without author field

### Documentation - Technical Reference

- [x] T047 [US2] Update `assets/levels/README.md` with "Author field" section including:
  - Plain text example
  - Markdown email example
  - Markdown URL example
  - When to use guidance
- [x] T048 [US2] Add author field to the "Field structure" documentation in `assets/levels/README.md`

### Documentation - User-Facing

- [x] T049 [US2] Update `docs/asset-format.md` with author field section including:
  - Purpose and attribution benefits
  - Plain text vs markdown format examples
  - Name extraction behavior documented
- [x] T050 [US2] Update `docs/developer-guide.md` with new section "Creating Levels with Metadata":
  - Complete level file example with both fields
  - Tips for good descriptions
  - Attribution best practices

### Verification

- [x] T051 [US2] Verify `cargo check` passes with author field and helper functions
- [x] T052 [US2] Verify `cargo clippy` reports no warnings for author code
- [x] T053 [US2] Verify rustdoc builds correctly and includes author documentation

---

## Phase 4: User Story 3 - Maintainer Reviews Level Documentation (P2)

*Independent Test: Open level files in text editor, read fields without special tooling, search functionality works*

**Note**: Depends on US1+US2 completion, can run in parallel with US4

### Documentation Quality

- [ ] T054 [US3] Update `assets/levels/README.md` with "Notes for designers" section:
  - When to document levels
  - Examples of good descriptions
  - How to add contact information
- [ ] T055 [US3] Ensure all example level files in README are syntactically correct RON
- [ ] T056 [US3] Add section to `docs/asset-format.md` about "Searching and Finding Levels"
- [ ] T057 [US3] Create guidelines document mentioning searchability of author field

### Example Levels

- [ ] T058 [US3] Update `level_001.ron` (if present) with description and author examples
- [ ] T059 [US3] Ensure `level_999.ron` has well-documented metadata for reference
- [ ] T060 [US3] Verify all example files render correctly in markdown viewers

### Testing

- [ ] T061 [US3] Run `cd docs && make html` to verify documentation builds without errors
- [ ] T062 [US3] Review generated HTML documentation for clarity and correct rendering
- [ ] T063 [US3] Verify README.md renders correctly with all code examples

---

## Phase 5: User Story 4 - Migration of Existing Levels (P3)

*Independent Test: Load all existing levels without modification, verify identical behavior*

**Note**: Can run in parallel after US1+US2 struct changes complete

### Backward Compatibility Tests

- [ ] T064 [P] [US4] Add test `test_level_without_metadata_backward_compat()` in `tests/level_definition.rs`
- [ ] T065 [P] [US4] Add test `test_level_with_only_description()` in `tests/level_definition.rs`
- [ ] T066 [P] [US4] Add test `test_level_with_only_author()` in `tests/level_definition.rs`
- [ ] T067 [P] [US4] Add test `test_level_with_full_metadata()` in `tests/level_definition.rs`

### Migration Tool Updates

- [x] T068 [US4] Migration tool removed - no longer needed
- [x] T069 [US4] Migration tool removed - no longer needed  
- [x] T070 [US4] Migration tool removed - no longer needed

### Integration Tests - Full Suite

- [ ] T071 [US4] Run `cargo test` with all level files from repository
- [ ] T072 [US4] Verify no level files fail to load after struct changes
- [ ] T073 [US4] Create comprehensive integration test loading all `assets/levels/*.ron` files
- [ ] T074 [US4] Run WASM build and test metadata parsing works on web target

### Documentation

- [ ] T075 [US4] Update `assets/levels/README.md` with migration guide section:
  - How to add metadata to existing levels
  - Bulk update strategies
  - Backward compatibility guarantees
- [ ] T076 [US4] Add note to `docs/CHANGELOG.md` about metadata field additions being backward compatible

### Final Verification

- [ ] T077 [US4] Run full test suite: `cargo test --all`
- [ ] T078 [US4] Run linter: `cargo clippy --all-targets --all-features`
- [ ] T079 [US4] Run formatter: `cargo fmt --all`
- [ ] T080 [US4] Verify Bevy linter: `bevy lint`

---

## Phase 6: Polish & Cross-Cutting Concerns

### Documentation Polish

- [ ] T081 Build complete documentation: `cd docs && make clean && make html`
- [ ] T082 Verify README.md renders correctly in GitHub web view
- [ ] T083 Verify all code examples in documentation are syntactically correct
- [ ] T084 Add feature overview to main `docs/index.md` if needed
- [ ] T085 Create example in `README.md` root showing level creation with metadata

### Code Quality

- [x] T086 Run `cargo fmt --all` to ensure consistent formatting
- [x] T087 Run `cargo clippy --all-targets --all-features` for warnings
- [x] T088 Address any clippy warnings in new code
- [x] T089 Verify no compiler warnings: `cargo build --all-targets`

### Final Testing

- [x] T090 Run full test suite with all features: `cargo test --all-features`
- [x] T091 Test game runs with metadata: `BK_LEVEL=999 cargo run --release`
- [x] T092 Test game with existing levels: `BK_LEVEL=1 cargo run --release`
- [x] T093 Build documentation: `cargo doc --no-deps`

### Git & Release

- [x] T094 Stage all changes: `git add .`
- [x] T095 Create commit with descriptive message including feature summary
- [x] T096 Update CHANGELOG.md with feature additions and backward compatibility note
- [x] T096 Push to feature branch for code review

---

## Task Execution Guide

### Dependencies & Parallelization

**Sequential (Must Complete)**:

1. T001-T008 (Phase 1 Setup) - prerequisite for all user stories
2. T009-T026 (US1) - foundation for T027+ and US2
3. T027-T053 (US2) - can overlap with T009-T026

**Parallel After T008**:

- US1 (T009-T026) + US2 (T027-T053): Can implement simultaneously, different files
- US3 (T054-T063): Depends on US1+US2 struct completion
- US4 (T064-T080): Can start after struct fields added

**Final Phase**:

- T081-T096: After all user stories complete

### Suggested Implementation Order

**Option A - Sequential (Simple)**:

1. Complete Phase 1 (T001-T008): 30 min
2. Complete Phase 2 (T009-T026): 60 min
3. Complete Phase 3 (T027-T053): 60 min
4. Complete Phase 4 (T054-T063): 30 min
5. Complete Phase 5 (T064-T080): 30 min
6. Complete Phase 6 (T081-T096): 20 min
   **Total: ~3.5 hours**

**Option B - Parallel (Faster)**:

1. Complete Phase 1 (T001-T008): 30 min
2. Parallel Phase 2+3 (T009-T026 + T027-T053): 90 min (both developers)
3. Complete Phase 4 (T054-T063): 30 min
4. Parallel Phase 5 (T064-T080): 30 min
5. Complete Phase 6 (T081-T096): 20 min
   **Total: ~2.5 hours (with parallelization)**

### MVP Completion Checklist

Minimum viable product = **P1 features only** (US1 + US2):

- [x] Struct updated with both fields
- [x] Helper functions implemented
- [x] Unit tests passing (T014-T042)
- [x] Integration tests passing (T043-T046, T018-T020)
- [x] Technical documentation updated (T021-T022, T047-T048)
- [x] User documentation updated (T023, T049-T050)
- [x] Backward compatibility verified
- [x] `cargo test` passes
- [x] `cargo clippy` clean
- [x] `cargo fmt` applied

**MVP Task Count**: T001-T053 (~50 tasks, ~2 hours)

---

## Acceptance Criteria by User Story

### US1: Level Designer Documents Level Intent

**Story Tasks**: T001-T026, T009-T026 core  
**Independent Test**:

- Create `level_test.ron` with `description: Some("Test description")`
- Load with `from_str::<LevelDefinition>(...)`
- Verify `level.has_description() == true`
- Verify `level.description == Some("Test description")`

**Completion When**:

- ✅ T009-T013 complete (struct + helpers)
- ✅ T014-T017 pass (description tests)
- ✅ T018-T020 pass (integration tests)
- ✅ T021-T023 complete (documentation)
- ✅ `cargo test` passes
- ✅ Backward compat verified (old levels still load)

### US2: Contributor Takes Credit for Work

**Story Tasks**: T027-T053 core  
**Independent Test**:

- Create level with `author: Some("[Jane Smith](mailto:jane@example.com)")`
- Load and verify `extract_author_name(author) == "Jane Smith"`
- Verify plain text author works: `"John Doe"` → `"John Doe"`

**Completion When**:

- ✅ T027-T034 complete (struct + helpers)
- ✅ T035-T042 pass (author tests)
- ✅ T043-T046 pass (integration tests)
- ✅ T047-T050 complete (documentation)
- ✅ `cargo test` passes
- ✅ Markdown extraction works correctly

### US3: Maintainer Reviews Level Documentation

**Story Tasks**: T054-T063  
**Independent Test**:

- Open `assets/levels/level_999.ron` in text editor
- Confirm description and author visible without special tools
- Confirm can search for author name across files

**Completion When**:

- ✅ T054-T060 complete (documentation + examples)
- ✅ T061-T063 pass (build & verification)
- ✅ Example files are well-documented
- ✅ README renders correctly

### US4: Migration of Existing Levels

**Story Tasks**: T064-T080  
**Independent Test**:

- Load all existing level files (`level_001.ron`, etc.)
- Verify no deserialization errors
- Verify behavior identical to before feature (backward compat)

**Completion When**:

- ✅ T064-T067 pass (backward compat tests)
- ✅ T068-T070 complete (migration tool)
- ✅ T071-T074 pass (integration tests)
- ✅ T075-T076 complete (documentation)
- ✅ T077-T080 pass (final verification)

---

## File Changes Summary

**Modified Files**:

- `src/level_loader.rs` - Add struct fields + helper functions
- `tests/level_definition.rs` - Create new test file
- `assets/levels/README.md` - Add metadata documentation
- `docs/asset-format.md` - Add user-facing documentation
- `docs/developer-guide.md` - Add level creation examples
- `tools/migrate-level-indices/src/main.rs` - Update struct
- `assets/levels/level_999.ron` - Create example with metadata

**Created Files**:

- `tests/level_definition.rs` - New test module
- `assets/levels/level_999.ron` - Example level

**No Breaking Changes**: All changes are backward compatible

---

## Validation Checklist

✅ **Format validation** (all tasks follow required format):

- [x] All tasks have checkbox: `- [ ]`
- [x] All tasks have Task ID: `T001`, `T002`, etc.
- [x] All tasks have description with file paths
- [x] P1 story tasks (T009-T026, T027-T053) have `[US1]`, `[US2]` labels
- [x] P2/P3 story tasks have `[US3]`, `[US4]` labels
- [x] Parallelizable tasks marked with `[P]`
- [x] Setup tasks (T001-T008) have no story labels
- [x] Polish tasks (T081-T096) have no story labels

✅ **Completeness validation**:

- [x] 96 total tasks covering all user stories
- [x] Phase 1 (Setup): 8 tasks
- [x] Phase 2 (US1): 18 tasks
- [x] Phase 3 (US2): 27 tasks
- [x] Phase 4 (US3): 10 tasks
- [x] Phase 5 (US4): 17 tasks
- [x] Phase 6 (Polish): 16 tasks

✅ **Dependency validation**:

- [x] US1 and US2 can execute in parallel after Phase 1
- [x] US3 depends on US1+US2 completion
- [x] US4 can start after Phase 2 struct changes
- [x] Polish phase depends on all user stories

✅ **Test coverage**:

- [x] Unit tests for all new functions (T014-T042)
- [x] Integration tests for backward compatibility (T018-T020, T064-T074)
- [x] Documentation build tests (T061-T063, T081-T084)
- [x] Code quality tests (T086-T093)

---

## Summary Statistics

- **Total Tasks**: 96
- **Estimated Hours**: ~3 hours (sequential), ~2.5 hours (parallel)
- **User Stories**: 4 (P1+P1+P2+P3)
- **Parallelizable Tasks**: 35+ (marked with `[P]`)
- **MVP Tasks** (P1 only): ~50 tasks, ~2 hours

---

## Implementation Notes

1. **Rust 1.81 Edition 2021**: Standard Rust patterns, no special requirements
2. **Serde + RON**: Leverage existing `#[serde(default)]` patterns from project
3. **No New Dependencies**: Uses only existing serde, ron, std library
4. **WASM Compatible**: String operations work identically on all platforms
5. **Backward Compatible**: Existing levels load unchanged
6. **Performance**: Zero impact on 60 FPS target (one-time parse per level load)
