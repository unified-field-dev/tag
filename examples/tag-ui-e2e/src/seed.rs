//! Harness-only seed endpoint for Playwright.

use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::e2e_valence::{e2e_fixtures, seed_fresh_owner_tag};
use crate::gate_demos::{write_e2e_auth_kind, E2eAuthKind};

#[derive(Debug, Deserialize)]
pub struct SeedRequest {
    /// `anonymous` | `owner` | `peer` | `unverified`
    #[serde(default = "default_auth")]
    pub auth: String,
    /// When true, mint a fresh owner-owned catalog tag and update fixtures.
    #[serde(default)]
    pub seed_catalog: bool,
}

fn default_auth() -> String {
    E2eAuthKind::Anonymous.as_str().to_string()
}

pub async fn seed_data(
    session: tower_sessions::Session,
    Json(body): Json<SeedRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let kind = E2eAuthKind::parse(&body.auth);
    write_e2e_auth_kind(&session, kind)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let fixtures = if body.seed_catalog {
        let name = format!("Fresh Tag {}", chrono::Utc::now().timestamp_millis());
        seed_fresh_owner_tag(&name).await.map_err(|e| {
            log::error!("seed_catalog failed: {e:#}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    } else {
        e2e_fixtures()
    };

    Ok(Json(serde_json::json!({
        "ok": true,
        "auth": kind.as_str(),
        "fixtures": {
            "tag_id": fixtures.tag_id,
            "tag_name": fixtures.tag_name,
            "peer_tag_id": fixtures.peer_tag_id,
            "peer_tag_name": fixtures.peer_tag_name,
        }
    })))
}
