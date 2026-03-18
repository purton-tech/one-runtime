# Wireframe Starter

This is a generic HTML wireframe starter for requirement gathering. It keeps a shared `index.html` shell and loads page partials with `fetch()`.

It also uses a compiled Tailwind/DaisyUI stylesheet built from `input.css` into `dist/tailwind.css`.

Because of that, do not open `index.html` with `file://`. Serve this folder over HTTP and run the Tailwind watcher when changing CSS.

## Quick start

From `/workspace/wireframe-starter` run:

```bash
python3 -m http.server 8000
```

Then open:

```text
http://localhost:8000
```

## Alternative

To rebuild CSS while editing:

```bash
tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch
```

If you want both in one command flow, use:

```bash
./wireframe-tmux.sh
```

This starts a tmux session with:

- a Python HTTP server on port `8000`
- a `tailwind-extra --watch` pane

If you already use Node.js, you can also run:

```bash
npx serve .
```

Then open the URL shown in the terminal.

## Starter structure

- `index.html` contains the shared shell, sidebar, and top bar.
- `app.js` contains the page registry, titles, and top-right action controls.
- `overview.html`, `items.html`, `item-detail.html`, `empty-state.html`, and `settings.html` are the generic example partials loaded into the shell.
- `input.css` is the Tailwind v4 entrypoint and the place to add shared `@apply` rules.

## How to customize

1. Update the page registry in `app.js`.
2. Add or remove entries from the sidebar in `index.html`.
3. Replace the example partials with project-specific requirement screens.
4. Rebuild `dist/tailwind.css` if you introduce new utility classes or components.

## Notes

- The starter intentionally includes only a small set of reusable screen patterns.
- Use `overview` for summary states, `items` for collection views, `item-detail` for forms and editing, `empty-state` for first-run flows, and `settings` for configuration.
- If the page body does not load, check that you are serving the folder over HTTP rather than opening the file directly.
