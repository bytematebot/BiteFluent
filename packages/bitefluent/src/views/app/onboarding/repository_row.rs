use crate::api::github::GithubRepositoryDto;
use crate::components::{IconKind, RenderIcon};
use dioxus::prelude::*;

#[component]
pub fn RepositoryRow(
    repository: GithubRepositoryDto,
    selected: bool,
    on_select: EventHandler<u64>,
) -> Element {
    let row_class = if selected {
        "border-[color:var(--bg-accent)] bg-[var(--bg-accent)]/[0.06]"
    } else {
        "border-white/[0.08] bg-white/[0.02] hover:border-white/[0.14] hover:bg-white/[0.035]"
    };

    let check_class = if selected {
        "flex size-6 shrink-0 items-center justify-center rounded-full bg-[var(--bg-accent)] text-xs font-bold text-white"
    } else {
        "flex size-6 shrink-0 items-center justify-center rounded-full border border-white/[0.16] bg-white/[0.02]"
    };

    let visibility = if repository.private {
        "Private"
    } else {
        "Public"
    };
    let repository_id = repository.id;

    rsx! {
        button {
            class: "group flex h-16 w-full items-center gap-4 rounded-xl border px-4 text-left transition {row_class}",
            type: "button",
            onclick: move |_| on_select.call(repository_id),

            span {
                class: "{check_class}",

                if selected {
                    RenderIcon {
                        kind: IconKind::CheckNoCircle,
                        class: Some("size-3".to_string()),
                    }
                }
            }

            RenderIcon {
                kind: IconKind::Repository,
                class: Some("size-5 shrink-0 text-[color:var(--text-muted)]".to_string()),
            }

            span {
                class: "min-w-0 flex-1 truncate text-sm font-semibold text-[color:var(--text-primary)]",
                "{repository.full_name}"
            }

            span {
                class: "rounded-full bg-white/[0.06] px-2.5 py-1 text-xs font-medium text-[color:var(--text-muted)]",
                "{visibility}"
            }
        }
    }
}
