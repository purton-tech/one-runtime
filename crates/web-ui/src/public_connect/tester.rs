#![allow(non_snake_case)]

use crate::{render, routes};
use dioxus::prelude::*;
use web_assets::files::*;

pub fn page() -> String {
    let hydrate = format!(
        "import init, {{ hydrate }} from '{}'; await init('{}'); hydrate(); document.addEventListener('mu:after-render', () => {{ hydrate(); }});",
        web_islands_js.name, web_islands_bg_wasm.name
    );
    let load_sdk =
        "import { createOneRuntime } from '/connect.js'; window.OneRuntime = { createOneRuntime };";

    let docs_href = routes::public_connect::Docs.to_string();
    let openapi_href = routes::public_connect::OpenApi.to_string();

    let page = rsx! {
        head {
            title { "Integration Connection Tester" }
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            link { rel: "stylesheet", href: tailwind_css.name }
            link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/daisyui@5" }
            link { rel: "icon", "type": "image/svg+xml", href: favicon_svg.name }
            script { src: "https://unpkg.com/@digicreon/mujs@1.4.3/dist/mu.min.js" }
            script { dangerous_inner_html: "mu.init();" }
            script { "type": "module", dangerous_inner_html: load_sdk }
            script { "type": "module", dangerous_inner_html: hydrate }
        }
        body {
            class: "min-h-screen bg-base-200 text-base-content",
            main {
                class: "mx-auto flex min-h-screen w-full max-w-7xl flex-col gap-6 px-4 py-8 lg:px-8",
                section {
                    class: "hero rounded-box border border-base-300 bg-base-100 shadow-sm",
                    div {
                        class: "hero-content w-full max-w-none flex-col items-start gap-6 lg:flex-row lg:items-end lg:justify-between",
                        div {
                            class: "space-y-3",
                            div {
                                class: "flex items-center gap-3",
                                img {
                                    src: logo_svg.name,
                                    alt: "One Runtime",
                                    class: "h-12 w-12 rounded-lg border border-base-300 bg-base-200 p-2"
                                }
                                div {
                                    p { class: "text-xs font-semibold uppercase tracking-[0.2em] text-base-content/60", "Public Tester" }
                                    h1 { class: "text-3xl font-semibold", "Integration connection tester" }
                                }
                            }
                            p {
                                class: "max-w-3xl text-sm leading-6 text-base-content/75",
                                "Paste a real org API key and an end-user id to load system integrations, see saved connection status, and launch the same hosted connect modal your customers use."
                            }
                        }
                        div {
                            class: "flex flex-wrap gap-2",
                            a {
                                class: "btn btn-outline btn-sm",
                                href: docs_href.clone(),
                                "REST docs"
                            }
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

                section {
                    id: "public-integration-tester",
                    class: "grid gap-6 lg:grid-cols-[22rem_minmax(0,1fr)]",
                    "data-list-url": "/api/public/integrations",
                    "data-create-session-url": "/api/public/hosted-connection-sessions",
                    "data-connect-base-url": "/",
                    aside {
                        class: "card border border-base-300 bg-base-100 shadow-sm",
                        div {
                            class: "card-body gap-4",
                            h2 { class: "card-title", "Credentials" }
                            p {
                                class: "text-sm text-base-content/70",
                                "The API key stays in this browser session and is sent directly to the public REST endpoints."
                            }
                            form {
                                id: "public-integration-form",
                                class: "space-y-4",
                                fieldset {
                                    class: "fieldset",
                                    legend { class: "fieldset-legend", "API key" }
                                    input {
                                        id: "public-api-key",
                                        class: "input input-bordered w-full font-mono text-xs",
                                        r#type: "password",
                                        autocomplete: "off",
                                        placeholder: "oru_<prefix>_<secret>"
                                    }
                                }
                                fieldset {
                                    class: "fieldset",
                                    legend { class: "fieldset-legend", "End user id" }
                                    input {
                                        id: "public-end-user-id",
                                        class: "input input-bordered w-full",
                                        r#type: "text",
                                        autocomplete: "off",
                                        placeholder: "customer-123"
                                    }
                                }
                                button {
                                    id: "public-load-button",
                                    class: "btn btn-primary w-full",
                                    r#type: "submit",
                                    "Load integrations"
                                }
                            }
                            div {
                                class: "rounded-box border border-base-300 bg-base-200/50 p-4 text-sm text-base-content/70",
                                p { class: "font-medium text-base-content", "Status model" }
                                p { "A card is marked connected when this org already has a saved connection for the same integration and exact end-user id." }
                            }
                        }
                    }

                    div {
                        class: "space-y-4",
                        div {
                            id: "public-feedback",
                            class: "hidden alert"
                        }
                        div {
                            class: "flex items-center justify-between gap-4",
                            div {
                                h2 { class: "text-xl font-semibold", "Integrations" }
                                p {
                                    id: "public-results-summary",
                                    class: "text-sm text-base-content/65",
                                    "Enter credentials to load integrations."
                                }
                            }
                        }
                        div {
                            id: "public-loading",
                            class: "hidden rounded-box border border-dashed border-base-300 bg-base-100 px-4 py-8 text-center text-sm text-base-content/60",
                            "Loading integrations..."
                        }
                        div {
                            id: "public-empty-state",
                            class: "rounded-box border border-dashed border-base-300 bg-base-100 px-4 py-10 text-center text-sm text-base-content/60",
                            "No integrations loaded yet."
                        }
                        div {
                            id: "public-integrations-grid",
                            class: "hidden grid gap-4 md:grid-cols-2 xl:grid-cols-3"
                        }
                    }
                }
            }
        }
    };

    render(page)
}
