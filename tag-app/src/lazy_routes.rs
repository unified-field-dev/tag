//! Lazy-loaded route views for WASM code-splitting (`cargo leptos --split`).

use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};

use crate::layout::TagAppLayout;
use crate::pages::{TagCreatePage, TagDetailPage, TagListPage};

/// Prefetch the tag family WASM chunk (leaf pages share split modules).
pub async fn prefetch_family() {
    TagListRoute::preload().await;
}

/// Eager authenticated+verified guard shell for `/tag/*` ParentRoute.
#[component]
pub fn TagVerifiedGuardRouteView() -> impl IntoView {
    view! {
        <uf_product::routes::RequireAuthenticated requires_email_verification=true>
            <TagAppLayout />
        </uf_product::routes::RequireAuthenticated>
    }
}

/// Lazy `/tag` list page.
#[derive(Clone, Copy, Debug, Default)]
pub struct TagListRoute;

#[lazy_route]
impl LazyRoute for TagListRoute {
    fn data() -> Self {
        Self
    }

    fn view(this: Self) -> AnyView {
        let _ = this;
        view! { <TagListPage /> }.into_any()
    }
}

/// Lazy `/tag/create` page.
#[derive(Clone, Copy, Debug, Default)]
pub struct TagCreateRoute;

#[lazy_route]
impl LazyRoute for TagCreateRoute {
    fn data() -> Self {
        Self
    }

    fn view(this: Self) -> AnyView {
        let _ = this;
        view! { <TagCreatePage /> }.into_any()
    }
}

/// Lazy `/tag/:id` detail page.
#[derive(Clone, Copy, Debug, Default)]
pub struct TagDetailRoute;

#[lazy_route]
impl LazyRoute for TagDetailRoute {
    fn data() -> Self {
        Self
    }

    fn view(this: Self) -> AnyView {
        let _ = this;
        view! { <TagDetailPage /> }.into_any()
    }
}
