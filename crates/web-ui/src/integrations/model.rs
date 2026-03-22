#[derive(Clone, Debug, PartialEq)]
pub struct IntegrationCatalogFilters {
    pub search_query: String,
    pub selected_category: String,
    pub categories: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntegrationCatalogItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner_kind: String,
    pub visibility: String,
    pub can_manage: bool,
    pub updated_at_label: String,
    pub logo_url: Option<String>,
    pub category: Option<String>,
    pub developer_name: Option<String>,
    pub website_url: Option<String>,
    pub support_url: Option<String>,
    pub overview_items: Vec<String>,
    pub operation_count: usize,
    pub edit_href: Option<String>,
    pub delete_href: Option<String>,
}

pub(crate) fn modal_trigger_id(id: &str) -> String {
    format!("integration-detail-{id}")
}
