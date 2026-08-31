# Tag

[![CI](https://github.com/unified-field-dev/tag/actions/workflows/ci.yml/badge.svg)](https://github.com/unified-field-dev/tag/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[GitHub](https://github.com/unified-field-dev/tag) · `cargo doc -p tag --features ssr --open`

## About

Tag is the Unified Field **shared label catalog** and admin UI: flat `Tag` rows
(name, taxonomy, owner), optional per-field `tag_history`, and a search source for
Orbital pickers. Product records attach tags through Valence **ManyToMany**
connections on their own schemas — this crate never stores assignment edges.

- **Domain (`tag`)** — Valence schemas, CRUD service, history side effect,
  ownership privacy, catalog search source
- **Admin UI (`tag-app`)** — Orbital `tag` app at `/tag` (list, create, detail)
  with embedded `HistoryTimeline` and reusable `TagCatalogPicker`
- **Composition** — products declare `connections: [ tags: { ManyToMany, … } ]`
  and call generated connection APIs

Crate-root rustdoc owns Concern → API tables. Start at
`cargo doc -p tag --features ssr --open`, then `tag-app` with `--features ssr`.

## Getting started

```toml
[dependencies]
# Pin tag or rev — do not use branch = "main".
tag = { git = "https://github.com/unified-field-dev/tag", package = "tag", rev = "REPLACE_WITH_PIN" }
tag-app = { git = "https://github.com/unified-field-dev/tag", package = "tag-app", rev = "REPLACE_WITH_PIN", default-features = false }
```

```rust,ignore
use tag_app::TagRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <TagRoutes />
    </Routes>
}
```

How products attach tags (illustrative schema fragment):

```rust,ignore
connections: [
    tags: {
        table: "tag",
        cardinality: ManyToMany,
        edge_table: "finance_transaction_tag",
        model: "tag::generated::Tag",
    },
],
```

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-tag
cargo test -p tag --features ssr
```

## Workspace

| Crate | Role |
|-------|------|
| [`tag`](tag/) | Tag Valence schema, CRUD, optional `tag_history`, search source |
| [`tag-app`](tag-app/) | `/tag` admin UI + Higgs server wrappers (`TagRoutes`) |

## Examples

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-tag-host`](examples/protected-tag-host/) | Catalog + ManyToMany + `/tag` | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-tag cargo run -p protected-tag-host` | Deny/allow + catalog + edges | Mount `TagRoutes` |

Copy `Cargo.toml` + `main.rs` (and the product-mount feature graph) from the
host README. More examples: [`examples/README.md`](examples/README.md).

## Security

Catalog ownership, server-fn session gates, and reporting:
[`SECURITY.md`](SECURITY.md). Report vulnerabilities privately — do not open a
public issue for security-sensitive reports.

## Verify

GitHub Actions (`.github/workflows/ci.yml`) runs the CI subset from
[`docs/VERIFICATION.md`](docs/VERIFICATION.md): fmt, clippy on `tag` (+ teaching
host), contract tests, `protected-tag-host` check/run, and tag rustdoc with
broken-intra-doc-link deny.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-tag
cargo fmt -p tag -p protected-tag-host -- --check
cargo clippy -p tag --all-targets --features ssr -- -D warnings
cargo clippy -p protected-tag-host --all-targets -- -D warnings
cargo test -p tag --test workspace_members --test product_surface
cargo test -p tag --features ssr --test tag_crud_contract --test tag_service_integration
cargo check -p protected-tag-host
cargo run -p protected-tag-host
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p tag --features ssr --no-deps
```

Teaching host success line:
`protected_tag_host: OK — /tag deny/allow + catalog + ManyToMany`.
Contribute: [`CONTRIBUTING.md`](CONTRIBUTING.md).

## FAQ

**Is it a standalone server?** No. `tag` is the domain library; `tag-app` mounts
into a composite host that already wires Valence, session chrome, and Higgs.

**Do I need the admin UI?** No. Backend hosts can depend on `tag` alone and call
`create` / `list` / `get` / `update` / `delete`. Mount `TagRoutes` when operators
need the catalog UI.

**Where do tag assignments live?** On the consuming product's Valence schema as a
ManyToMany connection (edge table + generated APIs). `tag` stores catalog rows only.

**How does history show up?** Definition edits append `tag_history` via
`TagHistoryWriter`. Detail pages embed `record_history_leptos::HistoryTimeline`;
`tag` itself has no history-read API.

## License

MIT. See [LICENSE](LICENSE).
