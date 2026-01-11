# Tasks: Gravity Indicator UI (021-gravity-bricks)

Generated: 2026-01-11 Feature: specs/021-gravity-bricks/spec.md Plan: specs/021-gravity-bricks/plan.md

## Phase 1: Setup

Goal: Establish project-prep and scaffolding to enable TDD-first implementation.

- [ ] T001 Verify texture assets exist in assets/textures/default/
- [ ] T002 Create test harness file scaffolding in tests/gravity_indicator_ui.rs
- [ ] T003 Ensure UI module path and plugin registration in src/ui/mod.rs
- [ ] T004 Add logging category for UI indicator in src/lib.rs (tracing init)

## Phase 2: Foundational

Goal: Shared prerequisites blocking all user stories.

- [ ] T005 Add resource `GravityIndicatorTextures` and loader in src/ui/gravity_indicator.rs
- [ ] T006 Ensure `GravityConfiguration` change detection is active (is_changed usage) in src/lib.rs
- [ ] T007 Add `GravityIndicator` marker component in src/ui/gravity_indicator.rs
- [ ] T008 Register spawn/update systems in UiPlugin in src/ui/mod.rs

## Phase 3: User Story 1 (P1) — See Current Gravity

Story Goal: Players can see the current gravity level at a glance.
Independent Test Criteria: Indicator spawns correctly and updates within one frame when gravity changes.

### Tests (write first; commit failing tests and record hash)

- [ ] T009 [US1] Add test `test_indicator_spawns_on_first_valid_frame` in tests/gravity_indicator_ui.rs (hash: TBD)
- [ ] T010 [US1] Add test `test_indicator_updates_within_one_frame_on_gravity_change` in tests/gravity_indicator_ui.rs (hash: TBD)
- [ ] T011 [US1] Add test `test_multiple_changes_last_write_wins` in tests/gravity_indicator_ui.rs (hash: TBD)
- [ ] T012 [US1] Add unit tests for `map_gravity_to_level` (exact and tolerance edges) in src/ui/gravity_indicator.rs (hash: TBD)
- [ ] T013 [US1] Record failing-test commit hash to specs/021-gravity-bricks/US1_FAILING_TEST_HASH.txt

### Implementation

- [ ] T014 [US1] Implement `map_gravity_to_level(Vec3)` in src/ui/gravity_indicator.rs
- [ ] T015 [US1] Implement `select_texture(GravityLevel)` in src/ui/gravity_indicator.rs
- [ ] T016 [US1] Implement `spawn_gravity_indicator()` with idempotent guard in src/ui/gravity_indicator.rs
- [ ] T017 [US1] Implement `update_gravity_indicator()` gated by `is_changed()` in src/ui/gravity_indicator.rs
- [ ] T018 [US1] Add asset load deferral and graceful skip (warn) in src/ui/gravity_indicator.rs

### Bevy 0.17 Gates — Acceptance

- [ ] T019 [US1] Verify no per-frame UI updates (Changed<GravityConfiguration> gate) in src/ui/gravity_indicator.rs
- [ ] T020 [US1] Verify specific queries (With<GravityIndicator>) and no panicking unwraps in src/ui/gravity_indicator.rs
- [ ] T021 [US1] Verify asset handle reuse from resource (no repeated loads) in src/ui/gravity_indicator.rs

## Phase 4: User Story 2 (P2) — Non-Intrusive Placement

Story Goal: Indicator appears bottom-left with 12px offsets and never overlaps developer indicator.
Independent Test Criteria: Position correct across window modes; opposite corner from developer indicator.

### Tests (write first; commit failing tests and record hash)

- [ ] T022 [US2] Add test `test_indicator_bottom_left_anchor_windowed_and_fullscreen` in tests/gravity_indicator_ui.rs (hash: TBD)
- [ ] T023 [US2] Add test `test_indicator_opposite_developer_indicator_no_overlap` in tests/gravity_indicator_ui.rs (hash: TBD)
- [ ] T024 [US2] Record failing-test commit hash to specs/021-gravity-bricks/US2_FAILING_TEST_HASH.txt

### Implementation

- [ ] T025 [US2] Implement absolute positioning (left/bottom 12px) in Node for spawn system in src/ui/gravity_indicator.rs
- [ ] T026 [US2] Validate developer indicator is bottom-right; adjust gravity indicator if layout changes in src/ui/gravity_indicator.rs

### Bevy 0.17 Gates — Acceptance

- [ ] T027 [US2] Verify hierarchy safety (no parent/child needed) and stable anchoring in src/ui/gravity_indicator.rs

### Overlay Visibility (FR-007)

- [ ] T041 [US2] Add test `test_indicator_visible_over_game_over_overlay` in tests/gravity_indicator_ui.rs (hash: TBD)
- [ ] T042 [US2] Ensure z-order/layering keeps indicator above overlays (e.g., UI z-index/order), document approach in src/ui/gravity_indicator.rs

## Phase 5: User Story 3 (P3) — Robust Through Pause & Life Loss

Story Goal: Indicator remains correct during pause and after life loss gravity reset.
Independent Test Criteria: Stability across pause; reflects level default after life loss.

### Tests (write first; commit failing tests and record hash)

- [ ] T028 [US3] Add test `test_indicator_static_during_pause` in tests/gravity_indicator_ui.rs (hash: TBD)
- [ ] T029 [US3] Add test `test_indicator_resets_to_level_default_on_life_loss` in tests/gravity_indicator_ui.rs (hash: TBD)
- [ ] T030 [US3] Add test `test_multi_frame_persistence_for_10_frames` in tests/gravity_indicator_ui.rs (hash: TBD)
- [ ] T031 [US3] Record failing-test commit hash to specs/021-gravity-bricks/US3_FAILING_TEST_HASH.txt

### Implementation

- [ ] T032 [US3] Ensure pause system doesn't mutate indicator; validate change detection only triggers when gravity changes in src/ui/gravity_indicator.rs
- [ ] T033 [US3] Hook life loss handler to update `GravityConfiguration.current` and rely on change detection in src/lib.rs

### Bevy 0.17 Gates — Acceptance

- [ ] T034 [US3] Verify change detection gates and multi-frame persistence; no unconditional overwrites in src/ui/gravity_indicator.rs

## Final Phase: Polish & Cross-Cutting

Goal: Stabilize, document, and verify compliance.

- [ ] T035 Add README entry in docs/ui-systems.md for gravity indicator
- [ ] T036 Add troubleshooting section to docs/troubleshooting.md (asset missing, tolerance edge cases)
- [ ] T037 Add performance note in docs/architecture.md (change detection efficiency)
- [ ] T038 Add accessibility/DPI note to docs/developer-guide.md (future support)
- [ ] T039 Run linters and formatters: cargo test, cargo clippy, cargo fmt --all
- [ ] T040 Update CHANGELOG.md and IMPLEMENTATION_SUMMARY.md with feature details

## Dependencies

- US1 → US2 → US3 (placement depends on spawn; robustness depends on correct update behavior)

## Parallel Execution Examples

- [P] T012 with T009–T011 (unit tests can be written in parallel to integration tests)
- [P] T014–T018 blocked by failing tests; within implementation, `select_texture` (T015) can be parallel to `map_gravity_to_level` (T014)
- [P] T022 and T023 can be authored in parallel
- [P] T028–T030 can be authored in parallel; life loss test may require existing pause test harness

## Implementation Strategy

- MVP: Complete US1 (spawn + update + mapping) with tests-first and Bevy 0.17 gates satisfied
- Incremental delivery: US2 (placement), US3 (robustness)
