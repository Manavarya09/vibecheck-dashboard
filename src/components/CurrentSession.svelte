<script lang="ts">
  import { onDestroy } from "svelte";
  import { liveUpdate, currentSession } from "../lib/stores";
  import {
    formatDuration,
    formatTime,
    categoryLabel,
    categoryColor,
    pct,
  } from "../lib/utils";
  import StatusIndicator from "./StatusIndicator.svelte";

  let elapsed = $state(0);
  let activity = $state("non_coding");
  let appName = $state("");
  let aiSecs = $state(0);
  let manualSecs = $state(0);
  let otherSecs = $state(0);

  let timer: ReturnType<typeof setInterval> | null = null;

  $effect(() => {
    const update = $liveUpdate;
    if (update) {
      elapsed = update.durationSecs;
      activity = update.currentActivity;
      appName = update.currentApp;
      aiSecs = update.aiAssistedSecs;
      manualSecs = update.manualCodingSecs;
      otherSecs = update.nonCodingSecs;
    }
  });

  $effect(() => {
    const session = $currentSession;
    if (session && session.status === "active") {
      timer = setInterval(() => {
        elapsed += 1;
      }, 1000);
    } else {
      if (timer) clearInterval(timer);
      timer = null;
    }
  });

  onDestroy(() => {
    if (timer) clearInterval(timer);
  });

  let sessionStatus = $derived($currentSession?.status ?? "idle");
  let startedAt = $derived($currentSession?.startedAt ?? "");
  let total = $derived(aiSecs + manualSecs + otherSecs);
  let aiPct = $derived(pct(aiSecs, total));
</script>

<div class="card">
  <div class="card-header">
    <h3>Current Session</h3>
    <StatusIndicator status={sessionStatus} />
  </div>

  {#if $currentSession}
    <div class="timer-row">
      <div class="timer">
        {formatDuration(elapsed)}
      </div>
      {#if startedAt}
        <span class="started-at">since {formatTime(startedAt)}</span>
      {/if}
    </div>

    <div class="activity">
      <span
        class="activity-dot"
        style="background: {categoryColor(activity)}"
      ></span>
      <span class="activity-label">{categoryLabel(activity)}</span>
      {#if appName}
        <span class="activity-app">{appName}</span>
      {/if}
    </div>

    {#if total > 0}
      <div class="mini-bar-container">
        <div class="mini-bar">
          <div class="mini-seg ai" style="width: {pct(aiSecs, total)}%"></div>
          <div
            class="mini-seg manual"
            style="width: {pct(manualSecs, total)}%"
          ></div>
          <div
            class="mini-seg other"
            style="width: {pct(otherSecs, total)}%"
          ></div>
        </div>
        <span class="mini-label">{aiPct}% AI-assisted</span>
      </div>
    {/if}
  {:else}
    <div class="empty">No active session. Start one from the sidebar.</div>
  {/if}
</div>

<style>
  .card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 20px 24px;
  }
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }
  h3 {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
  .timer-row {
    display: flex;
    align-items: baseline;
    gap: 12px;
    margin-bottom: 12px;
  }
  .timer {
    font-family: var(--font-mono);
    font-size: 40px;
    font-weight: 700;
    color: var(--text);
    line-height: 1;
  }
  .started-at {
    font-size: 12px;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
  }
  .activity {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 14px;
  }
  .activity-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .activity-label {
    font-weight: 500;
    color: var(--text);
  }
  .activity-app {
    color: var(--text-secondary);
    font-size: 13px;
  }
  .mini-bar-container {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .mini-bar {
    flex: 1;
    height: 4px;
    border-radius: 2px;
    background: var(--border);
    display: flex;
    overflow: hidden;
    gap: 1px;
  }
  .mini-seg {
    min-width: 1px;
    transition: width 0.3s ease;
  }
  .mini-seg.ai {
    background: var(--primary);
  }
  .mini-seg.manual {
    background: var(--success);
  }
  .mini-seg.other {
    background: var(--text-tertiary);
  }
  .mini-label {
    font-size: 11px;
    font-family: var(--font-mono);
    color: var(--text-secondary);
    white-space: nowrap;
    flex-shrink: 0;
  }
  .empty {
    color: var(--text-tertiary);
    font-size: 13px;
  }
</style>
