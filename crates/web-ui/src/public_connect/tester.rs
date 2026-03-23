#![allow(non_snake_case)]

use crate::render;
use dioxus::prelude::*;
use web_assets::files::*;

pub fn page() -> String {
    let hydrate = format!(
        "import init, {{ hydrate }} from '{}'; await init('{}'); hydrate(); document.addEventListener('mu:after-render', () => {{ hydrate(); }});",
        web_islands_js.name, web_islands_bg_wasm.name
    );
    let load_sdk =
        "import { createOneRuntime } from '/connect.js'; window.OneRuntime = { createOneRuntime };";

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
                    id: "public-integration-tester",
                    class: "hero rounded-box border border-base-300 bg-base-100 shadow-sm",
                    "data-list-url": "/api/public/integrations",
                    "data-create-session-url": "/api/public/hosted-connection-sessions",
                    "data-disconnect-url": "/api/public/disconnect",
                    "data-connect-base-url": "/",
                    div {
                        class: "hero-content w-full max-w-none flex-col items-start gap-6",
                        div {
                            class: "flex w-full flex-col gap-6",
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
                        }
                        form {
                            id: "public-integration-form",
                            class: "grid w-full gap-4 rounded-box border border-base-300 bg-base-200/40 p-4 lg:grid-cols-[minmax(0,1.8fr)_minmax(0,1fr)_auto]",
                            div {
                                class: "space-y-1",
                                label {
                                    class: "text-xs font-semibold uppercase tracking-wide text-base-content/60",
                                    r#for: "public-api-key",
                                    "API key"
                                }
                                input {
                                    id: "public-api-key",
                                    class: "input input-bordered w-full font-mono text-xs",
                                    r#type: "password",
                                    autocomplete: "off",
                                    placeholder: "oru_<prefix>_<secret>"
                                }
                            }
                            div {
                                class: "space-y-1",
                                label {
                                    class: "text-xs font-semibold uppercase tracking-wide text-base-content/60",
                                    r#for: "public-end-user-id",
                                    "End user id"
                                }
                                input {
                                    id: "public-end-user-id",
                                    class: "input input-bordered w-full",
                                    r#type: "text",
                                    autocomplete: "off",
                                    placeholder: "customer-123"
                                }
                            }
                            div {
                                class: "flex items-end",
                                button {
                                    id: "public-load-button",
                                    class: "btn btn-primary w-full lg:w-auto",
                                    r#type: "submit",
                                    "Load integrations"
                                }
                            }
                        }
                    }
                }

                section {
                    class: "space-y-4",
                    div {
                        id: "public-feedback",
                        class: "hidden alert"
                    }
                    div {
                        class: "flex flex-col gap-3 rounded-box border border-base-300 bg-base-100 px-4 py-4 shadow-sm sm:flex-row sm:items-end sm:justify-between",
                        div {
                            h2 { class: "text-xl font-semibold", "Integrations" }
                            p {
                                id: "public-results-summary",
                                class: "text-sm text-base-content/65",
                                "Enter credentials to load integrations."
                            }
                        }
                        div {
                            class: "text-sm text-base-content/65",
                            "The API key stays in this browser session and is sent directly to the public REST endpoints."
                        }
                    }
                    div {
                        class: "rounded-box border border-base-300 bg-base-100 p-4 text-sm text-base-content/70 shadow-sm",
                        p { class: "font-medium text-base-content", "Status model" }
                        p { "A card is marked connected when this org already has a saved connection for the same integration and exact end-user id." }
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
    };

    render(page)
}
