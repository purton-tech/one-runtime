#![allow(non_snake_case)]

use crate::integrations::page::{IntegrationCatalogItem, modal_trigger_id};
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
                class: Some("gap-4 rounded-box bg-base-100 transition-colors duration-150 group-hover:bg-base-200".to_string()),
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
