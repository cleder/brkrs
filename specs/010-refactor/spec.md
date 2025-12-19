# Feature Specification: UI Constitution Refactor

**Feature Branch**: `010-refactor` **Created**: 2025-12-19 **Status**: Draft **Input**: User description: "Refactor legacy UI code to comply with the Constitution; perform a Compliance Audit of src/ui and define a step-by-step refactoring plan."

This is a non-feature refactor focused on consistency, safety, and performance.
No player-facing UI changes are intended.

## Deliverables

- **Compliance audit artifact**: `specs/010-refactor/compliance-audit.md`
- **Refactoring plan artifact**: `specs/010-refactor/refactoring-plan.md`
- **Code refactor outcome**: `src/ui/**` is updated to comply with the in-scope Constitution rules (see FR-011)

The audit and plan artifacts are review deliverables independent from code changes (User Story 1 can complete without User Story 2).

## Clarifications

### Session 2025-12-19

- Q: When a `src/ui` compliance fix requires changes outside `src/ui`, what scope is allowed? → A: Primary scope is `src/ui`, but allow minimal supporting edits outside it when required for compliance.
- Q: Which Constitution rules are in-scope for the compliance audit? → A: Bevy 0.17 Mandates & Prohibitions (Section VIII) plus other applicable Constitution rules expressed as MUST/NEVER (e.g., Rustdoc for public APIs, code quality rules).

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**Approval Gate (Constitution VII)**: For each user story that introduces/updates tests, maintainers MUST explicitly approve the failing-tests state ("red") before any implementation changes proceed.
Evidence of the red state MUST be recorded in `specs/010-refactor/tasks.md` (commit hash + tests that are failing).

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.

  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Produce a Complete Compliance Audit (Priority: P1)

As a maintainer, I want a complete, reviewable compliance audit of the UI code so that we can confidently refactor legacy UI without missing prohibited patterns.

**Why this priority**: The audit is the prerequisite for all refactoring work; missing violations risks regressions, performance issues, or future maintenance debt.

**Independent Test**: Can be fully tested by reviewing the audit output for completeness and traceability (every finding maps to a Constitution rule and a code location).

**Acceptance Scenarios**:

1. **Given** the current Constitution and the current `src/ui` directory, **When** a compliance audit is produced, **Then** every violation is listed with (a) the affected file, (b) the violated Constitution rule (Mandate/Prohibition), and (c) a brief description of why it violates the rule.
2. **Given** an audit finding, **When** a maintainer follows the cited location and rule, **Then** they can independently confirm the violation without additional context.

**Audit format requirements** (reviewer confirmability):

- Each finding MUST include: file path, affected symbol or entity (when practical), rule citation (section + rule name), explanation, and a suggested remediation direction.
- Scoped-out findings MUST include: rationale, scope/out-of-scope classification, and explicit approver + date.

---

### User Story 2 - Refactor UI Code into Full Compliance (Priority: P2)

As a maintainer, I want the UI module refactored to comply with all applicable Constitution mandates and prohibitions so that the codebase remains consistent, safe, and performant.

**Why this priority**: This directly delivers the feature outcome (compliance) and prevents new work from being built on top of legacy patterns.

**Independent Test**: Can be tested by running the automated test suite and by verifying that the previously identified violations are resolved.

**Acceptance Scenarios**:

1. **Given** the set of audit findings, **When** the refactor is completed, **Then** each finding is either resolved or explicitly scoped out with a documented rationale.
2. **Given** the refactor is completed, **When** the automated tests are run, **Then** the full test suite passes.
3. **Given** the refactor is completed, **When** UI-relevant game state does not change, **Then** UI update logic does not execute work every frame (i.e., it is driven by data changes rather than per-frame polling).

**Definition: documented rationale for scoped-out findings**:

- Rationale is recorded in `specs/010-refactor/compliance-audit.md` alongside the finding (or in a dedicated “Scoped-out findings” section with a stable anchor).
- Each scoped-out finding MUST include: why it is deferred/out-of-scope, what risk it carries, and what follow-up tracking exists (e.g., a follow-up issue or explicit “not planned”).
- Each scoped-out finding MUST include explicit approver + date.

**Definition: UI-relevant game state** (non-exhaustive, but expected inputs for UI systems): score state, lives state, current level state, pause state, cheat mode state, and game-over state.

**Definition: “per-frame UI work”** (objective rule-of-thumb for reviews):

- Any mutation of UI components/entities (e.g., text, visibility, transforms, styles, materials) that occurs every frame when the underlying UI source data did not change.
- Any repeated `asset_server.load(..)` calls in hot paths (including repeated event handling for the same asset).

**Performance expectation (qualitative but reviewable)**:

- UI systems MUST be reactive: they only mutate UI entities when corresponding input data changed (`Changed<T>`, events/messages, or equivalent), or on initial spawn (`Added<T>`).
- UI systems MUST avoid avoidable per-frame allocations and repeated asset loads.

---

### User Story 3 - Preserve Player-Facing UI Behavior (Priority: P3)

As a player, I want the UI to continue behaving the same (score, lives, pause, cheat indicator, level label, and game-over overlay) so that refactoring does not introduce regressions.

**Why this priority**: Refactors that change behavior undermine confidence and create extra QA burden.

**Independent Test**: Can be tested by existing automated UI/gameplay integration tests and a short manual smoke test of the UI features.

**Acceptance Scenarios**:

1. **Given** gameplay state changes that affect score/lives/level display, **When** the state changes, **Then** the corresponding UI display updates to reflect the new state.
2. **Given** the game is paused/unpaused, **When** the pause state toggles, **Then** the pause overlay appears/disappears as expected.
3. **Given** optional UI inputs are missing (e.g., a relevant entity/resource is not present), **When** UI systems run, **Then** the game does not crash and the UI either remains unchanged or falls back to a safe default.

---

Player-facing UI surfaces in-scope for “no behavior change” include: score display, lives counter, pause overlay, cheat indicator, level label, and game-over overlay.

### Edge Cases

- **Multiple matching UI entities** exist (duplicate UI roots or duplicate text nodes).
  - Expected behavior: systems MUST NOT panic; they should treat this as non-fatal and either (a) do nothing (leave UI unchanged) or (b) update deterministically if a deterministic selection exists.
    Default for this feature: do nothing and return success.
- **Missing UI entities/resources** during transitions (e.g., rapid pause/unpause or restart).
  - Expected behavior: systems MUST return success and leave the UI unchanged; they may log diagnostics but must not spam logs every frame.
- **Rapid successive updates** (e.g., score increments rapidly, level changes during respawn).
  - Expected behavior: UI updates remain correct and stable; no crashes.
- **Assets not yet loaded / missing assets** (including early WASM frames).
  - Expected behavior: no crashes; UI systems skip updates until required assets are available or fall back to safe defaults.
- **Audit-only completion**.
  - Expected behavior: User Story 1 can be completed and reviewed without implementing User Story 2.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The project MUST have a documented compliance audit covering all files under `src/ui`.
- **FR-002**: The audit MUST identify every instance where `src/ui` code violates any applicable Constitution mandate or prohibition.
- **FR-003**: The audit MUST list violations in a way that is traceable to a specific code location and a specific Constitution rule.
- **FR-004**: A refactoring plan MUST be produced that groups small fixes into single tasks and separates complex logic changes into distinct tasks.
- **FR-005**: Every refactoring task MUST explicitly cite which Constitution rule(s) it addresses.
- **FR-006**: After refactoring, the `src/ui` code MUST comply with applicable Constitution mandates and prohibitions.
- **FR-007**: Refactoring MUST preserve player-facing UI behavior.
- **FR-008**: Refactoring MUST not introduce performance regressions attributable to per-frame UI updates.
- **FR-009**: All changes MUST follow TDD as described in the Constitution (tests-first, demonstrated failing tests before implementation changes).
- **FR-010**: When required to achieve UI compliance, the refactor MAY include minimal supporting changes outside `src/ui` (e.g., shared components/resources/events or wiring a UI plugin), but MUST remain narrowly scoped.
- **FR-011**: Compliance auditing MUST apply Bevy 0.17 Mandates & Prohibitions (Section VIII) plus other applicable Constitution rules expressed as MUST/NEVER.

#### Scope Guardrails for FR-010 (prevent scope creep)

- Allowed examples: registering a UI plugin from `src/ui` in the app, introducing a small shared error type or resource, adjusting system registration to use sets.
- Non-goals / disallowed examples: redesigning UI layout, changing textures/fonts/colors for aesthetic reasons, adding new UI features, changing gameplay rules, or refactoring unrelated non-UI systems.

### Key Entities

- **Constitution Rule**: A specific Mandate or Prohibition that UI code must follow.
- **Compliance Finding**: A documented instance where existing UI code violates a Constitution rule.
- **Refactoring Task**: A discrete, executable change that resolves one or more compliance findings.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of identified `src/ui` compliance findings are resolved or explicitly scoped out with documented rationale.
- **SC-002**: Automated test suite passes after refactor.
- **SC-003**: UI updates are driven by data changes; when relevant UI source data does not change, UI update work is not performed every frame.
- **SC-004**: A maintainer can take the refactoring plan and complete it without needing to infer missing steps or missing rule mappings.

**Measurability clarifications**:

- “Identified findings” means the set of findings recorded in `specs/010-refactor/compliance-audit.md` at the time User Story 2 begins; any new findings discovered during implementation MUST be added to the audit.
- “Resolved” means the audit item no longer applies in the updated code and is backed by tests and/or reviewer-confirmable code changes.
- “Scoped out” means explicitly deferred with rationale + approver + date in the audit.

## Assumptions

- Scope is primarily limited to `src/ui`; minimal supporting edits outside `src/ui` are allowed when required for compliance; no new UI features or redesign.
- The Constitution text is the source of truth for “compliance”.
- Existing tests represent the baseline expected behavior; new tests will be added where coverage is insufficient to prove behavior preservation (notably for cheat indicator, overlays, and level label safety/precedence).
- Cross-platform expectation: player-facing UI behavior should be consistent on native and WASM builds; differences in asset loading timing are acceptable as long as UI does not crash and eventually reaches the correct state.

## Constitution Versioning & Conflicts

- No explicit Constitution version pin is required for this feature.
- If the Constitution changes during the refactor (new/changed MUST/NEVER rules), the change MUST be recorded in `specs/010-refactor/notes.md` and the audit and plan MUST be updated accordingly.
- If a rule interpretation is ambiguous or rules appear to conflict, apply the stricter MUST/NEVER interpretation by default; document the decision in `specs/010-refactor/notes.md` and get maintainer approval.

## Dependencies

- Maintainer availability to approve the tests-first (red) commit before implementation changes.
- Automated tests can be run in CI for this branch.

**Evidence required for approval gate**:

- The `specs/010-refactor/tasks.md` approval-gate tasks MUST include: commit hash of the failing-tests commit, the failing test names/paths, and the approver + date.
