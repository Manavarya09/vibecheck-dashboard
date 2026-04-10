<script lang="ts">
  import { onMount } from "svelte";
  import { getHistoricalStats, getSpendingRates } from "../lib/api";
  import type { DailySummary } from "../lib/types";
  import type { SpendingRate } from "../lib/api";

  let data = $state<DailySummary[]>([]);
  let rates = $state<SpendingRate[]>([]);
  let loaded = $state(false);

  onMount(async () => {
    const [d, r] = await Promise.all([getHistoricalStats(30), getSpendingRates()]);
    data = d;
    rates = r;
    loaded = true;
  });

  let monthlyRate = $derived(
    rates.reduce((sum, r) => {
      if (r.billingPeriod === "yearly") return sum + r.rateValue / 12;
      return sum + r.rateValue;
    }, 0)
  );

  let dailyRate = $derived(monthlyRate / 30);

  let dailySpend = $derived(
    data.map(d => ({
      date: d.date,
      spend: (d.aiAssistedSecs / 3600) * (dailyRate > 0 ? dailyRate / (d.totalSecs / 3600 || 1) : 0),
      hours: d.totalSecs / 3600,
    }))
  );

  let maxSpend = $derived(Math.max(...dailySpend.map(d => d.spend), 1));
  let totalSpend = $derived(dailySpend.reduce((s, d) => s + d.spend, 0));

  const BAR_W = 700;
  const BAR_H = 160;
  const PAD = 30;
</script>

<div class="spending-chart">
  {#if loaded && data.length > 0}
    <div class="summary-row">
      <div class="summary-item">
        <span class="summary-value">${totalSpend.toFixed(2)}</span>
        <span class="summary-label">30-day spend</span>
      </div>
      <div class="summary-item">
        <span class="summary-value">${monthlyRate.toFixed(2)}</span>
        <span class="summary-label">Monthly rate</span>
      </div>
    </div>

    <svg width={BAR_W} height={BAR_H} viewBox="0 0 {BAR_W} {BAR_H}">
      {#each dailySpend as day, i}
        {@const barW = Math.max((BAR_W - PAD * 2) / dailySpend.length - 1, 2)}
        {@const barH = (day.spend / maxSpend) * (BAR_H - PAD * 2)}
        <rect
          x={PAD + i * ((BAR_W - PAD * 2) / dailySpend.length)}
          y={BAR_H - PAD - barH}
          width={barW}
          height={barH}
          fill="var(--primary)"
          rx="1"
          opacity="0.7"
        >
          <title>${day.spend.toFixed(2)} on {day.date}</title>
        </rect>
      {/each}
    </svg>
  {:else if loaded}
    <p class="empty">No spending data yet.</p>
  {:else}
    <p class="empty">Loading...</p>
  {/if}
</div>

<style>
  .spending-chart { position: relative; }
  .summary-row { display: flex; gap: 32px; margin-bottom: 16px; }
  .summary-item { display: flex; flex-direction: column; }
  .summary-value { font-size: 24px; font-weight: 700; font-family: var(--font-mono); color: var(--primary); }
  .summary-label { font-size: 11px; color: var(--text-tertiary); text-transform: uppercase; letter-spacing: 0.06em; font-weight: 600; }
  .empty { font-size: 13px; color: var(--text-tertiary); text-align: center; padding: 24px; }
</style>
