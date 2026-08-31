//! Tag catalog CRUD service (SSR-only, no product connection helpers).
//!
//! This is the domain layer called by `tag-app`'s server functions; it owns
//! Valence reads/writes, owner-display resolution, and history side effects.
//!
//! # Errors
//!
//! Public CRUD returns [`TagError`]. See the crate-root Concern → API table
//! for the split with `ServerFnError` (app) and Valence privacy denials.

mod error;
mod service;

pub use error::TagError;
pub use service::*;
