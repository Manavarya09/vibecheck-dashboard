<script lang="ts">
  import { liveUpdate } from "../lib/stores";
  import { formatDuration, pct } from "../lib/utils";

  let update = $derived($liveUpdate);
  let total = $derived(
    (update?.aiAssistedSecs ?? 0) +
      (update?.manualCodingSecs ?? 0) +
      (update?.nonCodingSecs ?? 0)
  );

  let rows = $derived([
    {
      label: "AI-Assisted",
      secs: update?.aiAssistedSecs ?? 0,
      color: "var(--primary)",
    },
    {
      label: "Manual Coding",
      secs: update?.manualCodingSecs ?? 0,
      color: "var(--success)",
    },
    {
      label: "Non-Coding",
      secs: update?.nonCodingSecs ?? 0,
      color: "var(--text-tertiary)",
    },
  ]);
</script>

<div class="card">
  <div class="header">
    <div>
      <p class="eyebrow">Behavior Split</p>
      <h3>Session Breakdown</h3>
    </div>
  </div>

  {#if total > 0}
    <div class="rows">
      {#each rows as row}
        <div class="row">
          <div class="row-header">
            <span class="row-label">{row.label}</span>
            <span class="row-stats">
              {formatDuration(row.secs)}
              <span class="row-pct">{pct(row.secs, total)}%</span>
            </span>
          </div>
          <div class="row-bar-bg">
            <div
              class="row-bar-fill"
              style="width: {pct(row.secs, total)}%; background: {row.color}"
            ></div>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="empty">Start a session to see the breakdown.</div>
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
  .rows {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .row-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 4px;
  }
  .row-label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
  }
  .row-stats {
    font-family: var(--font-mono);
    font-size: 12px;
    color: var(--text-secondary);
  }
  .row-pct {
    margin-left: 6px;
    color: var(--text-tertiary);
  }
  .row-bar-bg {
    height: 6px;
    border-radius: 3px;
    background: var(--border);
    overflow: hidden;
  }
  .row-bar-fill {
    height: 100%;
    border-radius: 3px;
    transition: width 0.3s ease;
    min-width: 2px;
  }
  .empty {
    color: var(--text-tertiary);
    font-size: 13px;
  }
</style>
