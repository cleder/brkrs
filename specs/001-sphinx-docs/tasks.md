# Tasks: Create Sphinx / MyST docs + rustdoc embed (001-sphinx-docs)

**Input**: Design documents from `/specs/001-sphinx-docs/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓

**Tests**: Not explicitly requested in spec — omitted per task generation rules.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Documentation source**: `docs/` at repository root
- **CI workflows**: `.github/workflows/`
- **RTD config**: `readthedocs.yml` at repository root
- **Rustdoc staging**: `docs/_static/rustdoc/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create docs directory structure and install dependencies

- [x] T001 Create docs/ directory structure with conf.py, index.md, requirements.txt in docs/
- [x] T002 [P] Configure Sphinx conf.py with MyST-Parser and furo theme in docs/conf.py
- [x] T003 [P] Add Python dependencies (sphinx, myst-parser, furo, sphinx-copybutton) in docs/requirements.txt
- [x] T004 [P] Create .gitignore entries for docs/_build/ and docs/_static/rustdoc/ in .gitignore

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: CI infrastructure and Read the Docs configuration — MUST complete before user story content

**⚠️ CRITICAL**: No user story pages can be published until CI and RTD are configured

- [x] T005 Create docs-pr CI workflow for fast PR validation in .github/workflows/docs-pr.yml
- [x] T006 [P] Create docs-main CI workflow with rustdoc generation in .github/workflows/docs-main.yml
- [x] T007 [P] Create Read the Docs configuration file at readthedocs.yml
- [x] T008 Create rustdoc staging script to copy target/doc/ to docs/_static/rustdoc/ in scripts/stage-rustdoc.sh
- [x] T009 [P] Add Makefile or justfile targets for local docs build in docs/Makefile

**Checkpoint**: CI pipelines and RTD config ready — user story content can now be authored

---

## Phase 3: User Story 1 — Quickstart for new players (Priority: P1) 🎯 MVP

**Goal**: Provide a concise quickstart so new users can run the game locally without reading source code

**Independent Test**: Follow the Quickstart from a clean environment; verify the game starts and a sample level launches within 10 minutes

### Implementation for User Story 1

- [x] T010 [US1] Create Quickstart page with prerequisites, build, and run steps in docs/quickstart.md
- [x] T011 [P] [US1] Add troubleshooting section with common issues and solutions in docs/troubleshooting.md
- [x] T012 [P] [US1] Add sample screenshot or GIF from assets/ to demonstrate running game in docs/_static/images/
- [x] T013 [US1] Cross-reference quickstart from index.md and add to toctree in docs/index.md

**Checkpoint**: User Story 1 complete — Quickstart page published and independently verifiable

---

## Phase 4: User Story 2 — Developer onboarding & contribution guide (Priority: P2)

**Goal**: Provide a developer guide so contributors can run tests, add content, and submit PRs confidently

**Independent Test**: A new contributor follows the guide to run tests, add a simple level asset, and validate locally

### Implementation for User Story 2

- [x] T014 [US2] Create Developer Guide page covering repo structure, tests, adding content in docs/developer-guide.md
- [x] T015 [P] [US2] Create Contributing page with PR workflow, checklist, and code style in docs/contributing.md
- [x] T016 [P] [US2] Document level and asset format with examples in docs/asset-format.md
- [x] T017 [P] [US2] Add architecture overview diagram or description in docs/architecture.md
- [x] T018 [US2] Add all US2 pages to toctree and cross-reference from index.md in docs/index.md

**Checkpoint**: User Story 2 complete — Developer onboarding docs published and independently verifiable

---

## Phase 5: User Story 3 — Documentation publishing & versioning (Priority: P3)

**Goal**: Ensure documentation is automatically published and versioned so users can browse stable docs

**Independent Test**: Merge to main triggers RTD build; public URL shows updated content with version selector

### Implementation for User Story 3

- [x] T019 [US3] Configure RTD version selector and branch/tag builds in readthedocs.yml
- [x] T020 [P] [US3] Add API reference landing page that links to embedded rustdoc in docs/api-reference.md
- [x] T021 [P] [US3] Ensure rustdoc artifacts are discoverable via navigation in docs/conf.py (html_extra_path or similar)
- [x] T022 [US3] Update README.md with link to published Read the Docs URL in README.md
- [x] T023 [US3] Add version badge and docs link to project README in README.md

**Checkpoint**: User Story 3 complete — RTD publishing verified, versioned docs accessible

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements affecting all user stories and final validation

- [ ] T024 [P] Add FAQ page with common questions in docs/faq.md
- [ ] T025 [P] Demonstrate MyST features (admonitions, cross-refs, code-blocks) in at least one page in docs/developer-guide.md
- [ ] T026 Run sphinx-linkcheck to validate all internal and external links
- [ ] T027 Run local quickstart.md validation steps from specs/001-sphinx-docs/quickstart.md
- [ ] T028 Verify mobile/desktop responsiveness of published furo-themed site
- [ ] T029 Final review: ensure all pages render, rustdoc embedded, RTD version selector works

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS publishing
- **User Stories (Phase 3+)**: Depend on Foundational phase completion
  - User stories can proceed in priority order (P1 → P2 → P3)
  - Or in parallel if multiple authors are available
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) — No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) — Independent of US1
- **User Story 3 (P3)**: Depends on Foundational (Phase 2); some tasks (T019) refine RTD config from T007

### Within Each Phase

- Tasks marked [P] can run in parallel
- Non-[P] tasks should be completed sequentially
- Story pages should be added to toctree after creation

### Parallel Opportunities

- Setup tasks T002, T003, T004 can run in parallel
- Foundational tasks T006, T007, T009 can run in parallel
- US1 tasks T011, T012 can run in parallel
- US2 tasks T015, T016, T017 can run in parallel
- US3 tasks T020, T021 can run in parallel
- Polish tasks T024, T025 can run in parallel

---

## Parallel Example: Phase 2 (Foundational)

```text
# Sequential (blocking):
T005: Create docs-pr CI workflow

# Parallel after T005:
T006: Create docs-main CI workflow
T007: Create readthedocs.yml
T009: Add Makefile targets

# Sequential (depends on T006):
T008: Create rustdoc staging script
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001–T004)
2. Complete Phase 2: Foundational (T005–T009)
3. Complete Phase 3: User Story 1 (T010–T013)
4. **STOP and VALIDATE**: Quickstart page renders, CI passes, RTD builds
5. Deploy/demo — MVP delivered

### Incremental Delivery

1. Setup + Foundational → CI and RTD ready
2. Add User Story 1 → Quickstart published (MVP!)
3. Add User Story 2 → Developer onboarding published
4. Add User Story 3 → Versioned publishing complete
5. Polish → FAQ, link validation, final checks

### Single-Author Strategy

1. Complete phases sequentially: Setup → Foundational → US1 → US2 → US3 → Polish
2. Commit after each task or logical group
3. Validate at each checkpoint before proceeding

---

## Notes

- No test tasks included — tests not explicitly requested in spec
- rustdoc embedding is handled via staging script (T008) and conf.py config (T021)
- RTD project creation is a manual step (maintainer action) — not a task
- PR validation (T005) should complete in <60s; main build (T006) allowed up to 3 minutes
- All paths are relative to repository root
