use crate::components::{Icon, IconPosition, RenderIcon};
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Danger,
}

#[derive(PartialEq, Clone)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
}

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
    #[props(into)]
    pub label: String,

    #[props(default = ButtonVariant::Primary)]
    pub variant: ButtonVariant,

    #[props(default = ButtonSize::Md)]
    pub size: ButtonSize,

    #[props(default = None)]
    pub icon: Option<Icon>,

    #[props(default = None)]
    pub to: Option<&'static str>,

    #[props(default = None)]
    pub href: Option<&'static str>,

    #[props(default = None)]
    pub on_click: Option<EventHandler<MouseEvent>>,

    #[props(default = false)]
    pub full_width: bool,

    #[props(default = false)]
    pub balanced: bool,

    #[props(default = None)]
    pub class: Option<String>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_class = match props.variant {
        ButtonVariant::Primary => "bg-[var(--bg-accent)] text-white hover:opacity-90",
        ButtonVariant::Secondary => {
            "border border-white/[0.12] bg-white/[0.03] text-white hover:bg-white/[0.06] hover:border-white/[0.20]"
        }
        ButtonVariant::Ghost => {
            "bg-transparent text-[var(--text-muted)] hover:text-white hover:bg-white/5"
        }
        ButtonVariant::Danger => "bg-red-500 text-white hover:bg-red-600",
    };

    let size_class = match props.size {
        ButtonSize::Sm => "px-6 py-2 text-sm",
        ButtonSize::Md => "px-8 py-3 text-sm",
        ButtonSize::Lg => "px-12 py-4 text-base",
    };

    let layout_class = if props.balanced {
        "grid grid-cols-[2rem_1fr_2rem] items-center"
    } else {
        "inline-flex items-center justify-center gap-2"
    };

    let width_class = if props.full_width { "w-full" } else { "" };

    let extra_class = props.class.clone().unwrap_or_default();

    let class = format!(
        "{layout_class} {width_class} rounded-lg font-medium transition {variant_class} {size_class} {extra_class}",
    );

    if let Some(href) = props.href {
        let is_external = href.starts_with("http://") || href.starts_with("https://");

        return rsx! {
            a {
                href: "{href}",
                class: "{class}",
                target: if is_external { "_blank" } else { "_self" },
                rel: if is_external { "noreferrer" } else { "" },

                ButtonContent {
                    label: props.label.clone(),
                    icon: props.icon.clone(),
                    balanced: props.balanced,
                }
            }
        };
    }

    if let Some(to) = props.to {
        return rsx! {
            Link {
                to,
                class: "{class}",

                ButtonContent {
                    label: props.label.clone(),
                    icon: props.icon.clone(),
                    balanced: props.balanced,
                }
            }
        };
    }

    let on_click = props.on_click;

    rsx! {
        button {
            class: "{class}",
            type: "button",
            onclick: move |event| {
                if let Some(on_click) = on_click {
                    on_click.call(event);
                }
            },

            ButtonContent {
                label: props.label.clone(),
                icon: props.icon.clone(),
                balanced: props.balanced,
            }
        }
    }
}

#[component]
fn ButtonContent(label: String, icon: Option<Icon>, balanced: bool) -> Element {
    rsx! {
        if let Some(icon) = icon.as_ref().filter(|icon| icon.position == IconPosition::Left) {
            RenderIcon {
                kind: icon.kind,
                class: icon.class.clone(),
            }
        } else if balanced {
            span {}
        }

        span {
            class: if balanced { "text-center" } else { "" },
            "{label}"
        }

        if let Some(icon) = icon.as_ref().filter(|icon| icon.position == IconPosition::Right) {
            RenderIcon {
                kind: icon.kind,
                class: icon.class.clone(),
            }
        } else if balanced {
            span {}
        }
    }
}
