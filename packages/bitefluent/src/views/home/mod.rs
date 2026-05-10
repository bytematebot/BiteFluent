mod cta;
mod features_section;
mod hero;
mod trust_item;
mod workflow;

use crate::components::{
    Button, ButtonSize, ButtonVariant, Icon, IconKind, IconPosition, MetaFor, RenderIcon,
};
use crate::views::home::cta::CtaSection;
use crate::views::home::features_section::FeaturesSection;
use crate::views::home::hero::Hero;
use crate::views::home::workflow::WorkflowSection;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        crate::components::Meta { r#for: MetaFor::Home },
        div {
            class: "min-h-screen bg-[image:var(--bg-main)] text-[color:var(--text-primary)]",
            Hero {}
            FeaturesSection {}
            WorkflowSection {}
            CtaSection {}
        }
    }
}
