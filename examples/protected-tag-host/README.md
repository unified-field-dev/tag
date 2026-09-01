# protected-tag-host

Protected **`/tag`** host: create catalog tags and teach product **ManyToMany**
attachment (edge table recipe) without a full Leptos UI.

Production Leptos hosts mount `TagRoutes` at **`/tag`** behind
`RequireAuthenticated` (email verification on). This example proves catalog
create/list + session-gated Axum without the SSR/WASM / Orbital graph. The
oneshot path `/tag` matches the Orbital app id/path (`tag` / `/tag`).

| | |
|---|---|
| **When to use** | First smoke of tag catalog + `/tag` auth + ManyToMany composition |
| **Command** | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-tag cargo run -p protected-tag-host` |
| **Success** | Stdout: `protected_tag_host: OK — /tag deny/allow + catalog + ManyToMany` |
| **Look next** | Mount [`TagRoutes`](../../tag-app/) ; declare ManyToMany on product schemas |

**Open first:** [`src/main.rs`](src/main.rs)

## Copy into your host

| File | What to take |
|------|----------------|
| This [`Cargo.toml`](Cargo.toml) | Axum oneshot shape + `tag` `ssr` (catalog create/list smoke) |
| Product mount `Cargo.toml` (below) | `tag` + `tag-app` with `ssr` / `hydrate` features |
| [`src/main.rs`](src/main.rs) | Catalog create/list, ManyToMany edge shape, protect `/tag` |
| Leptos sketch (below) | `<TagRoutes />` under `/tag` |

### Product mount dependencies

```toml
[dependencies]
tag = { git = "https://github.com/unified-field-dev/tag", package = "tag", rev = "REPLACE_WITH_PIN", default-features = false }
tag-app = { git = "https://github.com/unified-field-dev/tag", package = "tag-app", rev = "REPLACE_WITH_PIN", default-features = false }
uf-product = { /* your pin */, default-features = false }
uf-integrations = { /* your pin */, default-features = false }

[features]
ssr = [
    "tag/ssr",
    "tag-app/ssr",
    "uf-product/ssr",
    "uf-integrations/ssr",
]
hydrate = [
    "tag-app/hydrate",
    "uf-product/hydrate",
    "uf-integrations/hydrate",
]
```

### Leptos mount sketch

```rust,ignore
use tag_app::TagRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <TagRoutes />
    </Routes>
}
```

Catalog CRUD (Leptos-free):

```rust,ignore
use tag::types::TagCreateInput;
use tag::{create, list};

let row = create(
    TagCreateInput {
        name: "Office Supplies".into(),
        taxonomy: Some("spend".into()),
        description: None,
    },
    &valence,
)
.await?;
let rows = list(&valence, None, None).await?;
```

Products attach tags on their own schemas — Tag stores catalog rows only:

```rust,ignore
connections: [
    tags: {
        table: "tag",
        cardinality: ManyToMany,
        edge_table: "demo_note_tag",
        model: "tag::generated::Tag",
    },
]
```

Inventory names match `tag` / `/tag`. `TagRoutes` gates pages with
`RequireAuthenticated` + email verification.

For shell chrome (layout, fonts, Axum + Leptos boot), copy
[`shell-chrome-host`](https://github.com/unified-field-dev/unified-field-product/tree/main/examples/shell-chrome-host)
from unified-field-product, then mount `TagRoutes`.

## Run (documented gate)

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-tag
cargo check -p protected-tag-host
cargo run -p protected-tag-host
```

**Success:** stdout prints `protected_tag_host: OK — /tag deny/allow + catalog + ManyToMany`.

## Hydrate / browser

Out of gate for this host. Full admin UI needs a product binary with
`cargo-leptos`, `wasm32`, session chrome, and a working Orbital / `uf-product`
graph. Prefer the oneshot above.
