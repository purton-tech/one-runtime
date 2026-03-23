# static-website

Static marketing and documentation site.

## Structure

* `src/`: site generator, page definitions, and summary lists.
* `assets/`: static files copied into the generated site.
* `content/`: page and blog source content.
* `input.css`: Tailwind input stylesheet for the site build, including `@crate` directives for external Cargo packages.
* `cloudflare-build.sh`: production build entrypoint for deployment.

## Workflow

* Run static-site commands from `/workspace/crates/static-website`.
* The generator reads `assets/` and `content/` and writes `dist/` relative to the current working directory.
* Running the crate from `/workspace` will create the wrong root-level `dist/` folder.
* Keep site generation logic in `src/`.
* Add or update static assets in `assets/`.
* Add content under `content/`.
* Prefer the crate `Justfile` and `cloudflare-build.sh` for local or deployment-specific site builds.
* Keep this crate independent from the app crates unless there is a clear shared need.

## Examples

* `cd /workspace/crates/static-website && cargo run`
* `cd /workspace/crates/static-website && just wts`

## Content Types

* Use `src/pages/` plus `src/generator.rs` for bespoke Rust-rendered marketing pages.
* Use `content/blog/...` plus `src/blog_summary.rs` for dated blog content.
* Use `content/docs/...` plus a summary module registered as a document site for sidebar-style documentation pages.

## Generated Output

* Do not edit `dist/` by hand.
* `dist/` is generated output for the static site and can be replaced by a rebuild.
* If styles are missing from components rendered by external crates such as `ssg_whiz`, update the generated Tailwind input flow rather than adding ad hoc copies of generated CSS.

## Build Notes

* Tailwind for this crate is built through `/workspace/scripts/tailwind-crates`, which resolves `@crate "..."` directives into Cargo-registry `@source` entries before invoking `tailwind-extra`.
* `cloudflare-build.sh` is the deploy build path and should stay aligned with the local build flow.
