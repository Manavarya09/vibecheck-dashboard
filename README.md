# VibeCheck Dashboard

A desktop application that monitors your AI coding tool usage and surfaces patterns you might not see yourself. Built for developers who want to understand their relationship with AI-assisted programming.

VibeCheck sits in your system tray, quietly tracking which applications you use and how you split time between AI-assisted coding, manual coding, and everything else. It stores all data locally in SQLite -- nothing leaves your machine.

## What it tracks

- **Active window monitoring** -- detects VS Code, Cursor, Claude Code, JetBrains IDEs, terminal emulators, and browser-based AI tools (Claude, ChatGPT, Copilot, Gemini)
- **Activity classification** -- categorizes every 5-second interval as AI-assisted, manual coding, or non-coding
- **Session lifecycle** -- start, pause, resume, and stop coding sessions with duration tracking
- **Daily summaries** -- aggregated time breakdowns rolled up automatically

## Classification logic

The classifier runs a priority-ordered rule chain:

1. **AI coding apps** (always AI-assisted): Cursor, Claude Code, Windsurf, Codeium
2. **Browsers with AI tabs** (AI-assisted): Claude, ChatGPT, Copilot, Gemini, Perplexity, v0, Bolt, Replit
3. **Code editors** (manual, unless AI panel detected): VS Code, Zed
4. **Terminals running AI tools** (AI-assisted): detects claude, aider, codex via window title and child process inspection
5. **Terminals with editors** (manual): Neovim, Vim, Helix, Emacs
6. **Plain terminals** (manual): Terminal, iTerm2, Warp, Ghostty, Kitty, Alacritty
7. **JetBrains IDEs** (manual, unless AI Assistant active): IntelliJ, PyCharm, WebStorm, GoLand, RustRover, CLion, and others
8. **Everything else**: non-coding

## Tech stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust |
| Framework | Tauri v2 |
| Frontend | Svelte 5 |
| Database | SQLite (rusqlite, bundled) |
| Window detection | active-win-pos-rs |
| Serialization | serde |

## Build from source

**Prerequisites:** Rust 1.77+, Node 18+, npm

```sh
git clone https://github.com/Manavarya09/vibecheck-dashboard.git
cd vibecheck-dashboard
npm install
npm run tauri build --debug
```

The app bundle appears at `src-tauri/target/debug/bundle/macos/VibeCheck.app`.

For development with hot reload:

```sh
npm run tauri dev
```

## Project structure

```
src/                          Svelte frontend
  components/                 UI components (Sidebar, CurrentSession, etc.)
  lib/                        API wrappers, stores, types, utilities
src-tauri/                    Rust backend
  src/
    db/                       SQLite schema, models, queries
    monitor/                  Window detection, classification, session tracking
    commands/                 Tauri command handlers (frontend-backend bridge)
    tray.rs                   System tray setup
    lib.rs                    Application wiring
```

## Privacy

All data stays on your machine. The SQLite database lives at `~/Library/Application Support/com.vibecheck.dashboard/vibecheck.db` on macOS. No telemetry, no cloud sync, no network requests. VibeCheck records behavioral metadata only (timestamps, app names, categories) -- never code content, prompts, or file contents.

## macOS permissions

VibeCheck works without any special permissions. However, granting **Screen Recording** permission (System Settings > Privacy & Security > Screen Recording) enables window title detection, which improves classification accuracy for browser tabs and terminal-based tools.

## License

MIT
