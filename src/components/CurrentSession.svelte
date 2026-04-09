<script lang="ts">
  import { onDestroy } from "svelte";
  import { liveUpdate, currentSession } from "../lib/stores";
  import { formatDuration, categoryLabel, categoryColor } from "../lib/utils";
  import StatusIndicator from "./StatusIndicator.svelte";

  let elapsed = $state(0);
  let activity = $state("non_coding");
  let appName = $state("");

  let timer: ReturnType<typeof setInterval> | null = null;

  $effect(() => {
    const update = $liveUpdate;
    if (update) {
      elapsed = update.durationSecs;
      activity = update.currentActivity;
      appName = update.currentApp;
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
</script>

<div class="card">
  <div class="card-header">
    <h3>Current Session</h3>
    <StatusIndicator status={sessionStatus} />
  </div>

  {#if $currentSession}
    <div class="timer">
      {formatDuration(elapsed)}
    </div>
    <div class="activity">
      <span class="activity-dot" style="background: {categoryColor(activity)}"></span>
      <span class="activity-label">{categoryLabel(activity)}</span>
      {#if appName}
        <span class="activity-app">{appName}</span>
      {/if}
    </div>
  {:else}
    <div class="empty">
      No active session. Start one from the sidebar.
    </div>
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
  .timer {
    font-family: var(--font-mono);
    font-size: 40px;
    font-weight: 700;
    color: var(--text);
    line-height: 1;
    margin-bottom: 12px;
  }
  .activity {
    display: flex;
    align-items: center;
    gap: 8px;
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
  .empty {
    color: var(--text-tertiary);
    font-size: 13px;
  }
</style>
