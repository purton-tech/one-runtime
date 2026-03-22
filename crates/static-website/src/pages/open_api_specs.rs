use daisy_rsx::{Badge, BadgeColor, BadgeStyle, Card, CardBody};
use dioxus::prelude::*;
use ssg_whiz::{layouts::layout::Layout, Footer, Section};

pub fn page() -> String {
    let page = rsx! {
        Layout {
            title: "OpenAPI Specs | One Runtime".to_string(),
            description: "OpenAPI-backed integrations are managed in One Runtime and will later be exposed through API-backed catalog views.".to_string(),
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
                                "OpenAPI specs"
                            }
                            h1 {
                                class: "text-4xl font-bold lg:text-5xl",
                                "OpenAPI specs are managed in the application"
                            }
                            p {
                                class: "text-lg text-base-content/75",
                                "The static site no longer reads local spec files. The source of truth is now the One Runtime application database, and this page will later move to an API-backed catalog."
                            }
                        }

                        Card {
                            class: Some("border border-base-300 bg-base-100 shadow-sm".to_string()),
                            CardBody {
                                class: Some("gap-4".to_string()),
                                Badge {
                                    badge_style: BadgeStyle::Outline,
                                    badge_color: BadgeColor::Primary,
                                    "Temporary"
                                }
                                p {
                                    class: "text-base-content/75",
                                    "System integrations are seeded in the application database. Public rendering here is intentionally minimal until the external catalog API exists."
                                }
                                div {
                                    class: "flex flex-wrap gap-3",
                                    a {
                                        class: "btn btn-primary",
                                        href: "https://app.one-runtime.com",
                                        "Open the app"
                                    }
                                    a {
                                        class: "btn btn-outline",
                                        href: "/",
                                        "Back home"
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
