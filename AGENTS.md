# Rules and Guidelines

This is a [Rust on Nails](https://rust-on-nails.com/) project using Rust to build a full stack web application.

The workspace root is `/workspace`.
Environment variables are loaded from `/workspace/.env`.

## Tech Stack

* Axum: routing and handlers
* Clorinde: typed SQL query generation
* Dioxus: server-rendered UI
* Daisy UI / `daisy_rsx`: UI components
* DbMate: migrations
* Postgres: database
* Earthly: production builds

## Folder: crates/db

* For detailed database workflow and conventions, see `crates/db/README.md`.
* Migrations live in `crates/db/migrations`.
* SQL queries live in `crates/db/queries`.
* Generated query code lives in `crates/db-gen` and must not be edited directly.

## Folder: crates/web-assets

* For detailed asset workflow and conventions, see `crates/web-assets/README.md`.
* Source assets live in `crates/web-assets/images`.
* Generated frontend assets live in `crates/web-assets/dist`.

## Folder: crates/web-islands

* For detailed islands workflow and conventions, see `crates/web-islands/README.md`.
* Use this crate for client-side interactivity that cannot be handled with server rendering alone.

## Folder: crates/web-ui

* Every route has its own folder under `crates/web-ui`.
* The main page for a route lives in a file called `page.rs` inside that folder.
* Each page corresponds to a typed route defined in `crates/web-ui/routes.rs` and is called from the matching handler in `crates/web-server/src/handlers`.
* For detailed UI conventions in this crate, see `crates/web-ui/README.md`.

## Folder: crates/web-server

* For detailed server architecture and routing conventions, see `crates/web-server/README.md`.
* `crates/web-server` is the Axum composition crate: `main.rs` wires shared state, layers, MCP, static files, and merges handler-domain routers.
* Route domains live under `crates/web-server/src/handlers/<domain>/`.
* Handler convention: each route domain uses `loaders.rs` for GET/page-loading handlers, `actions.rs` for POST/mutation handlers, and `mod.rs` to define the domain `routes()` function used by `main.rs`.
* Use typed routing with `axum_extra::routing::{TypedPath, RouterExt}` for app routes where possible.
* UI/page route definitions live in `crates/web-ui/src/routes.rs`; server handlers consume those typed route structs directly.
* Each loader fetches data and renders `web-ui`; actions perform mutations and return redirects, form responses, or API responses.
* Public/API endpoints may use manual `.route(...)` wiring inside a domain `routes()` when typed routing is not the best fit.
* Shared request helpers belong in `crates/web-server/src/handlers/mod.rs`; request auth/bootstrap lives in `config.rs`, `jwt.rs`, and `authz.rs`.
* Static assets are served through `crates/web-server/src/static_files.rs` and `crates/web-assets`; do not bypass the typed static-file path with ad hoc file serving.

## Folder: crates/static-website

* For detailed workflow and conventions, see `crates/static-website/README.md`.
* This crate builds the static marketing/docs website.

## Earthfile

* We collect all docker containers into one build here.
* When creating new crates or services they may need to be added to this.

## Running the unit tests

* Run tests after any change affecting business logic, database queries, or handlers.
* `cargo test --workspace`
