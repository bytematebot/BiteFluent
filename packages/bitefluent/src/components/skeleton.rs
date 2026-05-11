use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SkeletonProps {
    #[props(default = false)]
    pub suspend: bool,

    #[props(default = String::new())]
    pub class: String,

    children: Element,
}

#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    if !props.suspend {
        return props.children;
    }

    rsx! {
        div {
            class: "bf-skeleton pointer-events-none select-none {props.class}",
            aria_hidden: "true",

            {props.children}
        }
    }
}