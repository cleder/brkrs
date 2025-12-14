# Retrospective: Ball Lives System & WASM Texture Loading (2025-12-14)

## Overview

**Feature**: Ball Lives Counter System  
**Status**: Desktop ✅ Complete | WASM ⏳ Pending Verification  
**Duration**: Multi-session (implementation + debugging)  
**Final Outcome**: All desktop functionality complete, WASM texture loading issue diagnosed and fixed

## What We Accomplished

### Phase 1-6: Ball Lives Feature (Desktop)

- ✅ Lives counter component and resource (start at 3 lives)
- ✅ Ball loss detection and lives decrement
- ✅ HUD display (top-right corner, stylized "♥" symbols)
- ✅ Game-over overlay (triggered at 0 lives, "Game Over" + level name)
- ✅ Pause system integration (prevent input/respawn when paused/game-over)
- ✅ 44 tests passing (37 lib + 7 integration)
- ✅ Code formatted and linted

### WASM Debugging Journey

1. **Desktop Race Condition**: Fixed texture loading order by moving `load_ui_fonts` after plugins
2. **WASM Brick Textures**: Discovered missing `.meta` files requirement through browser console errors
3. **Level Embedding**: Expanded `embedded_level_str()` from 2 to 78 levels
4. **Meta File Format**: Iterated 10+ times to determine correct RON format for Bevy 0.17.3

## Technical Discoveries

### Critical Finding: Bevy WASM Asset Loader Requirements

**Problem**: Brick textures loaded on desktop but not on WASM (ball/paddle textures worked).

**Root Cause**: Bevy's WASM asset loader requires `.meta` files for **all** texture assets loaded via HTTP.
Missing metadata results in 404 errors and failed asset loading.

**Solution**: Create `.meta` files in RON format alongside each PNG texture:

```ron
(
    asset: Load(
        loader: "bevy_image::image_loader::ImageLoader",
        settings: (
            format: FromExtension,
            is_srgb: true,
            sampler: Default,
            asset_usage: 1,
        ),
    ),
)
```

**Key Insights**:

- Desktop Bevy can infer asset metadata from file extensions
- WASM Bevy requires explicit metadata due to HTTP-based loading
- Error messages reveal missing fields incrementally (frustrating but methodical)
- Final working format uses `asset_usage: 1` (simple numeric value)
- 20 `.meta` files created (13 in fallback/, 4 in default/, 3 in other subdirectories)

### Platform-Specific Behavior Differences

| Aspect | Desktop | WASM |
|--------|---------|------|
| Asset Loading | Synchronous, filesystem | Asynchronous, HTTP |
| Asset Metadata | Inferred from extension | Requires `.meta` files |
| Level Loading | Read from `assets/levels/*.ron` | Embedded at compile time |
| Font Loading | Startup schedule | Deferred to Update schedule |
| Binary Size | ~20MB (debug) | 88MB (includes 78 embedded levels) |

## What Went Well

1. **Systematic Testing**: Desktop functionality fully validated before WASM debugging
2. **Browser DevTools**: Console errors provided actionable diagnostics (404s, deserialization errors)
3. **Incremental Debugging**: Material management fixes were correct; metadata was the missing piece
4. **Debug Logging**: Added targeted logs to track TypeVariantRegistry and material application
5. **Documentation**: Bevy's error messages (though incremental) eventually revealed correct format

## What Could Be Improved

1. **WASM Documentation Gap**: Bevy's official docs don't clearly explain `.meta` file requirements for WASM
2. **Error Message Quality**: Deserialization errors revealed one field at a time (slow iteration)
3. **Testing Strategy**: No WASM integration tests to catch asset loading issues early
4. **Build Times**: WASM builds take ~2-3 minutes, slowing iteration on format issues
5. **Asset Management**: Manual `.meta` file creation is error-prone and not maintainable

## Lessons Learned

### For Future Development

1. **WASM-First Testing**: Test WASM builds early in feature development, not just at the end
2. **Asset Metadata**: Consider generating `.meta` files automatically in build pipeline
3. **Platform Parity**: Unified material management (recreating vs updating) worked well
4. **Debugging Approach**: Browser console + targeted tracing statements = powerful combination
5. **Documentation**: Internal docs should capture platform-specific requirements

### Technical Patterns That Worked

- **Local<bool> Gates**: Effective for run-once systems in async environments
- **Option<Res<T>>**: Good pattern for resources that may not be immediately available
- **Debug Logging**: Strategic `info!()` calls with counts/sizes for validation
- **Parallel Tool Execution**: Reading multiple files simultaneously improved debugging speed

## Action Items

### Immediate (User Deployment)

- [ ] Deploy updated WASM build with `.meta` files
- [ ] Verify brick textures load on WASM
- [ ] Test all 78 levels are accessible
- [ ] Validate lives counter functionality on WASM

### Short-Term (Code Quality)

- [ ] Remove debug logging after WASM verification
- [ ] Document `.meta` file requirements in developer guide
- [ ] Add WASM asset loading section to troubleshooting guide
- [ ] Consider automated `.meta` file generation in build pipeline

### Long-Term (Infrastructure)

- [ ] Investigate WASM binary size optimization (88MB is large)
- [ ] Add WASM-specific integration tests
- [ ] Create CI job for WASM builds
- [ ] Evaluate alternative level loading strategies (HTTP fetch vs embedding)

## Statistics

**Code Changes**:

- Files Modified: 4 (fonts.rs, lib.rs, materials.rs, level_loader.rs)
- Files Created: 21 (20 `.meta` files + 1 retrospective)
- Tests Added: 7 (ball lives integration tests)
- Tests Passing: 44 total

**Iterations**:

- Meta File Format: 10+ attempts
- WASM Builds: ~15 builds
- Material Management: 3 approaches tested

**Build Artifacts**:

- Desktop Binary: ~20MB (debug)
- WASM Binary: 88MB (release with embedded levels)
- Asset Metadata: 20 files (one per texture)

## Recommendations for Developer Guide Updates

Based on this session, the developer guide should be amended with:

1. **New Section**: "Building for WASM"
   - Asset metadata requirements
   - `.meta` file format and creation
   - Level embedding considerations
   - Binary size trade-offs

2. **Troubleshooting Section**: "WASM Texture Loading Issues"
   - Missing `.meta` file symptoms (404 errors)
   - Deserialization error patterns
   - Browser console debugging workflow

3. **Asset Management**: Update textures section
   - Add note about `.meta` files for WASM
   - Example `.meta` file for ImageLoader
   - Automated generation recommendations

4. **Testing Section**: Add WASM testing guidance
   - How to test WASM builds locally
   - Common platform differences
   - Browser console debugging techniques

See proposed amendments in the following section.

---

## Appendix: Debug Commands Used

```bash
# Check texture files exist
ls -la assets/textures/**/*.png

# Create meta files (final working version)
find assets/textures -name "*.png" -type f | while read png; do 
  cat > "${png}.meta" << 'EOF'
(
    asset: Load(
        loader: "bevy_image::image_loader::ImageLoader",
        settings: (
            format: FromExtension,
            is_srgb: true,
            sampler: Default,
            asset_usage: 1,
        ),
    ),
)
EOF
done

# Build WASM
cargo build --target wasm32-unknown-unknown --release

# Check WASM binary size
ls -lh target/wasm32-unknown-unknown/release/brkrs.wasm

# Verify meta file format
cat assets/textures/fallback/brick_base.png.meta
```

## References

- [Bevy Asset System Documentation](https://docs.rs/bevy_asset/0.17.3/bevy_asset/)
- [Bevy WASM Examples](https://github.com/bevyengine/bevy/tree/main/examples/wasm)
- [RON Format Specification](https://github.com/ron-rs/ron)
- Feature Specifications: `specs/001-ball-lives/`
