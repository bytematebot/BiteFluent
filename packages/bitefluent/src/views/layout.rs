use crate::Route;
use crate::api::auth::fetch_current_user;
use crate::components::Footer;
use crate::views::Navbar;
use crate::views::project::navbar::ProjectNavbar;
use dioxus::prelude::*;

#[component]
pub fn AppLayout() -> Element {
    let current_user = use_resource(|| async { fetch_current_user().await.unwrap_or(None) });
    use_context_provider(|| current_user);

    let route = use_route::<Route>();

    let mut transition_tick = use_signal(|| 0usize);

    use_effect(use_reactive((&route,), move |(_route,)| {
        transition_tick.with_mut(|tick| *tick += 1);
    }));

    let transition_class = if transition_tick() % 2 == 0 {
        "route-transition route-transition-a"
    } else {
        "route-transition route-transition-b"
    };

    rsx! {
        div {
            class: "min-h-screen bg-[image:var(--bg-main)] font-sans text-[color:var(--text-primary)]",

            match route.clone() {
                Route::AppProject { project_id } => rsx! {
                    ProjectNavbar {
                        project_id,
                        on_synced: move |_| {}
                    }
                },

                _ => rsx! {
                    Navbar {}
                },
            }

            main {
                class: "{transition_class}",

                Outlet::<Route> {}
            }

            if !matches!(route.clone(),
                Route::AppProject { .. } |
                Route::AppOnboardingProject { .. } |
                Route::AppPlatform { ..}) {
                Footer {}
            }
        }
    }
}
