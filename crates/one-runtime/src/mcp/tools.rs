use std::borrow::Cow;

use rmcp::{
    ErrorData,
    handler::server::router::tool::{AsyncTool, ToolBase, ToolRouter},
    schemars::JsonSchema,
};
use serde::{Deserialize, Serialize};

use super::McpServer;

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct ToolSearchInput {
    pub query: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ToolSummary {
    pub integration: String,
    pub name: String,
    pub description: String,
    pub source: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ToolSearchOutput {
    pub matches: Vec<ToolSummary>,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct ToolHelpInput {
    pub name: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ToolParameter {
    pub name: String,
    pub location: String,
    pub required: bool,
    pub schema_type: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ToolHelpOutput {
    pub integration: String,
    pub name: String,
    pub description: String,
    pub usage: String,
    pub auth_requirement: String,
    pub parameters: Vec<ToolParameter>,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct RunPythonInput {
    pub code: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct RunPythonOutput {
    pub result: String,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct GetToolConnectUrlInput {
    pub tool_name: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct GetToolConnectUrlOutput {
    pub integration: String,
    pub connect_url: String,
    pub guidance: String,
}

pub fn router() -> ToolRouter<McpServer> {
    ToolRouter::new()
        .with_async_tool::<ToolSearchTool>()
        .with_async_tool::<ToolHelpTool>()
        .with_async_tool::<RunPythonTool>()
        .with_async_tool::<GetToolConnectUrlTool>()
}

pub struct ToolSearchTool;

impl ToolBase for ToolSearchTool {
    type Parameter = ToolSearchInput;
    type Output = ToolSearchOutput;
    type Error = ErrorData;

    fn name() -> Cow<'static, str> {
        "tool_search".into()
    }

    fn description() -> Option<Cow<'static, str>> {
        Some("Search One Runtime integrations and callable functions.".into())
    }
}

impl AsyncTool<McpServer> for ToolSearchTool {
    async fn invoke(
        service: &McpServer,
        param: Self::Parameter,
    ) -> Result<Self::Output, Self::Error> {
        let mut matches = service
            .state
            .openapi_registry
            .search(&param.query)
            .into_iter()
            .map(|item| ToolSummary {
                integration: item.integration,
                name: item.name,
                description: item.description,
                source: "openapi".to_string(),
            })
            .collect::<Vec<_>>();

        if built_in_matches("run_python", "sandboxed python execution", &param.query) {
            matches.push(ToolSummary {
                integration: "One Runtime".to_string(),
                name: "run_python".to_string(),
                description:
                    "Run sandboxed Python that can call One Runtime integration functions."
                        .to_string(),
                source: "builtin".to_string(),
            });
        }

        Ok(ToolSearchOutput { matches })
    }
}

pub struct ToolHelpTool;

impl ToolBase for ToolHelpTool {
    type Parameter = ToolHelpInput;
    type Output = ToolHelpOutput;
    type Error = ErrorData;

    fn name() -> Cow<'static, str> {
        "tool_help".into()
    }

    fn description() -> Option<Cow<'static, str>> {
        Some("Return usage details for a One Runtime tool or integration function.".into())
    }
}

impl AsyncTool<McpServer> for ToolHelpTool {
    async fn invoke(
        service: &McpServer,
        param: Self::Parameter,
    ) -> Result<Self::Output, Self::Error> {
        if param.name == "run_python" {
            return Ok(ToolHelpOutput {
                integration: "One Runtime".to_string(),
                name: "run_python".to_string(),
                description: "Execute sandboxed Python with access to built-in helpers and dynamically loaded integration functions.".to_string(),
                usage: "run_python({ code })".to_string(),
                auth_requirement: "Uses your One Runtime API key and any scoped end-user credentials available at execution time.".to_string(),
                parameters: vec![ToolParameter {
                    name: "code".to_string(),
                    location: "body".to_string(),
                    required: true,
                    schema_type: "string".to_string(),
                }],
            });
        }

        let detail = service
            .state
            .openapi_registry
            .detail(&param.name)
            .ok_or_else(|| ErrorData::invalid_params("Unknown tool name", None))?;

        let usage = if detail.parameters.is_empty() {
            format!("{}()", detail.name)
        } else {
            let signature = detail
                .parameters
                .iter()
                .map(|param| {
                    if param.required {
                        format!("{}: {}", param.name, param.schema_type)
                    } else {
                        format!("{}: {} = None", param.name, param.schema_type)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({signature})", detail.name)
        };

        Ok(ToolHelpOutput {
            integration: detail.integration,
            name: detail.name,
            description: detail.description,
            usage,
            auth_requirement:
                "Requires a valid One Runtime API key and any provider-specific end-user connection or upstream key required by this integration."
                    .to_string(),
            parameters: detail
                .parameters
                .into_iter()
                .map(|param| ToolParameter {
                    name: param.name,
                    location: param.location,
                    required: param.required,
                    schema_type: param.schema_type,
                })
                .collect(),
        })
    }
}

pub struct RunPythonTool;

impl ToolBase for RunPythonTool {
    type Parameter = RunPythonInput;
    type Output = RunPythonOutput;
    type Error = ErrorData;

    fn name() -> Cow<'static, str> {
        "run_python".into()
    }

    fn description() -> Option<Cow<'static, str>> {
        Some("Execute sandboxed Python with access to One Runtime functions.".into())
    }
}

impl AsyncTool<McpServer> for RunPythonTool {
    async fn invoke(
        service: &McpServer,
        param: Self::Parameter,
    ) -> Result<Self::Output, Self::Error> {
        let result = service
            .state
            .run_python
            .execute(param.code)
            .await
            .map_err(|err| ErrorData::internal_error(err.to_string(), None))?;

        Ok(RunPythonOutput { result })
    }
}

pub struct GetToolConnectUrlTool;

impl ToolBase for GetToolConnectUrlTool {
    type Parameter = GetToolConnectUrlInput;
    type Output = GetToolConnectUrlOutput;
    type Error = ErrorData;

    fn name() -> Cow<'static, str> {
        "get_tool_connect_url".into()
    }

    fn description() -> Option<Cow<'static, str>> {
        Some("Return a hosted One Runtime URL for connecting an integration.".into())
    }
}

impl AsyncTool<McpServer> for GetToolConnectUrlTool {
    async fn invoke(
        service: &McpServer,
        param: Self::Parameter,
    ) -> Result<Self::Output, Self::Error> {
        if param.tool_name == "run_python" {
            return Err(ErrorData::invalid_params(
                "run_python does not require a connection URL",
                None,
            ));
        }

        let integration = service
            .state
            .openapi_registry
            .detail(&param.tool_name)
            .map(|detail| detail.integration)
            .unwrap_or_else(|| param.tool_name.clone());

        let slug = slugify(&integration);
        let connect_url = format!(
            "{}/o/{}/connections/new?integration={}",
            service.state.config.app_base_url.trim_end_matches('/'),
            service.principal.org_public_id,
            slug
        );

        Ok(GetToolConnectUrlOutput {
            integration,
            connect_url,
            guidance:
                "If the integration is not connected yet, return this URL to the user so they can complete the One Runtime hosted connection flow."
                    .to_string(),
        })
    }
}

fn built_in_matches(name: &str, description: &str, query: &str) -> bool {
    let query = query.trim().to_ascii_lowercase();
    query.is_empty()
        || name.to_ascii_lowercase().contains(&query)
        || description.to_ascii_lowercase().contains(&query)
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for ch in value.chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            slug.push(lower);
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    slug.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::slugify;

    #[test]
    fn slugifies_integration_names() {
        assert_eq!(slugify("Google Calendar"), "google-calendar");
    }
}
