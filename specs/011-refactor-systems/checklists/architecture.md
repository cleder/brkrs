# Architecture & Constitution Compliance Checklist

**Purpose**: Validate requirements quality for Constitution 1.3.0 compliance refactor **Created**: 2025-12-19 **Scope**: Architecture-level requirements + high-risk subsystems (respawn, signals) **Focus**: Exception/recovery scenarios; requirement clarity

---

## Requirement Completeness

### Architecture-Level Requirements

- [ ] CHK001 - Are all nine functional requirements (FR-001..FR-009) defined with specific Constitution rule citations? [Completeness, Spec §Requirements]
- [ ] CHK002 - Is the error type for fallible systems (`anyhow::Result<()>`) explicitly specified in FR-001? [Clarity, Spec §FR-001, Clarifications]
- [ ] CHK003 - Are the exact marker components requiring `#[require(Transform, Visibility)]` listed in FR-005? [Completeness, Spec §FR-005]
- [ ] CHK004 - Is the shared signals module location (`crate::signals`) documented in FR-002? [Clarity, Spec §FR-002, Key Entities]
- [ ] CHK005 - Are System Set naming conventions (`*Systems` suffix) specified in FR-003? [Clarity, Spec §FR-003]

### Signal Unification Requirements

- [ ] CHK006 - Are requirements defined for both `UiBeep` and `BrickDestroyed` message unification? [Completeness, Spec §FR-002]
- [ ] CHK007 - Is the prohibition of dual `Message`/`Event` derivation explicit in FR-002? [Clarity, Spec §FR-002]
- [ ] CHK008 - Are engine event consumption patterns (observers) specified separately from gameplay messages? [Completeness, Spec §FR-009]
- [ ] CHK009 - Is the removal of duplicate `BrickDestroyed` definitions (audio, scoring) documented? [Completeness, Clarifications]

### System Set Organization Requirements

- [ ] CHK010 - Are the new System Set enums (`AudioSystems`, `PaddleSizeSystems`, `TextureOverrideSystems`) explicitly listed? [Completeness, Spec §Key Entities]
- [ ] CHK011 - Is the prohibition of tuple `.chain()` within system lists clear in FR-003? [Clarity, Spec §FR-003]
- [ ] CHK012 - Are allowed ordering mechanisms (`.configure_sets()`, `.after()`, `.before()`) specified? [Completeness, Spec §FR-003]

### Change Detection Requirements

- [ ] CHK013 - Are strict change-driven triggers (`Changed<T>`, `RemovedComponents<T>`, `OnAdd`) required in FR-004? [Clarity, Spec §FR-004]
- [ ] CHK014 - Is the prohibition of periodic fallback ticks explicit in FR-004? [Clarity, Spec §FR-004, Clarifications]
- [ ] CHK015 - Are specific systems requiring change detection listed (paddle visuals, textures, grid overlay)? [Completeness, Spec §Data Model]

---

## Requirement Clarity

### Fallible Systems

- [ ] CHK016 - Is "fallible system" quantified with return type and error propagation pattern? [Clarity, Spec §FR-001]
- [ ] CHK017 - Are prohibited patterns (`unwrap()`, panicking queries) explicitly forbidden? [Clarity, Spec §FR-001]
- [ ] CHK018 - Is the error recovery pattern (`let Some(..) = .. else { return Ok(()) }`) specified in FR-006? [Clarity, Spec §FR-006]

### Message vs Event Distinction

- [ ] CHK019 - Is "Message" defined as buffered with `MessageWriter`/`MessageReader`? [Clarity, Spec §FR-002]
- [ ] CHK020 - Is "Event" defined as observer-based with `commands.observe()`? [Clarity, Spec §FR-009]
- [ ] CHK021 - Is the rationale for choosing Messages over Events for gameplay signals documented? [Clarity, Assumptions]

### Required Components

- [ ] CHK022 - Is the `#[require(Transform, Visibility)]` syntax specified in FR-005? [Clarity, Spec §FR-005]
- [ ] CHK023 - Is "redundant bundle" defined (manual `(Transform, Visibility)` in spawns)? [Clarity, Spec §FR-005]

---

## Requirement Consistency

### Constitution Alignment

- [ ] CHK024 - Do all FRs cite specific Constitution sections (VIII: ...; Prohibitions: NO ...)? [Consistency, Spec §Requirements]
- [ ] CHK025 - Are requirements consistent with Clarifications decisions (anyhow, signals location, strict change detection)? [Consistency, Spec §Clarifications]
- [ ] CHK026 - Are System Set requirements consistent between FR-003 and Key Entities? [Consistency, Spec §FR-003, Key Entities]

### Cross-Requirement Alignment

- [ ] CHK027 - Are change detection requirements (FR-004) compatible with system set ordering (FR-003)? [Consistency]
- [ ] CHK028 - Are required component requirements (FR-005) aligned with spawn simplification goals? [Consistency]
- [ ] CHK029 - Are message unification requirements (FR-002) compatible with engine event handling (FR-009)? [Consistency]

---

## Scenario Coverage

### Primary Flows

- [ ] CHK030 - Are requirements defined for converting all systems to fallible? [Coverage, Spec §FR-001]
- [ ] CHK031 - Are requirements defined for unifying duplicate signal types? [Coverage, Spec §FR-002]
- [ ] CHK032 - Are requirements defined for creating new System Set enums? [Coverage, Spec §FR-003]
- [ ] CHK033 - Are requirements defined for adding change detection filters? [Coverage, Spec §FR-004]
- [ ] CHK034 - Are requirements defined for applying `#[require]` attributes? [Coverage, Spec §FR-005]

### Exception/Recovery Flows

- [ ] CHK035 - Are requirements defined for systems with missing resources (e.g., AssetServer absent)? [Coverage, Edge Cases]
- [ ] CHK036 - Are requirements defined for respawn execution with no pending request? [Coverage, Spec §User Story 1]
- [ ] CHK037 - Are requirements defined for WASM environments without file I/O? [Coverage, Edge Cases]
- [ ] CHK038 - Are requirements defined for multiple brick-destroy events in a single frame? [Coverage, Edge Cases]
- [ ] CHK039 - Are requirements defined for concurrent sound playback limits? [Coverage, Edge Cases]

### High-Risk Subsystem: Respawn

- [ ] CHK040 - Are error recovery requirements specified for respawn executor edge cases? [Coverage, Spec §FR-006]
- [ ] CHK041 - Are requirements defined for respawn queue overflow scenarios? [Coverage, Gap]
- [ ] CHK042 - Are requirements defined for respawn with missing paddle entity? [Coverage, Gap]
- [ ] CHK043 - Are requirements defined for change-driven respawn visual updates? [Coverage, Spec §FR-004]

### High-Risk Subsystem: Signals

- [ ] CHK044 - Are requirements defined for signal producer/consumer registration order? [Coverage, Gap]
- [ ] CHK045 - Are requirements defined for message buffering behavior across frame boundaries? [Coverage, Gap]
- [ ] CHK046 - Are requirements defined for signal schema versioning/evolution? [Coverage, Gap]

---

## Edge Case Coverage

### WASM-Specific

- [ ] CHK047 - Are WASM file I/O restrictions documented in Edge Cases? [Completeness, Edge Cases]
- [ ] CHK048 - Are WASM-specific asset loading mechanisms specified? [Gap]

### Resource Availability

- [ ] CHK049 - Are requirements defined for systems when `AssetServer` is unavailable? [Coverage, Edge Cases]
- [ ] CHK050 - Are requirements defined for systems when `Messages<AssetEvent<Image>>` is unavailable? [Coverage, Edge Cases]

### Concurrent Events

- [ ] CHK051 - Are requirements defined for multiple simultaneous brick destructions? [Coverage, Edge Cases]
- [ ] CHK052 - Are requirements defined for audio concurrency limits (4 per type)? [Coverage, Edge Cases]

---

## Acceptance Criteria Quality

### Measurability

- [ ] CHK053 - Are success criteria (SC-001..SC-004) objectively measurable? [Measurability, Spec §Success Criteria]
- [ ] CHK054 - Can "zero panics" (SC-001) be verified via test suite execution? [Measurability, Spec §SC-001]
- [ ] CHK055 - Can "no per-frame work" (SC-002) be verified via targeted tests? [Measurability, Spec §SC-002]
- [ ] CHK056 - Can "single-path messaging" (SC-003) be verified via producer/consumer tests? [Measurability, Spec §SC-003]
- [ ] CHK057 - Can "no tuple .chain()" (SC-004) be verified via observable state tests? [Measurability, Spec §SC-004]

---

## Traceability

### Specification References

- [ ] CHK058 - Does each FR reference Constitution mandates/prohibitions? [Traceability, Spec §Requirements]
- [ ] CHK059 - Do high-risk subsystem requirements reference specific systems/modules? [Traceability]
- [ ] CHK060 - Do clarifications reference originating questions and decisions? [Traceability, Spec §Clarifications]

### Cross-Document Consistency

- [ ] CHK061 - Are Key Entities consistent with plan.md data-model.md? [Traceability]
- [ ] CHK062 - Are System Sets listed in spec consistent with plan Phase 2 tasks? [Traceability]
- [ ] CHK063 - Are required markers (Paddle, Ball, etc.) consistent across spec/plan/data-model? [Traceability]

---

## Ambiguities & Conflicts

### Terminology

- [ ] CHK064 - Is "fallible system" used consistently (returns `Result`, not just "handles errors")? [Consistency]
- [ ] CHK065 - Is "change-driven" used consistently (vs "reactive", "on-change")? [Consistency]

### Potential Conflicts

- [ ] CHK066 - Are there conflicts between strict change detection (FR-004) and async asset loading? [Conflict]
- [ ] CHK067 - Are there conflicts between required components (FR-005) and existing spawn code? [Conflict, addressed in plan]
- [ ] CHK068 - Are there conflicts between message unification (FR-002) and existing test fixtures? [Conflict]

---

## Dependencies & Assumptions

### External Dependencies

- [ ] CHK069 - Is the `anyhow` crate dependency documented as required? [Dependency, Clarifications]
- [ ] CHK070 - Are Bevy 0.17 API changes (Messages, observers, required components) assumed stable? [Assumption]

### Assumptions Validation

- [ ] CHK071 - Is the assumption of "Messages for gameplay signals" explicitly stated? [Assumption, Spec §Assumptions]
- [ ] CHK072 - Is the assumption of "minor cross-module coordination" for required components documented? [Assumption, Spec §Assumptions]
- [ ] CHK073 - Is the assumption of "tests don't require framework internals" validated? [Assumption, Spec §Assumptions]

---

## Summary

**Total Items**: 73 **Coverage Breakdown**:

- Requirement Completeness: 15 items
- Requirement Clarity: 8 items
- Requirement Consistency: 6 items
- Scenario Coverage: 17 items (Primary: 5, Exception/Recovery: 5, Respawn: 4, Signals: 3)
- Edge Case Coverage: 6 items
- Acceptance Criteria: 5 items
- Traceability: 6 items
- Ambiguities & Conflicts: 5 items
- Dependencies & Assumptions: 5 items

**Priority Focus**: Architecture + Respawn + Signals subsystems; exception/recovery scenarios
