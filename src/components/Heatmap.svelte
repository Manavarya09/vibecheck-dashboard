<script lang="ts">
  import { onMount } from "svelte";
  import { getHeatmapData } from "../lib/api";
  import type { DailySummary } from "../lib/types";

  let data = $state<DailySummary[]>([]);
  let loaded = $state(false);
  let hoveredDay = $state<{ date: string; hours: number; x: number; y: number } | null>(null);

  const CELL_SIZE = 11;
  const CELL_GAP = 2;
  const WEEKS = 52;
  const DAYS = 7;
  const LABEL_WIDTH = 28;
  const WIDTH = LABEL_WIDTH + WEEKS * (CELL_SIZE + CELL_GAP);
  const HEIGHT = DAYS * (CELL_SIZE + CELL_GAP) + 20;

  const DAY_LABELS = ["", "Mon", "", "Wed", "", "Fri", ""];

  onMount(async () => {
    data = await getHeatmapData(365);
    loaded = true;
  });

  function getDateKey(weeksAgo: number, dayOfWeek: number): string {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const currentDay = today.getDay();
    const startOfGrid = new Date(today);
    startOfGrid.setDate(today.getDate() - (WEEKS * 7) - currentDay + dayOfWeek - (7 - 1 - weeksAgo) * 0 );

    // Calculate: go back (WEEKS - 1 - weeksAgo) weeks from this week's start, then add dayOfWeek
    const thisWeekStart = new Date(today);
    thisWeekStart.setDate(today.getDate() - currentDay);
    const targetDate = new Date(thisWeekStart);
    targetDate.setDate(thisWeekStart.getDate() - (WEEKS - 1 - weeksAgo) * 7 + dayOfWeek);

    return targetDate.toISOString().split("T")[0];
  }

  function getHours(dateKey: string): number {
    const entry = data.find((d) => d.date === dateKey);
    return entry ? entry.totalSecs / 3600 : 0;
  }

  function getColor(hours: number): string {
    if (hours === 0) return "var(--surface, #f0ebe3)";
    if (hours < 1) return "#f0c4a8";
    if (hours < 2) return "#e8a08a";
    if (hours < 4) return "#d97757";
    return "#c4554d";
  }

  function handleHover(week: number, day: number, e: MouseEvent) {
    const dateKey = getDateKey(week, day);
    const hours = getHours(dateKey);
    hoveredDay = { date: dateKey, hours, x: e.clientX, y: e.clientY };
  }

  function clearHover() {
    hoveredDay = null;
  }
</script>

<div class="heatmap-container">
  {#if !loaded}
    <p class="loading">Loading heatmap...</p>
  {:else}
    <svg
      width={WIDTH}
      height={HEIGHT}
      viewBox="0 0 {WIDTH} {HEIGHT}"
      role="img"
      aria-label="Session activity heatmap"
    >
      <!-- Day labels -->
      {#each DAY_LABELS as label, i}
        {#if label}
          <text
            x="0"
            y={i * (CELL_SIZE + CELL_GAP) + CELL_SIZE - 1}
            class="day-label"
            font-size="9"
            fill="var(--text-tertiary)"
          >
            {label}
          </text>
        {/if}
      {/each}

      <!-- Grid cells -->
      {#each Array(WEEKS) as _, week}
        {#each Array(DAYS) as _, day}
          {@const dateKey = getDateKey(week, day)}
          {@const hours = getHours(dateKey)}
          <rect
            x={LABEL_WIDTH + week * (CELL_SIZE + CELL_GAP)}
            y={day * (CELL_SIZE + CELL_GAP)}
            width={CELL_SIZE}
            height={CELL_SIZE}
            rx="2"
            fill={getColor(hours)}
            role="gridcell"
            aria-label="{hours.toFixed(1)} hours on {dateKey}"
            onmouseenter={(e) => handleHover(week, day, e)}
            onmouseleave={clearHover}
          />
        {/each}
      {/each}
    </svg>

    <!-- Legend -->
    <div class="legend">
      <span class="legend-label">Less</span>
      {#each [0, 0.5, 1.5, 3, 5] as h}
        <div class="legend-cell" style="background: {getColor(h)}"></div>
      {/each}
      <span class="legend-label">More</span>
    </div>

    <!-- Tooltip -->
    {#if hoveredDay}
      <div
        class="tooltip"
        style="left: {hoveredDay.x + 12}px; top: {hoveredDay.y - 30}px"
      >
        <strong>{hoveredDay.hours.toFixed(1)}h</strong> on {hoveredDay.date}
      </div>
    {/if}
  {/if}
</div>

<style>
  .heatmap-container {
    position: relative;
    overflow-x: auto;
  }
  .day-label {
    font-family: var(--font-mono);
  }
  .legend {
    display: flex;
    align-items: center;
    gap: 4px;
    margin-top: 8px;
    justify-content: flex-end;
  }
  .legend-label {
    font-size: 10px;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
  }
  .legend-cell {
    width: 11px;
    height: 11px;
    border-radius: 2px;
  }
  .tooltip {
    position: fixed;
    background: var(--text, #1a1a2e);
    color: white;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 11px;
    font-family: var(--font-mono);
    pointer-events: none;
    z-index: 100;
    white-space: nowrap;
  }
  .loading {
    color: var(--text-tertiary);
    font-size: 13px;
    padding: 24px;
  }
  rect {
    cursor: pointer;
    transition: opacity 0.1s;
  }
  rect:hover {
    opacity: 0.8;
  }
</style>
