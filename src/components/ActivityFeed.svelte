<script lang="ts">
  import { liveUpdate, currentSession } from "../lib/stores";
  import { relativeTime, categoryLabel, categoryColor } from "../lib/utils";
  import type { SessionUpdate } from "../lib/types";

  interface FeedEntry {
    timestamp: string;
    app: string;
    category: string;
    id: number;
  }

  let entries: FeedEntry[] = $state([]);
  let counter = $state(0);
  let lastApp = $state("");

  $effect(() => {
    const update: SessionUpdate | null = $liveUpdate;
    if (!update) return;
    if (!$currentSession) return;

    // Only add a new entry when the app changes
    if (update.currentApp !== lastApp && update.currentApp) {
      lastApp = update.currentApp;
      counter += 1;
      const entry: FeedEntry = {
        timestamp: new Date().toISOString(),
        app: update.currentApp,
        category: update.currentActivity,
        id: counter,
      };
      entries = [entry, ...entries].slice(0, 20);
    }
  });
</script>

<div class="card">
  <div class="header">
    <p class="eyebrow">Context Switching</p>
    <h3>Activity Feed</h3>
  </div>

  {#if entries.length > 0}
    <div class="feed">
      {#each entries as entry (entry.id)}
        <div class="feed-row">
          <span
            class="feed-dot"
            style="background: {categoryColor(entry.category)}"
          ></span>
          <span class="feed-app">{entry.app}</span>
          <span class="feed-cat">{categoryLabel(entry.category)}</span>
          <span class="feed-time">{relativeTime(entry.timestamp)}</span>
        </div>
      {/each}
    </div>
  {:else}
    <div class="empty">Waiting for activity...</div>
  {/if}
</div>

<style>
  .card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 20px 24px;
    backdrop-filter: blur(10px);
    box-shadow: var(--shadow-sm);
  }
  .header {
    margin-bottom: 16px;
  }
  .eyebrow {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.16em;
    color: var(--text-tertiary);
    font-weight: 800;
    margin-bottom: 4px;
  }
  h3 {
    font-size: 22px;
    font-weight: 800;
    color: var(--ink);
    letter-spacing: -0.03em;
  }
  .feed {
    display: flex;
    flex-direction: column;
    gap: 2px;
    max-height: 200px;
    overflow-y: auto;
  }
  .feed-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    border-radius: var(--radius-sm);
    font-size: 13px;
  }
  .feed-row:first-child {
    background: var(--surface-hover);
  }
  .feed-row:hover {
    background: var(--surface-hover);
  }
  .feed-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .feed-app {
    font-weight: 500;
    color: var(--text);
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .feed-cat {
    color: var(--text-secondary);
    font-size: 12px;
    flex-shrink: 0;
  }
  .feed-time {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-tertiary);
    flex-shrink: 0;
  }
  .empty {
    color: var(--text-tertiary);
    font-size: 13px;
  }
</style>
