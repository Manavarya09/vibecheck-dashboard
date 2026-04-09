<script lang="ts">
  import { recentSessions } from "../lib/stores";
  import { formatDuration, formatTime, formatDate } from "../lib/utils";

  let sessions = $derived($recentSessions.filter((s) => s.status === "completed"));
</script>

<div class="card">
  <h3>Recent Sessions</h3>

  {#if sessions.length > 0}
    <div class="list">
      {#each sessions as session (session.id)}
        <div class="session-row">
          <div class="session-info">
            <span class="session-date">{formatDate(session.startedAt)}</span>
            <span class="session-time">
              {formatTime(session.startedAt)}
              {#if session.endedAt}
                &ndash; {formatTime(session.endedAt)}
              {/if}
            </span>
          </div>
          <div class="session-duration">
            {formatDuration(session.durationSecs)}
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="empty">No completed sessions yet.</div>
  {/if}
</div>

<style>
  .card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 20px 24px;
  }
  h3 {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    margin-bottom: 16px;
  }
  .list {
    display: flex;
    flex-direction: column;
    gap: 2px;
    max-height: 240px;
    overflow-y: auto;
  }
  .session-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    border-radius: var(--radius-sm);
    transition: background 0.15s;
  }
  .session-row:hover {
    background: var(--surface-hover);
  }
  .session-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .session-date {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
  }
  .session-time {
    font-size: 12px;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
  }
  .session-duration {
    font-family: var(--font-mono);
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
  }
  .empty {
    color: var(--text-tertiary);
    font-size: 13px;
  }
</style>
