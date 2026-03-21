#![allow(non_snake_case)]

use crate::integrations::{
    card::IntegrationLogo,
    page::{IntegrationCatalogItem, modal_trigger_id},
};
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn IntegrationModal(org_id: String, integration: IntegrationCatalogItem) -> Element {
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
