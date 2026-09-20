# Deliverables tracker — native desktop port

## Goal
Port the attached `deliverables-tracker.html` into a lightweight native desktop app
using **Tauri**, primarily targeting **Linux**. The app must stay **100% offline** —
no network calls, no telemetry, nothing phoning home. No collaboration features for
this pass; that's deliberately out of scope for now.

"As light as possible to run" is the guiding constraint throughout: prefer Tauri's
plain HTML/CSS/JS template over any JS framework, avoid pulling in a bundler or
Node build step if it isn't needed, and avoid unnecessary dependencies in
`src-tauri/Cargo.toml`.

## Starting point
`deliverables-tracker.html` is a single self-contained file — no external scripts,
no fonts, no CDN calls, plain HTML/CSS/JS. It currently runs as a static page opened
directly in a browser. It should become the frontend of the Tauri app basically as-is.

## What to do
1. Scaffold a fresh Tauri project using the current official tooling (check Tauri's
   own docs/CLI for the current recommended command, since this evolves — as of this
   writing it's Tauri 2.x via `create-tauri-app` or `cargo tauri init`). Choose the
   **vanilla HTML/CSS/JS** template — no React/Vue/Svelte, no bundler — since the
   existing app doesn't need one.
2. Drop `deliverables-tracker.html` in as the frontend entry point (as `index.html`),
   keeping it working exactly as it does today as a first checkpoint.
3. **Replace `localStorage` with real file-backed storage** using Tauri's filesystem
   or store plugin, so data lives in an actual file on disk (e.g. under the OS's
   standard app-data directory) instead of browser storage tied to the webview.
   The current localStorage keys to migrate, all JSON-serialized:
   - `deliverables-tracker-workspaces` — `{ workspaces: [...], activeIds: [...] }`,
     the main data (workspaces, their deliverables, sub-deliverables)
   - `deliverables-tracker-labels` — customized UI text overrides
   - `deliverables-tracker-theme` — `"dark"` or `"light"`
   - `deliverables-tracker-export-prefs` — `{ includeCompleted: bool }`
   - `deliverables-tracker-items` — legacy key, only read once for migration on
     first run; safe to drop once ported
   Keep the same JSON shapes so the migration is a straight swap of the storage
   layer, not a data model change.
4. Verify these two browser-native features still work correctly inside Tauri's
   webview (WebKitGTK on Linux) and adjust if not:
   - `window.print()` for the PDF export (should route through the OS print dialog)
   - `<input type="date">.showPicker()` for the calendar picker
5. Get it running with `cargo tauri dev`, then produce a release build with
   `cargo tauri build` to confirm the actual installable output (AppImage/.deb on
   Linux) and check its size — that's the real test of "as light as possible."

## Explicitly out of scope for this pass
- No collaboration / sync of any kind
- No Scrum board preset or other new features — that comes after this port is
  solid and confirmed working
- No auto-update mechanism, no crash reporting, no network access of any kind

## Questions
If anything above is ambiguous or Tauri's current API differs from what's
described, ask before guessing — this is a small personal tool, not worth
over-engineering.
