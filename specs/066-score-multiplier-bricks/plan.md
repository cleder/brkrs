# Implementation Plan: Score Multiplier Bricks

**Branch**: `066-score-multiplier-bricks` | **Date**: 2026-05-30 | **Spec**: `/specs/066-score-multiplier-bricks/spec.md`
**Input**: Feature specification from `/specs/066-score-multiplier-bricks/spec.md`

## Summary

Add multiplier brick behavior for indices 26-29 so future brick-destruction scoring uses 1x/2x/3x/4x based on the latest hit multiplier brick.
Multiplier activation is forward-only (no retroactive scoring), resets only when lives actually decrement, persists through level transitions when lives are unchanged, and applies only to brick-destruction score sources.
Implementation will extend current scoring flow with a dedicated multiplier state resource, a score UI indicator shown beneath the score for `x2`/`x3`/`x4` and hidden at `1x`, plus message-driven reset/update logic validated by multi-frame persistence tests.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1 **Storage**: N/A (in-memory ECS state only) **Testing**: `cargo test` integration + unit tests (TDD-first) **Target Platform**: Native desktop (Linux/macOS/Windows) and WASM **Project Type**: Single Rust game project (ECS Bevy app) **Performance Goals**: Preserve 60 FPS gameplay behavior and no measurable frame-time regression from scoring logic **Constraints**: Message/Event separation compliance, no per-frame overwrite of runtime scoring state, fallible/non-panicking query patterns, UI updates gated by change detection rather than unconditional per-frame writes **Scale/Scope**: 4 new multiplier brick behaviors (26-29), single scoring pipeline, multi-ball + level-transition persistence paths

## Constitution Check

*GATE: Must pass before Phase 0 research.*
*Re-check after Phase 1 design.*

This check MUST verify compliance with the constitution, including **Test-Driven Development (TDD)** gates:

- Tests are defined and committed prior to implementation efforts for each story/feature.
- A proof-of-failure commit (tests that FAIL) MUST exist in the branch history prior to implementation commits.
- Tests MUST be reviewed and approved by the feature owner or requestor before implementation begins.

This check MUST also verify compliance with **Bevy 0.17 mandates & prohibitions** (if the feature touches ECS, rendering, assets, or scheduling):

- **Bevy Event System Guidance:**
  - For any feature using events, messages, or observers, the plan MUST explicitly state which system is used (Messages vs Observers) and why, referencing the constitution's "Bevy 0.17 Event, Message, and Observer Clarification" section.
  - Justify the choice (e.g., "Messages for batchable, cross-frame work; Observers for immediate, reactive logic").
- **Coordinate System Guidance (if feature involves spatial movement/physics):**
  - Plan MUST specify which axes are used for movement (XZ plane for horizontal, Y for vertical, etc.).
  - Clarify whether directional terms (forward/backward) refer to Bevy's Transform API (-Z forward), gameplay perspective, or direct axis references.
  - Document any `LockedAxes` constraints and their relationship to camera orientation.
- **Initialization System Idempotence (if feature has loader/initializer systems in Update):**
  - Systems that initialize or load state in `Update` schedule MUST be idempotent.
  - Use a guard field (e.g., `last_level_number: Option<u32>`) to track whether initialization has occurred.
  - ONLY perform initialization when context changes (e.g., level transition), NOT every frame.
  - This prevents runtime state changes (e.g., gravity from brick destruction) from being overwritten.
  - See 020-gravity-bricks retrospective for the bug this pattern prevents.
- **Multi-Frame Persistence Testing (if feature modifies runtime state):**
  - Tests for runtime state changes MUST verify persistence across multiple `app.update()` cycles.
  - Tests MUST include ALL systems that write to the affected resource to catch per-frame overwrites.
  - Minimum 10 frames of persistence checking recommended.
- Systems are fallible (`Result`) and do not panic on query outcomes (`?`, no `.unwrap()` on queries).
- Queries use `With<T>`/`Without<T>` filters and `Changed<T>` where appropriate (especially UI).
- **Message-Event Separation**: Verify correct use of `MessageWriter/Reader` for buffered, frame-agnostic streams and observers/`Trigger<T>` for immediate, reactive logic (e.g., UI/sound triggers).
- Assets are loaded once and handles are stored in Resources (no repeated `asset_server.load()` in loops).
- Hierarchies use `ChildOf::parent()` and `add_children()`/`remove::<Children>()` patterns.

Pre-Phase 0 Gate Review:

- TDD-first gate: PASS (plan requires red-first tests for all stories).
- Bevy Messages vs Observers decision: PASS.
  Use Messages for buffered score/life stream updates; no observer-only scoring path.
- Coordinate-system gate: N/A (no directional or physics-axis behavior added).
- Initialization idempotence gate: PASS.
  Multiplier state must not be reinitialized every frame; only changed by explicit events.
- Multi-frame persistence gate: PASS.
  Tests will include >=10 `app.update()` cycles with all multiplier writers active.
- Fallibility/query safety gate: PASS (no new panicking query usage planned).
- Hierarchy safety gate: PASS (feature does not mutate entity hierarchy).
- UI change-detection gate: PASS.
  Multiplier indicator updates will be driven only by multiplier-state changes.

Post-Phase 1 Gate Re-check:

- Design artifacts maintain all pre-gates with no violations.
- No constitution violations requiring complexity exceptions.

## Project Structure

### Documentation (this feature)

```text
specs/066-score-multiplier-bricks/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── gameplay-scoring-contract.md
└── tasks.md
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
src/
├── lib.rs
├── main.rs
├── signals.rs
├── game_state.rs
├── systems/
│   ├── scoring.rs
│   ├── game_state_transitions.rs
│   └── ...
└── ui/
  ├── mod.rs
  └── score_display.rs

tests/
├── scoring.rs
├── score_display.rs
├── change_detection.rs
├── life_loss_flow.rs
├── score_multiplier_bricks.rs
└── ...

docs/
└── bricks.md
```

**Structure Decision**: Use the existing single-project Bevy ECS layout.
Implement multiplier behavior in existing scoring/life systems, extend the existing score UI to render a multiplier indicator beneath it, and add focused integration tests under `tests/`.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |
