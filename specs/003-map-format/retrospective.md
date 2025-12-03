# Retrospective: Map Format Migration (22x22 → 20x20)

## Summary

Grid and level matrix migrated from 22x22 to 20x20 across code, assets, and documentation.
Four user stories plus polish phase delivered; success criteria SC-001..SC-010 all verified.
Final state is stable for native and WASM builds.

## What Went Well

- Incremental phase checkpoints prevented regressions and provided clear stop points.
- Two-stage unfreeze (BallFrozen removal then physics wake with micro impulse) cleanly resolved post-transition inert ball bug.
- Backward compatibility warnings (dimension + per-row truncation) greatly improved developer feedback for legacy files.
- Documentation (quickstart, spec, tasks) stayed synchronized with implementation—low cognitive load for new contributors.
- Early WASM build validation prevented late cross-compilation surprises.

## Key Technical Insights

- Bevy ECS deferred command application requires multi-frame activation patterns for physics-sensitive mutations.
- Rapier ball required an explicit wake (tiny ExternalImpulse) after Velocity cleanup to avoid hidden sleep state.
- Normalization function acts as a graceful migration layer—preserves leading data, logs truncation/padding decisions.

## Challenges & Resolutions

| Challenge | Cause | Resolution | Impact |
|-----------|-------|-----------|--------|
| Ball remained inert after transition | Velocity + sleep state persisted after marker removal | Staged unfreeze + micro impulse | Stable post-transition physics |
| Legacy level silent mismatch | No dimension validation pre-migration | Added normalize_matrix warnings | Easier legacy asset auditing |
| Patch failure updating tasks.md | Context mismatch during edit | Re-ran read + targeted patch | Maintained accurate task history |

## Documentation Quality Assessment

- Success criteria table gives auditable traceability.
- Tasks file now a historical artifact of the migration steps.
- Quickstart focuses on final workflow rather than obsolete before/after diffs—reduces clutter.

## Risk Mitigation

- Legacy format handled gracefully—no crashes on malformed dimensions.
- Physics transition guarded by state machine; reduced timing race potential.

## Opportunities for Improvement

- Add automated tests for level transition state machine (BallFrozen lifecycle & gravity restoration).
- Log capture test for backward compatibility warnings (assert presence & format).
- Consider extracting normalize_matrix into its own module for reuse/testing clarity.

## Suggested Next Features

1. Dynamic level sequencing (indexing multiple levels, progression rules).
2. Scoring + combo multiplier system tied to brick clears.
3. Audio feedback for transitions (growth, brick spawn, success).
4. Automated regression tests for success criteria.
5. Texture/profile overrides per level (if texture manifest enabled).

## Lessons Learned

- Explicit frame separation simplifies debugging of state transitions in ECS.
- Surfacing migration guidance early reduces downstream maintenance and support costs.
- Small, focused warnings outperform silent normalization for developer confidence.

## Action Items (Optional Follow-up)

| ID | Item | Priority | Owner (TBD) |
|----|------|----------|-------------|
| A01 | Add integration test for unfreeze sequence | High |  |
| A02 | Add warning log capture test | High |  |
| A03 | Introduce scoring component & systems | Medium |  |
| A04 | Refactor normalization into module | Medium |  |
| A05 | Implement audio event hooks | Low |  |

## Final Status

Migration complete; all tasks T001–T072 closed and committed.
Conventional commit recorded: `feat(map-format): finalize Phase 7 polish & verification`.
