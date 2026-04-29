<script lang="ts">
  import { todaySummary } from "../lib/stores";
  import { formatDuration, pct } from "../lib/utils";

  let summary = $derived($todaySummary);
  let total = $derived(summary?.totalSecs ?? 0);
  let ai = $derived(summary?.aiAssistedSecs ?? 0);
  let manual = $derived(summary?.manualCodingSecs ?? 0);
  let other = $derived(summary?.nonCodingSecs ?? 0);
</script>

<div class="card">
  <div class="card-header">
    <div>
      <p class="eyebrow">Daily Drift</p>
      <h3>Today</h3>
    </div>
    <span class="total">{formatDuration(total)}</span>
  </div>

  {#if total > 0}
    <div class="bar-container">
      <div class="bar-segment ai" style="width: {pct(ai, total)}%"></div>
      <div class="bar-segment manual" style="width: {pct(manual, total)}%"></div>
      <div class="bar-segment other" style="width: {pct(other, total)}%"></div>
    </div>

    <div class="legend">
      <div class="legend-item">
        <span class="legend-dot ai-dot"></span>
        <span class="legend-label">AI-Assisted</span>
        <span class="legend-value">{formatDuration(ai)}</span>
      </div>
      <div class="legend-item">
        <span class="legend-dot manual-dot"></span>
        <span class="legend-label">Manual</span>
        <span class="legend-value">{formatDuration(manual)}</span>
      </div>
      <div class="legend-item">
        <span class="legend-dot other-dot"></span>
        <span class="legend-label">Other</span>
        <span class="legend-value">{formatDuration(other)}</span>
      </div>
    </div>
  {:else}
    <div class="empty">No data recorded today.</div>
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
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
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
  .total {
    font-family: var(--font-mono);
    font-size: 18px;
    font-weight: 700;
    color: var(--text);
  }
  .bar-container {
    display: flex;
    height: 8px;
    border-radius: 4px;
    overflow: hidden;
    background: var(--border);
    margin-bottom: 16px;
    gap: 1px;
  }
  .bar-segment {
    min-width: 2px;
    transition: width 0.3s ease;
  }
  .bar-segment.ai {
    background: var(--primary);
  }
  .bar-segment.manual {
    background: var(--success);
  }
  .bar-segment.other {
    background: var(--text-tertiary);
  }
  .legend {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .legend-item {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .legend-dot {
    width: 8px;
    height: 8px;
    border-radius: 2px;
    flex-shrink: 0;
  }
  .ai-dot {
    background: var(--primary);
  }
  .manual-dot {
    background: var(--success);
  }
  .other-dot {
    background: var(--text-tertiary);
  }
  .legend-label {
    font-size: 13px;
    color: var(--text);
    flex: 1;
  }
  .legend-value {
    font-family: var(--font-mono);
    font-size: 13px;
    color: var(--text-secondary);
  }
  .empty {
    color: var(--text-tertiary);
    font-size: 13px;
  }
</style>
