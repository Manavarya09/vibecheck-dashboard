<script lang="ts">
  import { recentSessions } from "../lib/stores";
  import { getSessionStats } from "../lib/api";
  import type { SessionStats } from "../lib/types";
  import { formatDuration, pct } from "../lib/utils";

  let sessionA = $state<number | null>(null);
  let sessionB = $state<number | null>(null);
  let statsA = $state<SessionStats | null>(null);
  let statsB = $state<SessionStats | null>(null);

  async function loadStats(id: number, side: "a" | "b") {
    const stats = await getSessionStats(id);
    if (side === "a") statsA = stats;
    else statsB = stats;
  }

  function selectA(e: Event) {
    const val = parseInt((e.target as HTMLSelectElement).value);
    if (!isNaN(val)) { sessionA = val; loadStats(val, "a"); }
  }

  function selectB(e: Event) {
    const val = parseInt((e.target as HTMLSelectElement).value);
    if (!isNaN(val)) { sessionB = val; loadStats(val, "b"); }
  }

  let sessions = $derived($recentSessions);

  interface CompRow {
    label: string;
    a: string;
    b: string;
  }

  let rows = $derived<CompRow[]>(
    statsA && statsB ? [
      { label: "Duration", a: formatDuration(statsA.totalDurationSecs), b: formatDuration(statsB.totalDurationSecs) },
      { label: "AI Ratio", a: `${pct(statsA.aiAssistedSecs, statsA.totalDurationSecs)}%`, b: `${pct(statsB.aiAssistedSecs, statsB.totalDurationSecs)}%` },
      { label: "AI Time", a: formatDuration(statsA.aiAssistedSecs), b: formatDuration(statsB.aiAssistedSecs) },
      { label: "Manual", a: formatDuration(statsA.manualCodingSecs), b: formatDuration(statsB.manualCodingSecs) },
      { label: "Non-coding", a: formatDuration(statsA.nonCodingSecs), b: formatDuration(statsB.nonCodingSecs) },
    ] : []
  );
</script>

<div class="comparison">
  <div class="selectors">
    <select class="session-select" onchange={selectA}>
      <option value="">Select session A</option>
      {#each sessions as s}
        <option value={s.id}>#{s.id} -- {s.startedAt.split("T")[0]} ({formatDuration(s.durationSecs)})</option>
      {/each}
    </select>
    <span class="vs">vs</span>
    <select class="session-select" onchange={selectB}>
      <option value="">Select session B</option>
      {#each sessions as s}
        <option value={s.id}>#{s.id} -- {s.startedAt.split("T")[0]} ({formatDuration(s.durationSecs)})</option>
      {/each}
    </select>
  </div>

  {#if rows.length > 0}
    <table class="comp-table">
      <thead>
        <tr><th>Metric</th><th>Session A</th><th>Session B</th></tr>
      </thead>
      <tbody>
        {#each rows as row}
          <tr>
            <td class="metric">{row.label}</td>
            <td class="val">{row.a}</td>
            <td class="val">{row.b}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {:else if sessionA && sessionB}
    <p class="loading">Loading comparison...</p>
  {:else}
    <p class="hint">Select two sessions to compare</p>
  {/if}
</div>

<style>
  .comparison { max-width: 560px; }
  .selectors { display: flex; align-items: center; gap: 12px; margin-bottom: 16px; }
  .session-select {
    flex: 1; padding: 8px 10px; border: 1px solid var(--border); border-radius: var(--radius-sm);
    font-size: 12px; color: var(--text); background: var(--surface); font-family: var(--font-mono);
  }
  .vs { font-size: 12px; font-weight: 700; color: var(--text-tertiary); }
  .comp-table { width: 100%; border-collapse: collapse; }
  .comp-table th {
    font-size: 11px; font-weight: 600; color: var(--text-tertiary);
    text-transform: uppercase; letter-spacing: 0.06em; padding: 8px 0;
    border-bottom: 1px solid var(--border); text-align: left;
  }
  .comp-table td { padding: 10px 0; border-bottom: 1px solid var(--border); }
  .metric { font-size: 13px; font-weight: 600; color: var(--text); }
  .val { font-family: var(--font-mono); font-size: 13px; color: var(--primary); }
  .hint, .loading { font-size: 13px; color: var(--text-tertiary); text-align: center; padding: 24px; }
</style>
