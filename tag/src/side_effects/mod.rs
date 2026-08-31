//! Valence [`valence::SideEffect`] implementations for the tag domain.
//!
//! [`TagHistoryWriter`] appends `tag_history` rows. Failures stay as `anyhow`
//! here; the catalog service maps them to [`crate::TagError::Service`].

/// [`TagHistoryWriter`] — appends `tag_history` rows on tag mutations.
pub mod history_writer;

pub use history_writer::TagHistoryWriter;
