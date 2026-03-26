# web-server

Axum app composition, request handlers, auth/bootstrap, MCP server, and static file serving.

## Tree

```text
src/
  main.rs          app composition, shared state, layers, startup
  config.rs        env-backed server config
  jwt.rs           current-user request extractor
  authz.rs         request authorization/bootstrap helpers
  handlers/        route domains for UI and related HTTP endpoints
  static_files.rs  typed static route + cache-busted asset lookup
  mcp/             MCP server state, auth, handler, and tools
```

## Routing

- This crate uses `axum_extra::routing::{TypedPath, RouterExt}` for typed routing.
- Define page-oriented route structs in `web_ui::routes`.
- Derive route structs with `TypedPath` and `Deserialize`.
- Handlers consume typed route structs directly as extractors.
- Domain `mod.rs` files register typed handlers with `.typed_get(...)` and `.typed_post(...)`.
- `main.rs` is the composition root. Collect domain routers with `.merge(handlers::<domain>::routes())`.
- Use manual `.route(...)` only when typed routing is not the best fit for the endpoint shape.

```rust
#[derive(TypedPath, Deserialize)]
#[typed_path("/o/{org_id}/integrations")]
pub struct Index {
    pub org_id: String,
}

pub async fn loader(routes::integrations::Index { org_id }: routes::integrations::Index) { /* ... */ }

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().typed_get(loaders::loader)
}
```

## Request Auth

- `config.rs` loads environment-backed server config.
- `jwt.rs` provides the current-user extractor from forwarded identity headers or a JWT.
- `authz.rs` initializes request context, upserts the user, ensures default org membership, and sets DB request claims.
- UI handlers should use `Jwt` and `authz::init_request(...)` instead of reimplementing auth/bootstrap logic.

## UI and API

- UI loaders fetch data and render `web-ui`.
- Actions handle mutations, redirects, and form/error responses.
- Public/API endpoints may return `Json` or `Response` directly.
- A domain router may mix typed routes and manual `.route(...)` endpoints when needed.
- Keep response/header helpers near the owning domain unless they become shared infrastructure.

## Static Files

- `static_files.rs` uses a typed path route.
- Static assets are resolved through generated `web_assets::files::StaticFile` handles.
- Static file serving includes cache-busted asset lookup and cache headers.
- Do not add ad hoc file-serving routes that bypass this path.

## Rules

- Do not duplicate page path strings across crates when a typed route exists.
- Add or update UI route definitions in `web_ui::routes` before wiring handlers.
- When adding a route to an existing domain, register it in that domain's `routes()`, not directly in `main.rs`.
- Keep `main.rs` focused on app composition, shared state, layers/extensions, static files, MCP, and true top-level routes.
- Keep routing glue thin. Put business logic in DB queries, shared helpers, or focused modules.
- Put shared cross-domain handler helpers in `src/handlers/mod.rs`.
