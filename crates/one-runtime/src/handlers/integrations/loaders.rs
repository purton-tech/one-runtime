use crate::{CustomError, Jwt, authz, handlers};
use axum::{Extension, extract::Query, response::Html};
use clorinde::deadpool_postgres::Pool;
use one_runtime_ui::integrations;
use one_runtime_ui::integrations::model::{IntegrationCatalogFilters, IntegrationCatalogItem};
use one_runtime_ui::routes;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Default, Deserialize)]
pub struct IntegrationCatalogQuery {
    #[serde(default, rename = "q")]
    pub search_query: String,
    #[serde(default)]
    pub category: String,
}

pub async fn loader(
    routes::integrations::Index { org_id }: routes::integrations::Index,
    Query(query): Query<IntegrationCatalogQuery>,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let integrations = clorinde::queries::integrations::list_integrations()
        .bind(&transaction, &org_id)
        .all()
        .await?;
    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;

    transaction.commit().await?;

    let integrations = build_catalog_items(&org_id, integrations)?;
    let categories = available_categories(&integrations);
    let selected_category = normalize_selected_category(&query.category, &categories);
    let integrations = filter_integrations(integrations, &query.search_query, &selected_category);
    let html = integrations::page::page(
        org_id,
        balance_label,
        IntegrationCatalogFilters {
            search_query: query.search_query.trim().to_string(),
            selected_category,
            categories,
        },
        integrations,
    );
    Ok(Html(html))
}

pub async fn loader_new(
    routes::integrations::New { org_id }: routes::integrations::New,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;
    transaction.commit().await?;

    let html = integrations::upsert::page(org_id, balance_label, None, None, None);
    Ok(Html(html))
}

pub async fn loader_edit(
    routes::integrations::Edit { org_id, id }: routes::integrations::Edit,
    Extension(pool): Extension<Pool>,
    current_user: Jwt,
) -> Result<Html<String>, CustomError> {
    let integration_id = Uuid::parse_str(&id)
        .map_err(|_| CustomError::FaultySetup("Invalid integration id".to_string()))?;

    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let context = authz::init_request(&transaction, &current_user).await?;
    if context.org_id != org_id {
        return Err(CustomError::FaultySetup(
            "Requested org_id is not available for current user".to_string(),
        ));
    }

    let integration = clorinde::queries::integrations::get_integration_for_edit()
        .bind(&transaction, &integration_id, &org_id)
        .opt()
        .await?
        .ok_or_else(|| CustomError::FaultySetup("Integration not found".to_string()))?;
    let balance_label = handlers::load_balance_label(&transaction, &org_id).await?;

    transaction.commit().await?;

    let html = integrations::upsert::page(org_id, balance_label, Some(integration), None, None);
    Ok(Html(html))
}

fn build_catalog_items(
    org_id: &str,
    integrations: Vec<clorinde::queries::integrations::IntegrationCard>,
) -> Result<Vec<IntegrationCatalogItem>, CustomError> {
    integrations
        .into_iter()
        .map(|integration| {
            let spec: Value = serde_json::from_str(&integration.openapi_spec).map_err(|err| {
                CustomError::FaultySetup(format!(
                    "Invalid OpenAPI spec stored for integration: {err}"
                ))
            })?;

            Ok(IntegrationCatalogItem {
                id: integration.id.to_string(),
                name: integration.name,
                description: integration.description,
                owner_kind: integration.owner_kind,
                visibility: integration.visibility,
                can_manage: integration.can_manage,
                updated_at_label: integration.updated_at.format("%Y-%m-%d").to_string(),
                logo_url: info_string(&spec, "x-logo").or_else(|| {
                    spec.pointer("/info/x-logo/url")
                        .and_then(Value::as_str)
                        .map(ToString::to_string)
                }),
                category: info_string(&spec, "x-category"),
                developer_name: info_string(&spec, "x-developer"),
                website_url: info_string(&spec, "x-website"),
                support_url: info_string(&spec, "x-support"),
                overview_items: overview_items(&spec),
                operation_count: operation_count(&spec),
                edit_href: integration.can_manage.then(|| {
                    routes::integrations::Edit {
                        org_id: org_id.to_string(),
                        id: integration.id.to_string(),
                    }
                    .to_string()
                }),
                delete_href: integration.can_manage.then(|| {
                    routes::integrations::Delete {
                        org_id: org_id.to_string(),
                        id: integration.id.to_string(),
                    }
                    .to_string()
                }),
            })
        })
        .collect()
}

fn available_categories(integrations: &[IntegrationCatalogItem]) -> Vec<String> {
    let mut categories = vec!["All categories".to_string()];
    for integration in integrations {
        if let Some(category) = &integration.category
            && !categories.iter().any(|existing| existing == category)
        {
            categories.push(category.clone());
        }
    }
    categories
}

fn normalize_selected_category(selected_category: &str, categories: &[String]) -> String {
    let selected = selected_category.trim();
    if selected.is_empty() {
        return "All categories".to_string();
    }

    categories
        .iter()
        .find(|category| category.as_str() == selected)
        .cloned()
        .unwrap_or_else(|| "All categories".to_string())
}

fn filter_integrations(
    integrations: Vec<IntegrationCatalogItem>,
    search_query: &str,
    selected_category: &str,
) -> Vec<IntegrationCatalogItem> {
    let query = search_query.trim().to_ascii_lowercase();

    integrations
        .into_iter()
        .filter(|integration| {
            if selected_category != "All categories"
                && integration.category.as_deref() != Some(selected_category)
            {
                return false;
            }

            if query.is_empty() {
                return true;
            }

            let haystack = format!(
                "{} {} {} {}",
                integration.name,
                integration.description,
                integration.developer_name.clone().unwrap_or_default(),
                integration.overview_items.join(" ")
            )
            .to_ascii_lowercase();

            haystack.contains(&query)
        })
        .collect()
}

fn info_string(spec: &Value, key: &str) -> Option<String> {
    spec.pointer(&format!("/info/{key}"))
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .filter(|value| !value.trim().is_empty())
}

fn overview_items(spec: &Value) -> Vec<String> {
    if let Some(items) = spec
        .pointer("/info/x-overview")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        && !items.is_empty()
    {
        return items;
    }

    let mut items = Vec::new();
    let Some(paths) = spec.get("paths").and_then(Value::as_object) else {
        return items;
    };

    let methods = ["get", "post", "put", "patch", "delete", "options", "head"];
    for path_item in paths.values() {
        let Some(path_object) = path_item.as_object() else {
            continue;
        };
        for method in methods {
            let Some(operation) = path_object.get(method).and_then(Value::as_object) else {
                continue;
            };
            let summary = operation
                .get("summary")
                .and_then(Value::as_str)
                .or_else(|| operation.get("description").and_then(Value::as_str))
                .map(str::trim)
                .filter(|value| !value.is_empty());

            if let Some(summary) = summary {
                let summary = summary.to_string();
                if !items.iter().any(|item| item == &summary) {
                    items.push(summary);
                }
            }
            if items.len() >= 5 {
                return items;
            }
        }
    }

    items
}

fn operation_count(spec: &Value) -> usize {
    let Some(paths) = spec.get("paths").and_then(Value::as_object) else {
        return 0;
    };

    let methods = ["get", "post", "put", "patch", "delete", "options", "head"];
    paths
        .values()
        .filter_map(Value::as_object)
        .map(|path_object| {
            methods
                .iter()
                .filter(|method| path_object.contains_key(**method))
                .count()
        })
        .sum()
}
