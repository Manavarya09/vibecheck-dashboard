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
      <h1 class="brand-name">VibeCheck</h1>
      <p class="brand-tagline">Developer Wellness</p>
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
    width: 220px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 24px 20px;
    border-right: 1px solid var(--border);
    background: var(--bg);
  }
  .brand {
    margin-bottom: 32px;
  }
  .brand-name {
    font-size: 20px;
    font-weight: 700;
    color: var(--text);
    letter-spacing: -0.02em;
  }
  .brand-version {
    font-size: 10px;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
    margin-top: 2px;
  }
  .brand-tagline {
    font-size: 12px;
    color: var(--text-tertiary);
    margin-top: 2px;
  }
  .controls {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .btn {
    width: 100%;
    padding: 10px 16px;
    border-radius: var(--radius-md);
    font-size: 13px;
    font-weight: 600;
    transition: background 0.15s, opacity 0.15s;
  }
  .btn-primary {
    background: var(--primary);
    color: white;
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
    color: var(--text);
  }
  .stat-value.accent {
    color: var(--primary);
  }
</style>
