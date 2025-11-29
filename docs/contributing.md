# Contributing

Thank you for your interest in contributing to brkrs! This guide covers the contribution workflow.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:

   ```bash
   git clone https://github.com/YOUR_USERNAME/brkrs.git
   cd brkrs
   ```

3. **Add the upstream remote**:

   ```bash
   git remote add upstream https://github.com/cleder/brkrs.git
   ```

4. **Create a feature branch**:

   ```bash
   git checkout -b feature/your-feature-name
   ```

## Pull Request Workflow

### Before Submitting

- [ ] Run `cargo test` — all tests pass
- [ ] Run `cargo fmt --all` — code is formatted
- [ ] Run `cargo clippy --all-targets --all-features` — no warnings
- [ ] Run `bevy lint` — bevy-specific checks pass
- [ ] Update documentation if adding new features
- [ ] Add tests for new functionality

### Submitting a PR

1. Push your branch to your fork:

   ```bash
   git push origin feature/your-feature-name
   ```

2. Open a Pull Request against `main`
3. Fill in the PR template with:
   - Description of changes
   - Related issues (if any)
   - Testing performed
4. Wait for CI checks to pass
5. Address review feedback

### PR Guidelines

- **Keep PRs focused**: One feature or fix per PR
- **Write clear commit messages**: Describe what and why
- **Update tests**: Add or modify tests for changed behavior
- **Document public APIs**: Use rustdoc comments for new public items

## Code Style

### Rust Conventions

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `snake_case` for functions and variables
- Use `CamelCase` for types and traits
- Use `SCREAMING_SNAKE_CASE` for constants

### Documentation Style

- Document the **why**, not the **how**
- Include examples for non-trivial functions
- Use complete sentences in doc comments

Example:

```rust
/// Spawns a brick entity at the given grid position.
///
/// Use this function when loading levels to create brick entities
/// from the level matrix. The brick type determines which components
/// are attached.
///
/// # Panics
///
/// Panics if `grid_pos` is outside the 20x20 grid bounds.
pub fn spawn_brick(commands: &mut Commands, grid_pos: (usize, usize), brick_type: u8) {
    // implementation
}
```

### ECS Patterns

brkrs follows Bevy's ECS architecture:

- **Components**: Pure data, no behavior
- **Systems**: Functions that operate on component queries
- **Resources**: Global state shared across systems
- **Events**: Decoupled communication between systems

## Commit Messages

Follow conventional commit format:

```text
type(scope): short description

Longer explanation if needed.

Fixes #123
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

## Reporting Issues

When opening an issue, include:

1. **Description**: What happened vs. what you expected
2. **Steps to reproduce**: Minimal steps to trigger the issue
3. **Environment**: OS, Rust version (`rustc --version`)
4. **Logs/errors**: Full error messages or screenshots

## Questions?

- Check existing [GitHub Issues](https://github.com/cleder/brkrs/issues)
- Open a new issue with the "question" label
