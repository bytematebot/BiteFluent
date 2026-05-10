use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum MetaFor {
    Home,
    Start,
}

#[derive(PartialEq, Clone, Props)]
pub struct MetaProps {
    pub r#for: MetaFor,
}

#[component]
pub fn Meta(props: MetaProps) -> Element {
    let url = "https://bitefluent.meetz.li";
    let image = "https://test.bitefluent.com/og-image.png";
    let accent = "#8C78FF";

    match props.r#for {
        MetaFor::Home => {
            let title = "BiteFluent: Fluent-native translation management";
            let description = "Sync source keys, manage translations, review changes, and publish translations with a clean workflow built for Fluent projects.";

            rsx! {
                document::Title { "{title}" }

                document::Meta {
                    name: "description",
                    content: "{description}",
                }

                document::Meta {
                    name: "theme-color",
                    content: "{accent}",
                }

                document::Link {
                    rel: "canonical",
                    href: "{url}",
                }

                document::Meta {
                    property: "og:type",
                    content: "website",
                }

                document::Meta {
                    property: "og:site_name",
                    content: "BiteFluent",
                }

                document::Meta {
                    property: "og:title",
                    content: "{title}",
                }

                document::Meta {
                    property: "og:description",
                    content: "{description}",
                }

                document::Meta {
                    property: "og:url",
                    content: "{url}",
                }

                document::Meta {
                    property: "og:image",
                    content: "{image}",
                }

                document::Meta {
                    property: "og:image:width",
                    content: "540",
                }

                document::Meta {
                    property: "og:image:height",
                    content: "115",
                }

                document::Meta {
                    property: "og:image:alt",
                    content: "BiteFluent landing page preview",
                }

                document::Meta {
                    name: "twitter:card",
                    content: "summary_large_image",
                }

                document::Meta {
                    name: "twitter:title",
                    content: "{title}",
                }

                document::Meta {
                    name: "twitter:description",
                    content: "{description}",
                }

                document::Meta {
                    name: "twitter:image",
                    content: "{image}",
                }
            }
        }
        MetaFor::Start => {
            let title = "Get started with BiteFluent";
            let description =
                "Sign in to create your first BiteFluent project and connect a repository.";
            let canonical = format!("{url}/start");

            rsx! {
                document::Title { "{title}" }

                document::Meta {
                    name: "description",
                    content: "{description}",
                }

                document::Meta {
                    name: "theme-color",
                    content: "{accent}",
                }

                document::Link {
                    rel: "canonical",
                    href: "{canonical}",
                }

                document::Meta {
                    property: "og:type",
                    content: "website",
                }

                document::Meta {
                    property: "og:site_name",
                    content: "BiteFluent",
                }

                document::Meta {
                    property: "og:title",
                    content: "{title}",
                }

                document::Meta {
                    property: "og:description",
                    content: "{description}",
                }

                document::Meta {
                    property: "og:url",
                    content: "{canonical}",
                }

                document::Meta {
                    property: "og:image",
                    content: "{image}",
                }
            }
        }
    }
}
