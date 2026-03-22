# Rules and Guidelines

This is a [Rust on Nails](https://rust-on-nails.com/) project using Rust to build a full stack web application.

The workspace root is `/workspace`.
Environment variables are loaded from `/workspace/.env`.

## Tech Stack

* Axum              # Handles all the applications routes and actions https://github.com/tokio-rs/axum
* Clorinde          # Generates a Rust crate from `.sql` files with type-checked Postgres queries https://halcyonnouveau.github.io/clorinde/
* Dioxus rsx! macro # Used to create UI components and pages on the server side. https://dioxuslabs.com/
* Daisy UI          # Tailwind components https://daisyui.com/
* daisy_rsx         # A rust crate that implements the Daisy UI components in rsx!
* DbMate            # Database Migrations https://github.com/amacneil/dbmate
* Postgres          # Database
* Earthly           # Build system for production. https://earthly.dev/

## Folder: crates/db

* For detailed database workflow and conventions, see `crates/db/README.md`.
* Migrations live in `crates/db/migrations`.
* SQL queries live in `crates/db/queries`.
* Generated query code lives in `crates/db-gen` and must not be edited directly.

## Variables

`project_name=one-runtime`

## Folder: crates/${project_name}-assets

* Any images that are needed by the application are stored in a sub folder called images
* Also the tailwind config is stored here.
* The user will run `just tailwind` this will watch the tailwind `input.css` and src files for any changes.
* When changes occur the resulting `tailwind.css` is stored in a `dist` folder.
* There is a `build.rs` it uses a crate called `cache-busters` that sees the images and css files.
* It takes the hash of the files and creates a struct that gives us the ability to access the images by name in a typesafe way.
* For example the `tailwind.css` will be exported as `${project_name}::files::tailwind_css` in the app and we reference it by calling `${project_name}::files::tailwind.name`.

## Folder: crates/${project_name}-islands

This crate implements client-side interactivity using an Islands Architecture.
Use it for UI behavior that cannot be handled with server-side rendering.

It is compiled to WebAssembly and the output is deployed to the frontend via the assets crate.

Build commands (for reference only, not required for most changes):

- Compile to WASM:
  `cargo build -p ${project_name}-islands --target wasm32-unknown-unknown`

- Generate JS bindings:
  `wasm-bindgen target/wasm32-unknown-unknown/release/${project_name}_islands.wasm --target web --out-dir crates/${project_name}-assets/dist`

Do not run these commands unless explicitly required.

## Folder: crates/${project_name}-ui

* Every route has its own folder under `crates/${project_name}-ui`.
* The main page for a route lives in a file called `page.rs` inside that folder.
* Each page corresponds to a typed route defined in `crates/${project_name}-ui/routes.rs` and is called from the matching handler in `crates/${project_name}/handlers`.
* For detailed UI conventions in this crate, see `crates/one-runtime-ui/README.md`.

## Folder: crates/${project_name}

* Every route lives in its own folder under `crates/${project_name}/handlers`.
* Handler convention: each route domain in `crates/${project_name}/src/handlers/<domain>/` must use `loaders.rs` for GET handlers, `actions.rs` for POST handlers, and `mod.rs` to re-export both.
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
