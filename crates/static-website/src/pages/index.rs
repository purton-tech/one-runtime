use dioxus::prelude::*;
use ssg_whiz::{layouts::layout::Layout, Footer, Section};

use crate::components::homepage_hero_section::HomepageHeroSection;

pub fn page() -> String {
    let page = rsx!(
        Layout {
            title: "One Runtime".to_string(),
            description: "One Runtime is the hosted control plane for integrations, credentials, and agent-ready runtime access.".to_string(),
            image: Some("/logo.svg".to_string()),
            mobile_menu: None,
            section: Section::Home,
            main {
                class: "min-h-screen text-base-content",

                HomepageHeroSection {}

                Footer {
                    margin_top: Some("mt-0".to_string()),
                    links: crate::ui_links::footer_links(),
                }
            }
        }
    );

    ssg_whiz::render(page)
}
