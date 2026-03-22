#![allow(non_snake_case)]

use crate::{
    components::section_introduction::SectionIntroduction,
    integrations::model::{IntegrationCatalogFilters, IntegrationCatalogItem},
    integrations::{card::IntegrationCard, modal::IntegrationModal},
    layout::{ContentWidth, Layout, SideBar},
    render, routes,
};
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn page(
    org_id: String,
    balance_label: String,
    filters: IntegrationCatalogFilters,
    integrations: Vec<IntegrationCatalogItem>,
) -> String {
    let new_href = routes::integrations::New {
        org_id: org_id.clone(),
    }
    .to_string();
    let index_href = routes::integrations::Index {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "Integrations".to_string(),
            org_id: org_id.clone(),
            balance_label,
            selected_item: SideBar::Integrations,
            content_width: Some(ContentWidth::Max),
            header_left: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Agent Octo".to_string(),
                            href: Some("/".to_string()),
                        },
                        BreadcrumbItem {
                            text: "Integrations".to_string(),
                            href: None,
                        },
                    ]
                }
            ),
            header_right: Some(rsx!(
                Button {
                    button_type: ButtonType::Link,
                    href: new_href,
                    button_scheme: ButtonScheme::Primary,
                    "Add OpenAPI Spec"
                }
            )),
            div {
                SectionIntroduction {
                    header: "Integrations".to_string(),
                    subtitle: "Browse the OpenAPI integrations available to your organization. Open a card to inspect details and capabilities.".to_string(),
                    is_empty: integrations.is_empty(),
                    empty_text: "No integrations match this view yet.".to_string(),
                }

                form {
                    method: "get",
                    action: index_href.clone(),
                    class: "mt-6 flex flex-col gap-4 xl:flex-row xl:items-end xl:justify-between",
                    div {
                        class: "flex w-full flex-col gap-4 xl:max-w-3xl xl:flex-row",
                        Fieldset {
                            class: "flex-1".to_string(),
                            legend: "Search".to_string(),
                            input {
                                class: "input input-bordered w-full",
                                r#type: "search",
                                name: "q",
                                value: filters.search_query.clone(),
                                placeholder: "Search all integrations"
                            }
                        }
                        Fieldset {
                            class: "xl:w-64".to_string(),
                            legend: "Category".to_string(),
                            Select {
                                name: "category".to_string(),
                                value: Some(filters.selected_category.clone()),
                                for category in &filters.categories {
                                    SelectOption {
                                        value: category.clone(),
                                        selected_value: Some(filters.selected_category.clone()),
                                        "{category}"
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "flex gap-2",
                        Button {
                            button_type: ButtonType::Submit,
                            button_style: ButtonStyle::Outline,
                            "Apply"
                        }
                        Button {
                            button_type: ButtonType::Link,
                            href: index_href.clone(),
                            button_style: ButtonStyle::Ghost,
                            "Reset"
                        }
                    }
                }

                if integrations.is_empty() {
                    Card {
                        class: Some("mt-6 border border-dashed border-base-300 bg-base-100".to_string()),
                        CardBody {
                            h3 { class: "card-title", "No integrations found" }
                            p {
                                class: "text-sm text-base-content/70",
                                "Try a different search term or category."
                            }
                        }
                    }
                } else {
                    div {
                        class: "mt-6 grid gap-4 lg:grid-cols-2 2xl:grid-cols-3",
                        for integration in &integrations {
                            IntegrationCard {
                                integration: integration.clone()
                            }
                        }
                    }
                }

                for integration in integrations {
                    IntegrationModal {
                        org_id: org_id.clone(),
                        integration
                    }
                }
            }
        }
    };

    render(page)
}
