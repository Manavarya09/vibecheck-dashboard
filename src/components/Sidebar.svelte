<script lang="ts">
  import { currentSession, todaySummary, recentSessions } from "../lib/stores";
  import {
    startSession,
    stopSession,
    pauseSession,
    resumeSession,
    getRecentSessions,
    getTodaySummary,
  } from "../lib/api";
  import { liveUpdate } from "../lib/stores";
  import { formatDuration, pct } from "../lib/utils";

  interface Props {
    onNavigate: (view: "dashboard" | "settings") => void;
    currentView: "dashboard" | "settings";
  }

  let { onNavigate, currentView }: Props = $props();
  let loading = $state(false);
  let session = $derived($currentSession);
  let isActive = $derived(session?.status === "active");
  let isPaused = $derived(session?.status === "paused");
  let hasSession = $derived(session != null);
  let todayTotal = $derived($todaySummary?.totalSecs ?? 0);
  let todayAi = $derived($todaySummary?.aiAssistedSecs ?? 0);
  let todaySessions = $derived($todaySummary?.sessionCount ?? 0);
  let aiPct = $derived(pct(todayAi, todayTotal));

  async function handleStart() {
    loading = true;
    try {
      const s = await startSession();
      currentSession.set(s);
    } catch (e) {
      console.error("Failed to start session:", e);
    }
    loading = false;
  }

  async function handleStop() {
    loading = true;
    try {
      await stopSession();
      currentSession.set(null);
      liveUpdate.set(null);
      // Refresh data after stopping
      const [summary, recent] = await Promise.all([
        getTodaySummary(),
        getRecentSessions(),
      ]);
      todaySummary.set(summary);
      recentSessions.set(recent);
    } catch (e) {
      console.error("Failed to stop session:", e);
    }
    loading = false;
  }

  async function handlePause() {
    try {
      await pauseSession();
      if (session) {
        currentSession.set({ ...session, status: "paused" });
      }
    } catch (e) {
      console.error("Failed to pause session:", e);
    }
  }

  async function handleResume() {
    try {
      await resumeSession();
      if (session) {
        currentSession.set({ ...session, status: "active" });
      }
    } catch (e) {
      console.error("Failed to resume session:", e);
    }
  }
</script>

<aside class="sidebar" data-tauri-drag-region>
  <div class="sidebar-top">
    <div class="brand">
      <div class="brand-mark" aria-hidden="true">
        <span class="brand-mark-face"></span>
      </div>
      <h1 class="brand-name">VibeCheck</h1>
      <p class="brand-tagline">Panko's Desktop Companion</p>
      <p class="brand-version">v0.1.0</p>
    </div>

    <div class="controls">
      {#if !hasSession}
        <button
          class="btn btn-primary"
          data-testid="start-session"
          onclick={handleStart}
          disabled={loading}
        >
          Start Session
        </button>
      {:else}
        {#if isActive}
          <button class="btn btn-secondary" onclick={handlePause}>
            Pause
          </button>
        {:else if isPaused}
          <button class="btn btn-primary" onclick={handleResume}>
            Resume
          </button>
        {/if}
        <button class="btn btn-danger" data-testid="stop-session" onclick={handleStop} disabled={loading}>
          Stop
        </button>
      {/if}
    </div>
  </div>

  <div class="sidebar-bottom">
    <div class="nav-row">
      <button
        class="nav-link"
        class:active={currentView === "dashboard"}
        onclick={() => onNavigate("dashboard")}
      >
        Dashboard
      </button>
      <button
        class="nav-link"
        class:active={currentView === "settings"}
        onclick={() => onNavigate("settings")}
      >
        Settings
      </button>
    </div>
    <div class="stat-row">
      <span class="stat-label">Today</span>
      <span class="stat-value">{formatDuration(todayTotal)}</span>
    </div>
    {#if todayTotal > 0}
      <div class="stat-row">
        <span class="stat-label">AI ratio</span>
        <span class="stat-value accent">{aiPct}%</span>
      </div>
    {/if}
    {#if todaySessions > 0}
      <div class="stat-row">
        <span class="stat-label">Sessions</span>
        <span class="stat-value">{todaySessions}</span>
      </div>
    {/if}
  </div>
</aside>

<style>
  .sidebar {
    width: 248px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 28px 22px;
    border-right: 1px solid rgba(116, 78, 55, 0.12);
    background: linear-gradient(180deg, rgba(255, 251, 246, 0.92), rgba(246, 236, 223, 0.78));
    backdrop-filter: blur(14px);
  }
  .brand {
    margin-bottom: 32px;
    position: relative;
  }
  .brand-mark {
    width: 46px;
    height: 46px;
    border-radius: 16px;
    background: linear-gradient(145deg, rgba(219, 118, 85, 0.18), rgba(255, 255, 255, 0.8));
    border: 1px solid rgba(116, 78, 55, 0.12);
    margin-bottom: 16px;
    position: relative;
    box-shadow: var(--shadow-sm);
  }
  .brand-mark-face {
    position: absolute;
    inset: 9px;
    border-radius: 14px;
    background:
      radial-gradient(circle at 30% 34%, #242529 0 4px, transparent 5px),
      radial-gradient(circle at 70% 34%, #242529 0 4px, transparent 5px),
      radial-gradient(circle at 50% 56%, #242529 0 5px, transparent 6px),
      linear-gradient(#fffdf8, #fffdf8);
  }
  .brand-name {
    font-size: 27px;
    font-weight: 800;
    color: var(--ink);
    letter-spacing: -0.05em;
  }
  .brand-version {
    font-size: 10px;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
    margin-top: 2px;
  }
  .brand-tagline {
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 4px;
  }
  .controls {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .btn {
    width: 100%;
    padding: 12px 16px;
    border-radius: 14px;
    font-size: 13px;
    font-weight: 700;
    transition: background 0.15s, opacity 0.15s, transform 0.15s;
  }
  .btn:hover:not(:disabled) {
    transform: translateY(-1px);
  }
  .btn-primary {
    background: linear-gradient(135deg, var(--primary), #e69264);
    color: white;
    box-shadow: 0 14px 24px rgba(219, 118, 85, 0.24);
  }
  .btn-primary:hover:not(:disabled) {
    background: var(--primary-hover);
  }
  .btn-secondary {
    background: var(--surface);
    color: var(--text);
    border: 1px solid var(--border);
  }
  .btn-secondary:hover:not(:disabled) {
    background: var(--surface-hover);
  }
  .btn-danger {
    background: none;
    color: var(--danger);
    border: 1px solid var(--danger);
  }
  .btn-danger:hover:not(:disabled) {
    background: var(--danger);
    color: white;
  }
  .nav-row {
    display: flex;
    gap: 12px;
    margin-bottom: 16px;
  }
  .nav-link {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-tertiary);
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    transition: color 0.15s;
  }
  .nav-link:hover {
    color: var(--text);
  }
  .nav-link.active {
    color: var(--primary);
  }
  .sidebar-bottom {
    border-top: 1px solid var(--border);
    padding-top: 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .stat-row {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }
  .stat-label {
    font-size: 12px;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    font-weight: 600;
  }
  .stat-value {
    font-family: var(--font-mono);
    font-size: 15px;
    font-weight: 700;
    color: var(--ink);
  }
  .stat-value.accent {
    color: var(--primary);
  }
</style>
