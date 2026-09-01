//! Tag catalog list page.

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;
use uf_product::components::{
    Button, ButtonAppearance, Card, ContentContainer, Flex, FlexAlign, FlexGap, FlexJustify,
    SpacingSize, Title3,
};

use crate::components::{TagListFilters, TagListResults};
use crate::server::list_tags;

/// Tag catalog list: search/taxonomy filters, empty states, and a create-tag entry point.
#[component]
pub fn TagListPage() -> impl IntoView {
    let location = use_location();
    let search = RwSignal::new(String::new());
    let taxonomy = RwSignal::new(String::new());
    let refresh = RwSignal::new(0u32);

    let on_clear = Callback::new(move |()| {
        search.set(String::new());
        taxonomy.set(String::new());
        refresh.update(|n| *n += 1);
    });

    let tags = Resource::new(
        move || {
            (
                search.get(),
                taxonomy.get(),
                refresh.get(),
                location.pathname.get(),
            )
        },
        |(search_val, tax_val, _, _)| async move {
            let search_arg = if search_val.trim().is_empty() {
                None
            } else {
                Some(search_val)
            };
            let tax_arg = if tax_val.trim().is_empty() {
                None
            } else {
                Some(tax_val)
            };
            list_tags(search_arg, tax_arg).await
        },
    );

    let filters_active =
        move || !search.get().trim().is_empty() || !taxonomy.get().trim().is_empty();

    view! {
        <ContentContainer max_width="1100px" data_testid="tag-list-page">
            <Flex vertical=true gap=FlexGap::Medium>
                <Card>
                    <Flex
                        justify=FlexJustify::SpaceBetween
                        align=FlexAlign::Center
                        gap=FlexGap::Medium
                        padding=SpacingSize::Size200.inset()
                    >
                        <Title3>"Tags"</Title3>
                        <A href="/tag/create">
                            <Button appearance=ButtonAppearance::Primary attr:data-testid="tag-create-button">
                                "Create tag"
                            </Button>
                        </A>
                    </Flex>
                    <TagListFilters search=search taxonomy=taxonomy on_clear=on_clear />
                </Card>

                <Card>
                    <TagListResults tags=tags filters_active=Signal::derive(filters_active) on_clear=on_clear />
                </Card>
            </Flex>
        </ContentContainer>
    }
}
