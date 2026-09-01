# tag-app

Orbital admin UI for the shared tag catalog (`TagRoutes` at `/tag`).

## Host integration

Mount `<TagRoutes />` under the host `<Routes>`. Domain CRUD and history writers
live in the `tag` crate; this package wraps them for operators and exposes
`TagCatalogPicker` for other product forms. Shell chrome comes from
`uf-integrations` (same pattern as counter-app).

## Documentation

- Crate rustdoc: `cargo doc -p tag-app --features ssr --open` (Organized by
  task, Owns, Concern → API, Examples)
- Root [`README.md`](../README.md)

## Tests

In-crate unit coverage is intentionally thin (error-mapping tests under
`server`). Behavioral contracts live in the sibling `tag` crate
(`product_surface`, `tag_crud_contract`, `tag_service_integration`) and
Playwright in `examples/tag-ui-e2e`.
