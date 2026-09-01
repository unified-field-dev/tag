//! Eager `/tag` routes for the Playwright host.
//!
//! Production [`tag_app::TagRoutes`] wraps leaf pages in `Lazy` for wasm-split.
//! Nested `Lazy` under `ParentRoute` still panics on hydrate in this Leptos pin,
//! so the lab host mounts the same page components without `Lazy`.

use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route},
    path,
};
use tag_app::{TagCreatePage, TagDetailPage, TagListPage, TagVerifiedGuardRouteView};

/// Same paths as [`tag_app::TagRoutes`], without Lazy route views.
#[component(transparent)]
pub fn TagRoutesEager() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("tag") view=TagVerifiedGuardRouteView>
            <Route path=path!("") view=TagListPage />
            <Route path=path!("create") view=TagCreatePage />
            <Route path=path!(":id") view=TagDetailPage />
        </ParentRoute>
    }
    .into_inner()
}
