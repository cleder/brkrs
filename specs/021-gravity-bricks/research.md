# Research: Gravity Indicator UI

**Date**: 2026-01-11 | **Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)

## Phase 0: Design Decisions

This document resolves all "NEEDS CLARIFICATION" items from Technical Context and justifies key design choices.

## 1. UI Framework Choice

**Decision**: Use Bevy's built-in UI system (`bevy::ui`) with `ImageNode` and `Node` components.

**Rationale**:

- Already used for existing indicators (cheat mode, lives counter, score display)
- Consistent with project architecture (no external UI dependencies)
- Native support for absolute positioning and asset loading
- WASM-compatible

**Alternatives Considered**:

- `bevy_egui`: Rejected - adds unnecessary dependency for simple image display
- Custom mesh rendering: Rejected - overcomplicated for a static UI element
- Text-based indicator: Rejected - user requested icon-based display

## 2. Gravity Mapping Algorithm

**Decision**: Map X/Z axes only; round each component to nearest integer with ±0.5 tolerance; select highest recognized magnitude (0, 2, 10, 20) among axes; fallback to question icon.

**Rationale**:

- Clarification resolved: Y axis always 0, only X/Z matter for gameplay (top-down view)
- ±0.5 tolerance accounts for floating-point variance in physics calculations
- Highest magnitude selection reflects dominant gravity effect on ball behavior
- Question icon provides visual feedback for unknown/random gravity states

**Alternatives Considered**:

- Vector magnitude approach: Rejected - less intuitive for mixed-axis gravity (e.g., X=2, Z=10)
- Y-axis priority: Rejected - clarification specified X/Z only
- Exact matching (no tolerance): Rejected - too brittle for physics float values

## 3. Spawn Timing Strategy

**Decision**: Spawn at first frame when both `GravityConfiguration` resource and `GravityIndicatorTextures` resource are available.
Use idempotent spawn system with `if !existing.is_empty()` guard.

**Rationale**:

- Clarification resolved: Defer spawn until both dependencies ready to prevent blank display
- Idempotent pattern prevents duplicate spawns if system runs multiple times
- Consistent with existing UI spawn patterns in codebase (see `lives_counter::spawn_lives_counter`)
- Avoids asset loading race conditions in WASM builds

**Alternatives Considered**:

- Startup schedule: Rejected - GravityConfiguration may not exist yet
- Retry loop: Rejected - unnecessary complexity; Update system naturally retries each frame
- Separate "ready" event: Rejected - overengineered for simple check

## 4. Update Trigger Mechanism

**Decision**: Use `Changed<GravityConfiguration>` filter in update system to detect when gravity changes.
Instant icon swap (no transition animation).

**Rationale**:

- Clarification resolved: Instant swap matches immediate physics feedback expectations
- `Changed<T>` is idiomatic Bevy 0.17 pattern for reactive UI (constitution mandate)
- Avoids per-frame overhead when gravity is static
- Last-write-wins naturally handles multiple changes per frame (matches physics behavior)

**Alternatives Considered**:

- Per-frame comparison: Rejected - violates "No Universal Query Updates" prohibition
- Fade transition: Rejected - clarification specified instant swap
- Observer pattern: Rejected - overkill for resource change detection; Messages/Observers for events, not resource state

## 5. Asset Path Convention

**Decision**: Load assets using paths `"textures/default/weight-{0,2,10,20,question}.png"` via `AssetServer` in Startup schedule.
Store handles in `GravityIndicatorTextures` resource.

**Rationale**:

- Assets already exist at specified paths (user provided)
- Consistent with existing texture loading pattern (see `cheat_indicator::CheatIndicatorTexture`)
- Startup loading ensures assets ready before spawn system runs (satisfies deferred spawn requirement)
- Asset server path omits `assets/` prefix per Bevy convention

**Alternatives Considered**:

- Lazy loading: Rejected - risks blank display and doesn't satisfy spawn timing requirement
- Asset manifest: Rejected - not needed for 5 simple images
- Embedded assets: Rejected - prefer flexible file-based loading

## 6. Multi-Frame Correctness Strategy

**Decision**: Integration tests will call `app.update()` 10+ times after gravity change and verify indicator level remains correct.
Include all gravity-writing systems in test app.

**Rationale**:

- Constitution mandate for runtime state changes (see 020-gravity-bricks retrospective)
- Catches bugs where initialization systems overwrite runtime changes
- Validates that `is_changed()` filter prevents unnecessary updates
- 10 frames chosen per constitution minimum recommendation

**Alternatives Considered**:

- Single-frame assertion: Rejected - insufficient per constitution (misses per-frame overwrites)
- Manual verification: Rejected - automated tests required per TDD mandate
- Separate test per system: Rejected - need full system integration to catch interactions

## Summary

All clarifications resolved.
Design decisions favor:

- Simplicity (existing UI framework, no custom rendering)
- Robustness (idempotent spawn, deferred loading, change detection)
- Constitution compliance (TDD, change detection, asset handle reuse, no unconditional writes)
- Consistency with existing codebase patterns (cheat indicator, lives counter)

**No outstanding unknowns**.
Ready for Phase 1 (data model & contracts).
