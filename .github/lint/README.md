# Message/Event Lint

This small static analysis script flags likely misuse of `MessageWriter<T>` where
an immediate side-effect (e.g., spawning entities, playing audio, or inserting resources)
occurs in the same function. Such cases often indicate that buffered messages
are being used when an observer/trigger pattern is required, per our
Bevy 0.17 architecture mandates.

Usage:

```bash
python3 .github/lint/message_event_lint.py
```

CI Integration: The repository CI runs this script (Python) and also the more
accurate Rust-based linter (`tools/message_event_lint`) and fails the build if any
violations are found. Violations should be reviewed manually and corrected by:
- Moving immediate side-effects into an observer system driven by `Trigger<T>` or `Event`.
- Or replacing the buffered message usage with an immediate API where appropriate.

Testing: Unit tests exist for both linters:
- Python lint tests: `.github/lint/tests/test_message_event_lint.py` (run with `pytest`)
- Rust linter: `cargo test --manifest-path tools/message_event_lint/Cargo.toml`

The script is intentionally conservative and may generate false positives; it
is a guiding tool for code review, not an automated fixer.