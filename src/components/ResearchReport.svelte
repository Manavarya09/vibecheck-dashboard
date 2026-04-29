<script lang="ts">
  import { onMount } from "svelte";
  import { exportResearchReport, getHistoricalStats } from "../lib/api";
  import type { DailySummary } from "../lib/types";
  import { formatDuration } from "../lib/utils";

  let history = $state<DailySummary[]>([]);
  let loaded = $state(false);
  let exporting = $state<"json" | "csv" | null>(null);

  onMount(async () => {
    history = await getHistoricalStats(30);
    loaded = true;
  });

  let activeDays = $derived(history.filter((day) => day.totalSecs > 0).length);
  let longDays = $derived(history.filter((day) => day.totalSecs >= 7200).length);
  let aiHeavyDays = $derived(
    history.filter((day) => day.totalSecs > 0 && day.aiAssistedSecs / day.totalSecs >= 0.7).length
  );
  let totalTracked = $derived(history.reduce((sum, day) => sum + day.totalSecs, 0));
  let totalAi = $derived(history.reduce((sum, day) => sum + day.aiAssistedSecs, 0));
  let meanDay = $derived(activeDays > 0 ? Math.round(totalTracked / activeDays) : 0);
  let aiShare = $derived(totalTracked > 0 ? Math.round((totalAi / totalTracked) * 100) : 0);

  async function handleExport(format: "json" | "csv") {
    exporting = format;
    try {
      const payload = await exportResearchReport(format);
      const blob = new Blob([payload], {
        type: format === "json" ? "application/json" : "text/csv",
      });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `vibecheck-research-report.${format}`;
      a.click();
      URL.revokeObjectURL(url);
    } finally {
      exporting = null;
    }
  }
</script>

<section class="report">
  <div class="report-header">
    <div>
      <p class="eyebrow">Research Export</p>
      <h3>Paper-Friendly Session Report</h3>
    </div>
    <div class="actions">
      <button class="action" onclick={() => handleExport("json")} disabled={exporting !== null}>
        Export JSON
      </button>
      <button class="action" onclick={() => handleExport("csv")} disabled={exporting !== null}>
        Export CSV
      </button>
    </div>
  </div>

  {#if loaded}
    <div class="stats">
      <div class="stat">
        <span class="label">Tracked window</span>
        <strong>30 days</strong>
      </div>
      <div class="stat">
        <span class="label">Active days</span>
        <strong>{activeDays}</strong>
      </div>
      <div class="stat">
        <span class="label">Mean active day</span>
        <strong>{formatDuration(meanDay)}</strong>
      </div>
      <div class="stat">
        <span class="label">AI share</span>
        <strong>{aiShare}%</strong>
      </div>
      <div class="stat">
        <span class="label">Long days</span>
        <strong>{longDays}</strong>
      </div>
      <div class="stat">
        <span class="label">AI-heavy days</span>
        <strong>{aiHeavyDays}</strong>
      </div>
    </div>

    <div class="notes">
      <p>
        The exported report includes completed-session metrics such as AI/manual/non-coding time,
        AI share, app switches, distinct apps, browser-AI ticks, terminal-AI ticks, long-session
        flags, and local start hour.
      </p>
    </div>
  {:else}
    <p class="loading">Loading report metrics...</p>
  {/if}
</section>

<style>
  .report {
    padding: 22px 24px;
    border-radius: var(--radius-lg);
    background: var(--surface);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-sm);
    backdrop-filter: blur(10px);
  }
  .report-header {
    display: flex;
    justify-content: space-between;
    align-items: start;
    gap: 12px;
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
  .actions {
    display: flex;
    gap: 8px;
  }
  .action {
    padding: 10px 12px;
    border-radius: 14px;
    background: linear-gradient(135deg, var(--primary), #e69264);
    color: white;
    font-size: 12px;
    font-weight: 700;
  }
  .stats {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 10px;
    margin-bottom: 14px;
  }
  .stat {
    padding: 14px;
    border-radius: 18px;
    background: rgba(255, 255, 255, 0.68);
  }
  .label {
    display: block;
    margin-bottom: 6px;
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--text-tertiary);
    font-weight: 800;
  }
  .stat strong {
    font-size: 20px;
    color: var(--ink);
  }
  .notes p,
  .loading {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.55;
  }
  @media (max-width: 1040px) {
    .stats {
      grid-template-columns: 1fr 1fr;
    }
  }
</style>
