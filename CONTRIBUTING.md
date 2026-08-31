# Contributing to Tag

Thank you for improving this project.

## Development setup

1. Clone [unified-field-dev/tag](https://github.com/unified-field-dev/tag)
2. Install Rust stable
3. From the repository root:

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-tag
cargo fmt --all -- --check
cargo check -p tag --features ssr
```

Prefer `cargo check -p tag --features ssr` when the `tag-app` / Orbital graph
is broken upstream. Full gates:
[`docs/VERIFICATION.md`](docs/VERIFICATION.md).

## Code of conduct

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Security
reports: [`SECURITY.md`](SECURITY.md).

## Pull requests

- Prefer small, focused PRs.
- Update [`README.md`](README.md) when public API or UI flows change.
