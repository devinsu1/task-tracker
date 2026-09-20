# Task Tracker

A small, offline desktop task tracker built with [Tauri](https://tauri.app), targeting Linux. It's a straight port of a single-file HTML/CSS/JS web app into a native app, kept deliberately light: vanilla JS, no framework, no bundler, no network access of any kind.

## Features

**Workspaces & tasks**
- Multiple workspaces (projects), each with its own list of tasks and sub-tasks
- Due dates, priority (High/Medium/Low), and free-text notes per task
- Manual reordering (▲/▼) — tasks stay in the order you put them, not an automatic sort
- Filter the list by search text, priority, or due-date bucket (Today / This week / Later / No date)
- Drag-and-drop task reordering between workspaces via the workspace tab bar

**Board (Scrum-lite)**
- Switch any workspace between **List** view and **Board** view
- Board view is a Kanban board with To do / In progress / Done columns (column names are editable via *Customize text*)
- Drag cards between columns to change status; checking a task or its sub-tasks keeps status in sync automatically (completing a sub-task moves the parent to *In progress*, for example)
- **Sprints**: create sprints with a name, goal, and start/end dates; assign tasks to a sprint or leave them in the backlog; filter the board/list to a specific sprint; rename or delete sprints from *Manage sprints*
- **Story points**: set a point estimate per task; each board column shows a running point total

**Changes log**
- The sidebar has an "Overview" tab (upcoming tasks across all workspaces) and a "Changes" tab
- Changes tracks additions, completions, edits, deletions (with undo), and sprint/board changes, with a filter for Task related / Text edits / Deletions / Undos / Everything

**Export**
- **Export PDF**: opens an in-app preview first (since there's no OS print dialog involved), with a toggle between a plain **Task list** report and a **Scrum** report grouped by board status, and a sprint picker to scope the export to one sprint, the backlog, or everything. Saving goes straight to a native "Save As" file dialog — the PDF is written directly, no print dialog appears.
- **Export/Import backup**: a JSON snapshot of all workspaces and labels, via native file dialogs.

**Everything else**
- Light/dark theme (follows the OS by default, toggleable)
- All on-screen labels are editable in place via *Customize text*
- 100% offline — no analytics, no update checks, no network calls of any kind

## How it's built

- **Tauri 2**, vanilla HTML/CSS/JS frontend (`src/index.html` — a single file, no build step)
- **Storage**: [`tauri-plugin-store`](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/store) — data is a JSON file in the OS app-data directory, not the browser's `localStorage`. On Linux that's `~/.local/share/com.devinsu.deliverablestracker/store.json`.
- **Backup export/import**: [`tauri-plugin-dialog`](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/dialog) + [`tauri-plugin-fs`](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/fs) for native file dialogs and reads/writes.
- **PDF export**: a small Rust command (`export_pdf` in `src-tauri/src/lib.rs`) that drives WebKitGTK's print pipeline directly, targeting GTK's "Print to File" backend — so it writes a PDF straight to the path the user picked, with no OS print dialog in the way.
- None of the above JS plugin packages are installed via npm; the frontend calls them directly through `window.__TAURI__.core.invoke(...)`, since the official JS wrappers assume a bundler and this project intentionally doesn't have one.

## Running it

Prerequisites: [Rust](https://www.rust-lang.org/tools/install), Node.js, and Tauri's [Linux system dependencies](https://tauri.app/start/prerequisites/) (WebKitGTK, GTK3, etc.).

```bash
npm install
npm run tauri dev
```

## Building a release

```bash
npm run tauri build
```

Produces a `.deb`, `.rpm`, and a distro-agnostic `.AppImage` in `src-tauri/target/release/bundle/`. On Arch (or any distro without apt/rpm), use the AppImage, or just build from source — Arch's `webkit2gtk-4.1` and `gtk3` packages are enough.

## Project layout

```
src/index.html        the entire frontend — markup, styles, and app logic
src-tauri/             Rust side: Tauri config, plugins, and the PDF export command
deliverables-tracker.html   the original standalone web app this was ported from, kept for reference
```

## License

[PolyForm Noncommercial License 1.0.0](LICENSE) — free to use, modify, and redistribute for any noncommercial purpose. Not for commercial use or resale.
