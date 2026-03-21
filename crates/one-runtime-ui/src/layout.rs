#![allow(non_snake_case)]
use crate::base_layout::BaseLayout;
use crate::routes;
use daisy_rsx::*;
use dioxus::prelude::*;
use one_runtime_assets::files::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum SideBar {
    ApiKeys,
    Integrations,
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Eq, Debug)]
pub enum ContentWidth {
    Normal,
    Max,
    Full,
}

impl ContentWidth {
    fn container_class(&self) -> &'static str {
        match self {
            Self::Normal => "p-4 w-full max-w-3xl mx-auto",
            Self::Max => "p-4 w-full max-w-6xl mx-auto",
            Self::Full => "p-4 w-full",
        }
    }
}

impl std::fmt::Display for SideBar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[component]
pub fn Layout(
    title: String,
    org_id: String,
    balance_label: String,
    header_left: Element,
    header_right: Option<Element>,
    children: Element,
    selected_item: SideBar,
    content_width: Option<ContentWidth>,
    content_class: Option<String>,
) -> Element {
    let api_keys_icon = api_keys_svg.name;
    let integrations_icon = integrations_svg.name;
    let api_keys_href = routes::api_keys::Index {
        org_id: org_id.clone(),
    }
    .to_string();
    let integrations_href = routes::integrations::Index {
        org_id: org_id.clone(),
    }
    .to_string();
    let content_class = content_class.unwrap_or_else(|| {
        content_width
            .unwrap_or(ContentWidth::Normal)
            .container_class()
            .to_string()
    });

    rsx! {
        BaseLayout {
            title,
            fav_icon_src: Some(favicon_svg.name.into()),
            web_assembly: (
                one_runtime_islands_js.name.into(),
                one_runtime_islands_bg_wasm.name.into()
            ),
            stylesheets: vec![tailwind_css.name.to_string(), "https://cdn.jsdelivr.net/npm/daisyui@5".into()],
            header_left,
            header_right,
            sidebar: rsx!(
                NavGroup {
                    heading: "Your Menu",
                    content:  rsx!(
                        NavItem {
                            id: SideBar::ApiKeys.to_string(),
                            selected_item_id: selected_item.to_string(),
                            href: api_keys_href,
                            icon: api_keys_icon,
                            title: "API Keys"
                        }
                        NavItem {
                            id: SideBar::Integrations.to_string(),
                            selected_item_id: selected_item.to_string(),
                            href: integrations_href,
                            icon: integrations_icon,
                            title: "Integrations"
                        }
                    )
                }
            ),
            sidebar_header: rsx!(
                div {
                    class: "flex aspect-square size-8 items-center justify-center rounded-lg overflow-hidden",
                    img {
                        src: logo_svg.name,
                        alt: "App logo",
                        class: "size-8 object-contain"
                    }
                }
                div {
                    class: "ml-3 flex flex-col gap-0.5 leading-none",
                    span {
                        class: "font-semibold uppercase",
                        "READY RUN"
                    }
                    span {
                        class: "",
                        "v1.0.1"
                    }
                }
            ),
            sidebar_footer: rsx!(
                div {
                    class: "space-y-1 text-sm",
                    div {
                        class: "text-base-content/70",
                        "Remaining Balance"
                    }
                    div {
                        span { class: "font-semibold", "{balance_label}" }
                    }
                }
            ),
            div {
                class: "{content_class}",
                {children}
            }
        }
    }
}
