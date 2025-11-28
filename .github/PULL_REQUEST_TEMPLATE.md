# PR checklist

Please check the items below before requesting review or merging this PR.

- [ ] I ran `cargo test` locally and verified changes pass the test-suite
- [ ] I ran `cargo clippy --all-targets --all-features` and fixed warnings
- [ ] I ran `cargo fmt --all` and formatted the code
- [ ] Relevant docs and/or CHANGELOG.md updated
- [ ] If this PR touches `assets/levels/`, the migration script `scripts/migrate-assets.sh` was run (or migration parity test passes) and backups were created

Optional notes for reviewers:
