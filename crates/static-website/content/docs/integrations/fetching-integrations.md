# Fetching Integrations

Use the authenticated integrations endpoint from your backend to load the integrations available to one of your users and the current connection status for each one.

This is the endpoint to use when you already know which end user is connecting and you want to show:

- which integrations are available
- which ones are already connected
- which ones still need a hosted connection flow

## Endpoint

`GET /v1/integrations?end_user_id=<END_USER_ID>`

- Send your One Runtime org API key in the `Authorization` header.
- Keep the API key on your server. Do not expose it to the browser.
- Pass your own user identifier as `end_user_id`.
- This endpoint is customer-specific because the response includes connection state for that end user.

## Example

```bash
export API_KEY='oru_your_api_key_here'
```

```bash
curl --request GET \
  --url 'https://app.one-runtime.com/v1/integrations?end_user_id=user_123' \
  --header "Authorization: Bearer $API_KEY" \
  --header 'Accept: application/json'
```

## Response Shape

The response includes the requested `end_user_id` plus an `integrations` array. Each integration includes:

- `id`
- `slug`
- `name`
- `description`
- `logo_url`
- `category`
- `status`
- `supported_auth_types`

`status` is:

- `connected` when that end user already has a connection for the integration
- `not_connected` when they do not

Example response:

```json
{
  "end_user_id": "user_123",
  "integrations": [
    {
      "id": "e6fd14df-9d18-4b0f-9bde-4a6fd8d63a67",
      "slug": "hubspot",
      "name": "HubSpot",
      "description": "CRM and marketing automation.",
      "logo_url": "data:image/svg+xml;base64,...",
      "category": "CRM",
      "status": "connected",
      "supported_auth_types": ["api_key"]
    }
  ]
}
```

## What To Do With It

Typical backend flow:

1. Fetch integrations for the signed-in user.
2. Return the list to your frontend.
3. Show connected integrations separately from available ones.
4. When the user selects an unconnected integration, create a hosted connection session and open the One Runtime popup.

If you only need the public catalog without end-user state, use [Fetching the Catalog](/docs/integrations/fetching-the-catalog).

See [Getting Started](/docs/getting-started) for the full hosted connection flow.
