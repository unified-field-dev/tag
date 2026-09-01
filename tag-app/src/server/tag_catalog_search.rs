//! Catalog search for tag picker (search-source pattern).

use leptos::prelude::*;
use uf_search_core::{SearchSourceItem, SearchSourceKey};

#[cfg(feature = "ssr")]
use super::{into_server_error, require_session, valence_from_ctx, TagServerError};

/// Upper bound for picker catalog search results (resource abuse guard).
#[cfg(feature = "ssr")]
const SEARCH_TAG_CATALOG_MAX: u32 = 50;

/// Search the tag catalog for [`uf_integrations::SearchSourcePicker`] /
/// [`TagCatalogPicker`](crate::components::TagCatalogPicker) search-source pattern.
///
/// `source_keys` must be empty or only [`tag::TagSearchSourceId::Catalog`]; other
/// source ids return an invalid-source [`ServerFnError`].
#[uf_product_macros::server]
pub async fn search_tag_catalog(
    /// Search sources requested by the picker (catalog only for this fn).
    source_keys: Vec<SearchSourceKey>,
    /// Optional free-text query matched against tag names.
    query: Option<String>,
    /// Optional exact taxonomy to filter by.
    taxonomy: Option<String>,
    /// Maximum number of results requested by the picker (clamped to 1..=50).
    limit: u32,
) -> Result<Vec<SearchSourceItem>, ServerFnError> {
    let catalog_id = tag::TagSearchSourceId::Catalog.as_str();
    for key in &source_keys {
        if key.id != catalog_id {
            return Err(into_server_error(
                "search_tag_catalog",
                TagServerError::InvalidSource(key.id.clone()),
            ));
        }
    }

    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx).map_err(|e| into_server_error("search_tag_catalog", e))?;
    let v = valence_from_ctx(&ctx).map_err(|e| into_server_error("search_tag_catalog", e))?;

    let search = query.filter(|q| !q.trim().is_empty());
    let tax = taxonomy.filter(|t| !t.trim().is_empty());
    let take = limit.clamp(1, SEARCH_TAG_CATALOG_MAX) as usize;

    let rows = tag::list(&v, search, tax)
        .await
        .map_err(|e| into_server_error("search_tag_catalog", TagServerError::from(e)))?;

    Ok(
        tag::search_sources::catalog_items_from_list(rows, catalog_id)
            .into_iter()
            .take(take)
            .collect(),
    )
}
