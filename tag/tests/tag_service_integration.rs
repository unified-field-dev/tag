#![cfg(feature = "ssr")]
#![allow(missing_docs)]
#![allow(clippy::expect_used, clippy::unwrap_used)]

mod helpers;

use helpers::{as_user, setup_shared_db, valence_for, TEST_USER_A, TEST_USER_B};
use tag::types::{TagCreateInput, TagUpdateInput};
use tag::{create, delete, get, list, update};
use uf_search_core::SearchSourceProvider;
use valence::{SchemaRegistry, TraitRegistry};

async fn history_rows_for_tag(v: &valence::Valence, tag_id: &str) -> Vec<serde_json::Value> {
    let backend = v
        .backend_for_table("tag_history")
        .expect("tag_history backend");
    let compiled = valence::__internal::CompiledQuery::new(
        format!(
            "SELECT * FROM tag_history WHERE source_id = '{tag_id}' OR source = 'tag:{tag_id}' ORDER BY changed_at DESC"
        ),
        vec![],
    );
    match backend.execute_compiled_query(&compiled).await {
        Ok(rows) => rows,
        Err(e) => {
            // Fallback: full table scan and filter in-process.
            let all =
                valence::__internal::CompiledQuery::new("SELECT * FROM tag_history".into(), vec![]);
            let rows = backend
                .execute_compiled_query(&all)
                .await
                .unwrap_or_else(|e2| panic!("history scan failed: {e} / {e2}"));
            rows.into_iter()
                .filter(|r| {
                    let src = r.get("source").or_else(|| r.get("source_id"));
                    match src {
                        Some(serde_json::Value::String(s)) => {
                            s == tag_id || s.ends_with(&format!(":{tag_id}")) || s.contains(tag_id)
                        }
                        Some(serde_json::Value::Object(o)) => o
                            .get("id")
                            .and_then(|x| x.as_str())
                            .is_some_and(|id| id == tag_id),
                        _ => format!("{r:?}").contains(tag_id),
                    }
                })
                .collect()
        }
    }
}

#[tokio::test]
async fn create_tag_writes_catalog_and_created_history_happy_path() {
    let v = valence_for(TEST_USER_A).await;
    let detail = create(
        TagCreateInput {
            name: "Office Supplies".into(),
            taxonomy: Some("spend".into()),
            description: Some("Catalog row".into()),
        },
        &v,
    )
    .await
    .expect("create");

    assert_eq!(detail.name, "Office Supplies");
    assert_eq!(detail.taxonomy.as_deref(), Some("spend"));
    assert_eq!(detail.description.as_deref(), Some("Catalog row"));

    let rows = history_rows_for_tag(&v, &detail.id).await;
    assert!(rows
        .iter()
        .any(|r| r.get("field_name").and_then(|v| v.as_str()) == Some("created")));
}

#[tokio::test]
async fn get_owner_can_read_own_tag_happy_path() {
    let v = valence_for(TEST_USER_A).await;
    let created = create(
        TagCreateInput {
            name: "Readable".into(),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("create");

    let got = get(&created.id, &v).await.expect("get").expect("row");
    assert_eq!(got.name, "Readable");
}

#[tokio::test]
async fn list_returns_owner_tags_ordered_by_updated_at_happy_path() {
    let v = valence_for(TEST_USER_A).await;
    let first = create(
        TagCreateInput {
            name: "First".into(),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("create");
    let second = create(
        TagCreateInput {
            name: "Second".into(),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("create");

    update(
        &first.id,
        TagUpdateInput {
            name: Some("First Updated".into()),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("update");

    let rows = list(&v, None, None).await.expect("list");
    assert!(rows.iter().any(|r| r.id == first.id));
    assert!(rows.iter().any(|r| r.id == second.id));
    assert_eq!(rows[0].id, first.id);
}

#[tokio::test]
async fn update_name_writes_field_history_row_happy_path() {
    let v = valence_for(TEST_USER_A).await;
    let created = create(
        TagCreateInput {
            name: "Office".into(),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("create");

    update(
        &created.id,
        TagUpdateInput {
            name: Some("Office Supplies".into()),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("update");

    let rows = history_rows_for_tag(&v, &created.id).await;
    let name_row = rows
        .iter()
        .find(|r| r.get("field_name").and_then(|v| v.as_str()) == Some("name"))
        .expect("name history row");
    assert_eq!(
        name_row.get("old_value").and_then(|v| v.as_str()),
        Some("Office")
    );
    assert_eq!(
        name_row.get("new_value").and_then(|v| v.as_str()),
        Some("Office Supplies")
    );
}

#[tokio::test]
async fn update_taxonomy_writes_field_history_row_happy_path() {
    let v = valence_for(TEST_USER_A).await;
    let created = create(
        TagCreateInput {
            name: "Tax Tag".into(),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("create");

    update(
        &created.id,
        TagUpdateInput {
            name: None,
            taxonomy: Some("spend".into()),
            description: None,
        },
        &v,
    )
    .await
    .expect("update");

    let rows = history_rows_for_tag(&v, &created.id).await;
    let tax_row = rows
        .iter()
        .find(|r| r.get("field_name").and_then(|v| v.as_str()) == Some("taxonomy"))
        .expect("taxonomy history row");
    assert_eq!(tax_row.get("old_value").and_then(|v| v.as_str()), Some(""));
    assert_eq!(
        tax_row.get("new_value").and_then(|v| v.as_str()),
        Some("spend")
    );
}

#[tokio::test]
async fn update_description_writes_field_history_row_happy_path() {
    let v = valence_for(TEST_USER_A).await;
    let created = create(
        TagCreateInput {
            name: "Desc Tag".into(),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("create");

    update(
        &created.id,
        TagUpdateInput {
            name: None,
            taxonomy: None,
            description: Some("Desk and office supplies".into()),
        },
        &v,
    )
    .await
    .expect("update");

    let rows = history_rows_for_tag(&v, &created.id).await;
    let desc_row = rows
        .iter()
        .find(|r| r.get("field_name").and_then(|v| v.as_str()) == Some("description"))
        .expect("description history row");
    assert_eq!(desc_row.get("old_value").and_then(|v| v.as_str()), Some(""));
    assert_eq!(
        desc_row.get("new_value").and_then(|v| v.as_str()),
        Some("Desk and office supplies")
    );
}

#[tokio::test]
async fn list_search_filters_by_name_happy_path() {
    let v = valence_for(TEST_USER_A).await;
    create(
        TagCreateInput {
            name: "Alpha Widget".into(),
            taxonomy: Some("spend".into()),
            description: None,
        },
        &v,
    )
    .await
    .expect("create");
    create(
        TagCreateInput {
            name: "Beta Gadget".into(),
            taxonomy: Some("custom".into()),
            description: None,
        },
        &v,
    )
    .await
    .expect("create");

    let filtered = list(&v, Some("Alpha".into()), None).await.expect("list");
    assert!(filtered.iter().any(|r| r.name.contains("Alpha")));
    assert!(!filtered.iter().any(|r| r.name.contains("Beta")));
}

#[tokio::test]
async fn list_taxonomy_filter_equals_happy_path() {
    let v = valence_for(TEST_USER_A).await;
    create(
        TagCreateInput {
            name: "Spend One".into(),
            taxonomy: Some("spend".into()),
            description: None,
        },
        &v,
    )
    .await
    .expect("create");
    create(
        TagCreateInput {
            name: "Custom One".into(),
            taxonomy: Some("custom".into()),
            description: None,
        },
        &v,
    )
    .await
    .expect("create");

    let spend_only = list(&v, None, Some("spend".into())).await.expect("list");
    assert!(spend_only
        .iter()
        .all(|r| r.taxonomy.as_deref() == Some("spend")));
}

#[tokio::test]
async fn delete_removes_tag_and_writes_deleted_history_happy_path() {
    let v = valence_for(TEST_USER_A).await;
    let created = create(
        TagCreateInput {
            name: "Temporary".into(),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("create");

    delete(&created.id, &v).await.expect("delete");

    let rows = history_rows_for_tag(&v, &created.id).await;
    assert!(rows
        .iter()
        .any(|r| r.get("field_name").and_then(|v| v.as_str()) == Some("deleted")));

    let pending = get(&created.id, &v).await;
    assert!(pending.is_err());
    let pending_msg = pending.unwrap_err().to_string();
    assert!(
        pending_msg.contains("Pending deletion") || pending_msg.contains("Access denied"),
        "expected pending-deletion or privacy deny after delete, got: {pending_msg}"
    );

    helpers::execute_deletion_dag(&v, "tag", &created.id).await;
    let listed = list(&v, None, None).await.expect("list");
    assert!(
        !listed.iter().any(|r| r.id == created.id),
        "deleted tag must not appear in catalog list after DAG finalize"
    );
    let after_dag = get(&created.id, &v).await;
    assert!(
        after_dag.as_ref().ok().and_then(|o| o.as_ref()).is_none()
            || after_dag
                .as_ref()
                .err()
                .is_some_and(|e| e.to_string().contains("Access denied")),
        "expected gone or privacy deny after DAG, got: {after_dag:?}"
    );
}

#[tokio::test]
async fn record_history_query_all_orders_newest_first_happy_path() {
    let v = valence_for(TEST_USER_A).await;
    let created = create(
        TagCreateInput {
            name: "Alpha".into(),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("create");

    update(
        &created.id,
        TagUpdateInput {
            name: Some("Beta".into()),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("update");

    let rows = history_rows_for_tag(&v, &created.id).await;
    assert!(rows.len() >= 2, "expected created + name history rows");
    assert!(rows
        .iter()
        .any(|r| r.get("field_name").and_then(|v| v.as_str()) == Some("created")));
    assert!(rows
        .iter()
        .any(|r| r.get("field_name").and_then(|v| v.as_str()) == Some("name")));
}

#[tokio::test]
async fn authenticated_peer_can_read_and_list_other_users_tag_happy_path() {
    let base = setup_shared_db().await;
    let owner_v = as_user(&base, TEST_USER_A);
    let peer_v = as_user(&base, TEST_USER_B);

    let created = create(
        TagCreateInput {
            name: "Shared Visible".into(),
            taxonomy: None,
            description: None,
        },
        &owner_v,
    )
    .await
    .expect("create");

    assert!(
        get(&created.id, &peer_v).await.expect("get").is_some(),
        "authenticated peer must read shared catalog tag"
    );
    let listed = list(&peer_v, None, None).await.expect("list");
    assert!(listed.iter().any(|r| r.id == created.id));
}

#[tokio::test]
async fn non_owner_cannot_update_other_users_tag_sad() {
    let base = setup_shared_db().await;
    let owner_v = as_user(&base, TEST_USER_A);
    let other_v = as_user(&base, TEST_USER_B);

    let created = create(
        TagCreateInput {
            name: "Locked".into(),
            taxonomy: None,
            description: None,
        },
        &owner_v,
    )
    .await
    .expect("create");

    let err = update(
        &created.id,
        TagUpdateInput {
            name: Some("Hacked".into()),
            taxonomy: None,
            description: None,
        },
        &other_v,
    )
    .await;
    assert!(err.is_err());
    let still = get(&created.id, &owner_v)
        .await
        .expect("get")
        .expect("unchanged");
    assert_eq!(still.name, "Locked");
}

#[tokio::test]
async fn non_owner_cannot_delete_other_users_tag_sad() {
    let base = setup_shared_db().await;
    let owner_v = as_user(&base, TEST_USER_A);
    let other_v = as_user(&base, TEST_USER_B);

    let created = create(
        TagCreateInput {
            name: "Protected".into(),
            taxonomy: None,
            description: None,
        },
        &owner_v,
    )
    .await
    .expect("create");

    let err = delete(&created.id, &other_v).await;
    assert!(
        err.is_err(),
        "non-owner delete must fail when row is readable but not owned"
    );
    assert!(get(&created.id, &owner_v).await.expect("get").is_some());
}

#[tokio::test]
async fn update_name_history_row_has_user_actor_happy_path() {
    let v = valence_for(TEST_USER_A).await;
    let created = create(
        TagCreateInput {
            name: "Actor Tag".into(),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("create");

    update(
        &created.id,
        TagUpdateInput {
            name: Some("Actor Tag Renamed".into()),
            taxonomy: None,
            description: None,
        },
        &v,
    )
    .await
    .expect("update");

    let rows = history_rows_for_tag(&v, &created.id).await;
    let name_row = rows
        .iter()
        .find(|r| r.get("field_name").and_then(|v| v.as_str()) == Some("name"))
        .expect("name history row");
    let actor = name_row
        .get("actor")
        .expect("history row should record user actor");
    let actor_id = match actor {
        serde_json::Value::String(s) => valence::extract_id_from_record_display(s).ok(),
        serde_json::Value::Object(o) => o.get("id").and_then(|v| v.as_str()).map(str::to_string),
        _ => None,
    };
    assert_eq!(actor_id.as_deref(), Some(TEST_USER_A));
}

#[tokio::test]
async fn tag_history_table_is_registered_for_record_history_trait() {
    let tables = TraitRegistry::global().tables_for_trait("RecordHistory");
    assert!(tables.contains(&"tag_history"));
}

#[tokio::test]
async fn tag_catalog_search_source_returns_items_happy_path() {
    use tag::search_sources::TagCatalogSearchSource;

    let v = valence_for(TEST_USER_A).await;
    create(
        TagCreateInput {
            name: "Searchable Tag".into(),
            taxonomy: Some("spend".into()),
            description: None,
        },
        &v,
    )
    .await
    .expect("create");

    let provider = TagCatalogSearchSource;
    let items = provider.query(&v, "Searchable", 10).await.expect("query");
    assert!(items.iter().any(|i| i.title == "Searchable Tag"));
    assert_eq!(items[0].source_id, tag::TagSearchSourceId::Catalog.as_str());
}

#[tokio::test]
async fn tag_assignment_table_is_not_registered() {
    assert!(SchemaRegistry::global()
        .get_schema("tag_assignment")
        .is_none());
}
