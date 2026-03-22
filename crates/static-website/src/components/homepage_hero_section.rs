use dioxus::prelude::*;

#[component]
pub fn HomepageHeroSection() -> Element {
    rsx! {
        section {
            id: "hero",
            class: "px-6 pt-28 pb-20 md:pt-36 md:pb-28 lg:px-12",
            div {
                class: "mx-auto max-w-5xl",
                div {
                    class: "flex flex-col items-center text-center gap-6",
                    img {
                        src: "/logo.svg",
                        alt: "One Runtime logo",
                        class: "h-20 w-20 md:h-24 md:w-24",
                    }
                    div {
                        class: "badge badge-outline badge-primary",
                        "One Runtime"
                    }
                    h1 {
                        class: "max-w-4xl text-5xl font-extrabold tracking-tight md:text-7xl",
                        "Hosted integrations and credentials for agent products"
                    }
                    p {
                        class: "max-w-3xl text-lg text-base-content/70 md:text-xl",
                        "Use One Runtime to broker provider connections, manage API and OAuth credentials, and expose integrations to your application through a single runtime layer."
                    }
                    p {
                        class: "max-w-2xl text-sm text-base-content/60 md:text-base",
                        "This site is intentionally minimal while the product and docs are being reset."
                    }
                    div {
                        class: "flex flex-wrap items-center justify-center gap-4 pt-2",
                        a {
                            class: "btn btn-primary",
                            href: "https://app.one-runtime.com",
                            "Open App"
                        }
                        a {
                            class: "btn btn-outline",
                            href: "/open-api-specs/",
                            "OpenAPI Specs"
                        }
                        a {
                            class: "btn btn-outline",
                            href: "https://github.com/purton-tech/one-runtime",
                            "View GitHub"
                        }
                    }
                }
            }
        }
    }
}
