# Spec 016 Code Review Remediation Plan

## Overview

This document outlines the comprehensive plan to address all code review comments for spec 016 (audio wall delay fix).
The plan is organized by priority and implementation phases.

## Code Review Summary

The PR received 29 code review comments covering:

- Architecture violations (Event/Message misuse)
- Production code quality issues (debug prints)
- Documentation inconsistencies
- Logic errors and safety issues
- Experimental feature concerns

## Phase 1: Critical Architecture Fixes (Must Fix)

### 1.1 BallWallHit Event/Message Architecture Violation

**Issue**: `BallWallHit` derives both `Event` and `Message`, violating constitution's clear separation **Priority**: P1 (Critical)

**Actions**:

- Remove `Message` derive from `BallWallHit` struct in `src/signals.rs`
- Change `app.add_message::<BallWallHit>()` to `app.add_event::<BallWallHit>()` in `src/lib.rs`
- Update test in `tests/integration/wall_audio.rs` to use consistent `add_event` pattern

**Files**: `src/signals.rs`, `src/lib.rs`, `tests/integration/wall_audio.rs`

### 1.2 Observer Registration Architecture Fix

**Issue**: `on_ball_wall_hit_sound` observer registered in `lib.rs` instead of `AudioPlugin`, creating fragile dependency **Priority**: P1 (Critical)

**Actions**:

- Move `app.add_observer(on_ball_wall_hit_sound)` from `src/lib.rs` to `AudioPlugin::build()` in `src/systems/audio.rs`
- Ensure observer registration happens within the plugin for self-containment

**Files**: `src/lib.rs`, `src/systems/audio.rs`

### 1.3 Remove Production Debug Code

**Issue**: `println!` in collision detection system spams console during gameplay **Priority**: P2 (Major)

**Actions**:

- Replace `println!` with `debug!` macro using `tracing` crate in `detect_ball_wall_collisions()`
- Use structured logging: `debug!(target: "physics", ball_entity = ?ball_entity, wall_entity = ?other_entity, "BallWallHit event emitted")`

**Files**: `src/lib.rs`

## Phase 2: Documentation & Contract Fixes

### 2.1 Contract Documentation Accuracy

**Issue**: Contract specifies wrong field names (`ball`/ `wall` vs `ball_entity`/ `wall_entity`) and non-existent fields **Priority**: Major

**Actions**:

- Update `specs/016-audio-wall-delay-fix/contracts/ball_wall_hit_event.md` to match actual struct fields
- Remove references to non-existent `impulse` and `timestamp` fields
- Update data model documentation in `specs/016-audio-wall-delay-fix/data-model.md`

**Files**: `specs/016-audio-wall-delay-fix/contracts/ball_wall_hit_event.md`, `specs/016-audio-wall-delay-fix/data-model.md`

### 2.2 Documentation Cleanup

**Issue**: Placeholders, redundant text, and formatting issues in documentation **Priority**: Minor-Major

**Actions**:

- Replace placeholder in `.github/agents/copilot-instructions.md` with actual storage info
- Remove redundant text in `docs/developer-guide.md` (duplicate "not interchangeable" phrases)
- Fix duplicate success criteria in spec
- Remove leftover comments in `src/lib.rs`

**Files**: `.github/agents/copilot-instructions.md`, `docs/developer-guide.md`, `specs/016-audio-wall-delay-fix/spec.md`, `src/lib.rs`

## Phase 3: Logic & Safety Fixes

### 3.1 Lower Border Rendering Fix

**Issue**: Lower border spawned with zero width (`Cuboid::new(0.0, 5.0, PLANE_W)`), making it invisible **Priority**: Major

**Actions**:

- Change lower border dimensions to `Cuboid::new(5.0, 5.0, PLANE_W)` with matching collider
- Ensure lower border is visible and properly blocks balls

**Files**: `src/lib.rs`

### 3.2 Asset Loading Consistency

**Issue**: Only wall hit sounds check asset loading, creating inconsistent error handling **Priority**: Medium

**Actions**:

- Evaluate if asset loading checks should be universal or wall-hit specific
- Document the rationale for selective checking
- Consider making asset checks universal if appropriate

**Files**: `src/systems/audio.rs`

### 3.3 Concurrent Sound Count Logic Fix

**Issue**: Incorrect decrement logic when asset loading fails **Priority**: Medium

**Actions**:

- Only decrement sound count if increment previously succeeded
- Restructure logic to pair increment/decrement operations

**Files**: `src/systems/audio.rs`

### 3.4 UI Beep Playback Limiting

**Issue**: Unbounded beep playback could cause audio spam **Priority**: Medium

**Actions**:

- Limit beep sounds per frame (e.g., max 4 beeps)
- Or play single beep when any messages received

**Files**: `src/systems/audio.rs`

## Phase 4: Entity Safety & Testing

### 4.1 Entity Reference Validity

**Issue**: `BallWallHit` contains entity references that may become invalid **Priority**: Medium

**Actions**:

- Add entity existence checks in observer functions
- Document expected entity lifetime in contract
- Consider defensive programming patterns

**Files**: `src/systems/audio.rs`, contract documentation

### 4.2 Test Pattern Consistency

**Issue**: Test uses `add_event` but production used `add_message` (now fixed) **Priority**: Medium

**Actions**:

- Verify test and production use identical registration patterns
- Ensure test emission matches production (`commands.trigger()` vs `app.world.trigger()`)

**Files**: `tests/integration/wall_audio.rs`

## Phase 5: Follow-up Issues (Separate PRs)

### 5.1 Parallax Example Improvements

**Issue**: Camera movement precision and material scope issues **Priority**: Low (Experimental)

**Actions**: Create follow-up issue for experimental feature improvements

### 5.2 Observer Registration for Other Events

**Issue**: Other observer events (`WallHit`, `BrickHit`, `BallHit`) may need `add_event` calls **Priority**: Medium

**Actions**: Investigate and potentially add missing `add_event` registrations

## Implementation Order

1. **Immediate (This PR)**: Architecture violations (P1 issues)
2. **This PR**: Documentation accuracy and safety fixes
3. **This PR**: Logic fixes (border rendering, sound counting)
4. **Follow-up**: Experimental features and edge cases

## Validation Checklist

- [ ] All tests pass (`cargo test`)
- [ ] No compilation warnings (`cargo clippy`)
- [ ] Documentation builds (`cd docs && make html`)
- [ ] Contract matches implementation
- [ ] Observer pattern works correctly
- [ ] No console spam in production
- [ ] Lower border visible and functional
- [ ] Audio system handles missing assets gracefully

## Risk Assessment

- **High Risk**: Architecture changes (Event/Message separation) - could break observer system
- **Medium Risk**: Observer registration moves - could affect plugin loading order
- **Low Risk**: Documentation and debug code changes

## Testing Strategy

- Unit tests for BallWallHit event structure
- Integration tests for wall audio emission and observation
- Manual testing for audio feedback timing
- Regression testing for all audio features

---

*Last Updated: 29 December 2025* *Spec: 016-audio-wall-delay-fix*
