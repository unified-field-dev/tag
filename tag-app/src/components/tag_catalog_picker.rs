//! Multi-select tag catalog picker (Thaw `TagPicker` + search-source data).

use leptos::prelude::*;
use leptos::task::spawn_local_scoped;
use leptos_router::components::A;
use orbital_macros::component_doc;
use tag::TagSearchSourceId;
use uf_product::components::{
    Caption1, Field, Flex, FlexGap, Input, InputAppearance, Tag, TagPicker, TagPickerBind,
    TagPickerControl, TagPickerGroup, TagPickerInput, TagPickerOption, TagPickerSize,
};
use uf_search_core::{SearchSourceItem, SearchSourceKey};

use crate::server::search_tag_catalog;

const CATALOG_LIMIT: u32 = 50;

fn title_for_id(options: &[SearchSourceItem], id: &str) -> String {
    options
        .iter()
        .find(|item| item.id == id)
        .map_or_else(|| id.to_string(), |item| item.title.clone())
}

/// Multi-select shared tag catalog (TagPicker + tag search source).
///
/// # When to use
///
/// - Connection or association UIs that pick tags from the shared catalog
/// - Forms where selected tag ids are stored and synced via `on_change`
///
/// For custom tag chips or a generic multi-select without the shared catalog, use
/// **Data Display** (`Tag`, `Tag Group`) and **Inputs** (`Tag Picker`) previews.
///
/// # Usage
///
/// Bind `selected` to the current tag id list and wire `on_change` to your connection APIs.
/// Type in the catalog search field to refetch options via
/// [`search_tag_catalog`](crate::server::search_tag_catalog); `TagPickerInput` still
/// filters the loaded option list locally.
#[component_doc(
    category = "Unified Field",
    preview_slug = "tag-catalog-picker",
    preview_label = "Tag Catalog Picker",
    preview_icon = icondata::AiTagOutlined,
    preview = "manual",
)]
#[component]
pub fn TagCatalogPicker(
    selected: RwSignal<Vec<String>>,
    on_change: Callback<Vec<String>>,
    #[prop(optional)] taxonomy_filter: Option<String>,
    #[prop(optional, into)] manage_tags_href: Option<String>,
) -> impl IntoView {
    let manage_href = manage_tags_href.unwrap_or_else(|| "/tag".to_string());
    let taxonomy_filter = StoredValue::new(taxonomy_filter);
    let catalog_options = RwSignal::new(Vec::<SearchSourceItem>::new());
    let error = RwSignal::new(None::<String>);
    let request_seq = RwSignal::new(0u32);
    let query = RwSignal::new(String::new());

    let source_keys = Signal::derive(|| {
        vec![SearchSourceKey::new(
            TagSearchSourceId::Catalog.as_str(),
            TagSearchSourceId::Catalog.label(),
        )]
    });

    let fetch_catalog = move |query: Option<String>| {
        let id = request_seq.get_untracked().saturating_add(1);
        request_seq.set(id);
        let taxonomy = taxonomy_filter.get_value();
        spawn_local_scoped(async move {
            match search_tag_catalog(source_keys.get_untracked(), query, taxonomy, CATALOG_LIMIT)
                .await
            {
                Ok(rows) if request_seq.get_untracked() == id => {
                    catalog_options.set(rows);
                    error.set(None);
                }
                Err(err) if request_seq.get_untracked() == id => {
                    error.set(Some(err.to_string()));
                }
                Ok(_) | Err(_) => {}
            }
        });
    };

    Effect::new(move |_| {
        let q = query.get();
        let arg = if q.trim().is_empty() { None } else { Some(q) };
        fetch_catalog(arg);
    });

    Effect::new(move |_| {
        on_change.run(selected.get());
    });

    view! {
        <div data-testid="tag-catalog-picker">
            <Flex vertical=true gap=FlexGap::Small>
            <Field label="Search catalog">
                <div data-testid="tag-catalog-picker-search">
                    <Input
                        bind=query
                        appearance=InputAppearance::with_placeholder("Filter tags by name...")
                    />
                </div>
            </Field>
            <TagPicker bind=TagPickerBind::new(selected) size=Signal::from(TagPickerSize::Medium)>
                <TagPickerControl slot>
                    <TagPickerGroup>
                        <For
                            each=move || selected.get()
                            key=|id| id.clone()
                            let:id
                        >
                            <Tag value=id.clone()>
                                {move || title_for_id(&catalog_options.get(), &id)}
                            </Tag>
                        </For>
                    </TagPickerGroup>
                    <TagPickerInput />
                </TagPickerControl>
                <For
                    each=move || catalog_options.get()
                    key=|item| item.id.clone()
                    let:item
                >
                    <TagPickerOption value=item.id.clone() text=item.title />
                </For>
            </TagPicker>
            <div data-testid="tag-catalog-picker-manage">
                <A href=manage_href>
                    <Caption1>"Manage tags →"</Caption1>
                </A>
            </div>
            {move || error.get().map(|msg| view! {
                <Caption1>{msg}</Caption1>
            })}
            </Flex>
        </div>
    }
}
