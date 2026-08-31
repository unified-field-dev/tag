//! List page result body (loading / error / empty / table).

use leptos::prelude::*;
use leptos_router::components::A;
use tag::types::TagRowDto;
use uf_product::components::{
    Body1, Button, ButtonAppearance, EmptyState, Flex, FlexGap, FlexJustify, MessageBar,
    MessageBarIntent, SpacingSize,
};

use crate::components::TagListTable;

/// Transition body for the tag list card.
#[component]
pub(crate) fn TagListResults(
    tags: Resource<Result<Vec<TagRowDto>, ServerFnError>>,
    filters_active: Signal<bool>,
    on_clear: Callback<()>,
) -> impl IntoView {
    view! {
        <Transition fallback=move || view! {
            <Flex padding=SpacingSize::Size200.inset()>
                <Body1>"Loading tags..."</Body1>
            </Flex>
        }>
            {move || match tags.get() {
                None => view! {
                    <Flex padding=SpacingSize::Size200.inset()>
                        <Body1>"Loading tags..."</Body1>
                    </Flex>
                }.into_any(),
                Some(Err(err)) => view! {
                    <Flex padding=SpacingSize::Size200.inset()>
                        <MessageBar intent=MessageBarIntent::Error>{err.to_string()}</MessageBar>
                    </Flex>
                }.into_any(),
                Some(Ok(items)) if items.is_empty() && filters_active.get() => view! {
                    <Flex
                        vertical=true
                        gap=FlexGap::Medium
                        padding=SpacingSize::Size240.inset()
                    >
                        <div data-testid="tag-list-filtered-empty">
                            <MessageBar intent=MessageBarIntent::Info>
                                "No tags match your filters."
                            </MessageBar>
                        </div>
                        <Flex justify=FlexJustify::Center>
                            <div data-testid="tag-list-clear-filters">
                                <Button appearance=ButtonAppearance::Subtle on_click=Callback::new(move |_| on_clear.run(()))>
                                    "Clear filters"
                                </Button>
                            </div>
                        </Flex>
                    </Flex>
                }.into_any(),
                Some(Ok(items)) if items.is_empty() => view! {
                    <Flex
                        vertical=true
                        gap=FlexGap::Medium
                        padding=SpacingSize::Size240.inset()
                    >
                        <EmptyState
                            message="No tags yet"
                            description="Create your first tag to organize records across apps."
                        />
                        <Flex justify=FlexJustify::Center>
                            <A href="/tag/create">
                                <Button appearance=ButtonAppearance::Primary attr:data-testid="tag-create-button">
                                    "Create tag"
                                </Button>
                            </A>
                        </Flex>
                    </Flex>
                }.into_any(),
                Some(Ok(items)) => view! {
                    <TagListTable items=items />
                }.into_any(),
            }}
        </Transition>
    }
}
