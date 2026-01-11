# Implementation Plan: Create Sphinx / MyST docs + rustdoc embed (001-sphinx-docs)

**Branch**: `001-sphinx-docs` | **Date**: 2025-11-29 | **Spec**: `specs/001-sphinx-docs/spec.md`
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Create a public, versioned documentation site for the project using Sphinx + MyST with the furo theme.
The site will embed the project's full rustdoc output (pre-generated in CI) so API reference pages are discoverable and versioned with the rest of the documentation.
Read the Docs will be the authoritative publishing target, while CI will validate docs on PRs and run a full build on `main` for publication.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 1.81 (project), Python 3.11 for docs toolchain **Primary Dependencies**: Sphinx, MyST-Parser, furo-theme, sphinx-rtd-theme-compat (if needed), `cargo doc` for rustdoc generation **Storage**: N/A — documentation stored in repo under `/docs/` and `specs/` for plans **Testing**: CI docs jobs using `sphinx-build` and `cargo doc`; link-checkers like `sphinx-linkcheck` for PR checks **Target Platform**: Static HTML for web (Read the Docs), CI runners (Linux) for build tasks **Project Type**: Documentation website + CI automation **Performance Goals**: PR docs validation under 60s (fast checks); full `main` build (including rustdoc) allowed up to 3 minutes with caching **Constraints**: Must include embedded rustdoc artifacts; RTD build differences must be accounted for in CI integration; avoid memory-heavy generation on PR runs **Scale/Scope**: Hundreds of pages with embedded rustdoc for a small-to-medium codebase; artifacts expected <100MB

## Constitution Check

The Brkrs constitution contains general engineering quality and performance rules.
This documentation feature does not violate constitutional principles: we are not modifying runtime game systems, it's purely documentation-focused, and includes performance goals for CI builds (not game frame rate).
Gates passed.

*GATE: Must pass before Phase 0 research.*
            *Re-check after Phase 1 design.*

Gates: No constitutional violations detected for documentation-only feature.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: Place documentation source in `docs/` at the repository root; use `specs/001-sphinx-docs/` for plan & artifacts.
CI workflows will be added under `.github/workflows/` for PR validation and `main` full builds; Read the Docs integration will be configured in `readthedocs.yml` at repo root.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
