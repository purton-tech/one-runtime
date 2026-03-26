use anyhow::{Context, Result};
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::openai;

#[tokio::main]
async fn main() -> Result<()> {
    let _api_key = std::env::var("OPENAI_API_KEY")
        .context("OPENAI_API_KEY is required to run examples/rust/rig.rs")?;

    let client = openai::Client::from_env();
    let agent = client
        .agent(openai::GPT_4O)
        .preamble("You are a friendly assistant. Reply with a short hello.")
        .build();

    let response = agent.prompt("Say hello in one short sentence.").await?;

    println!("{response}");
    Ok(())
}
