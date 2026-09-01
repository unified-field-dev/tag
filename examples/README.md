# Examples

Runnable teaching hosts for this product. Each card: when to use · command ·
success · look next. Copy `Cargo.toml` + `main.rs` (and the product mount
snippets in the host README) into your composite host.

## Canonical path

### `protected-tag-host` — catalog + ManyToMany + `/tag`

**Teaches:** session auth on `/tag`, catalog create/list, and the product-side
ManyToMany edge recipe. Inventory names match the `tag` `uf_app!` id/path
(`/tag`) and `RequireAuthenticated` on `TagRoutes`.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-tag
cargo run -p protected-tag-host
```

**Success:** stdout prints `protected_tag_host: OK — /tag deny/allow + catalog + ManyToMany`.

**Next step:** Mount `<TagRoutes />` from `tag-app` in a product host.
Copy table + product mount `Cargo.toml`: [`protected-tag-host/README.md`](protected-tag-host/README.md).

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-tag-host`](protected-tag-host/) | Catalog + ManyToMany + `/tag` oneshot smoke | `cargo run -p protected-tag-host` | Deny/allow + catalog + edges | Product host with `TagRoutes` |
| [`tag-ui-e2e`](tag-ui-e2e/) | Playwright Layer 2 (eager Tag pages + harness auth) | `cargo leptos end-to-end --project tag-ui-e2e` | Playwright green on `:3160` | Product host keeps Lazy `TagRoutes` |
