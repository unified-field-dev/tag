//! Typed errors for tag catalog CRUD.

use std::fmt;

/// Library-facing failures from [`super::create`], [`super::update`],
/// [`super::delete`], [`super::get`], and [`super::list`].
///
/// Distinct variants keep not-found and ownership denials inspectable before
/// `tag-app` collapses them into `ServerFnError`. Valence / history failures
/// land in [`Self::Service`].
#[derive(Debug)]
pub enum TagError {
    /// No tag row for the given bare id (update path).
    NotFound {
        /// Bare Valence record id (safe to log).
        id: String,
    },
    /// Caller failed an ownership / privacy check.
    AccessDenied {
        /// Policy rule name (e.g. `tag::TAG_DATA_OWNER`).
        policy: &'static str,
    },
    /// Underlying Valence, history, or ownership service failure.
    Service {
        /// Operation label (e.g. `create`, `delete`, `list`).
        operation: &'static str,
        /// Source error.
        source: anyhow::Error,
    },
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { id } => write!(f, "tag not found: {id}"),
            Self::AccessDenied { policy } => {
                write!(f, "Access denied by policy: {policy}")
            }
            Self::Service { operation, source } => {
                write!(f, "tag catalog {operation} failed: {source}")
            }
        }
    }
}

impl std::error::Error for TagError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Service { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl TagError {
    pub(crate) fn not_found(id: impl Into<String>) -> Self {
        Self::NotFound { id: id.into() }
    }

    pub(crate) const fn access_denied(policy: &'static str) -> Self {
        Self::AccessDenied { policy }
    }

    pub(crate) fn service(operation: &'static str, source: impl Into<anyhow::Error>) -> Self {
        Self::Service {
            operation,
            source: source.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TagError;
    use std::error::Error;

    #[test]
    fn display_and_source_for_variants() {
        let missing = TagError::not_found("abc");
        assert!(missing.to_string().contains("tag not found"));
        assert!(missing.to_string().contains("abc"));
        assert!(missing.source().is_none());

        let denied = TagError::access_denied("tag::TAG_DATA_OWNER");
        assert!(denied.to_string().contains("Access denied"));
        assert!(denied.to_string().contains("tag::TAG_DATA_OWNER"));
        assert!(denied.source().is_none());

        let service = TagError::service("create", anyhow::anyhow!("backend down"));
        let msg = service.to_string();
        assert!(msg.contains("create"));
        assert!(msg.contains("backend down"));
        assert!(service.source().is_some());
    }
}
