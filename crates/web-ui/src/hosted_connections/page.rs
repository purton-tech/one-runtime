#![allow(non_snake_case)]

use crate::render;
use dioxus::prelude::*;
use web_assets::files::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HostedConnectionPageModel {
    pub integration_name: String,
    pub session_token: String,
    pub request_id: String,
    pub suggested_connection_name: String,
    pub end_user_id: String,
    pub end_user_name: String,
    pub end_user_email: String,
    pub error_message: Option<String>,
}

pub fn page(model: HostedConnectionPageModel) -> String {
    let HostedConnectionPageModel {
        integration_name,
        session_token,
        request_id,
        suggested_connection_name,
        end_user_id,
        end_user_name,
        end_user_email,
        error_message,
    } = model;

    let page = rsx! {
        head {
            title { "Connect {integration_name}" }
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            link { rel: "stylesheet", href: tailwind_css.name }
            link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/daisyui@5" }
            link { rel: "icon", "type": "image/svg+xml", href: favicon_svg.name }
        }
        body {
            class: "min-h-screen bg-base-200 text-base-content",
            main {
                class: "min-h-screen flex items-center justify-center p-4",
                div {
                    class: "w-full max-w-md space-y-4",
                    div {
                        class: "flex items-center justify-center",
                        img {
                            src: logo_svg.name,
                            alt: "One Runtime",
                            class: "h-12 w-12 object-contain"
                        }
                    }
                    div {
                        class: "card border border-base-300 bg-base-100 shadow-sm",
                        div {
                            class: "card-body gap-4",
                            div {
                                class: "space-y-2 text-center",
                                h1 { class: "text-2xl font-semibold", "Connect {integration_name}" }
                                p {
                                    class: "text-sm text-base-content/70",
                                    "Enter the API key for the account you want to connect."
                                }
                            }
                            if !end_user_name.is_empty() || !end_user_email.is_empty() || !end_user_id.is_empty() {
                                div {
                                    class: "rounded-box border border-base-300 bg-base-200/50 p-3 text-sm",
                                    p { class: "font-medium", "End user" }
                                    if !end_user_name.is_empty() {
                                        p { "{end_user_name}" }
                                    }
                                    if !end_user_email.is_empty() {
                                        p { "{end_user_email}" }
                                    }
                                    if !end_user_id.is_empty() {
                                        p { class: "font-mono text-xs text-base-content/70", "{end_user_id}" }
                                    }
                                }
                            }
                            if let Some(error_message) = error_message {
                                div {
                                    class: "alert alert-error",
                                    span { "{error_message}" }
                                }
                            }
                            form {
                                method: "post",
                                action: "/connect/submit",
                                class: "space-y-4",
                                input { r#type: "hidden", name: "session_token", value: "{session_token}" }
                                input { r#type: "hidden", name: "request_id", value: "{request_id}" }
                                fieldset {
                                    class: "fieldset",
                                    legend { class: "fieldset-legend", "Connection name" }
                                    input {
                                        class: "input input-bordered w-full",
                                        r#type: "text",
                                        name: "connection_name",
                                        required: true,
                                        maxlength: "120",
                                        value: "{suggested_connection_name}",
                                        placeholder: "Personal account"
                                    }
                                }
                                fieldset {
                                    class: "fieldset",
                                    legend { class: "fieldset-legend", "API key" }
                                    input {
                                        class: "input input-bordered w-full",
                                        r#type: "password",
                                        name: "api_key",
                                        required: true,
                                        autocomplete: "off",
                                        placeholder: "Paste API key"
                                    }
                                }
                                div {
                                    class: "flex justify-end gap-2",
                                    a {
                                        class: "btn btn-ghost",
                                        href: "javascript:window.close()",
                                        "Cancel"
                                    }
                                    button {
                                        class: "btn btn-primary",
                                        r#type: "submit",
                                        "Connect"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    render(page)
}

pub fn error_page(title: String, message: String) -> String {
    let page = rsx! {
        head {
            title { "{title}" }
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            link { rel: "stylesheet", href: tailwind_css.name }
            link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/daisyui@5" }
            link { rel: "icon", "type": "image/svg+xml", href: favicon_svg.name }
        }
        body {
            class: "min-h-screen bg-base-200 text-base-content",
            main {
                class: "min-h-screen flex items-center justify-center p-4",
                div {
                    class: "card w-full max-w-md border border-base-300 bg-base-100 shadow-sm",
                    div {
                        class: "card-body gap-4 text-center",
                        div {
                            class: "flex justify-center",
                            img {
                                src: logo_svg.name,
                                alt: "One Runtime",
                                class: "h-12 w-12 object-contain"
                            }
                        }
                        h1 { class: "text-2xl font-semibold", "{title}" }
                        div {
                            class: "alert alert-error justify-center",
                            span { "{message}" }
                        }
                        div {
                            class: "flex justify-center",
                            a {
                                class: "btn btn-primary",
                                href: "javascript:window.close()",
                                "Close"
                            }
                        }
                    }
                }
            }
        }
    };

    render(page)
}
