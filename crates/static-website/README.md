# static-website

Static marketing and documentation site.

## Structure

* `src/`: site generator, page definitions, and summary lists.
* `assets/`: static files copied into the generated site.
* `content/`: page and blog source content.
* `input.css`: Tailwind input stylesheet for the site build.
* `cloudflare-build.sh`: production build entrypoint for deployment.

## Workflow

* Keep site generation logic in `src/`.
* Add or update static assets in `assets/`.
* Add content under `content/`.
* Use the crate `Justfile` and `cloudflare-build.sh` for local or deployment-specific site builds.
* Keep this crate independent from the app crates unless there is a clear shared need.
