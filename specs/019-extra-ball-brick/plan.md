# Implementation Plan: Extra Ball Brick (Brick 41)

**Branch**: `019-extra-ball-brick` | **Date**: 2026-01-10 | **Spec**: [specs/019-extra-ball-brick/spec.md](specs/019-extra-ball-brick/spec.md)
**Input**: Feature specification from `/specs/019-extra-ball-brick/spec.md`

**Note**: Filled via `/speckit.plan` workflow.

## Summary

Add brick type 41 "Extra Ball": a destructible, single-hit brick that grants +1 life (clamped to max), awards 0 points, and plays a unique destruction sound.
Life gain and audio trigger use Messages (buffered events) aligned with existing brick hit flow; no new UI mechanics.
Assets are loaded once and reused via handles.

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, serde/ron for level data, tracing **Storage**: N/A (in-memory ECS state only) **Testing**: cargo test, cargo clippy, bevy lint; TDD with failing-first commit required **Target Platform**: Native desktop + WASM builds **Project Type**: Single game crate (Bevy app) with integration tests **Performance Goals**: Maintain 60 FPS; no added per-frame work beyond existing brick collision handling **Constraints**: No panicking queries; assets loaded once; message-event separation; clamp lives to configured max; avoid per-frame UI mutation without `Changed<T>` **Scale/Scope**: Localized feature (new brick type, sound, messaging)

## Constitution Check

*Gate status: PASS (pre-Phase 0).*
No violations identified.

- **TDD gate**: Plan assumes tests-first with a red commit before implementation; mandatory for brick 41 behaviors and audio.
- **Bevy event system**: Use Messages for life-award and audio trigger (batchable, cross-frame safe).
  No observers required; respects message-event separation.
- **Coordinate system**: Gameplay on XZ plane; Y locked.
  No new movement introduced; brick placement follows existing grid (XZ), camera top-down at +Y.
  Direction terms map to +Z (forward toward bricks), -Z toward paddle, ±X lateral.
- **Queries & safety**: Use `With/Without` filters; `Changed<T>` for UI if touched; no panicking queries; fallible helpers allowed with `Result` and logging.
- **Assets**: Unique destruction sound loaded once into a Resource; handle reused.
  Fallback to generic brick sound if missing.
- **Hierarchy**: No new hierarchy needs; if any child effects are added, use `add_child` APIs only.

*Post-Phase 1 design check: PASS.*
Research/design artifacts align with constitution mandates; no new risks introduced.

## Project Structure

### Documentation (this feature)

```text
specs/019-extra-ball-brick/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
└── tasks.md (created by /speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── level_loader.rs
├── physics_config.rs
├── systems/
├── ui/
└── ...

tests/
├── integration and feature tests (various files)
└── ...

assets/
├── levels/
├── audio/
└── ...
```

**Structure Decision**: Single Bevy crate with existing `src/` and `tests/` plus assets/levels and audio.
Feature docs live in `specs/019-extra-ball-brick/`.

## Complexity Tracking

No constitutional violations to justify.
