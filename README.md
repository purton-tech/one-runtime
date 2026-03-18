# 🧠 One Runtime

> **One Runtime. Thousands of integrations.**
> Run real code. Access APIs. Control the web.

---

## 🚀 Why?

Every agent project ends up reinventing the same thing:

* ❌ hundreds of tool definitions
* ❌ brittle function calling loops
* ❌ manual pagination & retries
* ❌ OAuth headaches
* ❌ context explosion

Agents today look like this:

```text
LLM → tool → LLM → tool → LLM → tool → 😩
```

---

## ✅ What if instead…

Your agent could just write code:

```python
from one_mcp import gmail, web

messages = gmail.messages.list(query="is:unread from:stripe", limit=5)

for msg in messages:
    full = gmail.messages.get(id=msg["id"])
    print(full["subject"])
```

No tool definitions. No loops in prompts. No glue code.

---

## ⚡ What is One Runtime?

**One Runtime is a runtime for agents.**

It gives your LLM:

* 🧩 **Thousands of integrations** (via OpenAPI)
* 🔐 **Built-in OAuth handling**
* 🌐 **Web search & fetch**
* 🐍 **A Python runtime for real execution**

All behind a **single MCP interface**.

---

## 🧱 Core idea

```text
OpenAPI → Commands → Python SDK → MCP → Agent
```

Instead of this:

```text
1000 APIs → 10,000 tools → chaos
```

You get:

```text
1 MCP → programmable runtime → everything
```

---

## 🛠 Example

### User request

> “Find unread Stripe emails and summarize them”

---

### What the agent runs

```python
from one_mcp import gmail

messages = gmail.messages.list(
    query="is:unread from:stripe",
    limit=5
)

results = []

for msg in messages:
    full = gmail.messages.get(id=msg["id"])
    results.append({
        "subject": full["subject"],
        "from": full["from"]
    })

set_result(results)
```

---

### That’s it.

No tool orchestration. No context juggling.

---

## 🔌 Integrations

One Runtime turns APIs into Python automatically:

```python
from one_mcp import gmail, github, slack, notion
```

```python
github.issues.create(
    repo="acme/api",
    title="Bug report",
    body="Steps to reproduce"
)

slack.messages.send(
    channel="#alerts",
    text="Deployment complete"
)
```

---

## 🌐 Web access

```python
from one_mcp import web

results = web.search("latest Kubernetes CVEs", limit=3)

for r in results:
    page = web.fetch(r.url)
    print(page.text[:500])
```

---

## 🧠 For agents, this changes everything

Instead of:

```text
LLM → tool → LLM → tool → LLM
```

You get:

```text
LLM → writes program → runtime executes → result
```

---

## 🔐 Auth included

No OAuth implementation needed:

```python
gmail.messages.list()
```

If not connected:

```text
Not connected: Google
→ call get_tool_connect_url("google")
```

---

## ⚙️ MCP Interface

Expose just a few tools:

* `tool_search`
* `tool_help`
* `get_tool_connect_url`
* `run_python`

That’s it.

---

## 🧪 Example (LangChain)

```python
from one_mcp import MCPTool

tool = MCPTool(endpoint="http://localhost:8080")

agent.run("""
Find unread emails and summarize them
""")
```

---

## 🏗 Architecture

```text
OpenAPI specs
    ↓
Normalization layer
    ↓
Command registry
    ↓
Python SDK generation
    ↓
MCP runtime
    ↓
Agent execution
```

---

## 🧩 Why not tools?

Tools don’t scale:

* too many
* too verbose
* hard to discover
* bad for context

---

## 🧠 Why code works

Agents are good at:

* loops
* conditionals
* iteration
* data shaping

Let them use those strengths.

---

## 📦 Install

```bash
pip install one-mcp
```

---

## ▶️ Run

```bash
one-mcp serve
```

---

## 🔗 Connect an account

```python
get_tool_connect_url("google")
```

---

## 🎯 Vision

> Agents shouldn’t call tools.
> They should run code.

---

## 🧱 Built for

* AI agents
* LLM frameworks
* enterprise automation
* on-prem deployments

---

## 🗺 Roadmap

* [ ] More providers (OpenAPI ingestion)
* [ ] Sandboxed execution improvements
* [ ] Typed SDK generation
* [ ] Streaming execution
* [ ] CLI runtime (bash)

---

## 🤝 Contributing

PRs welcome — especially:

* new OpenAPI integrations
* SDK improvements
* runtime enhancements

---

## ⭐️ If this clicks…

Give it a star.