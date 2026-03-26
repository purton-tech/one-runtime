use anyhow::{Context, Result};
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::openai;
use rmcp::{
    ServiceExt,
    model::{ClientCapabilities, ClientInfo, Implementation, Tool},
    transport::streamable_http_client::StreamableHttpClientTransportConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load secrets from `/workspace/.env` when the example is run from the repo root.
    dotenvy::dotenv().ok();

    // Read the credentials needed for both the model provider and One Runtime.
    let _openai_api_key = std::env::var("OPENAI_API_KEY")
        .context("OPENAI_API_KEY is required to run examples/rust/rig.rs")?;
    let one_runtime_api_key = std::env::var("ONE_RUNTIME_API_KEY")
        .context("ONE_RUNTIME_API_KEY is required to run examples/rust/rig.rs")?;
    let mcp_url = std::env::var("ONE_RUNTIME_MCP_URL")
        .unwrap_or_else(|_| "https://app.one-runtime.com/v1/mcp".to_string());

    // Open an authenticated MCP connection to One Runtime.
    let transport = rmcp::transport::StreamableHttpClientTransport::from_config(
        StreamableHttpClientTransportConfig::with_uri(mcp_url.clone())
            .auth_header(one_runtime_api_key),
    );

    let client_info = ClientInfo {
        capabilities: ClientCapabilities::default(),
        client_info: Implementation::from_build_env(),
        ..Default::default()
    };

    // Initialize the session so we can inspect the server metadata and capabilities.
    let mcp_client = client_info
        .serve(transport)
        .await
        .with_context(|| format!("failed to connect to One Runtime MCP at {mcp_url}"))?;

    let mut mcp_instructions = "No MCP instructions provided.".to_string();
    if let Some(server_info) = mcp_client.peer_info() {
        eprintln!(
            "Connected to MCP server: {} {}",
            server_info.server_info.name, server_info.server_info.version
        );
        mcp_instructions = server_info
            .instructions
            .clone()
            .unwrap_or_else(|| "No MCP instructions provided.".to_string());
        eprintln!("MCP instructions: {mcp_instructions}");
    }

    // Fetch the server's tool list so the example can show what MCP exposes.
    let tools: Vec<Tool> = mcp_client
        .list_tools(Default::default())
        .await
        .context("failed to list One Runtime MCP tools")?
        .tools;

    eprintln!("Available MCP tools:");
    for tool in &tools {
        let description = tool
            .description
            .as_deref()
            .unwrap_or("No description provided.");
        eprintln!("- {}: {}", tool.name, description);
    }

    // Build the model agent and fold the MCP instructions into the system prompt.
    let client = openai::Client::from_env();
    let preamble = format!(
        "You are using One Runtime over MCP.\n\nServer instructions:\n{mcp_instructions}\n\nWhen asked to execute Python, call the run_python tool instead of simulating the result."
    );
    let agent = client
        .agent(openai::GPT_4O)
        .preamble(&preamble)
        .rmcp_tools(tools, mcp_client.peer().to_owned())
        .build();

    // Trigger a small calculation so the agent demonstrates an actual MCP tool call.
    let response: String = agent
        .prompt(
            "Use the run_python tool to execute exactly this Python code:\nresult = (37 * 42) + 11\nprint(result)\nThen reply with one short sentence explaining the calculation and include the printed output.",
        )
        .max_turns(4)
        .await
        .context("agent failed while calling One Runtime MCP tools")?;

    println!("{response}");
    Ok(())
}
