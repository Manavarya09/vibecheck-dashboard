<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import "./app.css";

  import {
    currentSession,
    todaySummary,
    recentSessions,
    liveUpdate,
  } from "./lib/stores";
  import {
    getCurrentSession,
    getTodaySummary,
    getRecentSessions,
  } from "./lib/api";
  import type { SessionUpdate } from "./lib/types";

  import Sidebar from "./components/Sidebar.svelte";
  import CurrentSession from "./components/CurrentSession.svelte";
  import TodaySummary from "./components/TodaySummary.svelte";
  import ActivityBreakdown from "./components/ActivityBreakdown.svelte";
  import RecentSessions from "./components/RecentSessions.svelte";
  import ActivityFeed from "./components/ActivityFeed.svelte";
  import Settings from "./components/Settings.svelte";

  let view = $state<"dashboard" | "settings">("dashboard");

  onMount(async () => {
    try {
      const [session, summary, recent] = await Promise.all([
        getCurrentSession(),
        getTodaySummary(),
        getRecentSessions(),
      ]);
      currentSession.set(session);
      todaySummary.set(summary);
      recentSessions.set(recent);
    } catch (e) {
      console.error("Failed to load initial data:", e);
    }

    listen<SessionUpdate>(
      "session-update",
      (event) => {
        liveUpdate.set(event.payload);

        todaySummary.update((s) => {
          if (!s) return s;
          return {
            ...s,
            totalSecs:
              event.payload.aiAssistedSecs +
              event.payload.manualCodingSecs +
              event.payload.nonCodingSecs,
            aiAssistedSecs: event.payload.aiAssistedSecs,
            manualCodingSecs: event.payload.manualCodingSecs,
            nonCodingSecs: event.payload.nonCodingSecs,
          };
        });
      }
    );
  });
</script>

<Sidebar onNavigate={(v) => view = v} currentView={view} />
<main class="content">
  {#if view === "settings"}
    <Settings />
  {:else}
    <div class="grid">
      <div class="col-main">
        <CurrentSession />
        <ActivityBreakdown />
        <ActivityFeed />
      </div>
      <div class="col-side">
        <TodaySummary />
        <RecentSessions />
      </div>
    </div>
  {/if}
</main>

<style>
  .content {
    flex: 1;
    padding: 24px;
    overflow-y: auto;
  }
  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    max-width: 840px;
    margin: 0 auto;
  }
  .col-main,
  .col-side {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
</style>
