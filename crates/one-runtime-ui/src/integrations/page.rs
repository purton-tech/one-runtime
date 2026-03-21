#![allow(non_snake_case)]

use crate::{
    components::section_introduction::SectionIntroduction,
    layout::{ContentWidth, Layout, SideBar},
    render, routes,
};
use daisy_rsx::*;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct IntegrationCatalogFilters {
    pub search_query: String,
    pub selected_category: String,
    pub categories: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntegrationCatalogItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner_kind: String,
    pub visibility: String,
    pub can_manage: bool,
    pub updated_at_label: String,
    pub logo_url: Option<String>,
    pub category: Option<String>,
    pub developer_name: Option<String>,
    pub website_url: Option<String>,
    pub support_url: Option<String>,
    pub overview_items: Vec<String>,
    pub operation_count: usize,
    pub edit_href: Option<String>,
    pub delete_href: Option<String>,
}

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

#[component]
fn IntegrationCard(integration: IntegrationCatalogItem) -> Element {
    let trigger_id = modal_trigger_id(&integration.id);

    rsx! {
        button {
            class: "w-full cursor-pointer text-left",
            "data-target": trigger_id.clone(),
            r#type: "button",
            Card {
                class: Some("h-full border border-base-300 bg-base-100 shadow-sm transition-colors hover:border-primary/40 hover:bg-base-200/20 focus-within:border-primary/40".to_string()),
                CardBody {
                    class: Some("gap-4".to_string()),
                    div {
                        class: "flex items-start gap-4",
                        IntegrationLogo {
                            logo_url: integration.logo_url.clone(),
                            title: integration.name.clone(),
                        }
                        div {
                            class: "min-w-0 flex-1 space-y-2",
                            h2 {
                                class: "card-title line-clamp-2 text-xl",
                                "{integration.name}"
                            }
                            p {
                                class: "line-clamp-3 text-sm text-base-content/75",
                                "{integration.description}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn IntegrationModal(org_id: String, integration: IntegrationCatalogItem) -> Element {
    let trigger_id = modal_trigger_id(&integration.id);

    rsx! {
        Modal {
            trigger_id: trigger_id,
            submit_action: None,
            ModalBody {
                class: Some("max-w-4xl".to_string()),
                div {
                    class: "flex items-start justify-between gap-6",
                    div {
                        class: "flex items-start gap-4",
                        IntegrationLogo {
                            logo_url: integration.logo_url.clone(),
                            title: integration.name.clone(),
                        }
                        div {
                            class: "space-y-2",
                            h3 {
                                class: "text-3xl font-semibold",
                                "{integration.name}"
                            }
                            p {
                                class: "text-base text-base-content/75",
                                "{integration.description}"
                            }
                        }
                    }
                    button {
                        class: "btn btn-ghost btn-sm cancel-modal",
                        r#type: "button",
                        "Close"
                    }
                }

                div {
                    class: "mt-6 grid gap-4 text-sm text-base-content/75 md:grid-cols-2",
                    DetailRow {
                        label: "Ownership",
                        value: if integration.owner_kind == "system" {
                            "System".to_string()
                        } else {
                            "Organization".to_string()
                        }
                    }
                    if let Some(category) = &integration.category {
                        DetailRow {
                            label: "Category",
                            value: category.clone()
                        }
                    }
                    DetailRow {
                        label: "Visibility",
                        value: integration.visibility.clone()
                    }
                    DetailRow {
                        label: "Methods",
                        value: integration.operation_count.to_string()
                    }
                    DetailRow {
                        label: "Updated",
                        value: integration.updated_at_label.clone()
                    }
                }

                if !integration.overview_items.is_empty() {
                    div {
                        class: "mt-8 space-y-3",
                        h4 {
                            class: "text-xl font-semibold",
                            "Overview"
                        }
                        ul {
                            class: "list-disc space-y-2 pl-6 text-base-content/85",
                            for item in &integration.overview_items {
                                li { "{item}" }
                            }
                        }
                    }
                }

                if integration.website_url.is_some()
                    || integration.support_url.is_some()
                    || integration.developer_name.is_some()
                {
                    div {
                        class: "mt-8 grid gap-6 md:grid-cols-2",
                        if integration.website_url.is_some() || integration.support_url.is_some() {
                            div {
                                class: "space-y-3",
                                h4 {
                                    class: "text-xl font-semibold",
                                    "Links"
                                }
                                div {
                                    class: "flex flex-col gap-2",
                                    if let Some(website_url) = &integration.website_url {
                                        a {
                                            class: "link link-primary",
                                            href: website_url.clone(),
                                            target: "_blank",
                                            rel: "noreferrer",
                                            "Website"
                                        }
                                    }
                                    if let Some(support_url) = &integration.support_url {
                                        a {
                                            class: "link link-primary",
                                            href: support_url.clone(),
                                            target: "_blank",
                                            rel: "noreferrer",
                                            "Support"
                                        }
                                    }
                                }
                            }
                        }
                        if let Some(developer_name) = &integration.developer_name {
                            div {
                                class: "space-y-3",
                                h4 {
                                    class: "text-xl font-semibold",
                                    "Developed by"
                                }
                                p {
                                    class: "text-base-content/85",
                                    "{developer_name}"
                                }
                            }
                        }
                    }
                }

                ModalAction {
                    if let Some(edit_href) = &integration.edit_href {
                        Button {
                            button_type: ButtonType::Link,
                            href: edit_href.clone(),
                            button_style: ButtonStyle::Outline,
                            "Edit"
                        }
                    }
                    if let Some(delete_href) = &integration.delete_href {
                        form {
                            method: "post",
                            action: delete_href.clone(),
                            button {
                                class: "btn btn-warning",
                                r#type: "submit",
                                "Delete"
                            }
                        }
                    }
                    Button {
                        class: "cancel-modal",
                        button_style: ButtonStyle::Ghost,
                        "Close"
                    }
                }
            }
        }
    }
}

#[component]
fn DetailRow(label: &'static str, value: String) -> Element {
    rsx! {
        div {
            class: "space-y-1",
            p {
                class: "text-xs font-medium uppercase tracking-wide text-base-content/50",
                "{label}"
            }
            p {
                class: "text-sm text-base-content/85",
                "{value}"
            }
        }
    }
}

#[component]
fn IntegrationLogo(logo_url: Option<String>, title: String) -> Element {
    let initials = title
        .chars()
        .filter(|char| char.is_ascii_alphanumeric())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    rsx! {
        div {
            class: "flex h-14 w-14 shrink-0 items-center justify-center overflow-hidden rounded-box border border-base-300 bg-base-200/40",
            if let Some(logo_url) = logo_url {
                img {
                    class: "h-full w-full object-contain p-2",
                    src: logo_url,
                    alt: "{title} logo"
                }
            } else {
                span {
                    class: "text-sm font-semibold text-base-content/70",
                    "{initials}"
                }
            }
        }
    }
}

fn modal_trigger_id(id: &str) -> String {
    format!("integration-detail-{id}")
}
