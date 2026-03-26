# Fetching the Catalog

Use the public catalog endpoint when you want a general list of integrations without customer-specific connection state.

This endpoint is useful for:

- public documentation pages
- marketing pages
- general browsing before you know which end user is connecting

## Endpoint

`GET /v1/catalog/integrations`

- This endpoint does not require authentication.
- It returns the public catalog only.
- It does not include per-user connection state.

## Example

```bash
curl --request GET \
  --url 'https://app.one-runtime.com/v1/catalog/integrations' \
  --header 'Accept: application/json'
```

## Response Shape

The response includes an `integrations` array. Each integration includes:

- `id`
- `slug`
- `name`
- `description`
- `logo_url`
- `category`
- `supported_auth_types`

Example response:

```json
{
  "integrations": [
    {
      "id": "e6fd14df-9d18-4b0f-9bde-4a6fd8d63a67",
      "slug": "hubspot",
      "name": "HubSpot",
      "description": "CRM and marketing automation.",
      "logo_url": "data:image/svg+xml;base64,...",
      "category": "CRM",
      "supported_auth_types": ["api_key"]
    }
  ]
}
```

## When To Use It

Use this endpoint when you only need the catalog itself.

If you need to know whether a specific end user is already connected, use [Fetching Integrations](/docs/integrations/fetching-integrations) instead.
