# Implementation Plan: Textured Visuals Overhaul

**Branch**: `001-textured-visuals` | **Date**: 2025-11-26 | **Spec**: `/specs/001-textured-visuals/spec.md`
**Input**: Feature specification from `/specs/001-textured-visuals/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Deliver fully textured materials for all major gameplay objects by ingesting a RON-based texture manifest, mapping ball/brick types to `VisualAssetProfile`s, honoring per-level overrides, and exposing a Level Switch shortcut (Key **L**) so artists can audit each level’s presentation quickly.
Reliability hinges on the new `FallbackRegistry` resource that guarantees canonical materials load within one frame and logs each missing asset only once per session.

## Technical Context

**Language/Version**: Rust 1.81 (Rust 2021 edition) **Primary Dependencies**: Bevy 0.16 (rendering + ECS), bevy_rapier3d 0.31 (physics-driven entities), serde/ron (manifest parsing), tracing (structured logs) **Storage**: File-based RON assets under `assets/textures/` plus existing `assets/levels/` definitions **Testing**: `cargo test`, targeted module tests for manifest + fallback behavior, `cargo clippy --all-targets --all-features`, `bevy lint`, manual + WASM smoke runs via `cargo build --target wasm32-unknown-unknown --release` **Target Platform**: Native desktop (Linux/macOS/Windows) and WASM (Chrome/Firefox) **Project Type**: Single Bevy game crate (binary) **Performance Goals**: Maintain 60 FPS with zero visible “untextured” frames during loads; level switches complete in <2s **Constraints**: ECS-only state management, physics-driven gameplay, logging-only fallback handling, memory budget must fit baseline + one override pack simultaneously **Scale/Scope**: Two shipping levels today (001/002) with roadmap for multi-level campaigns; manifests target <50 profiles initially

## Constitution Check

*GATE: Must pass before Phase 0 research.*
  *Re-check after Phase 1 design.*

- **ECS-First**: Texture lookups, fallback tracking, and level switching implemented as resources + systems; no global mutable state. ✅
- **Physics-Driven Gameplay**: Visual systems decorate meshes spawned by physics entities without overriding Rapier ownership, so collisions remain authoritative. ✅
- **Modular Feature Design**: New `TextureManifestPlugin`, `FallbackRegistry`, and `LevelSwitchPlugin` register as opt-in system sets; supported by contracts + quickstart docs. ✅
- **Performance-First**: Lazy material baking plus single-frame fallbacks avoid stalls; plan includes profiling level switch to keep <2s target. ✅
- **Cross-Platform Compatibility**: Asset formats (PNG/KTX) already supported on native + WASM; manifest + fallback logic are platform-neutral, with WASM build included in quickstart checklist. ✅

## Project Structure

### Documentation (this feature)

```text
specs/001-textured-visuals/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── visual-assets.openapi.yaml
└── tasks.md        # generated during /speckit.tasks
```

### Source Code (repository root)

```text
assets/
├── levels/
│   ├── level_001.ron
│   └── level_002.ron
└── textures/
    ├── manifest.ron            # new canonical asset manifest
    └── fallback/

src/
├── main.rs                     # registers plugins + startup
├── level_loader.rs             # existing definition & spawn flows
└── systems/
    ├── mod.rs
    ├── grid_debug.rs
    ├── textures/
    │   ├── loader.rs           # manifest parsing + ECS resources
    │   ├── materials.rs        # fallback registry + material baking
    │   └── overrides.rs        # per-level texture application
    └── level_switch.rs         # **L** shortcut + wrapping logic

tests/
└── level_switcher.rs           # integration test for `KeyCode::L`
```

**Structure Decision**: Retain single-crate layout; add a focused `systems/textures/` module tree so manifest loading, fallback management, and overrides stay isolated yet testable. `tests/level_switcher.rs` hosts integration coverage for the shortcut without polluting gameplay modules.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | Constitution satisfied | N/A |
