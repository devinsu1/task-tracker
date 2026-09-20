# Task Tracker

A small, offline desktop task tracker built with [Tauri](https://tauri.app), targeting Linux. It's a straight port of a single-file HTML/CSS/JS web app into a native app, kept deliberately light: vanilla JS, no framework, no bundler, no network access of any kind.

The whole thing was vibe-coded, I'm not a software engineer, I just wanted freedom from paid services and bloated apps.

## Features

**Workspaces & tasks**
- Multiple workspaces (projects), each with its own list of tasks and sub-tasks
- Due dates, priority (High/Medium/Low), and free-text notes per task
- Manual reordering — drag a task or sub-task by its handle, or use the ▲/▼ buttons; tasks stay in the order you put them, not an automatic sort. Sub-tasks can only be reordered within their own parent task
- Deleting a task or sub-task asks for confirmation by default, with a "don't ask again" option and a toolbar toggle to switch back to always-confirm
- Filter the list by search text, priority, or due-date bucket (Today / This week / Later / No date)
- Drag-and-drop task reordering between workspaces via the workspace tab bar

<img width="1848" height="993" alt="image" src="https://github.com/user-attachments/assets/863a3738-a1af-4209-abdb-c5542abc2ce2" />
<img width="1844" height="993" alt="image" src="https://github.com/user-attachments/assets/69b6f0f3-eeb4-47a7-8905-e1f5b049e626" />

**Board (Scrum-lite)**
- Switch any workspace between **List** view and **Board** view
- Board view is a Kanban board with To do / In progress / Done columns (column names are editable via *Customize text*)
- Drag cards between columns to change status, or drag within/between columns to drop a card at a specific position; checking a task or its sub-tasks keeps status in sync automatically (completing a sub-task moves the parent to *In progress*, for example)
- **Sprints**: create sprints with a name, goal, and start/end dates; assign tasks to a sprint or leave them in the backlog; filter the board/list to a specific sprint; rename or delete sprints from *Manage sprints*
- **Story points**: set a point estimate per task; each board column shows a running point total

<img width="1848" height="993" alt="image" src="https://github.com/user-attachments/assets/cd1106cb-a951-485a-a270-0586cffede1a" />

**Changes log**
- The sidebar has an "Overview" tab (upcoming tasks across all workspaces) and a "Changes" tab
- Changes tracks additions, completions, edits, deletions (with undo), and sprint/board changes, with a filter for Task related / Text edits / Deletions / Undos / Everything

<img width="384" height="993" alt="image" src="https://github.com/user-attachments/assets/556ebb82-7c11-4c66-8ab4-9e47032c3752" />

**Export**
- **Export PDF**: opens an in-app preview first (since there's no OS print dialog involved), with a toggle between a plain **Task list** report and a **Scrum** report grouped by board status, and a sprint picker to scope the export to one sprint, the backlog, or everything. Saving goes straight to a native "Save As" file dialog — the PDF is written directly, no print dialog appears.
- **Export/Import backup**: a JSON snapshot of all workspaces and labels, via native file dialogs.

<img width="1844" height="993" alt="image" src="https://github.com/user-attachments/assets/76593f4a-a77a-47c3-9eec-17ee9e55a861" />
<img width="1844" height="993" alt="image" src="https://github.com/user-attachments/assets/7ca8e652-d61f-448d-82e6-6b66244b0a98" />

**Everything else**
- Light/dark theme (follows the OS by default, toggleable)
- All on-screen labels are editable in place via *Customize text*
- 100% offline — no analytics, no update checks, no network calls of any kind

<img width="1844" height="993" alt="image" src="https://github.com/user-attachments/assets/9fad5ec2-7178-4ff8-9dcb-b201f4e27f7f" />

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

**Automated releases**: pushing a `v*` tag (e.g. `v0.2.0`) triggers [`.github/workflows/release.yml`](.github/workflows/release.yml), which builds the Linux bundles on GitHub Actions and publishes them as a GitHub Release. Bump the version in `package.json`, `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.toml` first, then:

```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

## Project layout

```
src/index.html        the entire frontend — markup, styles, and app logic
src-tauri/             Rust side: Tauri config, plugins, and the PDF export command
deliverables-tracker.html   the original standalone web app this was ported from, kept for reference
```

## License

[PolyForm Noncommercial License 1.0.0](LICENSE) — free to use, modify, and redistribute for any noncommercial purpose. Not for commercial use or resale.
