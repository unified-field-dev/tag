//! Tag app server functions.

mod error;
mod tag_catalog_search;

use leptos::prelude::*;
use tag::types::{TagCreateInput, TagDetailDto, TagRowDto, TagUpdateInput};

pub use error::{into_server_error, TagServerError};
pub use tag_catalog_search::search_tag_catalog;

#[cfg(feature = "ssr")]
#[allow(clippy::missing_const_for_fn)] // Higgs session lookup is not const
fn require_session(ctx: &higgs::Higgs) -> Result<(), TagServerError> {
    if ctx.session_user_id().is_some() {
        Ok(())
    } else {
        Err(TagServerError::NotAuthenticated)
    }
}

#[cfg(feature = "ssr")]
fn valence_from_ctx(ctx: &higgs::Higgs) -> Result<valence::Valence, TagServerError> {
    ctx.valence()
        .map_err(|e| TagServerError::Valence(e.to_string()))
}

/// List tags, optionally filtered by `search` (name contains) and/or exact `taxonomy`.
#[uf_product_macros::server]
pub async fn list_tags(
    /// Optional case-insensitive search text matched against tag names.
    search: Option<String>,
    /// Optional exact taxonomy to filter by.
    taxonomy: Option<String>,
) -> Result<Vec<TagRowDto>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx).map_err(|e| into_server_error("list_tags", e))?;
    tag::list(
        &valence_from_ctx(&ctx).map_err(|e| into_server_error("list_tags", e))?,
        search,
        taxonomy,
    )
    .await
    .map_err(|e| into_server_error("list_tags", TagServerError::from(e)))
}

/// Load a single tag by id, or `None` if it does not exist.
#[uf_product_macros::server]
pub async fn get_tag(
    /// Unique identifier of the tag to fetch.
    id: String,
) -> Result<Option<TagDetailDto>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx).map_err(|e| into_server_error("get_tag", e))?;
    tag::get(
        &id,
        &valence_from_ctx(&ctx).map_err(|e| into_server_error("get_tag", e))?,
    )
    .await
    .map_err(|e| into_server_error("get_tag", TagServerError::from(e)))
}

/// Create a new tag from the given input.
#[uf_product_macros::server]
pub async fn create_tag(
    /// Fields describing the new tag.
    input: TagCreateInput,
) -> Result<TagDetailDto, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx).map_err(|e| into_server_error("create_tag", e))?;
    tag::create(
        input,
        &valence_from_ctx(&ctx).map_err(|e| into_server_error("create_tag", e))?,
    )
    .await
    .map_err(|e| into_server_error("create_tag", TagServerError::from(e)))
}

/// Apply a partial update to an existing tag.
#[uf_product_macros::server]
pub async fn update_tag(
    /// Unique identifier of the tag to update.
    id: String,
    /// Partial update to apply to the tag.
    input: TagUpdateInput,
) -> Result<TagDetailDto, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx).map_err(|e| into_server_error("update_tag", e))?;
    tag::update(
        &id,
        input,
        &valence_from_ctx(&ctx).map_err(|e| into_server_error("update_tag", e))?,
    )
    .await
    .map_err(|e| into_server_error("update_tag", TagServerError::from(e)))
}

/// Delete a tag by id.
#[uf_product_macros::server]
pub async fn delete_tag(
    /// Unique identifier of the tag to delete.
    id: String,
) -> Result<(), ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx).map_err(|e| into_server_error("delete_tag", e))?;
    tag::delete(
        &id,
        &valence_from_ctx(&ctx).map_err(|e| into_server_error("delete_tag", e))?,
    )
    .await
    .map_err(|e| into_server_error("delete_tag", TagServerError::from(e)))
}
