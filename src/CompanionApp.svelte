<script lang="ts">
  import "./app.css";
  import PandaCompanion from "./components/PandaCompanion.svelte";
  import { getCurrentSession, getRecentSessions, getTodaySummary } from "./lib/api";
  import { currentSession, recentSessions, todaySummary } from "./lib/stores";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import type { Session, SessionUpdate } from "./lib/types";
  import { autoPaused, liveUpdate } from "./lib/stores";

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
      console.error("Failed to load companion state:", e);
    }

    listen("session-auto-paused", () => {
      autoPaused.set(true);
      currentSession.update((s) => (s ? { ...s, status: "paused" } : s));
    });

    listen("session-auto-resumed", () => {
      autoPaused.set(false);
      currentSession.update((s) => (s ? { ...s, status: "active" } : s));
    });

    listen<Session>("session-started", (event) => {
      currentSession.set(event.payload);
      autoPaused.set(false);
    });

    listen<number>("session-stopped", async () => {
      currentSession.set(null);
      liveUpdate.set(null);
      autoPaused.set(false);
      todaySummary.set(await getTodaySummary());
    });

    listen<SessionUpdate>("session-update", (event) => {
      liveUpdate.set(event.payload);
    });
  });
</script>

<main class="companion-root" data-tauri-drag-region>
  <PandaCompanion />
</main>

<style>
  .companion-root {
    width: 100vw;
    height: 100vh;
    background: transparent;
    overflow: hidden;
  }
</style>
