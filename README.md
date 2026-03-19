# One Runtime

**An MCP server that turns tool calls into program execution.**

OAuth, 100's of integrations, and sandboxed runtime — all in one place.

## The Problem

Agents don’t fail because of reasoning.  
They fail because of execution.

A simple request like:

```
Take this photo of business cards and:

* Extract all contacts
* Add them to the CRM (skip duplicates)
* Enrich each contact using web research
* Draft a follow-up email for each new contact
```

Breaks down into real system problems:

- 🔐 Managing and securing OAuth across multiple integrations (CRM, email, etc.)
- 🧩 Tool bloat — hundreds of integrations means hundreds of tool definitions
- 🧠 Context bloat — too many tools degrade model performance
- 🔁 Multiple tool calls — each step requires another round trip through the model
- ⚙️ No execution model — no loops, no state, no composition
- 🧪 No safe runtime — logic lives in prompts, not in a sandbox

## The Solution

Give agents a runtime.

Instead of calling tools, the agent writes and executes code:

```python
from one_runtime import crm, web, email, vision

contacts = vision.extract_contacts(image)

for c in contacts:
    existing = crm.contacts.search(email=c["email"])

    if not existing:
        crm.contacts.create(...)
        email.drafts.create(...)

    data = web.search(f"{c['name']} {c['company']}")
```

## What One Runtime does

- 🔐 Manages OAuth per user
  - Each user connects their own accounts
  - Supports managed OAuth or bring-your-own credentials

- 🧩 Converts APIs into functions
  - OpenAPI → functions the model can call
  - No need to define hundreds of tools in prompts

- 🔍 Eliminates tool bloat
  - Small, discoverable interface (`tool_search`, `tool_help`)

- 🐍 Executes code in a sandbox
  - Safe, isolated runtime
  - Agents can run loops, branching, retries

- 🔁 Enables pipelining
  - Data flows through a single program
  - No repeated LLM round trips

- 🌐 Unifies web + APIs
  - `web.search`, `web.fetch`, and integrations in one place

## Example

```python
from openai import OpenAI

client = OpenAI()

response = client.responses.create(
    model="gpt--5.4",
    tools=[
        {
            "type": "mcp",
            "server_url": "https://api.one-runtime.com/mcp/your_user_123",
            "headers": {
                "Authorization": "Bearer oru_abc123"
            }
        }
    ],
    input="""
Take this photo of business cards and:

- Extract all contacts
- Add them to the CRM (skip duplicates)
- Enrich each contact using web research
- Draft a follow-up email for each new contact
"""
)

print(response.output_text)
```