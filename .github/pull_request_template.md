<!-- Please read CONTRIBUTING.md before submitting a pull request. -->

## Summary

Describe what changed and why.

## Type of Change

- [ ] Bug fix
- [ ] Feature
- [ ] Documentation
- [ ] Refactor or cleanup
- [ ] Performance improvement
- [ ] Build, packaging, or CI

## Verification

List the commands you ran and their results.

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

For widget changes:

```text
cargo fmt --manifest-path widget/Cargo.toml --all -- --check
cargo clippy --manifest-path widget/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path widget/Cargo.toml --all-targets
cargo check --manifest-path widget/Cargo.toml --examples
```

## Checklist

- [ ] The pull request is focused on one logical change.
- [ ] Tests were added or updated where appropriate.
- [ ] Documentation was updated where user-facing behavior changed.
- [ ] `CHANGELOG.md` was updated for notable user-facing changes.
- [ ] New dependencies are justified in the description.
- [ ] Screenshots, recordings, or logs are included for UI/rendering changes.

## Notes for Reviewers

Call out risky areas, follow-up work, compatibility concerns, or known gaps.
