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

## License

MIT
