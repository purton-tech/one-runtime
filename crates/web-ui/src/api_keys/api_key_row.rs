#![allow(non_snake_case)]

use daisy_rsx::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ApiKeyStatus {
    pub count: usize,
    pub label: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct ApiKeyRowProps {
    pub title: String,
    pub description: Element,
    pub footer: Element,
    pub status: ApiKeyStatus,
    pub action: Element,
}

#[component]
pub fn ApiKeyRow(props: ApiKeyRowProps) -> Element {
    rsx! {
        Card {
            class: Some("mt-4 card-border".to_string()),
            CardBody {
                class: Some("flex flex-row items-stretch justify-between gap-5".to_string()),
                div {
                    class: "min-w-0 flex-1",
                    h2 { class: "card-title truncate text-base", "{props.title}" }
                    div {
                        class: "mt-1 text-sm text-base-content/70",
                        {props.description}
                    }
                    div {
                        class: "mt-3 text-xs text-base-content/60",
                        {props.footer}
                    }
                }
                div {
                    class: "flex items-center gap-5",
                    div {
                        class: "text-center",
                        div { "{props.status.count}" }
                        div { class: "text-base-content/70", "{props.status.label}" }
                    }
                    div {
                        class: "ml-4 flex flex-col justify-center gap-2",
                        {props.action}
                    }
                }
            }
        }
    }
}
