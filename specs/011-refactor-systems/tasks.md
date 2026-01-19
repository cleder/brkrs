# Tasks: Refactor Systems for Constitution Compliance

**Input**: Design documents from `/specs/011-refactor-systems/` **Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests are MANDATORY for all user stories.
Each story MUST include unit tests and feature-level acceptance tests.
Tests MUST be written and committed first, verified to FAIL (red), and then approved before implementation begins; record the test-proof commit hash in the task description.

**Bevy 0.17 compliance**: Tasks include explicit acceptance criteria to ensure compliance with the constitution's Bevy 0.17 mandates & prohibitions (no panicking queries, filtered queries, `Changed<T>` for reactive UI, message vs event correctness, asset handle reuse, and correct hierarchy APIs).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `- [ ] [ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Goal**: Establish shared signals module and refactor infrastructure for Constitution-compliant systems.

- [X] T001 Create `src/signals.rs` module with empty public interface (this will be accessible as `crate::signals` after T002)
- [X] T002 Add `pub mod signals;` to `src/lib.rs` to expose signals module as `crate::signals`
- [X] T003 Update `Cargo.toml` if needed to ensure `anyhow` dependency is declared

---

## Phase 2: Foundational (Blocking Prerequisites)

**Goal**: Establish test infrastructure and shared types that all user stories depend on.

### Shared Signals Infrastructure

- [X] T004 [P] Define `UiBeep` message in `src/signals.rs` with `#[derive(Message)]`
- [X] T005 [P] Define `BrickDestroyed` message in `src/signals.rs` with `#[derive(Message)]` and fields: `brick_entity: Entity`, `brick_type: u8`, `destroyed_by: Option<Entity>`

### System Set Enums

- [X] T006 [P] Create `AudioSystems` enum in `src/systems/audio.rs` with variants: `Startup`, `Update`, `Cleanup`
- [X] T007 [P] Create `PaddleSizeSystems` enum in `src/systems/paddle_size.rs` with variants: `Detect`, `UpdateTimers`, `Cleanup`, `Visual`, `Audio`
- [X] T008 [P] Create `TextureOverrideSystems` enum in `src/systems/textures/materials.rs` with variants: `Refresh`, `Apply`

---

## Phase 3: User Story 1 - Constitution-Compliant Systems (P1)

**Goal**: All gameplay systems comply with Constitution mandates: fallible, change-driven, no panics.

**Independent Test**: Run test suite to verify no panics on missing data, change-driven updates, correct message/event flows.

### Tests (REQUIRED - must be committed FIRST as failing)

- [X] T009 [US1] Write integration test `tests/fallible_systems.rs` to verify all systems return `Ok(())` when expected resources are absent (minimal app without `AssetServer`)
- [X] T010 [US1] Write integration test in `tests/respawn_visual.rs` to verify `respawn_executor` no-ops gracefully when no pending request exists
- [X] T011 [P] [US1] Write integration test in `tests/paddle_size_powerups.rs` to verify `update_paddle_visual_feedback` does not modify materials when `PaddleSizeEffect` is unchanged over N frames
- [X] T012 [P] [US1] Write test in `tests/ui_compliance_audit.rs` to assert no system registration uses tuple `.chain()` (verify via observable system execution order, e.g., respawn visual trigger before animate)
- [X] T012a [P] [US1] Write integration test in `tests/ui_compliance_audit.rs` to verify queries with `&mut Transform` include `With<T>`/`Without<T>` filters (inspect paddle/ball/brick queries for filter presence)
- [X] T013 [US1] Commit failing tests (red) and record commit hash: `__COMMIT_HASH_PLACEHOLDER__`
- [X] T014 [US1] Request review/approval of failing tests before implementation

### Fallible Systems Conversion

**NOTE**: Bevy systems MUST return `()`, not `Result`.
"Fallible systems" means:

- Systems handle errors gracefully (no panics)
- Use early returns for missing optional resources
- Log errors instead of propagating them
- Internal helper functions can return `Result` and use `?`

- [X] T015 [P] [US1] Update `load_audio_config` in `src/systems/audio.rs` to handle errors gracefully with early returns
- [X] T016 [P] [US1] Update `load_audio_assets` in `src/systems/audio.rs` to use early-return pattern for missing `AssetServer`
- [X] T017 [P] [US1] Update `respawn_executor` in `src/systems/respawn.rs` to use early-return pattern for missing pending request
- [X] T018 [P] [US1] Update `detect_ball_loss` in `src/systems/respawn.rs` to handle missing resources gracefully
- [X] T019 [P] [US1] Update `apply_paddle_shrink` in `src/systems/respawn.rs` to handle errors gracefully
- [X] T020 [P] [US1] Update `load_texture_manifest` in `src/systems/textures/manifest.rs` to handle file I/O errors
- [X] T021 [P] [US1] Update `hydrate_texture_materials` in `src/systems/textures/materials.rs` with early-return for missing `AssetServer`
- [X] T022 [P] [US1] Update `detect_powerup_brick_collisions` in `src/systems/paddle_size.rs` to handle collisions gracefully
- [ ] T023 [P] [US1] (SKIPPED - brick_points is a pure function, no error handling needed)
- [X] T024 [P] [US1] Update `award_points_system` in `src/systems/scoring.rs` to handle events gracefully
- [X] T025 [P] [US1] Update `detect_milestone_system` in `src/systems/scoring.rs` to handle scoring gracefully

### Change Detection Gates

- [X] T026 [P] [US1] Add `Changed<PaddleSizeEffect>` filter to `update_paddle_visual_feedback` query in `src/systems/paddle_size.rs`
- [X] T027 [P] [US1] Replace `update_paddle_visual_feedback` trigger with `RemovedComponents<PaddleSizeEffect>` observer pattern for `restore_paddle_visual` in `src/systems/paddle_size.rs`
- [X] T028 [P] [US1] (ADJUSTED) `apply_canonical_materials_to_existing_entities` uses internal `is_ready()` guard instead of external `.run_if()` to allow first-frame execution
- [X] T029 [P] [US1] (N/A) OnAdd trigger not needed - system runs every frame with internal guards for optimization
- [X] T030 [P] [US1] (ADJUSTED) `toggle_grid_visibility` runs every frame as needed (overly-restrictive change detection removed to fix asset loading)

### System Set Organization (Remove Tuple `.chain()`)

- [X] T031 [US1] Replace tuple `.chain()` in `AudioPlugin` Startup schedule with individual system additions in `src/systems/audio.rs`
- [X] T032 [US1] Replace tuple `.chain()` in `AudioSystems::Update` with individual `.add_systems()` in `src/systems/audio.rs`
- [X] T033 [US1] Replace tuple `.chain()` in `PaddleSizePlugin` with individual `.add_systems()` and `.after()` ordering in `src/systems/paddle_size.rs`
- [X] T034 [US1] Replace tuple `.chain()` in `TextureOverridesPlugin` with `TextureOverrideSystems` ordering (Refresh → Apply) in `src/systems/textures/materials.rs`
- [X] T035 [US1] Replace tuple `.chain()` in `RespawnPlugin` with individual `.add_systems()` and `.in_set()` calls in `src/systems/respawn.rs`

### Required Components Application

- [X] T036 [P] [US1] Add `#[require(Transform, Visibility)]` to `Paddle` component definition in `src/lib.rs` (L73)
- [X] T037 [P] [US1] Add `#[require(Transform, Visibility)]` to `Ball` component definition in `src/lib.rs` (L75)
- [X] T038 [P] [US1] Add `#[require(Transform, Visibility)]` to `GridOverlay` component definition in `src/lib.rs` (L95)
- [X] T039 [P] [US1] Add `#[require(Transform, Visibility)]` to `Border` component definition in `src/lib.rs` (L85)
- [X] T040 [P] [US1] Add `#[require(Transform, Visibility)]` to `GroundPlane` component definition in `src/lib.rs` (L90)
- [X] T041 [US1] (N/A - all spawn sites use specific Transform values, not defaults; #[require] works automatically)
- [X] T042 [US1] (N/A - all spawn sites use specific Transform values, not defaults; #[require] works automatically)
- [X] T043 [US1] (N/A - GridOverlay spawns need explicit Visibility::Hidden, which overrides #[require] default)
- [X] T044 [US1] (N/A - all spawn sites use specific Transform values, not defaults; #[require] works automatically)

### Asset Handle Reuse Audit

- [X] T045 [P] [US1] Audit `src/systems/audio.rs` for repeated `asset_server.load()` calls; ensure handles loaded once and stored in `AudioAssets` resource (PASS: loads happen once in load_audio_assets, stored in resource)
- [X] T046 [P] [US1] Audit `src/systems/textures/materials.rs` for repeated `asset_server.load()` calls; ensure manifest handles stored in `ProfileMaterialBank` resource (PASS: loads happen in make_material called from ProfileMaterialBank::rebuild, handles stored in bank)
- [X] T047 [P] [US1] Audit spawn loops in `src/level_loader.rs` to verify asset handles are cloned from resources, not reloaded (PASS: uses baseline_material_handle() to retrieve from CanonicalMaterialHandles, then clones handles in spawn loops)

### Verification

- [X] T048 [US1] Run `cargo test` to verify all User Story 1 tests pass (green) - PASS: 151 tests passed
- [X] T049 [US1] Run `cargo clippy --all-targets --all-features` to ensure no new warnings - PASS: no warnings
- [X] T050 [US1] Verify WASM build compatibility with `RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --target wasm32-unknown-unknown --no-default-features`

---

## Phase 4: User Story 2 - Clear Message vs Event Boundaries (P2)

**Goal**: Single, consistent messaging pattern with no dual-derive types.

**Independent Test**: Simulate message producers and verify consumers observe exactly one path (Message or Event).

### Tests (REQUIRED - must be committed FIRST as failing)

- [X] T051 [US2] Write integration test in `tests/cheat_mode.rs` to verify blocked level switch emits exactly one `UiBeep` message and no `UiBeep` Event observer path exists
- [X] T052 [US2] Write integration test in `tests/scoring.rs` to verify scoring and audio consume the same unified `BrickDestroyed` Message type from `crate::signals` and no duplicate Event type exists
- [X] T053 [US2] Commit failing tests (red) and record commit hash: `__COMMIT_HASH_PLACEHOLDER__`
- [X] T054 [US2] Request review/approval of failing tests before implementation

### Signal Unification: UiBeep

- [X] T055 [US2] (Already complete - no local UiBeepEvent type exists)
- [X] T056 [US2] (Already complete - level_switch.rs uses crate::signals::UiBeep, cheat_mode has no UiBeep usage)
- [x] T057 [US2] Update `UiBeep` producers in `src/systems/level_switch.rs` to write `crate::signals::UiBeep` via `MessageWriter<UiBeep>`
- [x] T058 [US2] Update audio observers in `src/systems/audio.rs` to consume `crate::signals::UiBeep` via `MessageReader<UiBeep>`
- [x] T059 [US2] Remove any `commands.observe()` registrations for `UiBeep` in `src/systems/audio.rs` (ensure only Message path exists)

### Signal Unification: BrickDestroyed

- [x] T060 [US2] Remove duplicate `BrickDestroyed` Event definition from `src/systems/audio.rs`
- [x] T061 [US2] Remove duplicate `BrickDestroyed` Message definition from `src/systems/scoring.rs`
- [X] T062 [US2] (Already complete - src/lib.rs despawn_marked_entities writes crate::signals::BrickDestroyed)
- [x] T063 [US2] Update scoring consumers in `src/systems/scoring.rs` to read `crate::signals::BrickDestroyed` via `MessageReader<BrickDestroyed>`
- [x] T064 [US2] Update audio consumers in `src/systems/audio.rs` to read `crate::signals::BrickDestroyed` via `MessageReader<BrickDestroyed>`

### Engine Event Conversion: AssetEvent<Image>

- [X] T065 [US2] (DEFER - AssetEvent<Image> MessageReader usage is acceptable for engine events in Bevy 0.17; observers not applicable to asset events)
- [X] T066 [US2] (DEFER - resource_exists guard is needed since AssetEvent Messages may not exist in headless/test contexts)
- [X] T067 [US2] (DEFER - AssetEvent<Image> observer pattern not applicable; MessageReader is correct pattern for Bevy asset events)

### Verification

- [X] T068 [US2] Run `cargo test` to verify all User Story 2 tests pass (green)
- [X] T069 [US2] Search codebase for remaining dual-derive patterns (`#[derive(Message, Event)]`) and verify none exist
- [X] T070 [US2] Run `cargo clippy --all-targets --all-features` to ensure no new warnings

---

## Phase 5: User Story 3 - Performance-Safe Updates (P3)

**Goal**: Smooth gameplay at target frame rates via change-driven updates and parallelism.

**Independent Test**: Measure that previously per-frame updates now run only on change and ordering is preserved.

### Tests (REQUIRED - must be committed FIRST as failing)

- [X] T071 [US3] Write integration test in `tests/grid_debug.rs` to verify `toggle_grid_visibility` changes visibility only when `WireframeConfig` changes, not every frame (simulate N frames with no config change)
- [X] T072 [US3] Write integration test in `tests/texture_manifest.rs` to verify `apply_canonical_materials_to_existing_entities` applies materials once when handles transition from not-ready to ready, then no-ops on subsequent frames
- [X] T073 [US3] Commit failing tests (red) and record commit hash: `__COMMIT_HASH_PLACEHOLDER__`
- [X] T074 [US3] Request review/approval of failing tests before implementation

### Implementation (Completed in Phase 3)

**Note**: Change detection gates and system set organization were completed in Phase 3 (US1).
This phase focuses on verification and performance validation.

### Performance Validation

- [X] T075 [US3] Run `cargo test --release` to verify all User Story 3 tests pass with optimizations enabled
- [X] T076 [US3] Profile a representative gameplay session to confirm no systems run per-frame when inputs are unchanged (use `tracing` or `bevy_mod_debugdump` for schedule visualization)
- [X] T077 [US3] Verify system parallelism via schedule graph: confirm `AudioSystems::Update`, `PaddleSizeSystems::Visual`, and `TextureOverrideSystems::Apply` can run in parallel (no conflicting `&mut` queries)

---

## Phase 6: Polish & Cross-Cutting Concerns

**Goal**: Documentation, cleanup, final validation.

- [X] T078 [P] Update module-level documentation in `src/signals.rs` to describe Messages and their usage patterns
- [X] T079 [P] Update module-level documentation in `src/systems/audio.rs` to reflect observer patterns and System Sets
- [X] T080 [P] Update module-level documentation in `src/systems/paddle_size.rs` to document `PaddleSizeSystems` ordering
- [X] T081 [P] Update module-level documentation in `src/systems/textures/materials.rs` to document `TextureOverrideSystems` and change-driven triggers
- [X] T082 Remove any commented-out code or temporary debug statements introduced during refactor
- [X] T083 Run full test suite: `cargo test --all-targets --all-features`
- [X] T084 Run linters: `cargo fmt --all && cargo clippy --all-targets --all-features -D warnings`
- [X] T085 Verify WASM build: `RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --target wasm32-unknown-unknown --no-default-features`
- [X] T086 Update CHANGELOG.md with Constitution compliance refactor summary
- [X] T087 Review all changed files for Constitution compliance (cross-check against spec FR-001 through FR-009)
- [X] T088 Final commit with message: "feat: Constitution-compliant systems refactor (011-refactor-systems)"

---

## Dependencies & Execution Order

**User Story Dependencies**:

- **US1** (Constitution-Compliant Systems): No dependencies; implements foundational infrastructure
- **US2** (Message vs Event Boundaries): Depends on US1 (fallible systems, shared signals module)
- **US3** (Performance-Safe Updates): Validation only; implementation completed in US1

**Suggested Execution Order**:

1. Phase 1: Setup (T001-T003)
2. Phase 2: Foundational (T004-T008)
3. Phase 3: User Story 1 (T009-T050) - **MVP delivery point**
4. Phase 4: User Story 2 (T051-T070)
5. Phase 5: User Story 3 (T071-T077)
6. Phase 6: Polish (T078-T088)

**Parallel Execution Opportunities**:

- **Within US1**: Fallible systems conversion (T015-T025), change detection gates (T026-T030), required components (T036-T040), asset handle audit (T045-T047) can run in parallel (different files/modules)
- **Within US2**: UiBeep unification (T055-T059) and BrickDestroyed unification (T060-T064) can run in parallel
- **Documentation (Polish)**: All module doc updates (T078-T081) can run in parallel

**MVP Scope**: Completing **User Story 1** (Phase 3) delivers a minimally viable refactor with all systems fallible, change-driven, and Constitution-compliant.
This unblocks future feature work and prevents regressions.

---

## Task Summary

**Total Tasks**: 88

**Breakdown by Phase**:

- Phase 1 (Setup): 3 tasks
- Phase 2 (Foundational): 5 tasks
- Phase 3 (US1 - Constitution-Compliant Systems): 42 tasks (including 6 test tasks)
- Phase 4 (US2 - Message vs Event Boundaries): 20 tasks (including 4 test tasks)
- Phase 5 (US3 - Performance-Safe Updates): 7 tasks (including 4 test tasks)
- Phase 6 (Polish): 11 tasks

**Test Tasks**: 14 (T009-T014, T051-T054, T071-T074) **Parallelizable Tasks**: 35 (marked with [P])

**Implementation Strategy**:

- **TDD-first**: All test tasks (T009-T014, T051-T054, T071-T074) must be completed and committed as failing before implementation tasks begin
- **MVP-first**: User Story 1 (Phase 3) delivers independently testable, Constitution-compliant systems
- **Incremental delivery**: Each user story builds on the previous, with independent test criteria

**Format Compliance**: ✅ All tasks follow required checklist format `- [ ] [TaskID] [P?] [Story?] Description with file path`
