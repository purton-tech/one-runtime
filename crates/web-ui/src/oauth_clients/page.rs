#![allow(non_snake_case)]

use crate::{
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use clorinde::queries::oauth_clients::OAuthClientCard;
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn page(org_id: String, balance_label: String, oauth_clients: Vec<OAuthClientCard>) -> String {
    let create_action = routes::oauth_clients::Create {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "OAuth Clients".to_string(),
            org_id: org_id.clone(),
            balance_label,
            selected_item: SideBar::OAuthClients,
            header_left: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Agent Octo".to_string(),
                            href: Some("/".to_string()),
                        },
                        BreadcrumbItem {
                            text: "OAuth Clients".to_string(),
                            href: None,
                        },
                    ]
                }
            ),
            header_right: Some(rsx!(
                Button {
                    button_scheme: ButtonScheme::Primary,
                    popover_target: "create-oauth-client-modal",
                    "Create OAuth Client"
                }
            )),
            SectionIntroduction {
                header: "OAuth Client Registry".to_string(),
                subtitle: "Store org-scoped OAuth2 client credentials for providers that need a client ID and secret.".to_string(),
                is_empty: oauth_clients.is_empty(),
                empty_text: "No OAuth clients created yet.".to_string()
            }
            div {
                class: "mt-4 grid gap-4",
                for oauth_client in oauth_clients {
                    Card {
                        class: "card-border bg-base-100",
                        CardBody {
                            div {
                                class: "flex flex-col gap-1 md:flex-row md:items-start md:justify-between",
                                div {
                                    h2 { class: "card-title", "{oauth_client.provider}" }
                                    p { class: "font-mono text-sm break-all", "{oauth_client.client_id}" }
                                }
                                span {
                                    class: "text-xs text-base-content/70",
                                    "Created {oauth_client.created_at.to_rfc3339()}"
                                }
                            }
                        }
                    }
                }
            }
            Modal {
                trigger_id: "create-oauth-client-modal".to_string(),
                submit_action: create_action,
                ModalBody {
                    h3 { class: "text-lg font-semibold", "Create OAuth Client" }
                    p { class: "text-sm text-base-content/70 mt-1", "Register a provider client ID and secret for this organization." }
                    div { class: "mt-4 space-y-3",
                        Fieldset {
                            legend: "Provider".to_string(),
                            Input {
                                name: "provider".to_string(),
                                placeholder: Some("google".to_string()),
                                required: Some(true),
                            }
                        }
                        Fieldset {
                            legend: "Client ID".to_string(),
                            Input {
                                name: "client_id".to_string(),
                                placeholder: Some("1234567890-abcdef.apps.googleusercontent.com".to_string()),
                                required: Some(true),
                            }
                        }
                        Fieldset {
                            legend: "Client Secret".to_string(),
                            Input {
                                input_type: Some(InputType::Password),
                                name: "client_secret".to_string(),
                                placeholder: Some("Paste provider secret".to_string()),
                                required: Some(true),
                            }
                        }
                    }
                    ModalAction {
                        Button {
                            class: "cancel-modal",
                            button_style: ButtonStyle::Ghost,
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Create Client"
                        }
                    }
                }
            }
        }
    };

    render(page)
}
