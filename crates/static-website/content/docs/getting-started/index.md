# Getting Started

One Runtime gives you a hosted way to let your users connect third-party tools without you building each provider flow yourself.

The customer journey looks like this:

1. Get an API key from One Runtime.
2. Use the API to list integrations by category.
3. Let users search or browse integrations in your product.
4. Trigger the One Runtime connection popup when a user chooses an integration.
5. Let One Runtime handle the credential flow and return the created connection.

## 1. Get an API key

Start in the One Runtime app and create an API key for your organization.

You will use that key from your backend when you:

- list integrations
- search integrations
- create or manage connections on behalf of your users

Keep the API key server-side. Your frontend should call your backend, and your backend should call One Runtime.

## 2. List integrations by category

Use the API to fetch the integration catalog and group results by category in your product.

Typical categories might include:

- CRM
- Support
- Marketing
- Productivity
- Data

This lets you build a directory or onboarding flow where users can browse the integrations that make sense for their use case.

## 3. Search integrations

Use the API search capability when you want users to find an integration directly by name or keyword.

A common pattern is:

- show featured categories first
- add a search input above the catalog
- filter the visible list as the user types

The result is a familiar "pick a tool" flow before the user starts authentication.

## 4. Launch the connection flow

When the user selects an integration, call One Runtime's JavaScript from your page and open the hosted popup.

The popup is where One Runtime collects whatever is required for that integration and completes the connection flow.

```html
<button id="connect-hubspot">Connect HubSpot</button>
<script type="module">
  import { createOneRuntime } from "https://cdn.one-runtime.com/sdk.js";

  const oneRuntime = createOneRuntime({
    publishableKey: "pk_live_your_app_key",
  });

  const button = document.getElementById("connect-hubspot");

  button.addEventListener("click", async () => {
    const result = await oneRuntime.connections.open({
      integrationSlug: "hubspot",
      endUserId: "user_123",
      endUserName: "Taylor",
      endUserEmail: "taylor@example.com",
    });

    if (result.status === "connected") {
      console.log("Connection created", result.connectionId);
    }
  });
</script>
```

The popup helper resolves a promise when the flow finishes, so your UI can update immediately after a successful connection.

## 5. Let One Runtime handle credentials

Different integrations need different authentication methods. One Runtime handles the credential collection flow for you.

### API key integrations

For API key integrations, the popup asks the user for the required key or token and stores it in the connection.

### OAuth2 integrations

For OAuth2 integrations, the popup takes the user through the provider's authorization flow and completes the redirect handling for the connection.

### No-credential integrations

Some integrations do not require credentials from the user. In those cases, the popup can complete the flow without asking for secrets.

## What your product is responsible for

Your app is responsible for:

- deciding which user is connecting
- showing the catalog or search experience
- triggering the popup
- reacting to the returned connection result

One Runtime is responsible for:

- rendering the hosted connection UI
- collecting credentials
- handling OAuth2 redirects
- creating the connection record

## Recommended end-user flow

For most products, the best experience is:

1. Show categories and search.
2. Let the user pick an integration.
3. Open the hosted popup.
4. Wait for the promise to resolve.
5. Refresh your UI to show the new connected state.

That gives you a single integration UX even though the underlying auth method may be API key, OAuth2, or no credentials.
