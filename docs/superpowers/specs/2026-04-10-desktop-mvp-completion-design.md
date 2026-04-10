# Desktop MVP Completion -- Design Spec

## Approach
Feature-by-feature (Approach A). Each feature built end-to-end with backend + frontend + tests before moving to the next. Ordered by dependency.

## Feature 1: Settings System (Foundation)

**Database:** New `settings` table: `(key TEXT PRIMARY KEY, value TEXT, updated_at TEXT)`.

**Defaults:**
- `polling_interval_secs` = 5
- `break_interval_mins` = 25
- `break_duration_mins` = 5
- `idle_threshold_mins` = 5
- `auto_start_on_coding` = false
- `auto_stop_on_idle` = false
- `startup_at_login` = false
- `break_enforcer_enabled` = true

**Rust:** New `settings_commands.rs` with `get_settings`, `update_setting`, `reset_defaults`. Settings cached in `Arc<RwLock<HashMap<String, String>>>` (SettingsState). Monitor loop reads from cache, not SQLite.

**Svelte:** `Settings.svelte` panel toggled from sidebar. Form with labeled inputs, toggles, number steppers. No routing -- toggle view in App.svelte.

## Feature 2: Break Enforcer

**Rust:** Track `continuous_ai_secs` in monitor loop. Increments by polling interval each tick when category is `ai_assisted`. Resets on non-AI activity or pause. When >= `break_interval_mins * 60`, emit `break-reminder` event, reset counter.

**Svelte:** `BreakOverlay.svelte` -- full-screen semi-transparent overlay. Shows continuous coding duration, countdown timer (break_duration_mins). "Dismiss" and "Snooze 5 min" buttons. Listens for `break-reminder` events. Non-coercive -- always dismissible.

## Feature 3: Auto-pause on Idle

**Detection:** macOS `ioreg -c IOHIDSystem | grep HIDIdleTime`. Parse nanoseconds to seconds. New `get_idle_seconds()` in `detector.rs`.

**Logic:** Every monitor tick, check idle time. If idle >= `idle_threshold_mins * 60` and session active → auto-pause, emit `session-auto-paused`. When idle ends → auto-resume, emit `session-auto-resumed`.

**State:** New `auto_paused: AtomicBool` in MonitorState. User-pause takes priority over auto-pause.

**UI:** StatusIndicator shows "Idle -- auto-paused". Sidebar distinguishes auto-pause from manual pause.

## Feature 4: Session Auto-start/stop

**Auto-start:** On app launch (if `auto_start_on_coding` enabled), monitor loop runs in "detection mode" without an active session. When a coding app (editor, terminal, AI tool) is detected, automatically create session and start monitoring. Emit `session-auto-started`.

**Auto-stop:** If `auto_stop_on_idle` enabled and idle exceeds 2x `idle_threshold_mins`, auto-stop the session entirely (not just pause). Emit `session-auto-stopped`.

**UI:** Sidebar shows "Auto-started" badge. Session list marks auto-started sessions.

## Feature 5: Screen Recording Permission

**Check:** On app startup, call `CGPreflightScreenCaptureAccess()` via Rust FFI (or shell out to `osascript`). If not granted, window titles come back empty.

**Prompt:** If permission not granted, show a one-time dialog explaining why VibeCheck needs it (window title classification). Link to System Settings > Privacy > Screen Recording. Don't block the app -- degrade gracefully (classify by app name only).

**Rust:** New `permissions.rs` module. `check_screen_recording() -> bool`. Called during setup.

**Svelte:** `PermissionPrompt.svelte` -- modal dialog on first launch if permission missing.

## Feature 6: Startup at Login

**Implementation:** Use `tauri-plugin-autostart` crate. Toggle via settings.

**Rust:** Add plugin to Cargo.toml and wire in lib.rs. New command `set_autostart(enabled: bool)` that calls the plugin API.

**Svelte:** Toggle in Settings.svelte. Bound to `startup_at_login` setting.

## Feature 7: Data Export

**Formats:** JSON and CSV.

**Rust:** New `export_commands.rs`. `export_sessions(format: "json" | "csv", date_range: Option<(String, String)>)`. Queries all sessions + activity entries in range. Uses Tauri file dialog for save location.

**JSON:** Array of session objects with nested activity entries.
**CSV:** Flat table: one row per activity entry with session fields denormalized.

**Svelte:** Export button in sidebar bottom. Dropdown for format selection. Optional date range picker (default: all time).

## Testing Strategy
Each feature gets:
- Rust unit tests for new queries/logic
- Integration test for the command layer where applicable
- Manual verification of Svelte UI

## PR Strategy
One PR per feature, ordered 1-7. Each PR reviewed and merged before the next.
