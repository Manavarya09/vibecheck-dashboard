<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { get } from "svelte/store";
  import "./app.css";

  import {
    currentSession,
    todaySummary,
    recentSessions,
    liveUpdate,
    autoPaused,
  } from "./lib/stores";
  import {
    getCurrentSession,
    getTodaySummary,
    getRecentSessions,
    pauseSession,
    resumeSession,
  } from "./lib/api";
  import type { Session, SessionUpdate } from "./lib/types";

  import Sidebar from "./components/Sidebar.svelte";
  import TabBar from "./components/TabBar.svelte";
  import CurrentSession from "./components/CurrentSession.svelte";
  import TodaySummary from "./components/TodaySummary.svelte";
  import ActivityBreakdown from "./components/ActivityBreakdown.svelte";
  import RecentSessions from "./components/RecentSessions.svelte";
  import ActivityFeed from "./components/ActivityFeed.svelte";
  import AnalyticsTab from "./components/AnalyticsTab.svelte";
  import SpendingTab from "./components/SpendingTab.svelte";
  import WrappedTab from "./components/WrappedTab.svelte";
  import Settings from "./components/Settings.svelte";
  import BreakOverlay from "./components/BreakOverlay.svelte";
  import PermissionPrompt from "./components/PermissionPrompt.svelte";
  import ResearchInsights from "./components/ResearchInsights.svelte";
  import SessionJournal from "./components/SessionJournal.svelte";

  let view = $state<"dashboard" | "settings">("dashboard");
  let activeTab = $state("overview");

  const TABS = [
    { id: "overview", label: "Overview" },
    { id: "analytics", label: "Analytics" },
    { id: "spending", label: "Spending" },
    { id: "wrapped", label: "Wrapped" },
  ];

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

    listen("session-auto-paused", () => {
      autoPaused.set(true);
      currentSession.update((s) => s ? { ...s, status: "paused" } : s);
    });

    listen("session-auto-resumed", () => {
      autoPaused.set(false);
      currentSession.update((s) => s ? { ...s, status: "active" } : s);
    });

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

    listen<Session>("session-started", (event) => {
      currentSession.set(event.payload);
      autoPaused.set(false);
    });

    listen<number>("session-stopped", async () => {
      currentSession.set(null);
      liveUpdate.set(null);
      autoPaused.set(false);
      const [summary, recent] = await Promise.all([
        getTodaySummary(),
        getRecentSessions(),
      ]);
      todaySummary.set(summary);
      recentSessions.set(recent);
    });

    listen("tray-pause-toggle", async () => {
      const session = get(currentSession);
      if (!session) return;
      if (session.status === "active") {
        await pauseSession();
        currentSession.set({ ...session, status: "paused" });
        autoPaused.set(false);
      } else if (session.status === "paused") {
        await resumeSession();
        currentSession.set({ ...session, status: "active" });
        autoPaused.set(false);
      }
    });
  });
</script>

<PermissionPrompt />
<BreakOverlay />
<Sidebar onNavigate={(v) => view = v} currentView={view} />
<main class="content">
  {#if view === "settings"}
    <Settings />
  {:else}
    <section class="workspace-intro">
      <div>
        <p class="workspace-kicker">Desktop Research Build</p>
        <h1>Panko is tracking your vibe coding before it becomes a full AI-induced flow loop.</h1>
      </div>
      <p class="workspace-copy">
        Built for Claude Code, VS Code, terminals, JetBrains, and browser copilots.
        The goal is not just monitoring time. It is detecting when convenience strips away
        your natural stopping points.
      </p>
    </section>

    <TabBar tabs={TABS} active={activeTab} onSelect={(id) => activeTab = id} />

    {#if activeTab === "overview"}
      <div class="overview">
        <ResearchInsights />

        <div class="grid">
          <div class="col-main">
            <CurrentSession />
            <ActivityBreakdown />
            <SessionJournal />
            <ActivityFeed />
          </div>
          <div class="col-side">
            <TodaySummary />
            <RecentSessions />
          </div>
        </div>
      </div>
    {:else if activeTab === "analytics"}
      <AnalyticsTab />
    {:else if activeTab === "spending"}
      <SpendingTab />
    {:else if activeTab === "wrapped"}
      <WrappedTab />
    {/if}
  {/if}
</main>

<style>
  .content {
    flex: 1;
    padding: 28px 28px 32px;
    overflow-y: auto;
  }
  .workspace-intro {
    display: grid;
    grid-template-columns: minmax(0, 1.3fr) minmax(260px, 0.8fr);
    gap: 20px;
    align-items: end;
    margin: 0 auto 22px;
    max-width: 1120px;
  }
  .workspace-kicker {
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.18em;
    text-transform: uppercase;
    color: var(--primary);
    margin-bottom: 10px;
  }
  h1 {
    font-size: clamp(32px, 4vw, 54px);
    line-height: 0.96;
    letter-spacing: -0.05em;
    color: var(--ink);
    max-width: 12ch;
  }
  .workspace-copy {
    color: var(--text-secondary);
    font-size: 14px;
    line-height: 1.6;
    max-width: 36ch;
    padding-bottom: 4px;
  }
  .overview {
    max-width: 1120px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 18px;
  }
  .grid {
    display: grid;
    grid-template-columns: minmax(0, 1.35fr) minmax(280px, 0.82fr);
    gap: 18px;
  }
  .col-main,
  .col-side {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  @media (max-width: 1040px) {
    .workspace-intro,
    .grid {
      grid-template-columns: 1fr;
    }
  }
</style>
