use crate::Route;
use crate::components::Footer;
use crate::views::Navbar;
use dioxus::prelude::*;

#[component]
pub fn AppLayout() -> Element {
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

            Navbar {}

            main {
                class: "{transition_class}",

                Outlet::<Route> {}
            }

            Footer {}
        }
    }
}
