use ssg_whiz::SitePage;

use crate::pages;

fn output_page(path: &str, html: String) -> SitePage {
    SitePage {
        path: path.to_string(),
        html,
    }
}

async fn fetch_integrations() -> Result<Vec<pages::integrations::IntegrationCard>, String> {
    let api_base_url = "https://app.one-runtime.com".to_string();
    let url = format!(
        "{}/v1/catalog/integrations",
        api_base_url.trim_end_matches('/')
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|err| format!("Failed to fetch integrations catalog from {url}: {err}"))?;

    let response = response
        .error_for_status()
        .map_err(|err| format!("Integrations catalog request failed for {url}: {err}"))?;

    let payload: pages::integrations::IntegrationCatalogResponse = response
        .json()
        .await
        .map_err(|err| format!("Failed to decode integrations catalog response: {err}"))?;

    Ok(payload.integrations)
}

pub async fn generate_static_pages() -> Vec<SitePage> {
    let integrations = fetch_integrations()
        .await
        .unwrap_or_else(|message| panic!("{message}"));

    vec![
        output_page("", pages::index::page()),
        output_page("pricing", pages::pricing::page()),
        output_page("integrations", pages::integrations::page(integrations)),
    ]
}
