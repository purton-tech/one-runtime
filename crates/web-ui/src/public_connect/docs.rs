#![allow(non_snake_case)]

use crate::{render, routes};
use dioxus::prelude::*;
use web_assets::files::*;

pub fn page() -> String {
    let tester_href = routes::public_connect::Tester.to_string();
    let openapi_href = routes::public_connect::OpenApi.to_string();
    let sdk_import = "import { createOneRuntime } from 'https://your-app.example.com/connect.js';";
    let list_curl = "curl -H 'Authorization: Bearer oru_<prefix>_<secret>' \\\n  'https://your-app.example.com/api/public/integrations?end_user_id=customer-123'";
    let create_session_curl = "curl -X POST \\\n  -H 'Authorization: Bearer oru_<prefix>_<secret>' \\\n  -H 'Content-Type: application/json' \\\n  'https://your-app.example.com/api/public/hosted-connection-sessions' \\\n  -d '{\"integration_slug\":\"serper-search\",\"end_user_id\":\"customer-123\"}'";

    let page = rsx! {
        head {
            title { "Public Integrations API" }
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            link { rel: "stylesheet", href: tailwind_css.name }
            link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/daisyui@5" }
            link { rel: "icon", "type": "image/svg+xml", href: favicon_svg.name }
        }
        body {
            class: "min-h-screen bg-base-200 text-base-content",
            main {
                class: "mx-auto flex min-h-screen w-full max-w-5xl flex-col gap-6 px-4 py-8 lg:px-8",
                div {
                    class: "rounded-box border border-base-300 bg-base-100 p-6 shadow-sm",
                    div {
                        class: "flex flex-wrap items-start justify-between gap-4",
                        div {
                            class: "space-y-2",
                            h1 { class: "text-3xl font-semibold", "Public integrations REST API" }
                            p {
                                class: "max-w-3xl text-sm leading-6 text-base-content/75",
                                "These endpoints let customers list connectable integrations for an org API key, check whether a given end user is already connected, and create hosted popup sessions."
                            }
                        }
                        div {
                            class: "flex flex-wrap gap-2",
                            a { class: "btn btn-outline btn-sm", href: tester_href.clone(), "Open tester" }
                            a {
                                class: "btn btn-ghost btn-sm",
                                href: openapi_href.clone(),
                                target: "_blank",
                                rel: "noreferrer",
                                "OpenAPI JSON"
                            }
                        }
                    }
                }

                div {
                    class: "grid gap-6 lg:grid-cols-[minmax(0,1fr)_20rem]",
                    div {
                        class: "space-y-6",
                        section {
                            class: "card border border-base-300 bg-base-100 shadow-sm",
                            div {
                                class: "card-body gap-4",
                                h2 { class: "card-title", "Authentication" }
                                p { class: "text-sm text-base-content/75", "Send your org API key in the Authorization header." }
                                pre {
                                    class: "mockup-code text-xs",
                                    code { "Authorization: Bearer oru_<prefix>_<secret>" }
                                }
                            }
                        }

                        section {
                            class: "card border border-base-300 bg-base-100 shadow-sm",
                            div {
                                class: "card-body gap-4",
                                h2 { class: "card-title", "List integrations" }
                                p { class: "text-sm text-base-content/75", "Returns system integrations plus derived connection status for the supplied end user id." }
                                pre {
                                    class: "mockup-code text-xs whitespace-pre-wrap",
                                    code { "{list_curl}" }
                                }
                            }
                        }

                        section {
                            class: "card border border-base-300 bg-base-100 shadow-sm",
                            div {
                                class: "card-body gap-4",
                                h2 { class: "card-title", "Create hosted popup session" }
                                p { class: "text-sm text-base-content/75", "Creates a short-lived popup session for the real customer-facing `/connect` flow." }
                                pre {
                                    class: "mockup-code text-xs whitespace-pre-wrap",
                                    code { "{create_session_curl}" }
                                }
                                p { class: "text-sm text-base-content/65", "The response includes a `session_token` and `connect_url`. You can open the popup with `/connect.js` or by navigating to `connect_url`." }
                            }
                        }
                    }

                    aside {
                        class: "space-y-6",
                        div {
                            class: "rounded-box border border-base-300 bg-base-100 p-5 shadow-sm",
                            h3 { class: "text-lg font-semibold", "Connect SDK" }
                            p { class: "mt-2 text-sm text-base-content/75", "Browser clients can load the same lightweight popup helper used by the tester page." }
                            pre {
                                class: "mockup-code mt-3 text-xs whitespace-pre-wrap",
                                code { "{sdk_import}" }
                            }
                        }

                        div {
                            class: "rounded-box border border-base-300 bg-base-100 p-5 shadow-sm",
                            h3 { class: "text-lg font-semibold", "OpenAPI" }
                            p { class: "mt-2 text-sm text-base-content/75", "Machine-readable contract for the public REST API." }
                            a {
                                class: "link link-primary mt-3 inline-flex",
                                href: openapi_href.clone(),
                                target: "_blank",
                                rel: "noreferrer",
                                "{openapi_href}"
                            }
                        }
                    }
                }
            }
        }
    };

    render(page)
}
