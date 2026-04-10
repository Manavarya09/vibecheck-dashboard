# Phase 2A: Desktop Analytics -- Design Spec

## Approach
Approach A: Extend current dashboard with tab navigation and new components. All 9 features.

## Navigation Refactor
Content area tabs: Overview | Analytics | Spending | Wrapped. Settings stays in sidebar. Tab bar: text links with terracotta underline active state.

## Database Changes

### New tables:
```sql
CREATE TABLE spending_rates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tool_name TEXT NOT NULL UNIQUE,
    rate_type TEXT NOT NULL CHECK(rate_type IN ('subscription', 'per_hour', 'per_token')),
    rate_value REAL NOT NULL,
    billing_period TEXT CHECK(billing_period IN ('monthly', 'yearly', 'hourly')),
    updated_at TEXT NOT NULL
);

CREATE TABLE session_notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL REFERENCES sessions(id),
    note TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE session_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL REFERENCES sessions(id),
    tag TEXT NOT NULL,
    UNIQUE(session_id, tag)
);

CREATE TABLE budget_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    period TEXT NOT NULL CHECK(period IN ('daily', 'weekly', 'monthly')),
    limit_amount REAL NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1
);
```

### Preset spending rates (seeded):
- Claude Pro: $20/mo subscription
- Cursor Pro: $20/mo subscription
- ChatGPT Plus: $20/mo subscription
- GitHub Copilot: $10/mo subscription
- Windsurf: $15/mo subscription
- Manual entries supported

## Feature 1: Session Heatmap
GitHub contribution graph style. 52 weeks x 7 days grid. Color intensity = total coding hours that day. Terracotta palette (light cream → deep terracotta). Pure SVG rendered in Svelte. Data from daily_summaries table. Click a cell to see that day's breakdown.

## Feature 2: Historical Analytics
Line/area charts for trends. Chart.js via svelte-chartjs (or lightweight LayerCake). Views: 7d / 30d / 90d / all time. Metrics: total hours, AI ratio, session count, avg session length. Moving averages (7-day). Rust query: aggregate activity_entries by day with category breakdown.

## Feature 3: Spending Tracker
Settings-style config page for rates. Preset rate cards + manual override. Calculate daily spend: (ai_assisted_secs / 3600) * hourly_rate for per-hour tools. For subscriptions: prorate by days-in-month. Store in spending_rates table.

## Feature 4: Spending Dashboard
Bar chart: daily/weekly/monthly spend. Running total. Breakdown by tool. Uses spending_rates joined with daily activity data.

## Feature 5: Budget Alerts
Configure limits per period (daily/weekly/monthly). When spending approaches 80% → warning event. At 100% → alert event. Frontend: toast-style notifications. Checked in the monitor loop alongside break enforcer.

## Feature 6: Session Notes & Tags
Text notes per session. Free-form tags (comma-separated input). Displayed in RecentSessions and session detail views. Useful for "what was I working on" recall.

## Feature 7: Session Comparison
Side-by-side view of two sessions. Select from RecentSessions. Shows: duration, AI ratio, app breakdown, activity timeline. Helps users see if habits are changing.

## Feature 8: VibeCheck Wrapped
Monthly summary report. Stats: total hours, AI ratio, most-used tool, longest session, break compliance, spending. Rendered as a styled card. "Share" button generates a PNG (html2canvas or dom-to-image). Accessible from Wrapped tab.

## Feature 9: Database Backup/Restore
Export: copy SQLite file to user-chosen location via Tauri file dialog. Restore: copy file back. Simple file copy -- SQLite is a single file. Buttons in Settings page.

## Charting Library
LayerCake (Svelte-native, SVG-based, lightweight). Falls back to hand-rolled SVG for heatmap.

## PR Strategy
One PR per feature, ordered by dependency:
1. Navigation refactor (tabs)
2. Charting library setup (LayerCake)
3. Session heatmap
4. Historical analytics
5. Spending rates config
6. Spending dashboard
7. Budget alerts
8. Session notes & tags
9. Session comparison
10. VibeCheck Wrapped
11. Database backup/restore
