//! Reusable UI components for the Tags app (catalog picker, list filters/table).
//!
//! Hosts should prefer [`TagCatalogPicker`]. List filter/table helpers are
//! `pub(crate)` for the catalog pages.

mod tag_catalog_picker;
mod tag_list_filters;
mod tag_list_results;
mod tag_list_table;

pub use tag_catalog_picker::TagCatalogPicker;
#[cfg(feature = "preview")]
pub use tag_catalog_picker::{TAGCATALOGPICKER_DOC, TAGCATALOGPICKER_PROPS};
pub(crate) use tag_list_filters::TagListFilters;
pub(crate) use tag_list_results::TagListResults;
pub(crate) use tag_list_table::TagListTable;
