# Quickstart: UI Constitution Refactor (010-refactor)

## Local Development & Verification

This document describes how to verify the UI Constitution refactor work locally.

### Prerequisites

- Rust 1.81+ (managed by `rustup`)
- Bevy 0.17.3 + dependencies (see Cargo.toml)

### Running Tests

**All UI compliance tests** (validates audit artifact, behavior preservation):

```bash
cargo test --test ui_compliance_audit
```

**All tests in the repository**:

```bash
cargo test
```

**Tests for a specific user story** (when implementation begins):

```bash
cargo test --test ui_*
```

### Code Quality Checks

**Run all code quality checks** (as enforced by pre-commit hooks):

```bash
cargo fmt --all
cargo clippy --all-targets --all-features
bevy lint
```

**Fix formatting issues**:

```bash
cargo fmt --all
```

**Fix clippy warnings** (introduced by refactor only; do not fix unrelated):

```bash
cargo clippy --fix --allow-dirty --all-targets --all-features
```

### Verification Workflow

Before committing changes to this refactor, verify:

1. **Tests pass**:

   ```bash
   cargo test --lib --test '*'
   ```

2. **Code is formatted**:

   ```bash
   cargo fmt --all
   ```

3. **No new clippy warnings** (check diff before/after):

   ```bash
   cargo clippy --all-targets --all-features 2>&1 | grep warning
   ```

4. **Bevy-specific lints pass**:

   ```bash
   bevy lint
   ```

### Approving Red Tests

When creating failing tests (red state before implementation), commit the failing tests with a message like:

```text
RED: Add failing test for [feature] (T[task_number])

Test fails until: [list what needs to be fixed]
```

After approval (recorded in `specs/010-refactor/tasks.md` with commit hash + approver + date), proceed with the implementation.

### Documentation

- **Compliance Audit**: [compliance-audit.md](compliance-audit.md)
- **Refactoring Plan**: [refactoring-plan.md](refactoring-plan.md)
- **Feature Spec**: [spec.md](spec.md)
- **Implementation Plan**: [plan.md](plan.md)
- **Task List**: [tasks.md](tasks.md)

### Tips

- Use `cargo test --test ui_compliance_audit -- --nocapture` to see audit test output.
- Pre-commit hooks will run automatically; if they fail, fix with `cargo fmt` and retry.
- When working on a specific user story, check the corresponding section in `tasks.md` for detailed requirements.
