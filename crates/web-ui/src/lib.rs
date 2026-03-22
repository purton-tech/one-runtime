pub mod api_keys;
mod base_layout;
pub mod components;
pub mod hosted_connections;
pub mod integrations;
mod layout;
pub mod oauth_clients;
pub mod routes;

use dioxus::prelude::*;

pub fn render(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
    format!("<!DOCTYPE html><html lang='en'>{html}</html>")
}
