# Implementation Plan: Sea Mine Brick

**Branch**: `067-add-seamine-brick` | **Date**: 2026-05-31 | **Spec**: [specs/067-add-seamine-brick/spec.md](specs/067-add-seamine-brick/spec.md)
**Input**: Feature specification from [specs/067-add-seamine-brick/spec.md](specs/067-add-seamine-brick/spec.md)

## Summary

Add sea mine brick type 31 to the brick registry so destroying it spawns a spinning sea mine hazard with arbitrary XZ launch direction, minimum linear speed of 3.0 u/s, and minimum angular spin of 180 deg/s.
The mine detonates on wall, paddle, or brick index > 90 contact, destroys balls and the paddle within a 30-unit blast radius, triggers exactly one life loss when the paddle is caught, and uses `bevy_hanabi` for the explosion particle burst.

## Technical Context

**Language/Version**: Rust 1.89 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, bevy_hanabi 0.17.0, tracing 0.1 **Storage**: N/A (in-memory ECS state only) **Testing**: `cargo test`, targeted integration tests, `cargo fmt --all`, `cargo clippy --all-targets --all-features`, `bevy lint` **Target Platform**: Native desktop and WASM **Project Type**: Single Rust Bevy ECS game project **Performance Goals**: Maintain 60 FPS while adding a new hazard, detonation pass, and particle burst **Constraints**: Message/observer separation, no per-frame overwrite of sea mine motion state, initial motion constrained to XZ plane, all asset handles loaded once, no panicking queries, feature requires a Rust 1.89-compatible toolchain because `bevy_hanabi` 0.17.0 does not build on Rust 1.81 **Scale/Scope**: One new brick index (31), one new hazard entity, one Hanabi explosion effect, and one new gameplay message flow for spawn/detonation resolution

## Constitution Check

*GATE: Must pass before Phase 0 research.*
*Re-check after Phase 1 design.*

This check MUST verify compliance with the constitution, including **Test-Driven Development (TDD)** gates:

- Tests are defined and committed prior to implementation efforts for each story/feature.
- A proof-of-failure commit (tests that FAIL) MUST exist in the branch history prior to implementation commits.
- Tests MUST be reviewed and approved by the feature owner or requestor before implementation begins.

This check MUST also verify compliance with **Bevy 0.17 mandates & prohibitions** (if the feature touches ECS, rendering, assets, or scheduling):

- **Bevy Event System Guidance:**
  - Use **Messages** for buffered gameplay work: spawn request, detonation resolution, ball/paddle destruction, and life-loss propagation.
  - Use **Observers** only for immediate presentation work: the Hanabi explosion burst and any same-frame audiovisual reaction.
  - This keeps gameplay state deterministic while allowing the particle effect to respond instantly.
- **Coordinate System Guidance:**
  - The mine moves on the XZ plane with Y locked.
  - Arbitrary launch direction means a randomized XZ vector, not Bevy `Transform::forward()` semantics.
  - Blast radius is measured in world-space distance from the detonation point.
- **Initialization System Idempotence:**
  - Spawn queues and effect resources must initialize once per feature lifetime, not every frame.
  - Any Update-schedule setup must be guarded so the mine motion is not overwritten after spawn.
- **Multi-Frame Persistence Testing:**
  - Tests must verify the mine remains active across at least 10 `app.update()` cycles before detonation.
  - Tests must include all systems that influence motion, explosion, and life loss.
- Systems are fallible and do not panic on query outcomes.
- Queries use `With<T>` / `Without<T>` filters and `Changed<T>` where needed.
- Assets are loaded once and stored in resources, including the Hanabi effect asset.
- Hierarchies use `add_child` / `set_parent`; no manual child mutation.

Pre-Phase 0 Gate Review:

- TDD-first gate: PASS.
- Bevy Messages vs Observers decision: PASS.
- Coordinate-system gate: PASS.
- Initialization idempotence gate: PASS.
- Multi-frame persistence gate: PASS.
- Fallibility/query safety gate: PASS.
- Hierarchy safety gate: PASS.
- UI change-detection gate: N/A for this feature.

Post-Phase 1 Gate Re-check:

- Design artifacts must preserve the same gates with no new violations.

## Project Structure

### Documentation (this feature)

```text
specs/067-add-seamine-brick/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── gameplay-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── lib.rs
├── signals.rs
├── level_format/
│   └── mod.rs
└── systems/
    ├── merkaba.rs
    ├── particle_fx.rs
    └── sea_mine.rs

tests/
├── sea_mine_brick.rs
├── sea_mine_particles.rs
└── sea_mine_lifecycle.rs

assets/
└── textures/
```

**Structure Decision**: Keep the feature inside the existing single-project Bevy ECS layout.
Extend the current gameplay modules for brick spawning, collision resolution, and shared signals; add a small particle-effects module that owns the Hanabi effect resource and burst spawning.

## Complexity Tracking

No constitution violations require justification.
