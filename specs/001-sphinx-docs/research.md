# Research: 001-sphinx-docs

**Purpose**: Record research findings and decisions required to implement the Sphinx/MyST documentation site with Read the Docs and embedded rustdoc.

## Decision log

### Decision: Hosting target — Read the Docs (preferred)

Rationale: Read the Docs (RTD) is a battle-tested hosting platform tailored to Sphinx and MyST.
It provides automatic versioning for tags/branches, hosted builds, and easy integration with GitHub webhooks and CI.
Choosing RTD reduces operational burden — RTD performs the final authoritative builds and hosts the site.

Alternatives considered:

- GitHub Pages: straightforward but lacks RTD's built-in Sphinx integration and versioned builds.
- Netlify/Static hosts: offer previews and fast hosting but require additional management for versions.

### Decision: Embed full rustdoc output in site

Rationale: Embedding the project's rustdoc provides a single discoverable documentation site for both user and API content.
It improves discoverability over linking out to docs.rs and ensures version parity with the rest of the docs.

Alternatives considered:

- Link to docs.rs: low operational cost but can lead to mismatched versions or broken links.

### Decision: Produce rustdoc artifacts in CI and stage into Sphinx

Rationale: Pre-generating rustdoc artifacts in a reproducible CI step reduces reliance on remote builders and allows deterministic integration of API docs into the Sphinx output.
CI-based generation can be cached and run selectively (PRs vs main), giving a good balance between speed and correctness.

Alternatives considered:

- Let RTD generate rustdoc during its build (works but increases RTD build time and complicates debugging).
- Pre-generate and store artifacts externally (storage + hosting complexity).

### Decision: CI validation + Read the Docs publishing (both)

Rationale: CI PR validation provides fast feedback to contributors preventing regressions early, while Read the Docs remains the canonical publishing target for released and versioned documentation.

### Decision: PR fast-check target 60s, main build allowance 3m

Rationale: A 60s PR target forces fast checks (linting, link validation) and avoids slowing contributor feedback loops.
The full main build will include rustdoc generation which is heavier — 3 minutes is an acceptable upper bound for main branch runs if caching/incremental builds are used.

## Operational notes / required actions

- RTD account & project: a Read the Docs project needs to be created and linked to the repository.
  The maintainer (repo owner/organization) must provide RTD access or create the project.
- CI changes: Add two docs CI jobs: (1) `docs-pr` — quick checks without full rustdoc, enforce under 60s; (2) `docs-main` — full build on `main` including `cargo doc --no-deps` artifacts staged into Sphinx and published.
- Caching: Use cargo build cache and Sphinx incremental builds (doctree caching) to reduce runtime.

## Risks & mitigations

- Large rustdoc builds may exceed allowed CI time on constrained runners.
  Mitigate by caching/partial generation, or split rustdoc generation to a separate job.
- Read the Docs build behavior can differ from CI — replicate integration steps locally and in CI to minimize surprises.
