//! Logical embedded database name for tag Valence schemas.

use valence::{Database, DatabaseFromEngine, SQLITE_ENGINE_ID};

/// Logical database name tag schemas are registered under.
pub const DEFAULT_LOGICAL_NAME: &str = "default";

/// [`DatabaseFromEngine`] pointing at [`DEFAULT_LOGICAL_NAME`] on the embedded `SQLite` engine.
pub const DEFAULT_STORAGE: DatabaseFromEngine =
    Database::from_engine(DEFAULT_LOGICAL_NAME, SQLITE_ENGINE_ID);

/// Logical names test/server routers should link for `tag` models to resolve.
pub const EMBEDDED_SURREAL_LOGICAL_NAMES: &[&str] = &[DEFAULT_LOGICAL_NAME];
