# Quickstart: Post-Refactor QA & Sanitation

**Feature**: Post-Refactor QA & Sanitation **Status**: Draft

## Overview

This feature is a maintenance pass to ensure code quality and compliance with the project constitution.
It does not introduce new gameplay features.

## Verification Steps

1. **Run Tests**:
    ```bash
    cargo test
    ```
    Ensure all tests pass and no "fake tests" (comment-only) remain.

2. **Check Visibility**:
    Verify that `BALL_RADIUS` and other constants in `src/lib.rs` are `pub(crate)` instead of `pub`.

3. **Check Startup Order**:
    Inspect `src/lib.rs` to verify that startup systems are chained or explicitly ordered.

## Troubleshooting

- **Test Failures**: If tests fail after removing "fake tests", it means the suite was relying on false positives.
  Investigate the failing logic.
- **Compilation Errors**: If visibility changes cause errors, ensure all usages are within the crate (they should be).
