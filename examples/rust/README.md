# Rust Examples

Runnable Rust examples for One Runtime.

## `rig.rs`

`rig.rs` connects a `rig` agent to One Runtime's MCP server, gives it the available tools, and prompts it to call `run_python`.

## Requirements

- `OPENAI_API_KEY`
- `ONE_RUNTIME_API_KEY`
- Optional: `ONE_RUNTIME_MCP_URL` (defaults to `https://app.one-runtime.com/v1/mcp`)

The example loads variables from `.env` when you run it from the repo root.

## Run

```bash
cargo run -p examples-rust --bin rig
```
