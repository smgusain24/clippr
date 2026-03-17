# Clippr

A lightweight clipboard manager for macOS that lives in your menu bar. Built with Tauri v2, Svelte 5, and SQLite.

![macOS](https://img.shields.io/badge/macOS-000?logo=apple&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri_v2-24C8D8?logo=tauri&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte_5-FF3E00?logo=svelte&logoColor=white)

## Install

### Quick install (requires a published release)

```bash
curl -sL https://raw.githubusercontent.com/smgusain24/clippr/main/install.sh | bash
```

### Download

Grab the latest `.dmg` from [Releases](https://github.com/smgusain24/clippr/releases).

## Features

- **Menu bar app** — no dock icon, always accessible
- **Auto-capture** — polls clipboard every 500ms, deduplicates automatically
- **Search** — filter clips with debounced search
- **Pin** — keep important clips from being pruned
- **Categories** — organize clips with custom categories and icons
- **Auto-cleanup** — non-pinned clips older than 7 days are pruned automatically
- **Global shortcut** — `Cmd+Shift+V` to toggle the window
- **Dark theme** — warm, minimal UI

## Usage

| Action | How |
|---|---|
| Open/close | Click tray icon or `Cmd+Shift+V` |
| Copy a clip | Click on it |
| Pin/unpin | Right-click → Pin |
| Categorize | Right-click → Category |
| Delete | Right-click → Delete |
| Search | Type in the search bar |
| Dismiss | `Esc` or click outside |

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/)
- Xcode Command Line Tools (`xcode-select --install`)

### Run locally

```bash
npm install
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

The `.dmg` and `.app` will be in `src-tauri/target/release/bundle/`.

### Release

Push a version tag to trigger the GitHub Actions build:

```bash
git tag v0.1.0
git push origin v0.1.0
```

## Tech Stack

- **Backend**: Rust, Tauri 2, rusqlite, objc2 (native NSPasteboard)
- **Frontend**: Svelte 5 (runes), Vite 6
- **Storage**: SQLite (bundled)

## License

MIT
