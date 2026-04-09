# VibeCheck -- Development Roadmap

Status as of April 9, 2026. Maps the vision document to concrete execution.

---

## What exists today

**Desktop App (v0.1.0)** -- `vibecheck-dashboard` repo, Rust + Tauri v2 + Svelte

- System tray with session controls
- Active window monitoring (5s polling)
- Classification engine: 20+ AI tools, editors, terminals, browsers
- Background AI process detection (tracks Claude Code even when unfocused)
- SQLite local storage (sessions, activity entries, daily summaries)
- Dashboard UI: live timer, activity breakdown, feed, recent sessions
- 33 tests (25 classifier + 8 database)
- CI pipeline, Dependabot, issue/PR templates
- 55 merged PRs, 58 open issues covering future work

---

## Phase 1: Foundation (Now -- Week 4)

The goal is to have two working products and a public presence. The Chrome extension is the primary data collection layer per the vision document. The landing page is the distribution channel.

### 1A. Desktop App -- Remaining MVP features

**Repo:** `vibecheck-dashboard` (existing)

Priority features to ship before moving on:

| Feature | Effort | Issue |
|---------|--------|-------|
| Break enforcer (Pomodoro overlay after N minutes) | 3 days | #6 |
| Settings page (polling interval, break config, idle timeout) | 2 days | #34 |
| Auto-pause on idle (5 min non-coding threshold) | 1 day | #2 |
| Session auto-start/stop (detect coding without manual trigger) | 2 days | #49 |
| Startup at login (tauri-plugin-autostart) | 0.5 day | #40 |
| Screen Recording permission prompt | 0.5 day | #1 |
| Data export (JSON/CSV) | 1 day | #19 |

### 1B. Chrome Extension -- "VibeCheck Monitor"

**New repo:** `vibecheck-monitor`

The browser-side data collection layer. Detects when the user is on AI coding platforms and passively monitors engagement patterns.

**Tech stack:** TypeScript, Chrome Manifest V3, IndexedDB

**Core features (MVP):**
- Detect AI platform tabs: Claude, ChatGPT, Copilot Chat, Gemini, Cursor (web), v0, Bolt, Replit, Devin, Lovable
- Session timer: floating badge showing time on current AI platform
- Prompt counter: detect form submissions / message sends on each platform
- Inter-prompt interval tracking: measure time between submissions
- Rate-limit event detection: detect when the user hits a rate limit
- Break reminders: configurable interval notifications
- Weekly Vibe Report: auto-generated summary (session count, total hours, longest session, prompt count)
- All data in IndexedDB, zero cloud transmission

**Architecture:**
```
vibecheck-monitor/
  manifest.json          -- Manifest V3, permissions for AI platform URLs
  src/
    background.ts        -- Service worker: session lifecycle, timer, storage
    content/
      detector.ts        -- Injected into AI platforms, detects prompts/rate-limits
      platforms/
        claude.ts        -- Claude-specific DOM selectors for prompt detection
        chatgpt.ts       -- ChatGPT-specific selectors
        copilot.ts       -- Copilot Chat selectors
        gemini.ts        -- Gemini selectors
        generic.ts       -- Fallback textarea/form detection
    popup/
      Popup.svelte       -- Quick stats view when clicking extension icon
      popup.html
    options/
      Options.svelte     -- Settings page (break interval, cost rates)
      options.html
    lib/
      storage.ts         -- IndexedDB wrapper
      types.ts           -- Shared types
      platforms.ts       -- Platform URL matching rules
```

**Key implementation details:**
- Content scripts injected only on matched AI platform URLs (no broad permissions)
- Prompt detection per platform: Claude uses a specific submit button selector, ChatGPT uses another. Each platform module exports `detectSubmission()` and `detectRateLimit()`
- The floating timer is a shadow DOM element injected by the content script -- styled to be subtle and draggable
- Background service worker manages session state, aggregates stats, handles alarms for break reminders
- IndexedDB stores: `sessions`, `prompts`, `daily_stats` (mirrors desktop schema for future sync)

### 1C. Landing Page -- vibecheck.dev

**New repo:** `vibecheck-web`

**Tech stack:** Next.js 14, Tailwind CSS, Vercel

**Pages:**
- `/` -- Hero with the elevator pitch, research stats, product screenshots, download links
- `/research` -- Summary of the research paper, AIFL framework diagram, survey results
- `/download` -- Platform-specific download links (macOS .dmg, Chrome Web Store)
- `/blog` -- MDX-powered blog ("The Vibe Check Journal")
- `/about` -- Manav's profile, BITS Pilani, research context

**First 3 blog posts (part of Phase 1 deliverable per vision doc):**
1. "Why We Built VibeCheck: The Social Dilemma for AI Coding Tools"
2. "I Tracked My AI Coding Habits for 30 Days -- Here's What I Found"
3. "The AI-Induced Flow Loop: Why You Can't Stop Vibe Coding"

---

## Phase 2: Depth (Weeks 5-8)

Deepen the desktop app with analytics and intelligence. Ship the VS Code extension.

### 2A. Desktop App -- Analytics and Spending

| Feature | Effort | Issue |
|---------|--------|-------|
| Session heatmap (GitHub contribution graph style) | 2 days | #33 |
| Historical analytics (trend lines, moving averages) | 3 days | #8 |
| Spending tracker (user inputs API cost rates) | 2 days | #7 |
| Spending dashboard (daily/weekly/monthly burn rate) | 2 days | #7 |
| Budget alerts (threshold notifications) | 1 day | #7 |
| Session notes and tags | 1 day | #20 |
| Session comparison view | 1 day | #37 |
| VibeCheck Wrapped (monthly shareable report) | 3 days | #22 |
| Database backup/restore | 1 day | #41 |

### 2B. VS Code Extension -- "VibeCheck for IDE"

**New repo:** `vibecheck-vscode`

**Tech stack:** TypeScript, VS Code Extension API

**Features:**
- Detect Copilot/AI assistant completions (accepted vs rejected count)
- Status bar widget: session time, prompt count, estimated spend
- Session wrap-up command: on-demand summary of what was built
- Sync data with desktop app via local file or socket

### 2C. CLI Tool -- "vibecheck"

**New repo:** `vibecheck-cli`

**Tech stack:** Rust, clap, crossterm

**Commands:**
- `vibecheck start` -- begin session
- `vibecheck status` -- current stats (TUI dashboard)
- `vibecheck stop` -- end session with summary
- `vibecheck report` -- weekly/monthly report
- `vibecheck budget 50` -- set monthly spending cap

Communicates with the desktop app via local Unix socket or shared SQLite file.

---

## Phase 3: Intelligence (Weeks 9-12)

The differentiator. No other wellness tool does what this engine does.

### 3A. AIFL Loop Detector

**In desktop app repo:** new `src-tauri/src/intelligence/` module

A lightweight classifier trained on session telemetry to detect when the developer has entered an AI-Induced Flow Loop.

**Features (input signals):**
- Prompt frequency (rolling 10-minute window)
- Inter-prompt interval variance
- Session duration relative to user baseline
- Time-of-day deviation from habitual patterns
- Break deficit (time since last pause exceeding 5 minutes)

**Intervention escalation:**
1. Awareness: status indicator color shifts from green to yellow to red
2. Nudge: notification with stats ("Your last 12 prompts averaged 1.4 min apart")
3. Intervention: suggest exit point ("Your pattern matches sessions that lasted 3+ more hours")

**Implementation:** Rule-based heuristics first (no ML needed for v1). Train scikit-learn model later when enough user data accumulates. Deploy as ONNX in the Rust binary.

### 3B. Burnout Prediction Model

**Wellness Score (0-100) computed daily:**
- Break compliance (25%)
- Sleep-time coding avoidance (25%)
- Spending trajectory (20%)
- Session regularity (15%)
- Session-to-break ratio (15%)

New `wellness_scores` SQLite table. Dashboard card showing score with trend arrow.

### 3C. Spending Anomaly Detection

Modeled on responsible gambling "cooling-off" features. If daily spending exceeds 2x the rolling 7-day average, show a confirmation prompt before any AI tool purchase.

### 3D. Smart Session Summaries

Uses the Anthropic API (user provides their own key) to generate natural-language session summaries. Only sends metadata (timestamps, app names, categories), never code content.

---

## Phase 4: Social (Weeks 13-16)

### 4A. Gamification -- Healthy Coder Challenges

| Challenge | Condition | Badge |
|-----------|-----------|-------|
| Break Streak | 10-min break every 90 min for 7 days | Zen Coder |
| Budget Guardian | Stay within spending budget for 30 days | Budget Boss |
| Unplugged Hour | 1 hour of coding with zero AI | Manual Mode |
| Early Bird | End all AI sessions before 11 PM for 14 days | Sunrise Coder |
| The Detox | Full 48-hour break from AI coding | Digital Detox |

### 4B. Inverted Leaderboards

Rank by wellness, not productivity. Requires optional cloud sync (Supabase backend on the web dashboard).

### 4C. VibeCheck Wrapped (expanded)

Monthly and annual shareable cards for X/Twitter and LinkedIn. The viral growth mechanism.

### 4D. Accountability Partners

Opt-in buddy system with wellness score visibility and late-night coding alerts.

---

## Phase 5: Hardware (Weeks 17-24)

### 5A. Smartwatch Integration

Apple Watch / WearOS companion app:
- Heart rate monitoring during sessions
- Haptic tap after configurable sitting duration
- Session timer as watch face complication
- Research angle: correlate HRV and skin conductance with AIFL phases

### 5B. Posture and Fatigue Detection

MediaPipe or ONNX runtime, entirely on-device:
- Slouching detection via webcam
- Face proximity to screen
- Yawning detection
- Posture deterioration reminders

---

## Phase 6: Scale (Weeks 25-48)

### 6A. Open Dataset

Anonymous, opt-in, aggregated data on Kaggle / HuggingFace. The first open dataset on AI coding behavior.

### 6B. Team Features (B2B)

Team wellness dashboards, anonymized manager views, group challenges. $3/user/month tier.

### 6C. Mobile Companion App

React Native. Today's stats, push notification reminders, shareable Wrapped cards.

### 6D. Research Publications

- "State of Vibe Coding" annual survey (target: 1,000+ responses Year 1)
- A/B Testing of Interventions paper (CHI/CSCW submission)
- Validated AIFL-Q psychometric scale

---

## Execution order (what to build next)

The vision document maps to 6 repos:

| Repo | Status | Next action |
|------|--------|-------------|
| `vibecheck-dashboard` | v0.1.0 shipped | Add break enforcer, settings page, auto-pause |
| `vibecheck-monitor` | Not started | Scaffold Chrome extension, ship MVP |
| `vibecheck-web` | Not started | Next.js landing page + first 3 blog posts |
| `vibecheck-vscode` | Not started | After Chrome extension ships |
| `vibecheck-cli` | Not started | After VS Code extension ships |
| `vibecheck-mobile` | Not started | Phase 6 |

**Immediate next session priorities:**

1. Finish desktop MVP features (break enforcer, settings, auto-pause)
2. Scaffold the Chrome extension repo and build the platform detection layer
3. Build the landing page with download links and research narrative

These three deliver the Phase 1 vision document milestone: Chrome Extension + Landing Page + research paper submitted.
