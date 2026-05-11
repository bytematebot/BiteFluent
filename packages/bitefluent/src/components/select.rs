use dioxus::prelude::*;
use crate::components::{IconKind, RenderIcon};

#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectProps {
    pub label: &'static str,

    #[props(into)]
    pub value: String,

    pub options: Vec<SelectOption>,

    #[props(default = None)]
    pub leading: Option<Element>,

    #[props(default = None)]
    pub class: Option<String>,

    #[props(default = None)]
    pub on_change: Option<EventHandler<String>>,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let mut open = use_signal(|| false);

    let extra_class = props.class.clone().unwrap_or_default();

    rsx! {
        div {
            class: "relative {extra_class}",

            label {
                class: "text-sm font-medium text-[color:var(--text-secondary)]",
                "{props.label}"
            }

            button {
                class: "mt-3 flex h-12 w-full items-center justify-between rounded-xl border border-white/[0.10] bg-white/[0.025] px-4 text-left transition hover:border-white/[0.16] hover:bg-white/[0.04]",
                type: "button",
                onclick: move |_| {
                    open.toggle();
                },

                span {
                    class: "flex min-w-0 items-center gap-3",

                    if let Some(leading) = props.leading.clone() {
                        {leading}
                    }

                    span {
                        class: "truncate text-sm font-medium text-[color:var(--text-primary)]",
                        "{props.value}"
                    }
                }

                span {
                    class: if open() {
                        "rotate-180 text-[color:var(--text-muted)] transition"
                    } else {
                        "text-[color:var(--text-muted)] transition"
                    },

                    RenderIcon {
                        kind: IconKind::ChevronDown,
                        class: Some("size-3.5".to_string()),
                    }
                }
            }

            if open() {
                div {
                    class: "absolute left-0 right-0 top-full z-50 mt-2 overflow-hidden rounded-xl border border-white/[0.10] bg-[#111217] shadow-2xl shadow-black/40",

                    for option in props.options.iter() {
                        button {
                            class: "flex h-11 w-full items-center px-4 text-left text-sm text-[color:var(--text-secondary)] transition hover:bg-white/[0.05] hover:text-[color:var(--text-primary)]",
                            type: "button",
                            onclick: {
                                let value = option.value.clone();
                                let on_change = props.on_change.clone();

                                move |_| {
                                    open.set(false);

                                    if let Some(on_change) = on_change.as_ref() {
                                        on_change.call(value.clone());
                                    }
                                }
                            },

                            "{option.label}"
                        }
                    }
                }
            }
        }
    }
}