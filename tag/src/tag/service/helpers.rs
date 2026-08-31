//! DTO mapping and ownership helpers for catalog CRUD.

use valence::ownership::OwnershipService;
use valence::Valence;

use crate::generated::Tag;
use crate::types::{TagDetailDto, TagRowDto};

use super::TagError;

pub(super) async fn owner_display(tag_id: &str, v: &Valence) -> String {
    let bare = valence::ownership::normalize_record_id_for_ownership(tag_id);
    if let Some(uid) = v.actor().user_id() {
        let actor_owner = valence::ownership::normalize_record_id_for_ownership(uid);
        if let Ok(Some(row)) = OwnershipService::get_ownership_json("tag", &bare, v).await {
            let owner_id = row
                .get("owner_id")
                .and_then(|x| x.as_str())
                .map(valence::ownership::normalize_record_id_for_ownership);
            if owner_id.as_deref() == Some(actor_owner.as_str()) {
                return "you".to_string();
            }
            if let Some(owner_id) = owner_id {
                return owner_id;
            }
        }
    }
    "owner".to_string()
}

pub(super) fn record_id_str(tag: &Tag) -> String {
    tag.id()
        .and_then(|r| valence::extract_id_from_record(r).ok())
        .unwrap_or_default()
}

pub(super) fn to_row_dto(tag: &Tag, owner_display: String) -> TagRowDto {
    TagRowDto {
        id: record_id_str(tag),
        name: tag.name().clone(),
        taxonomy: tag.taxonomy().cloned(),
        description: tag.description().cloned(),
        owner_display,
        updated_at: *tag.updated_at(),
    }
}

pub(super) fn to_detail_dto(tag: &Tag, owner_display: String) -> TagDetailDto {
    TagDetailDto {
        id: record_id_str(tag),
        name: tag.name().clone(),
        taxonomy: tag.taxonomy().cloned(),
        description: tag.description().cloned(),
        owner_display,
        created_at: *tag.created_at(),
        updated_at: *tag.updated_at(),
    }
}

pub(super) fn to_detail_dto_with_id(tag: &Tag, id: &str, owner_display: String) -> TagDetailDto {
    let mut dto = to_detail_dto(tag, owner_display);
    if dto.id.is_empty() {
        dto.id = id.to_string();
    }
    dto
}

pub(super) async fn ensure_caller_may_delete_tag(id: &str, v: &Valence) -> Result<(), TagError> {
    if v.actor().user_id().is_none() {
        return Ok(());
    }
    let Some(user_id) = v.actor().user_id() else {
        return Ok(());
    };
    let bare = valence::ownership::normalize_record_id_for_ownership(id);
    let actor_owner = valence::ownership::normalize_record_id_for_ownership(user_id);
    let ownership = OwnershipService::get_ownership_json("tag", &bare, v)
        .await
        .map_err(|e| TagError::service("delete", e))?
        .ok_or_else(|| TagError::access_denied("tag::TAG_DATA_OWNER"))?;
    let owner_id = ownership
        .get("owner_id")
        .and_then(|x| x.as_str())
        .map(valence::ownership::normalize_record_id_for_ownership);
    if owner_id.as_deref() == Some(actor_owner.as_str()) {
        Ok(())
    } else {
        Err(TagError::access_denied("tag::TAG_DATA_OWNER"))
    }
}
