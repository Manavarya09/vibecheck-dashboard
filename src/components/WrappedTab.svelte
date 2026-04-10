<script lang="ts">
  import { onMount } from "svelte";
  import { getHistoricalStats, getSpendingRates } from "../lib/api";
  import type { DailySummary } from "../lib/types";
  import type { SpendingRate } from "../lib/api";
  import { formatDuration } from "../lib/utils";

  let data = $state<DailySummary[]>([]);
  let rates = $state<SpendingRate[]>([]);
  let loaded = $state(false);

  onMount(async () => {
    const [d, r] = await Promise.all([getHistoricalStats(30), getSpendingRates()]);
    data = d;
    rates = r;
    loaded = true;
  });

  let totalSecs = $derived(data.reduce((s, d) => s + d.totalSecs, 0));
  let aiSecs = $derived(data.reduce((s, d) => s + d.aiAssistedSecs, 0));

  let sessionCount = $derived(data.reduce((s, d) => s + d.sessionCount, 0));
  let activeDays = $derived(data.filter(d => d.totalSecs > 0).length);
  let longestDay = $derived(
    data.length > 0
      ? data.reduce((max, d) => d.totalSecs > max.totalSecs ? d : max, data[0])
      : null
  );
  let aiPct = $derived(totalSecs > 0 ? Math.round((aiSecs / totalSecs) * 100) : 0);
  let avgDaily = $derived(activeDays > 0 ? totalSecs / activeDays : 0);
  let monthlySpend = $derived(
    rates.reduce((sum, r) => sum + (r.billingPeriod === "yearly" ? r.rateValue / 12 : r.rateValue), 0)
  );
</script>

<div class="wrapped">
  {#if !loaded}
    <p class="loading">Generating your Wrapped...</p>
  {:else}
    <div class="wrapped-card" id="wrapped-card">
      <div class="wrapped-header">
        <p class="wrapped-eyebrow">Your Monthly</p>
        <h2 class="wrapped-title">VibeCheck Wrapped</h2>
        <p class="wrapped-period">Last 30 days</p>
      </div>

      <div class="stat-grid">
        <div class="stat">
          <span class="stat-value">{formatDuration(totalSecs)}</span>
          <span class="stat-label">Total coding</span>
        </div>
        <div class="stat">
          <span class="stat-value">{aiPct}%</span>
          <span class="stat-label">AI-assisted</span>
        </div>
        <div class="stat">
          <span class="stat-value">{sessionCount}</span>
          <span class="stat-label">Sessions</span>
        </div>
        <div class="stat">
          <span class="stat-value">{activeDays}</span>
          <span class="stat-label">Active days</span>
        </div>
        <div class="stat">
          <span class="stat-value">{formatDuration(Math.round(avgDaily))}</span>
          <span class="stat-label">Avg per day</span>
        </div>
        <div class="stat">
          <span class="stat-value">${monthlySpend.toFixed(0)}</span>
          <span class="stat-label">Monthly spend</span>
        </div>
      </div>

      {#if longestDay}
        <div class="highlight">
          <span class="highlight-label">Longest day</span>
          <span class="highlight-value">{longestDay.date} -- {formatDuration(longestDay.totalSecs)}</span>
        </div>
      {/if}

      <div class="wrapped-footer">
        <p class="footer-text">VibeCheck -- Developer Wellness</p>
      </div>
    </div>
  {/if}
</div>

<style>
  .wrapped { max-width: 420px; margin: 0 auto; }
  .wrapped-card {
    background: linear-gradient(135deg, var(--text, #1a1a2e) 0%, #2d2d4a 100%);
    border-radius: 16px; padding: 32px; color: white;
  }
  .wrapped-header { text-align: center; margin-bottom: 24px; }
  .wrapped-eyebrow {
    font-size: 10px; text-transform: uppercase; letter-spacing: 0.15em;
    color: rgba(255,255,255,0.5); font-weight: 600;
  }
  .wrapped-title { font-size: 24px; font-weight: 700; margin: 4px 0; }
  .wrapped-period { font-size: 12px; color: rgba(255,255,255,0.4); font-family: var(--font-mono); }
  .stat-grid {
    display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 20px;
    margin-bottom: 20px;
  }
  .stat { text-align: center; }
  .stat-value {
    display: block; font-size: 20px; font-weight: 700;
    font-family: var(--font-mono); color: var(--primary, #d97757);
  }
  .stat-label {
    display: block; font-size: 10px; color: rgba(255,255,255,0.5);
    text-transform: uppercase; letter-spacing: 0.06em; font-weight: 600; margin-top: 2px;
  }
  .highlight {
    text-align: center; padding: 12px; border-top: 1px solid rgba(255,255,255,0.1);
    margin-top: 8px;
  }
  .highlight-label {
    display: block; font-size: 10px; color: rgba(255,255,255,0.4);
    text-transform: uppercase; letter-spacing: 0.06em; margin-bottom: 4px;
  }
  .highlight-value { font-family: var(--font-mono); font-size: 13px; color: rgba(255,255,255,0.8); }
  .wrapped-footer {
    text-align: center; margin-top: 20px; padding-top: 12px;
    border-top: 1px solid rgba(255,255,255,0.1);
  }
  .footer-text { font-size: 10px; color: rgba(255,255,255,0.3); letter-spacing: 0.1em; text-transform: uppercase; }
  .loading { font-size: 13px; color: var(--text-tertiary); text-align: center; padding: 48px; }
</style>
