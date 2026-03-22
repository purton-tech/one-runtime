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
* Each page corresponds to a typed route defined in `crates/web-ui/routes.rs` and is called from the matching handler in `crates/web-server/handlers`.
* For detailed UI conventions in this crate, see `crates/web-ui/README.md`.

## Folder: crates/web-server

* Every route lives in its own folder under `crates/web-server/src/handlers`.
* Handler convention: each route domain in `crates/web-server/src/handlers/<domain>/` must use `loaders.rs` for GET handlers, `actions.rs` for POST handlers, and `mod.rs` to re-export both.
* POST endpoints are implemented in `actions.rs` with functions prefixed by `action_`.
* `mod.rs` re-exports the loader and actions and defines the `routes()` helper used by `main.rs`.
* Each loader function fetches data from the database and renders the page.
* Actions call the appropriate database functions before redirecting the browser.

## Earthfile

* We collect all docker containers into one build here.
* When creating new crates or services they may need to be added to this.

## Running the unit tests

* Run tests after any change affecting business logic, database queries, or handlers.
* `cargo test --workspace`
