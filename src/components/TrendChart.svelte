<script lang="ts">
  import { onMount } from "svelte";
  import { getHistoricalStats } from "../lib/api";
  import type { DailySummary } from "../lib/types";

  let data = $state<DailySummary[]>([]);
  let period = $state(30);
  let loaded = $state(false);

  async function load() {
    loaded = false;
    data = await getHistoricalStats(period);
    loaded = true;
  }

  onMount(load);

  let maxHours = $derived(Math.max(...data.map(d => d.totalSecs / 3600), 1));

  const CHART_W = 700;
  const CHART_H = 200;
  const PAD = 40;

  function xPos(i: number): number {
    if (data.length <= 1) return PAD;
    return PAD + (i / (data.length - 1)) * (CHART_W - PAD * 2);
  }

  function yPos(secs: number): number {
    const h = secs / 3600;
    return CHART_H - PAD - (h / maxHours) * (CHART_H - PAD * 2);
  }

  let totalLine = $derived(
    data.map((d, i) => `${i === 0 ? "M" : "L"}${xPos(i)},${yPos(d.totalSecs)}`).join(" ")
  );

  let aiLine = $derived(
    data.map((d, i) => `${i === 0 ? "M" : "L"}${xPos(i)},${yPos(d.aiAssistedSecs)}`).join(" ")
  );

  // 7-day moving average
  function movingAvg(values: number[], window: number): number[] {
    return values.map((_, i) => {
      const start = Math.max(0, i - window + 1);
      const slice = values.slice(start, i + 1);
      return slice.reduce((a, b) => a + b, 0) / slice.length;
    });
  }

  let avgLine = $derived(() => {
    const avgs = movingAvg(data.map(d => d.totalSecs), 7);
    return avgs.map((v, i) => `${i === 0 ? "M" : "L"}${xPos(i)},${yPos(v)}`).join(" ");
  });
</script>

<div class="trend-chart">
  <div class="period-selector">
    {#each [7, 30, 90, 365] as p}
      <button
        class="period-btn"
        class:active={period === p}
        onclick={() => { period = p; load(); }}
      >
        {p === 365 ? "1y" : `${p}d`}
      </button>
    {/each}
  </div>

  {#if loaded && data.length > 0}
    <svg width={CHART_W} height={CHART_H} viewBox="0 0 {CHART_W} {CHART_H}">
      <!-- Grid lines -->
      {#each [0, 0.25, 0.5, 0.75, 1] as frac}
        <line
          x1={PAD} x2={CHART_W - PAD}
          y1={CHART_H - PAD - frac * (CHART_H - PAD * 2)}
          y2={CHART_H - PAD - frac * (CHART_H - PAD * 2)}
          stroke="var(--border)" stroke-width="0.5"
        />
        <text
          x={PAD - 6}
          y={CHART_H - PAD - frac * (CHART_H - PAD * 2) + 3}
          font-size="9" fill="var(--text-tertiary)" text-anchor="end"
          font-family="var(--font-mono)"
        >
          {(maxHours * frac).toFixed(0)}h
        </text>
      {/each}

      <!-- AI line -->
      <path d={aiLine} fill="none" stroke="var(--primary)" stroke-width="1.5" opacity="0.4" />
      <!-- Total line -->
      <path d={totalLine} fill="none" stroke="var(--primary)" stroke-width="2" />
      <!-- Moving average -->
      <path d={avgLine()} fill="none" stroke="var(--text-tertiary)" stroke-width="1" stroke-dasharray="4 2" />
    </svg>

    <div class="legend">
      <span class="legend-item"><span class="legend-swatch total"></span> Total</span>
      <span class="legend-item"><span class="legend-swatch ai"></span> AI-assisted</span>
      <span class="legend-item"><span class="legend-swatch avg"></span> 7-day avg</span>
    </div>
  {:else if loaded}
    <p class="empty">No data yet. Start tracking to see trends.</p>
  {:else}
    <p class="empty">Loading...</p>
  {/if}
</div>

<style>
  .trend-chart { position: relative; }
  .period-selector {
    display: flex; gap: 4px; margin-bottom: 12px;
  }
  .period-btn {
    padding: 4px 10px; font-size: 11px; font-weight: 600;
    border: 1px solid var(--border); border-radius: var(--radius-sm);
    background: var(--surface); color: var(--text-tertiary); cursor: pointer;
    font-family: var(--font-mono); transition: all 0.15s;
  }
  .period-btn.active {
    background: var(--primary); color: white; border-color: var(--primary);
  }
  .legend {
    display: flex; gap: 16px; margin-top: 8px;
  }
  .legend-item {
    font-size: 10px; color: var(--text-tertiary);
    display: flex; align-items: center; gap: 4px;
  }
  .legend-swatch {
    width: 12px; height: 2px; border-radius: 1px;
  }
  .legend-swatch.total { background: var(--primary); }
  .legend-swatch.ai { background: var(--primary); opacity: 0.4; }
  .legend-swatch.avg { background: var(--text-tertiary); border-top: 1px dashed var(--text-tertiary); }
  .empty {
    font-size: 13px; color: var(--text-tertiary); padding: 24px; text-align: center;
  }
</style>
