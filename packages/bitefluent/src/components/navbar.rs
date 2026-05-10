use dioxus::prelude::*;

use crate::api::auth::AuthUserResource;
use crate::components::{Button, ButtonSize, ButtonVariant};
const LOGO: Asset = asset!("/assets/logo.svg");

#[component]
pub fn Navbar() -> Element {
    let current_user = use_context::<AuthUserResource>();
    let user_state = current_user.read().clone();

    rsx! {
        header {
            class: "absolute inset-x-0 top-0 z-50",

            div {
                class: "mx-auto flex h-20 w-full max-w-7xl items-center justify-between px-6 lg:px-10",

                Link {
                    to: "/",
                    class: "flex items-center gap-3",

                    div {
                        class: "flex h-8 w-8 items-center justify-center",

                        img {
                            src: LOGO,
                            alt: "BiteFluent Logo",
                            class: "h-6 w-6",
                        }
                    }

                    span {
                        class: "font-serif text-2xl font-medium tracking-[-0.03em] text-[color:var(--text-primary)]",
                        "BiteFluent"
                    }
                }

                nav {
                    class: "hidden items-center gap-8 md:flex",

                    NavLink {
                        href: "#features",
                        label: "Features",
                    }

                    NavLink {
                        href: "#workflow",
                        label: "Workflow",
                    }

                    NavLink {
                        href: "#pricing",
                        label: "Pricing",
                    }

                    NavLink {
                        href: "#docs",
                        label: "Docs",
                    }

                    NavLink {
                        href: "https://github.com",
                        label: "GitHub",
                    }
                }



                div {
                    class: "hidden items-center gap-3 md:flex",

                    match user_state.as_ref() {
                        Some(Some(user)) => rsx! {
                            UserMenu {
                                name: user.name.clone(),
                                image: user.image.clone(),
                            }
                        },
                        Some(None) => rsx! {
                            Button {
                                label: "Get Started",
                                variant: ButtonVariant::Primary,
                                size: ButtonSize::Md,
                                icon: None,
                                to: Some("/start")
                            }
                        },
                        None => rsx! {
                            div {
                                class: "h-10 w-28 animate-pulse rounded-lg bg-white/[0.04]",
                            }
                        },
                    }


                }

                button {
                    class: "inline-flex h-10 w-10 items-center justify-center rounded-lg border border-white/[0.10] bg-white/[0.03] text-[color:var(--text-primary)] md:hidden",
                    aria_label: "Open menu",

                    svg {
                        class: "h-5 w-5",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",

                        path {
                            d: "M4 7H20M4 12H20M4 17H20",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavLink(href: &'static str, label: &'static str) -> Element {
    rsx! {
        a {
            href: "{href}",
            class: "text-sm font-medium text-[color:var(--text-secondary)] transition hover:text-[color:var(--text-primary)]",
            "{label}"
        }
    }
}

#[component]
fn UserMenu(name: Option<String>, image: Option<String>) -> Element {
    let fallback = name
        .as_deref()
        .and_then(|name| name.chars().next())
        .unwrap_or('?');

    let alt = name.as_deref().unwrap_or("User avatar").to_string();

    rsx! {
        details {
            class: "group relative",

            summary {
                class: "flex h-10 cursor-pointer select-none list-none items-center gap-3 rounded-lg border border-white/[0.12] bg-white/[0.03] px-4 text-sm font-medium text-white transition hover:border-white/[0.20] hover:bg-white/[0.06]",
                aria_label: "Open account menu",

                span {
                    class: "flex size-6 items-center justify-center overflow-hidden rounded-full border border-white/[0.12] bg-white/[0.04] text-xs font-semibold text-[color:var(--text-primary)]",

                    if let Some(image) = image.as_ref() {
                        img {
                            src: "{image}",
                            alt: "{alt}",
                            class: "size-full object-cover",
                        }
                    } else {
                        span {
                            class: "uppercase",
                            "{fallback}"
                        }
                    }
                }

                span {
                    class: "hidden sm:block",
                    "Account"
                }

                svg {
                    class: "size-4 text-[color:var(--text-secondary)] transition group-hover:text-[color:var(--text-primary)]",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 20 20",
                    fill: "currentColor",

                    path {
                        d: "M5.23 7.21a.75.75 0 0 1 1.06.02L10 10.97l3.71-3.74a.75.75 0 1 1 1.06 1.06l-4.24 4.28a.75.75 0 0 1-1.06 0L5.21 8.27a.75.75 0 0 1 .02-1.06Z",
                    }
                }
            }

            div {
                class: "pointer-events-none absolute right-0 top-[calc(100%+0.5rem)] z-50 w-52 overflow-hidden rounded-lg border border-white/[0.12] bg-[rgba(14,14,18,0.96)] p-1 shadow-xl shadow-black/30 backdrop-blur select-none opacity-0 translate-y-2 scale-[0.98] transition duration-150 ease-out group-open:pointer-events-auto group-open:opacity-100 group-open:translate-y-0 group-open:scale-100",

                Link {
                    to: "/app",
                    class: "flex w-full items-center rounded-md px-4 py-2.5 text-sm font-medium text-white transition hover:bg-white/[0.06]",
                    "Open app"
                }

                a {
                    href: "/auth/signout",
                    class: "flex w-full items-center rounded-md px-4 py-2.5 text-sm font-medium text-[color:var(--text-secondary)] transition hover:bg-white/[0.06] hover:text-white",
                    "Sign out"
                }
            }
        }
    }
}
