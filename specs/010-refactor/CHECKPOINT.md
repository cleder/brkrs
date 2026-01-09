# Checkpoint: UI Constitution Refactor (010-refactor) — Session 2025-12-19

**Status**: Phase 3 (User Story 2) — Ready to implement

## Completed Work

### ✅ Phase 1: User Story 1 — Compliance Audit (COMPLETE)

- **T005**: Audit-artifact test created and committed
  - Red commit: `43835344ee5fb4f0f6cabdc8f5fa7b4bcf94bf13`
  - Test: `tests/ui_compliance_audit.rs` with 5 test cases
  - Validates: all `src/ui/*.rs` referenced, Plugin-Based Architecture + System Organization findings present

- **T006**: Approval gate (approved 2025-12-19)
  - Maintainer approved red test state

- **T007–T008**: Compliance audit completed
  - Updated `specs/010-refactor/compliance-audit.md` with Plugin-Based Architecture + System Organization findings
  - Green commit: `255d74e`
  - All 5 audit tests **PASS** ✓

### ✅ Phase 2: Foundation Infrastructure (COMPLETE)

- **T001–T004**:
  - T001: Feature docs verified present (spec.md, plan.md, compliance-audit.md, refactoring-plan.md)
  - T002: Quickstart guide created (`specs/010-refactor/quickstart.md`)
  - T003: `UiSystemError` type added to `src/ui/mod.rs`
    - Enum with variants: EntityNotFound, ResourceNotFound, AssetNotAvailable, SetupError, Other
    - Implements Display + Error traits
  - T004: Query failure policies documented in `docs/ui-systems.md`
    - Patterns: Required single entity, Optional entity, Required resource, Multiple entities handling
    - Code examples for each pattern
  - Commit: `17c5edd`

### ✅ Phase 3: User Story 2 — Tests Created (READY FOR IMPLEMENTATION)

- **T009–T011**: Documentation tests created
  - T009: `tests/ui_palette_change_detection.rs` — Change-driven feedback patterns
  - T010: `tests/ui_cheat_indicator_caching.rs` — Asset caching patterns
  - T011: `tests/ui_fallible_systems.rs` — Result-returning system patterns
  - Commit: `f1dc84dcd4b3e7e555d3df30ae489d6b762327d9`
  - All tests currently **PASS** (documentation/placeholder implementations)

- **T012**: Approval gate (approved 2025-12-19)
  - User approved proceeding with implementation

## Current Branch State

**Branch**: `010-refactor` **Commits**: 6 feature commits + 1 foundation commit = **7 total** **Test Status**: All existing tests pass, all audit tests pass

```text
git log --oneline (last 7):
664c8de Record T009–T011 test file commits and prepare T012 approval gate
f1dc84d RED/Documentation: Add US2 test files (T009–T011)
17c5edd Complete Phase 1 & 2 setup: Foundation ready (T001–T004)
55c404e Mark T005–T008 complete: User Story 1 (Compliance Audit) ✓
255d74e GREEN: Complete compliance audit with Plugin-Based Architecture and System Organization findings
43835344 RED: Add failing audit-artifact compliance test (T005)
```

## Ready to Implement: T013–T021 (User Story 2)

### Scope: Refactor UI Code into Constitution Compliance

**Architecture Changes Required**:

1. **T013**: Convert all UI systems to return `Result<(), UiSystemError>` (7 files)
2. **T014**: Replace panicking query patterns with safe error recovery
3. **T015**: Implement change-driven palette feedback (`Changed<SelectedBrick>`, `Added<PalettePreview>`)
4. **T016**: Cache ghost preview materials; avoid per-frame allocations
5. **T017**: Introduce cached cheat indicator asset handle resource
6. **T018**: Add `#[require(Transform, Visibility)]` to marker components
7. **T019**: Fill rustdoc gaps for public items
8. **T020**: Implement self-contained UI plugin architecture
9. **T021**: Organize systems into system sets with `.configure_sets()`

### Implementation Files to Modify

- `src/ui/score_display.rs` ✓ (type signature ready)
- `src/ui/lives_counter.rs`
- `src/ui/level_label.rs`
- `src/ui/game_over_overlay.rs`
- `src/ui/pause_overlay.rs`
- `src/ui/cheat_indicator.rs`
- `src/ui/palette.rs` (largest refactor)
- `src/ui/fonts.rs` (startup systems)
- `src/lib.rs` (system registration, plugin architecture)
- `src/main.rs` (optional: UI plugin usage)

### Key Integration Points

- All UI systems must be registered via a `UiPlugin` struct
- System sets: `UiSystems::Spawn`, `UiSystems::Update`, `UiSystems::Input`
- Error handling: Systems return Result; app logs diagnostics and reschedules on failure

### Estimated Effort

- T013–T014: 1.5 hours (system conversion + registration)
- T015–T016: 1 hour (palette refactoring)
- T017–T018: 30 min (asset caching, required components)
- T019–T021: 1.5 hours (rustdoc, plugin, system sets)
- **Total**: ~4–5 hours for intensive implementation session

## Next Actions

1. Continue with T013: Convert systems to return Result
2. Update lib.rs system registration to handle Result
3. Implement T014–T021 per task order
4. Mark tasks complete as each is finished
5. User Story 3 (T022–T026): Behavior preservation tests
6. Polish (T027–T030): Code quality checks

## Notes for Implementer

- All UI systems should follow the query failure patterns documented in `docs/ui-systems.md`
- Use `?` operator for error propagation in Result-returning systems
- Avoid `.unwrap()`, `.expect()`, `.single()`, `.single_mut()` — use `.get_single()`, `.get_single_mut()`
- Change detection: Use `Changed<T>`, `Added<T>`, `Removed<T>` filters instead of per-frame work
- Asset caching: Load once in startup, store in Resource, reuse handles
- Required components: Use `#[require(Transform, Visibility)]` on marker components for 3D entities
- System sets: Group related systems (spawn, update, input) into named sets for reusability

---

**Ready to begin intensive implementation phase?**
   **Proceed with T013–T021.**
