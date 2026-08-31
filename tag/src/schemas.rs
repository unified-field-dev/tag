//! Trait schema includes for inventory. Entity `valence_schema!` files are
//! codegen inputs only (`build.rs`); including them here would double-register
//! stub `SchemaMetadata` (entity fields only) and overwrite trait-merged codegen.
//!
//! Record-history trait sources are vendored under `tag/schemas/` so standalone
//! checkout (no monorepo sibling) can compile and rustdoc.

#[cfg(feature = "ssr")]
mod record_history_trait {
    include!("../schemas/record_history_valence_trait.rs");
}

#[cfg(feature = "ssr")]
mod history_source_trait {
    include!("../schemas/history_source_valence_trait.rs");
}
