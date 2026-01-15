# [PROJECT NAME] Development Guidelines

Auto-generated from all feature plans. Last updated: [DATE]

## Active Technologies

[EXTRACTED FROM ALL PLAN.MD FILES]

## Project Structure

```text
[ACTUAL STRUCTURE FROM PLANS]
```

## Commands

[ONLY COMMANDS FOR ACTIVE TECHNOLOGIES]

## Code Style

[LANGUAGE-SPECIFIC, ONLY FOR LANGUAGES IN USE]

## Testing & TDD

- The project mandates **Test-Driven Development** for all feature work: tests must be written and committed before implementation and a failing-test commit (red) must exist as proof before implementation begins.
- Include unit tests, integration/acceptance tests for user scenarios, and WASM-targeted tests when behavior differs on the web.
- CI pipelines MUST enforce tests and reject merges that do not comply with the tests-first proof.
- CI includes a static analysis check for Message/Event misuse (see `.github/lint/message_event_lint.py` and `tools/message_event_lint`) to help enforce the Message-Event Separation requirement.
- CI includes a PR-only "Tests-first (red→green) check" (`.github/scripts/tests_first_check.sh` + `.github/workflows/ci.yaml`) that validates a failing-test commit exists in the PR history and that tests pass at PR head.
## Recent Changes

[LAST 3 FEATURES AND WHAT THEY ADDED]

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
