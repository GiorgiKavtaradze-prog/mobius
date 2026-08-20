# Contributing to Mobius

Thank you for considering a contribution to Mobius. The project benefits from
focused pull requests, clear issue reports, careful protocol changes, and
documentation that stays close to the implementation.

For large features, breaking behavior, or protocol changes, open an issue first
so the design can be discussed before implementation.

## Code of Conduct

By participating in this project, you agree to follow the
[Code of Conduct](CODE_OF_CONDUCT.md). Keep discussions respectful,
constructive, and focused on the work.

## Development Requirements

Install the following tools before working on the project:

- Rust stable toolchain with Rust 2024 edition support.
- Cargo, rustfmt, and clippy.
- Git.
- Nix, optional, for reproducible builds and module testing.

Recommended setup:

```bash
rustup update stable
rustup component add rustfmt clippy
```

With Nix:

```bash
nix develop
```

## Repository Layout

```text
mobius/
|-- src/                 # Terminal runtime, renderer, input, VT, RGP, camera, scene code
|-- src/scene/           # 3D presentation modes and Mobius strip projection
|-- src/shaders/         # WGSL shaders
|-- config/              # Default mobius.toml
|-- assets/objects/      # Bundled OBJ, GLB, and STL assets
|-- protocols/           # Protocol specifications
|-- widget/              # ratatui-mobius crate and examples
|-- nix/                 # Nix package definition and docs
|-- website/             # Project website assets
|-- flake.nix            # Nix flake, modules, checks, and dev shell
`-- Cargo.toml           # Main Mobius crate
```

## Workflow

1. Fork and clone the repository.
2. Create a topic branch:

   ```bash
   git checkout -b feature/short-description
   ```

3. Keep the change focused. Avoid mixing behavior changes with broad formatting
   or unrelated cleanup.
4. Add or update tests when behavior changes.
5. Update documentation when user-facing behavior, configuration, RGP, or widget
   APIs change.
6. Run the verification commands before opening a pull request.

## Building and Running

```bash
# Build debug binary
cargo build

# Build optimized binary
cargo build --release

# Run Mobius
cargo run -- -T "Mobius"

# Run Mobius with a custom config
cargo run -- -c config/mobius.toml

# Run Mobius with an explicit shell or command
cargo run -- -e bash
```

The `-e` / `--command` argument captures the command and its arguments, so place
it at the end of the Mobius command line.

## Verification

Run these checks for changes in the main crate:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

For widget changes, also run:

```bash
cargo fmt --manifest-path widget/Cargo.toml --all -- --check
cargo clippy --manifest-path widget/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path widget/Cargo.toml --all-targets
cargo check --manifest-path widget/Cargo.toml --examples
```

For Nix changes, run the relevant subset:

```bash
nix fmt
nix build
nix flake check
```

If a command is not available on your platform, note that in the pull request
description.

## Coding Standards

- Follow the Rust API Guidelines and idiomatic Rust naming conventions.
- Prefer small, explicit types over loosely structured data.
- Keep `unsafe` code rare, isolated, and documented with the invariant that
  makes it sound.
- Avoid panics in runtime paths unless the condition is truly unrecoverable.
- Keep allocations out of hot rendering, parsing, and input paths where
  practical.
- Do not add dependencies casually. Explain the purpose and tradeoff of each
  new dependency in the pull request.
- Keep public API items documented with `///` comments.

## Testing Guidance

Use focused tests that exercise the behavior being changed:

- Parser and protocol changes should include unit tests for valid, partial,
  malformed, and backward-compatible inputs.
- Configuration changes should include deserialization and default-value tests.
- Input changes should cover modifier collisions and platform-sensitive key
  behavior where possible.
- Rendering changes should include tests for pure transformation math when the
  GPU path cannot be tested directly.
- Widget changes should cover generated RGP sequences and buffer placement.

## Documentation Guidance

Update documentation in the same pull request when behavior changes:

- `README.md` for user-facing setup, configuration, key bindings, and examples.
- `protocols/graphics.md` for any RGP wire-format or semantic change.
- `widget/README.md` for `ratatui-mobius` API and examples.
- `nix/README.md` for flake, package, NixOS, or Home Manager changes.
- `CHANGELOG.md` for notable user-facing changes.

## Pull Request Checklist

- [ ] The change is focused and has a clear motivation.
- [ ] Tests or verification steps were added or updated where appropriate.
- [ ] Documentation and changelog entries were updated when needed.
- [ ] Formatting, linting, and tests were run, or skipped with a reason.
- [ ] No unrelated files or generated artifacts were included.
- [ ] New dependencies are justified.

## Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/) where possible:

```text
<type>(<scope>): <summary>
```

Common types:

| Type | Use for |
| :--- | :------ |
| `feat` | New user-facing functionality |
| `fix` | Bug fixes |
| `docs` | Documentation-only changes |
| `test` | Test additions or updates |
| `refactor` | Internal restructuring without behavior changes |
| `perf` | Performance improvements |
| `build` | Build system, packaging, or dependency changes |
| `ci` | Continuous integration changes |
| `chore` | Maintenance work |

Examples:

```text
feat(rgp): add chunked payload registration
fix(keyboard): avoid numpad camera slot collisions
docs(nix): document Home Manager gpuBackend usage
```

## Issue Reports

Before opening an issue, search existing issues and pull requests.

For bugs, include:

- What happened and what you expected.
- Minimal reproduction steps.
- Operating system, GPU, driver, shell, and Mobius version or commit.
- Configuration file snippets, if relevant.
- Logs, screenshots, or terminal output when useful.

For feature requests, include:

- The problem or workflow you want to improve.
- The proposed behavior.
- Alternatives you considered.
- Any compatibility concerns, especially for RGP or configuration changes.

## License

By contributing, you agree that your contributions will be licensed under the
[MIT License](LICENSE).
