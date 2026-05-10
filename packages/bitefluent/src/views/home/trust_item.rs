use crate::components::{IconKind, RenderIcon};
use dioxus::prelude::*;

#[component]
pub fn TrustItem(icon: IconKind, label: &'static str) -> Element {
    rsx! {
        div {
            class: "inline-flex items-center gap-2 text-sm font-medium leading-none text-[color:var(--text-muted)]",

            span {
                class: "flex size-4 shrink-0 items-center justify-center text-[color:var(--text-muted)]",

                RenderIcon {
                    kind: icon,
                    class: Some("size-4".to_string()),
                }
            }

            span {
                class: "leading-none",
                "{label}"
            }
        }
    }
}
