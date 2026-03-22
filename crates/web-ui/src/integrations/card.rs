#![allow(non_snake_case)]

use crate::integrations::model::{IntegrationCatalogItem, modal_trigger_id};
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn IntegrationCard(integration: IntegrationCatalogItem) -> Element {
    let trigger_id = modal_trigger_id(&integration.id);

    rsx! {
        Card {
            popover_target: Some(trigger_id),
            class: Some("group h-full cursor-pointer border border-base-300 bg-base-100 text-left shadow-sm transition-all duration-150 hover:-translate-y-0.5 hover:border-primary/40 hover:shadow-md".to_string()),
            CardBody {
                class: Some("gap-3 rounded-box bg-base-100 transition-colors duration-150 group-hover:bg-base-200".to_string()),
                div {
                    class: "flex items-center gap-3",
                    IntegrationLogo {
                        logo_url: integration.logo_url.clone(),
                        title: integration.name.clone(),
                    }
                    div {
                        class: "min-w-0 flex-1 space-y-1",
                        p {
                            class: "truncate text-base font-medium text-base-content",
                            "{integration.name}"
                        }
                        p {
                            class: "line-clamp-2 text-xs font-medium text-base-content/70",
                            "{integration.description}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn IntegrationLogo(logo_url: Option<String>, title: String) -> Element {
    let initials = title
        .chars()
        .filter(|char| char.is_ascii_alphanumeric())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    rsx! {
        div {
            class: "flex h-10 w-10 shrink-0 items-center justify-center overflow-hidden rounded-lg border border-base-300 bg-base-200/40",
            if let Some(logo_url) = logo_url {
                img {
                    class: "h-full w-full object-cover",
                    src: logo_url,
                    alt: "{title} logo"
                }
            } else {
                span {
                    class: "text-xs font-semibold text-base-content/70",
                    "{initials}"
                }
            }
        }
    }
}
