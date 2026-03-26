use daisy_rsx::{Badge, BadgeColor, BadgeStyle, Card, CardBody};
use dioxus::prelude::*;
use ssg_whiz::{layouts::layout::Layout, Footer, Section};

pub fn page() -> String {
    let page = rsx! {
        Layout {
            title: "Integrations | One Runtime".to_string(),
            description: "Browse the integrations surface in One Runtime and jump into the API docs for fetching available integrations and connection state.".to_string(),
            image: Some("/logo.svg".to_string()),
            mobile_menu: None,
            section: Section::McpServers,
            main {
                class: "min-h-screen text-base-content",

                section {
                    class: "px-6 pt-28 pb-16 md:pt-40 lg:px-12",
                    div {
                        class: "mx-auto max-w-5xl space-y-10",
                        div {
                            class: "max-w-3xl space-y-4",
                            Badge {
                                badge_style: BadgeStyle::Outline,
                                badge_color: BadgeColor::Primary,
                                "Integrations"
                            }
                            h1 {
                                class: "text-4xl font-bold lg:text-5xl",
                                "Integrations are managed in One Runtime"
                            }
                            p {
                                class: "text-lg text-base-content/75",
                                "Use One Runtime to expose an integrations catalog to your customers, show connection state for each end user, and launch the hosted connection flow from your product."
                            }
                        }

                        Card {
                            class: Some("border border-base-300 bg-base-100 shadow-sm".to_string()),
                            CardBody {
                                class: Some("gap-4".to_string()),
                                Badge {
                                    badge_style: BadgeStyle::Outline,
                                    badge_color: BadgeColor::Primary,
                                    "API-first"
                                }
                                p {
                                    class: "text-base-content/75",
                                    "Your backend can fetch the integrations available to an end user with an org API key, then drive the hosted connection flow for the integrations the user selects."
                                }
                                div {
                                    class: "flex flex-wrap gap-3",
                                    a {
                                        class: "btn btn-primary",
                                        href: "/docs/integrations/fetching-integrations",
                                        "Read the docs"
                                    }
                                    a {
                                        class: "btn btn-outline",
                                        href: "https://app.one-runtime.com",
                                        "Open the app"
                                    }
                                }
                            }
                        }
                    }
                }

                Footer {
                    margin_top: Some("mt-0".to_string()),
                    links: crate::ui_links::footer_links(),
                }
            }
        }
    };

    ssg_whiz::render(page)
}
