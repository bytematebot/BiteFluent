use dioxus::prelude::*;

use crate::components::{IconKind, RenderIcon};

const LOGO: Asset = asset!("/assets/logo.svg");

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "border-t border-white/[0.06] bg-black/[0.12]",

            div {
                class: "mx-auto w-full max-w-7xl px-6 py-12 lg:px-10",

                div {
                    class: "grid gap-10 md:grid-cols-[1.4fr_1fr_1fr_1fr]",

                    div {
                        class: "max-w-sm",

                        a {
                            href: "/",
                            class: "inline-flex items-center gap-3",

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

                        p {
                            class: "mt-4 max-w-xs text-sm leading-6 text-[color:var(--text-muted)]",
                            "Fluent-native translation management for modern development teams."
                        }

                        div {
                            class: "mt-6 flex items-center gap-4",

                            SocialLink {
                                href: "https://github.com",
                                label: "GitHub",
                                icon: IconKind::Github,
                            }

                            SocialLink {
                                href: "#",
                                label: "Twitter",
                                icon: IconKind::Twitter,
                            }

                            SocialLink {
                                href: "#",
                                label: "Discord",
                                icon: IconKind::Discord,
                            }
                        }
                    }

                    FooterColumn {
                        title: "Product",
                        links: vec![
                            FooterLinkData { label: "Features", href: "#features" },
                            FooterLinkData { label: "Workflow", href: "#workflow" },
                            FooterLinkData { label: "Pricing", href: "#pricing" },
                            FooterLinkData { label: "Changelog", href: "#changelog" },
                        ],
                    }

                    FooterColumn {
                        title: "Resources",
                        links: vec![
                            FooterLinkData { label: "Docs", href: "#docs" },
                            FooterLinkData { label: "Guides", href: "#guides" },
                            FooterLinkData { label: "API", href: "#api" },
                            FooterLinkData { label: "Blog", href: "#blog" },
                        ],
                    }

                    FooterColumn {
                        title: "Company",
                        links: vec![
                            FooterLinkData { label: "GitHub", href: "https://github.com" },
                            FooterLinkData { label: "Roadmap", href: "#roadmap" },
                            FooterLinkData { label: "License", href: "#license" },
                            FooterLinkData { label: "Contact", href: "#contact" },
                        ],
                    }
                }

                div {
                    class: "mt-12 flex flex-col gap-3 border-t border-white/[0.06] pt-6 text-xs text-[color:var(--text-muted)] sm:flex-row sm:items-center sm:justify-between",

                    p {
                        class: "leading-6",
                        "Copyright 2026 BiteFluent. Open source under the "
                        a {
                            href: "#license",
                            class: "text-[color:var(--bg-accent)] transition hover:opacity-80",
                            "AGPL-3.0"
                        }
                        "."
                    }

                    div {
                        class: "flex items-center gap-6",

                        a {
                            href: "#privacy",
                            class: "text-[color:var(--text-muted)] transition hover:text-[color:var(--text-primary)]",
                            "Privacy"
                        }

                        a {
                            href: "#terms",
                            class: "text-[color:var(--text-muted)] transition hover:text-[color:var(--text-primary)]",
                            "Terms"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct FooterLinkData {
    label: &'static str,
    href: &'static str,
}

#[component]
fn FooterColumn(title: &'static str, links: Vec<FooterLinkData>) -> Element {
    rsx! {
        div {
            h3 {
                class: "text-sm font-semibold text-[color:var(--text-primary)]",
                "{title}"
            }

            ul {
                class: "mt-4 space-y-3",

                for link in links {
                    li {
                        a {
                            href: "{link.href}",
                            class: "text-sm text-[color:var(--text-muted)] transition hover:text-[color:var(--text-primary)]",
                            "{link.label}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SocialLink(href: &'static str, label: &'static str, icon: IconKind) -> Element {
    rsx! {
        a {
            href: "{href}",
            aria_label: "{label}",
            class: "text-[color:var(--text-muted)] transition hover:text-[color:var(--text-primary)]",

            RenderIcon {
                kind: icon,
                class: Some("size-5".to_string()),
            }
        }
    }
}
