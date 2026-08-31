//! List page filter controls.

use leptos::prelude::*;
use uf_product::components::{
    Button, ButtonAppearance, Field, Flex, FlexAlign, FlexGap, FlexWrap, Input, InputAppearance,
    SpacingSize,
};

/// Search/taxonomy filter row above the tag list, with a clear-filters action.
#[component]
pub(crate) fn TagListFilters(
    search: RwSignal<String>,
    taxonomy: RwSignal<String>,
    on_clear: Callback<()>,
) -> impl IntoView {
    view! {
        <Flex
            gap=FlexGap::Medium
            align=FlexAlign::End
            wrap=FlexWrap::Wrap
            padding=SpacingSize::Size200.inset()
        >
            <Field label="Search">
                <div data-testid="tag-list-search">
                    <Input
                        bind=search
                        appearance=InputAppearance::with_placeholder("Search tags...")
                    />
                </div>
            </Field>
            <Field label="Taxonomy">
                <div data-testid="tag-list-taxonomy-filter">
                    <Input
                        bind=taxonomy
                        appearance=InputAppearance::with_placeholder("e.g. spend")
                    />
                </div>
            </Field>
            <div data-testid="tag-list-clear-filters">
                <Button appearance=ButtonAppearance::Subtle on_click=Callback::new(move |_| on_clear.run(()))>
                    "Clear"
                </Button>
            </div>
        </Flex>
    }
}
