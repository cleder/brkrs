# PR #150 Code Review Tasks

**PR**: [017 brick material textures](https://github.com/cleder/brkrs/pull/150) **Branch**: `017-brick-material-textures` **Status**: Open, Blocked (requires review resolution) **Files**: 5 changed, +56/-30 lines

---

## Blocking Issues (P1 - MUST FIX BEFORE MERGE)

### ✅ Material System & Textures

- [ ] T001 [P1] Fix parallax_depth_scale edge case in [src/systems/textures/materials.rs](src/systems/textures/materials.rs#L474-L476)
  - **Issue**: `parallax_depth_scale` set unconditionally even when `depth_map` is None
  - **Current**: `parallax_depth_scale: profile.depth_scale,`
  - **Fix**: Conditionally set to 0.0 when depth texture missing:
    ```rust
    parallax_depth_scale: if depth_texture.is_some() {
        profile.depth_scale
    } else {
        0.0
    },
    ```
  - **Test**: Verify no parallax applied when depth_map is None
  - **Reference**: Line 474-476, qodo-code-review P1 issue

- [ ] T002 [P1] Remove test texture from production fallback profile in [assets/textures/manifest.ron](assets/textures/manifest.ron#L39)
  - **Issue**: Test asset `test_emissive.png` appears in brick/default (production fallback)
  - **Current**: Line 39 has `emissive_path: Some("test_emissive.png"),`
  - **Fix**: Replace with production asset or remove emissive_path from brick/default profile
  - **Reason**: Test assets must not be in production configs
  - **Reference**: Line 40, coderabbitai (flagged twice)

- [ ] T003 [P1] Fix emissive texture color space in [src/systems/textures/materials.rs](src/systems/textures/materials.rs#L430)
  - **Issue**: Emissive texture loaded with `is_srgb=true` (should be `false` for linear color space consistency)
  - **Current**:
    ```rust
    settings.is_srgb = true,  // Line 430
    ```
  - **Fix**: Change to `is_srgb = false` (like ORM and normal maps):
    ```rust
    settings.is_srgb = false,
    ```
  - **Reason**: Color space consistency with other texture types (ORM, normal maps)
  - **Reference**: coderabbit pre-merge check warning

---

## Important Issues (P2 - MUST ADDRESS BEFORE MERGE)

### ✅ Configuration & Behavior Changes

- [ ] T004 [P2] Document ORM scalar behavior change in [src/systems/textures/materials.rs](src/systems/textures/materials.rs#L454) and [docs/api-reference.md](docs/api-reference.md)
  - **Issue**: ORM scalar handling changed from ignored-when-texture-present to always-acts-as-multiplier
  - **Context**: Line 454 shows ORM scalar applied unconditionally (behavioral change)
  - **Action**:
    - [ ] T004a: Add migration note in code comments explaining behavior change
    - [ ] T004b: Document breaking change in api-reference.md with asset compatibility notes
    - [ ] T004c: Create follow-up issue for asset validation task
  - **Reference**: copilot-reviewer line 454, llamapreview P2 behavioral change

- [ ] T005 [P2] Add missing material asset fallback handling in [src/systems/textures/materials.rs](src/systems/textures/materials.rs#L226)
  - **Issue**: Silent failure if material asset unloads from bank
  - **Current**: No fallback or error recovery
  - **Action**:
    - [ ] T005a: Add material recreation logic when asset missing
    - [ ] T005b: Add error logging for missing asset scenarios
    - [ ] T005c: Test asset unload + reload cycle
  - **Reference**: codereviewbot-ai line 226

- [ ] T006 [P2] Evaluate variant-to-material tinting completeness in [src/systems/textures/materials.rs](src/systems/textures/materials.rs#L409)
  - **Issue**: Emissive color parameter added but feature incomplete (unused data flow)
  - **Current**: Parameter accepted at line 409 but not fully wired to material variants
  - **Options**:
    - Option A: Complete the variant-to-material connection (if feature needed)
    - Option B: Document as incomplete feature with tracking issue (if deferring)
  - **Action**: Choose option and either:
    - Complete: Wire emissive_color to all material variants and test
    - Document: Add TODO comment explaining incomplete state, create follow-up issue
  - **Reference**: copilot-reviewer line 409, llamapreview P2 design gap

---

## Code Quality Issues (P3 - RECOMMENDED FOR NEXT ITERATION)

### ⚠️ Refactoring & Improvements

- [ ] T007 [P3] Extract texture loading helper function in [src/systems/textures/materials.rs](src/systems/textures/materials.rs)
  - **Issue**: Texture loading pattern repeated 4+ times (code duplication)
  - **Locations**: Lines 427-434 (emissive), similar patterns for ORM and normal maps
  - **Action**:
    - [ ] T007a: Create helper function `load_optional_texture(path: Option<&str>) -> Option<Handle<Image>>`
    - [ ] T007b: Replace all 4 instances with helper calls
    - [ ] T007c: Test all texture types still load correctly
  - **Reference**: gemini-code-assist line 444

- [ ] T008 [P3] Clean up inconsistent depth configuration in [assets/textures/manifest.ron](assets/textures/manifest.ron)
  - **Issue**: `depth_scale` field set without corresponding `depth_path` in 5 profiles
  - **Action**:
    - [ ] T008a: Audit all profiles, list inconsistent ones
    - [ ] T008b: Either add valid `depth_path` or remove unused `depth_scale` field
    - [ ] T008c: Document profile consistency rules in asset-format.md
  - **Reference**: coderabbitai line 54

- [ ] T009 [P3] Add runtime validation for depth_scale bounds in [src/systems/textures/materials.rs](src/systems/textures/materials.rs#L475)
  - **Issue**: `depth_scale` field lacks bounds validation
  - **Action**:
    - [ ] T009a: Add validation in `make_material()` to ensure depth_scale in valid range [0.0, 1.0]
    - [ ] T009b: Log warning if out of bounds
    - [ ] T009c: Test invalid configs are caught at load time
  - **Reference**: coderabbitai line 475

---

## Verification & Testing Tasks

- [ ] T010 [P1] Run test suite for material system
  - **Command**: `cargo test --test ball_material_startup --test depth_textures --test orm_textures --test emissive_textures`
  - **Pass criteria**: All tests passing
  - **Reference**: Pre-merge validation

- [ ] T011 [P1] Verify no test assets in production configs
  - **Command**: Grep for "test_" in manifest.ron, exclude test profiles
  - **Pass criteria**: No test assets in production fallback profiles
  - **Reference**: T002 verification

- [ ] T012 [P2] Document breaking changes in CHANGELOG.md
  - **Action**: Add entry under "Breaking Changes" section documenting ORM scalar behavior change
  - **Reference**: T004 completion

- [ ] T013 [P3] Run clippy to detect code duplication patterns
  - **Command**: `cargo clippy --all-targets -- -W clippy::manual_memcpy`
  - **Reference**: T007 verification

---

## Merge Checklist

- [ ] All P1 issues fixed (T001, T002, T003)
- [ ] All P2 issues resolved or documented (T004, T005, T006)
- [ ] Tests passing (T010, T011)
- [ ] Breaking change documented (T012)
- [ ] PR approved by maintainers
- [ ] PR #150 mergeable_state changed from "blocked" to "clean"

---

## Follow-up Issues (Post-Merge)

**Create new GitHub issues for:**

1. **[P3]** Code duplication: Extract texture loading helper (T007)
2. **[P3]** Asset configuration consistency: Document and enforce depth_scale usage (T008)
3. **[P3]** Material system robustness: Add depth_scale validation (T009)
4. **[P2]** Asset compatibility: Create migration guide for ORM scalar behavior change
5. **[P2]** Feature completion: Complete variant-to-material tinting (if option A chosen in T006)

---

## Summary

| Priority | Count | Status | Examples |
|----------|-------|--------|----------|
| P1 | 3 | **BLOCKING** | parallax_depth_scale, test texture, color space |
| P2 | 3 | **IMPORTANT** | behavior change docs, asset fallback, feature completion |
| P3 | 4 | **RECOMMENDED** | code duplication, config consistency, validation |
| **Tests** | 4 | **REQUIRED** | test suite, asset verification, doc update, clippy |

**Immediate Action**: Apply fixes for T001, T002, T003 and push new commit to unblock PR.
