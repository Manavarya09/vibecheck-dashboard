# Changelog

All notable changes to VibeCheck Dashboard are documented here.

## [0.1.0] - 2026-04-09

### Added
- System tray application with show/pause/quit menu
- Active window monitoring with 5-second polling interval
- Activity classification engine with rule-based detection
  - Standalone AI apps: Cursor, Claude Code, Windsurf, Codeium, Supermaven, Tabnine, Amazon Q
  - Browser AI tools: Claude, ChatGPT, Copilot, Gemini, Perplexity, v0, Bolt, Replit, Devin, Lovable
  - Code editors: VS Code, Zed, JetBrains IDEs, Xcode
  - Terminal emulators with AI tool and editor detection
- Background AI process detection (tracks Claude Code even when window not focused)
- SQLite database with sessions, activity entries, and daily summaries
- Svelte dashboard with real-time updates via Tauri event system
  - Current session card with live timer and AI percentage
  - Today summary with stacked category bar
  - Activity breakdown with proportional bars
  - Live activity feed with app transitions
  - Recent sessions list
- Sidebar with session controls and daily stats
- Tray tooltip showing live session duration
- Claude-branded design system (warm cream, terracotta accent)
- 25 classifier unit tests and 8 database integration tests
- CI pipeline with Rust fmt/clippy/test and frontend type checking
