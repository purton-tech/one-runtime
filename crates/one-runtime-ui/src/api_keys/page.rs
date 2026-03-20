#![allow(non_snake_case)]

use crate::{
    components::card_item::{CardItem, CountLabel},
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use clorinde::queries::auth::OrgApiKeyCard;
use daisy_rsx::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CreatedApiKeyState {
    pub label: String,
    pub token: String,
}

pub fn page(
    org_id: String,
    balance_label: String,
    api_keys: Vec<OrgApiKeyCard>,
    draft_label: Option<String>,
    error_message: Option<String>,
    created_key: Option<CreatedApiKeyState>,
) -> String {
    let create_action = routes::api_keys::Create {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "API Keys".to_string(),
            org_id: org_id.clone(),
            balance_label,
            selected_item: SideBar::ApiKeys,
            header_left: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Agent Octo".to_string(),
                            href: Some("/".to_string()),
                        },
                        BreadcrumbItem {
                            text: "API Keys".to_string(),
                            href: None,
                        },
                    ]
                }
            ),
            header_right: Some(rsx!(
                Button {
                    button_scheme: ButtonScheme::Primary,
                    popover_target: "create-api-key-modal",
                    "Create API Key"
                }
            )),
            SectionIntroduction {
                header: "One Runtime API Keys".to_string(),
                subtitle: "Create org-scoped bearer keys for MCP and API access. Keys are shown only once when created.".to_string(),
                is_empty: api_keys.is_empty(),
                empty_text: "No API keys created yet.".to_string()
            }
            if let Some(message) = error_message {
                Alert {
                    class: "mt-4".to_string(),
                    alert_color: Some(AlertColor::Error),
                    span { "{message}" }
                }
            }
            if let Some(created) = created_key {
                Card {
                    class: "card-border bg-base-100 mt-4",
                    CardBody {
                        h2 { class: "card-title", "API key created" }
                        p { class: "text-sm text-base-content/70", "Copy this token now. It will not be shown again." }
                        Fieldset {
                            legend: "Label".to_string(),
                            input {
                                class: "input input-bordered w-full",
                                value: created.label,
                                readonly: true,
                            }
                        }
                        Fieldset {
                            legend: "Bearer token".to_string(),
                            input {
                                class: "input input-bordered w-full font-mono text-xs",
                                value: created.token,
                                readonly: true,
                            }
                        }
                        div {
                            class: "text-xs text-base-content/70",
                            "Use as: Authorization: Bearer oru_..."
                        }
                    }
                }
            }
            for api_key in api_keys {
                CardItem {
                    class: None,
                    title: api_key.label.clone(),
                    description: Some(rsx!(
                        div {
                            class: "flex flex-col gap-1",
                            p { class: "font-mono text-xs", "{api_key.key_prefix}" }
                            p { class: "text-sm text-base-content/70", "Last used: {api_key.last_used_label}" }
                        }
                    )),
                    footer: Some(rsx!(
                        span {
                            "Created "
                            {api_key.created_at.to_rfc3339()}
                        }
                    )),
                    count_labels: vec![
                        CountLabel {
                            count: 1,
                            label: if api_key.revoked { "revoked".to_string() } else { "active".to_string() },
                        }
                    ],
                    action: Some(rsx!(
                        if api_key.revoked {
                            span { class: "text-xs text-base-content/60", "Revoked" }
                        } else {
                            form {
                                method: "post",
                                action: routes::api_keys::Revoke {
                                    org_id: org_id.clone(),
                                    id: api_key.id.to_string(),
                                }.to_string(),
                                button {
                                    class: "btn btn-outline btn-sm",
                                    r#type: "submit",
                                    "Revoke"
                                }
                            }
                        }
                    ))
                }
            }
            Modal {
                trigger_id: "create-api-key-modal".to_string(),
                submit_action: create_action,
                ModalBody {
                    h3 { class: "text-lg font-semibold", "Create API Key" }
                    p { class: "text-sm text-base-content/70 mt-1", "Create an org-scoped bearer token for MCP and API access." }
                    div {
                        class: "mt-4 flex flex-col gap-3",
                        label { class: "label", "Label" }
                        input {
                            class: "input input-bordered w-full",
                            name: "label",
                            placeholder: "Production MCP client",
                            value: draft_label.unwrap_or_default(),
                            required: true
                        }
                        p { class: "text-xs text-base-content/70", "The secret is shown once immediately after creation." }
                    }
                    ModalAction {
                        Button {
                            class: "cancel-modal",
                            button_scheme: ButtonScheme::Warning,
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Create Key"
                        }
                    }
                }
            }
        }
    };

    render(page)
}
