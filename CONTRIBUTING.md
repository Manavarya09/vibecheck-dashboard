# Contributing to VibeCheck

## Development Setup

1. Install Rust (1.77+): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install Node.js (18+): via nvm, fnm, or nodejs.org
3. Clone and install:

```sh
git clone https://github.com/Manavarya09/vibecheck-dashboard.git
cd vibecheck-dashboard
npm install
```

4. Run in development mode: `npm run tauri dev`
5. Run tests: `cd src-tauri && cargo test`

## Project Structure

- `src/` -- Svelte frontend (components, stores, utilities)
- `src-tauri/src/db/` -- SQLite schema, models, queries
- `src-tauri/src/monitor/` -- Window detection, classification, session tracking
- `src-tauri/src/commands/` -- Tauri command handlers
- `src-tauri/src/tray.rs` -- System tray
- `src-tauri/src/lib.rs` -- App wiring

## Code Style

- Rust: `cargo fmt` and `cargo clippy`
- Frontend: Prettier with 2-space indent
- Commits: imperative mood, explain why not what
- No emojis in code, UI, or commit messages

## Pull Requests

1. Branch from `main`
2. One concern per PR
3. Include tests for new classification rules
4. Run `cargo test` before pushing
5. Keep PRs small and reviewable

## Adding Classification Rules

Edit `src-tauri/src/monitor/classifier.rs`:

1. Add your rule to the appropriate section (AI app, browser, editor, terminal)
2. Add a test in the `#[cfg(test)]` module
3. Run `cargo test` to verify

## Labels

- `phase-1`, `phase-2`, `phase-3`: development phase
- `backend`, `frontend`: which codebase
- `monitor`: classification engine
- `good first issue`: newcomer-friendly
