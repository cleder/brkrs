# Tasks: Display Current Level (Phase 2 planning placeholders)

**Feature**: Display Current Level (`001-display-current-level`) **Created**: 2025-12-18

## Implementation tasks (suggested)

1. Add `LevelHud` systems and components
   - Path: `src/systems/hud/level_hud.rs`
   - Create component `LevelLabel` and system to update it on `LevelStarted` events
   - Add unit tests for component update

2. Accessibility helpers
   - Path: `src/systems/hud/accessibility.rs`
   - Implement cross-platform announcement utility (polite live region on WASM, native accessibility API wrappers)
   - Add tests or manual verification steps

3. Pause/Summary integration
   - Ensure pause/summary UI uses `PlayerProgress` (if present) to render final progress
   - Add integration test to assert `PauseRequested` shows summary with progress

4. Styling and design review
   - Add placeholder CSS/asset hooks for localization and scaling
   - Get design sign-off on top-center placement and font sizing

5. Cross-platform validation
   - Run WASM build and validate HUD and accessibility behavior
   - Profile for performance regressions

6. Documentation & quickstart
   - Update README/quickstart steps if run commands differ
   - Add developer note describing how to test accessibility announcement

## Acceptance criteria (implementation)

- Level label appears within 1 second on level start
- No in-play progress metrics are shown during active gameplay
- Pause/summary shows final progress when available
- Accessibility announcements work on supported targets
- Tests and manual checks are added to cover above

---

*Assign tasks to issues/PRs and iterate.*
