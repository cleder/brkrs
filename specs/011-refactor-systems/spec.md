# Feature Specification: Systems Constitution Refactor

**Branch**: `copilot/refactor-legacy-code-systems` | **Date**: 2025-12-19 | **ID**: 011-refactor-systems

## Overview

Bring `src/systems/` into compliance with the Brkrs Constitution (version 1.3.0) by refactoring legacy code to align with Section VIII: Bevy 0.17 Mandates & Prohibitions.

## Problem Statement

The `src/systems/` directory contains legacy code written before the Constitution (v1.3.0) was ratified. This code may contain violations of Bevy 0.17 best practices including:

- Panicking query patterns (`.unwrap()`, `.single()` without error handling)
- Non-fallible systems (not returning `Result`)
- Missing change detection (`Changed<T>`) for reactive systems
- Repeated asset loading instead of handle reuse
- Missing required components
- Improper system organization (no system sets)
- Lack of plugin-based architecture

## Goals

### Primary Goals

1. **Compliance Audit**: Produce a complete, traceable compliance audit for `src/systems` against Constitution Section VIII
2. **Refactoring**: Bring all systems code into full Constitution compliance
3. **Behavior Preservation**: Ensure no player-facing behavior changes

### Non-Goals

- Adding new features or functionality
- Refactoring code outside `src/systems/` (except minimal supporting changes)
- Performance optimization beyond Constitution requirements

## User Stories

### US1: Produce Complete Compliance Audit (P1) 🎯 MVP

**As a** maintainer  
**I want** a complete compliance audit of `src/systems/`  
**So that** I know exactly which Constitution violations need fixing

**Acceptance Criteria**:

- [ ] Audit document exists at `specs/011-refactor-systems/compliance-audit.md`
- [ ] Every file in `src/systems/` is referenced in the audit
- [ ] Every Constitution Section VIII mandate/prohibition is checked
- [ ] Each violation includes: file path, line number, Constitution rule, explanation
- [ ] Audit is traceable and verifiable

### US2: Refactor Systems into Full Compliance (P2)

**As a** developer  
**I want** all systems code to comply with Constitution mandates  
**So that** the codebase follows Bevy 0.17 best practices

**Acceptance Criteria**:

- [ ] All systems return `Result` and use fallible query patterns
- [ ] No panicking queries (`.unwrap()`, `.single()` without `?`)
- [ ] Reactive systems use `Changed<T>` filters
- [ ] Asset handles are loaded once and cached in Resources
- [ ] Required components use `#[require(Transform, Visibility)]`
- [ ] Systems organized into system sets with `*Systems` suffix
- [ ] Plugin-based architecture implemented
- [ ] All rustdoc gaps filled for public items

### US3: Preserve System Behavior (P3)

**As a** player  
**I want** all game systems to work exactly as before  
**So that** the refactor doesn't break existing gameplay

**Acceptance Criteria**:

- [ ] All existing tests pass
- [ ] Audio system plays sounds correctly
- [ ] Scoring system awards points correctly
- [ ] Paddle size effects work as expected
- [ ] Respawn system functions properly
- [ ] Level switching works correctly
- [ ] Multi-hit bricks behave correctly
- [ ] Cheat mode toggles work

## Technical Requirements

### Functional Requirements

- **FR-011.1**: All systems MUST return `Result<(), Box<dyn Error>>` or equivalent
- **FR-011.2**: All query methods MUST use `?` operator, never `.unwrap()`
- **FR-011.3**: Reactive systems MUST use `Changed<T>` filters
- **FR-011.4**: Asset handles MUST be loaded once and stored in Resources
- **FR-011.5**: Marker components MUST use `#[require(Transform, Visibility)]`
- **FR-011.6**: Systems MUST be organized into system sets with `*Systems` suffix
- **FR-011.7**: Each subsystem MUST be implemented as a Plugin

### Non-Functional Requirements

- **NFR-011.1**: Refactor MUST NOT change player-facing behavior
- **NFR-011.2**: All existing tests MUST continue to pass
- **NFR-011.3**: Code MUST pass `cargo clippy` and `bevy lint`
- **NFR-011.4**: Performance MUST NOT regress

## Files in Scope

### Primary Targets (src/systems/)

- `audio.rs` - Audio event system
- `cheat_mode.rs` - Cheat mode toggle
- `grid_debug.rs` - Debug grid visualization
- `level_switch.rs` - Level transition logic
- `mod.rs` - Module exports
- `multi_hit.rs` - Multi-hit brick logic
- `paddle_size.rs` - Paddle size powerups
- `respawn.rs` - Ball respawn system
- `scoring.rs` - Score tracking and milestones
- `textures/` - Texture manifest subsystem

### Supporting Files (minimal edits only)

- `src/lib.rs` - Plugin registration
- Test files in `tests/` - Add Constitution compliance tests

## Out of Scope

- Code outside `src/systems/` (unless minimal supporting edits required)
- New features or functionality
- UI systems (already refactored in 010-refactor)
- Performance optimization beyond Constitution requirements

## Dependencies

- Previous work: `specs/010-refactor/` (UI refactor pattern to follow)
- Constitution: `.specify/memory/constitution.md` (v1.3.0)
- Bevy 0.17.3 API documentation

## Success Metrics

- All Constitution violations in `src/systems/` resolved
- All existing tests passing
- No clippy or bevy lint warnings introduced
- No performance regressions
- All player-facing behavior preserved

## Timeline

**Estimated effort**: 2-3 days

1. Day 1: Compliance audit (US1)
2. Day 2: Refactoring (US2)
3. Day 3: Behavior validation (US3)

## References

- Constitution: `.specify/memory/constitution.md`
- UI Refactor: `specs/010-refactor/`
- Bevy 0.17 Migration Guide: https://bevyengine.org/learn/migration-guides/0-16-to-0-17/
