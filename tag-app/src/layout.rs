//! Shell layout for the Tags app: platform app bar + left nav wrapping the routed page [`Outlet`].

use crate::AppMetadata;
use lepton_shell::AppBarUserMenu;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use uf_integrations::{
    ShellAppBar, ShellAuthMenu, ShellLeftNav, UnifiedFieldAppBar, UnifiedFieldShellLayout,
};
use uf_product::components::{
    Navigation, NavigationBody, NavigationConfig, NavigationLink, NavigationMaterial,
};

/// Shell layout for the Tags app: app bar + left nav wrapping the routed page [`Outlet`].
///
/// Chrome comes from [`uf_integrations`] (same pattern as counter-app). Auth menu is
/// `ShellAuthMenu` + [`AppBarUserMenu`].
#[component]
pub fn TagAppLayout() -> impl IntoView {
    let app_name = AppMetadata::name().to_string();
    let selected_value = RwSignal::new(None::<String>);
    let open_categories = RwSignal::new(Vec::<String>::new());

    view! {
        <div data-testid="tag-app-root">
            <UnifiedFieldShellLayout>
                <ShellAppBar slot>
                    <UnifiedFieldAppBar
                        app_name=app_name
                        app_id=AppMetadata::id()
                        homepage_url="/".to_string()
                    >
                        <ShellAuthMenu slot:auth_menu>
                            <AppBarUserMenu />
                        </ShellAuthMenu>
                    </UnifiedFieldAppBar>
                </ShellAppBar>
                <ShellLeftNav slot>
                    <Navigation config=NavigationConfig::new().with_selected_value(selected_value).with_open_categories(open_categories)>
                        <NavigationMaterial slot />
                        <NavigationBody slot>
                            <NavigationLink path="/tag" value="/tag" icon=icondata::AiTagOutlined exact=true test_id="nav-tags">"Tags"</NavigationLink>
                        </NavigationBody>
                    </Navigation>
                </ShellLeftNav>
                <Outlet />
            </UnifiedFieldShellLayout>
        </div>
    }
}
