<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import {
    autoPaused,
    currentSession,
    liveUpdate,
    todaySummary,
  } from "../lib/stores";
  import { formatDuration, pct } from "../lib/utils";

  type VoiceMode = "roast" | "coach" | "gentle";
  type PandaMood = "idle" | "focused" | "warning" | "feral" | "paused";

  interface PandaPrompt {
    roast: string[];
    coach: string[];
    gentle: string[];
  }

  const SCRIPT: Record<PandaMood, PandaPrompt> = {
    idle: {
      roast: [
        "No chaos detected. I, Panko, will allow your spine a temporary victory.",
        "Quiet little desktop. Suspicious. Are we coding or just emotionally hovering?",
      ],
      coach: [
        "Panko is on watch. Start a session and I'll track your flow loops.",
        "No active session yet. That means your wrists still have hope.",
      ],
      gentle: [
        "Panko is resting nearby. Start a session when you're ready.",
        "No session running. Good. Breathe before the tabs multiply.",
      ],
    },
    focused: {
      roast: [
        "You're cooking. Keep the momentum, but no gremlin tab-hopping.",
        "Nice rhythm. This is flow, not a full-blown Claude-fueled episode. Yet.",
      ],
      coach: [
        "Steady pace. Keep decisions intentional while the AI handles the boring bits.",
        "Good cadence. Stay sharp and don't let convenience replace judgment.",
      ],
      gentle: [
        "You look settled in. Keep the session deliberate and take a break before fatigue sneaks in.",
        "Calm progress. Maintain the rhythm while your focus is still clean.",
      ],
    },
    warning: {
      roast: [
        "You've been vibe coding for {duration}. Touch grass is loading in the next patch.",
        "Friendly panda note: {duration} deep with {aiPct}% AI time is how people forget lunch exists.",
      ],
      coach: [
        "You've been in-session for {duration}. Consider a reset before the loop hardens.",
        "Sustained AI-assisted work detected. A short walk now will cost less than brain fog later.",
      ],
      gentle: [
        "You've been coding for {duration}. A small break now would help preserve quality.",
        "The session is getting long. Try water, posture reset, and a quick visual break.",
      ],
    },
    feral: {
      roast: [
        "Brother this is not productivity anymore. This is a woodland creature piloting Claude for {duration}.",
        "Emergency panda bulletin: {duration} and {aiPct}% AI time. Do 10 pushups before your soul compiles to mush.",
      ],
      coach: [
        "High AIFL risk. The session has stretched to {duration} with heavy AI reliance. Break the loop now.",
        "You are past sustainable focus. Pause, stand up, and decide the next step off-screen.",
      ],
      gentle: [
        "This has become a very long session. Please stop for a few minutes before continuing.",
        "You may be in a time-distortion loop. Step away, reset your body, then choose whether to return.",
      ],
    },
    paused: {
      roast: [
        "Auto-pause engaged. Even the app noticed your body left the building.",
        "You drifted off. I'm calling that a tactical retreat, not discipline.",
      ],
      coach: [
        "Session paused for inactivity. Resume only if you know what you're returning to do.",
        "Auto-pause caught a break in focus. Good moment to reset intentionally.",
      ],
      gentle: [
        "The session is paused. Take the break properly before jumping back in.",
        "Inactivity detected. Let the pause do its job.",
      ],
    },
  };

  const RECOVERY_ACTIONS: Record<PandaMood, string[]> = {
    idle: ["Warm up with one clear goal", "Keep tabs under control", "Save your shoulders for later"],
    focused: ["Hydrate in 5 minutes", "Stay with one task", "Stretch wrists soon"],
    warning: ["Stand up now", "Water break", "Do 10 pushups"],
    feral: ["Touch grass immediately", "Phone away, body move", "Hard stop for 10 minutes"],
    paused: ["Walk a lap", "Reset posture", "Return with one objective"],
  };

  let voiceMode = $state<VoiceMode>("roast");
  let promptIndex = $state(0);
  let tick: ReturnType<typeof setInterval> | null = null;

  let update = $derived($liveUpdate);
  let totalSecs = $derived(update?.durationSecs ?? $currentSession?.durationSecs ?? 0);
  let aiSecs = $derived(update?.aiAssistedSecs ?? 0);
  let manualSecs = $derived(update?.manualCodingSecs ?? 0);
  let otherSecs = $derived(update?.nonCodingSecs ?? 0);
  let sessionTotal = $derived(aiSecs + manualSecs + otherSecs);
  let aiPct = $derived(pct(aiSecs, sessionTotal));
  let todayTotal = $derived($todaySummary?.totalSecs ?? 0);
  let currentApp = $derived(update?.currentApp ?? "");
  let status = $derived($currentSession?.status ?? "idle");
  let workflowState = $derived(update?.workflowState ?? "settled");
  let contextSwitches = $derived(update?.contextSwitches10m ?? 0);
  let aiStreakSecs = $derived(update?.aiStreakSecs ?? 0);
  let promptLoopScore = $derived(update?.promptLoopScore ?? 0);

  let mood = $derived.by(() => {
    if (status !== "active") {
      return $autoPaused || status === "paused" ? "paused" : "idle";
    }
    if (
      totalSecs >= 7200 ||
      (totalSecs >= 5400 && aiPct >= 75) ||
      workflowState === "context_switch_storm" ||
      promptLoopScore >= 70
    ) {
      return "feral";
    }
    if (totalSecs >= 3600 || aiPct >= 70 || promptLoopScore >= 45) {
      return "warning";
    }
    return "focused";
  });

  let activePromptSet = $derived(SCRIPT[mood][voiceMode]);
  let rawPrompt = $derived(
    activePromptSet.length > 0
      ? activePromptSet[promptIndex % activePromptSet.length]
      : ""
  );
  let prompt = $derived(
    rawPrompt
      .replaceAll("{duration}", formatDuration(totalSecs))
      .replaceAll("{aiPct}", `${aiPct}%`)
  );

  let moodLabel = $derived(
    {
      idle: "Perched",
      focused: "Locked In",
      warning: "Loop Watch",
      feral: "Intervention",
      paused: "Break Mode",
    }[mood]
  );

  let moodAccent = $derived(
    {
      idle: "var(--ink-soft)",
      focused: "var(--success)",
      warning: "var(--warning)",
      feral: "var(--danger)",
      paused: "var(--primary)",
    }[mood]
  );

  let actionItems = $derived(RECOVERY_ACTIONS[mood]);
  let aiflScore = $derived(
    Math.min(100, Math.round(promptLoopScore * 0.55 + aiPct * 0.35 + totalSecs / 180 + todayTotal / 900))
  );

  $effect(() => {
    const key = `${mood}:${voiceMode}:${currentApp}`;
    key;
    promptIndex = 0;
  });

  onMount(() => {
    tick = setInterval(() => {
      promptIndex += 1;
    }, 18000);
  });

  onDestroy(() => {
    if (tick) clearInterval(tick);
  });
</script>

<aside class="panda-shell" aria-label="Panko the panda coding coach">
  <div class="speech">
    <div class="speech-meta">
      <span class="name">Panko</span>
      <span class="mood" style="color: {moodAccent}">{moodLabel}</span>
    </div>
    <p class="speech-copy">{prompt}</p>
    <div class="voice-row" role="tablist" aria-label="Panko voice mode">
      {#each [
        { id: "roast", label: "Roast" },
        { id: "coach", label: "Coach" },
        { id: "gentle", label: "Gentle" },
      ] as modeOption}
        <button
          class="voice-chip"
          class:active={voiceMode === modeOption.id}
          onclick={() => (voiceMode = modeOption.id as VoiceMode)}
        >
          {modeOption.label}
        </button>
      {/each}
    </div>
    <div class="signal-row">
      <div class="signal">
        <span class="signal-label">Loop Score</span>
        <strong>{aiflScore}</strong>
      </div>
      <div class="signal">
        <span class="signal-label">AI Share</span>
        <strong>{aiPct}%</strong>
      </div>
      <div class="signal">
        <span class="signal-label">Current App</span>
        <strong>{currentApp || "No active window"}</strong>
      </div>
    </div>
    <div class="workflow-row">
      <span>State: <strong>{workflowState.replaceAll("_", " ")}</strong></span>
      <span>Switches: <strong>{contextSwitches}</strong></span>
      <span>AI streak: <strong>{formatDuration(aiStreakSecs)}</strong></span>
    </div>
    <div class="action-row">
      {#each actionItems as item}
        <span class="action-pill">{item}</span>
      {/each}
    </div>
  </div>

  <div class="panda-dock" data-mood={mood}>
    <div class="glow"></div>
    <div class="panda">
      <div class="ear left"></div>
      <div class="ear right"></div>
      <div class="head">
        <div class="eye-patch left"></div>
        <div class="eye-patch right"></div>
        <div class="eye left"></div>
        <div class="eye right"></div>
        <div class="nose"></div>
        <div class="mouth"></div>
        <div class="blush left"></div>
        <div class="blush right"></div>
      </div>
      <div class="body"></div>
      <div class="paw left"></div>
      <div class="paw right"></div>
    </div>
  </div>
</aside>

<style>
  .panda-shell {
    position: fixed;
    right: 22px;
    bottom: 18px;
    z-index: 70;
    display: grid;
    grid-template-columns: minmax(260px, 320px) 132px;
    align-items: end;
    gap: 14px;
    pointer-events: none;
  }
  .speech,
  .panda-dock {
    pointer-events: auto;
  }
  .speech {
    position: relative;
    background: rgba(254, 249, 241, 0.96);
    border: 1px solid rgba(153, 101, 73, 0.18);
    border-radius: 24px;
    box-shadow: 0 18px 50px rgba(43, 32, 24, 0.16);
    padding: 16px 16px 14px;
    backdrop-filter: blur(14px);
  }
  .speech-meta {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    margin-bottom: 8px;
  }
  .name {
    font-size: 12px;
    font-weight: 800;
    letter-spacing: 0.18em;
    text-transform: uppercase;
    color: var(--ink);
  }
  .mood {
    font-family: var(--font-mono);
    font-size: 11px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }
  .speech-copy {
    color: var(--text);
    font-size: 13px;
    line-height: 1.55;
    margin-bottom: 12px;
  }
  .voice-row {
    display: flex;
    gap: 6px;
    margin-bottom: 12px;
    flex-wrap: wrap;
  }
  .voice-chip {
    border-radius: 999px;
    border: 1px solid rgba(153, 101, 73, 0.18);
    padding: 6px 11px;
    font-size: 11px;
    font-weight: 700;
    color: var(--text-secondary);
    background: rgba(255, 255, 255, 0.7);
  }
  .voice-chip.active {
    background: var(--primary);
    color: white;
    border-color: transparent;
  }
  .signal-row {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 8px;
    margin-bottom: 10px;
  }
  .signal {
    background: rgba(255, 255, 255, 0.72);
    border-radius: 16px;
    padding: 8px 10px;
    min-width: 0;
  }
  .signal-label {
    display: block;
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--text-tertiary);
    margin-bottom: 4px;
  }
  .signal strong {
    display: block;
    font-size: 12px;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .action-row {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }
  .workflow-row {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    margin-bottom: 10px;
    font-size: 11px;
    color: var(--text-secondary);
  }
  .workflow-row strong {
    color: var(--text);
    text-transform: capitalize;
  }
  .action-pill {
    padding: 6px 10px;
    border-radius: 999px;
    background: rgba(40, 31, 23, 0.06);
    color: var(--text-secondary);
    font-size: 11px;
    font-weight: 700;
  }
  .panda-dock {
    position: relative;
    height: 150px;
  }
  .glow {
    position: absolute;
    inset: auto 18px 8px 18px;
    height: 24px;
    border-radius: 50%;
    background: radial-gradient(circle, rgba(217, 119, 87, 0.35), transparent 70%);
    filter: blur(10px);
    animation: pulse 3.2s ease-in-out infinite;
  }
  .panda {
    position: absolute;
    inset: auto 6px 0 auto;
    width: 116px;
    height: 138px;
    animation: bob 3.8s ease-in-out infinite;
  }
  .ear,
  .head,
  .body,
  .paw,
  .eye-patch,
  .eye,
  .nose,
  .blush {
    position: absolute;
  }
  .ear {
    width: 30px;
    height: 30px;
    background: #1e1f22;
    border-radius: 50%;
    top: 10px;
  }
  .ear.left {
    left: 14px;
  }
  .ear.right {
    right: 14px;
  }
  .head {
    width: 94px;
    height: 86px;
    left: 11px;
    top: 18px;
    background: #fffdf8;
    border-radius: 44px 44px 36px 36px;
    border: 2px solid rgba(30, 31, 34, 0.08);
  }
  .eye-patch {
    width: 24px;
    height: 30px;
    background: #222428;
    top: 25px;
    border-radius: 48% 48% 52% 52%;
  }
  .eye-patch.left {
    left: 16px;
    transform: rotate(18deg);
  }
  .eye-patch.right {
    right: 16px;
    transform: rotate(-18deg);
  }
  .eye {
    width: 6px;
    height: 8px;
    background: white;
    border-radius: 50%;
    top: 36px;
    animation: blink 5.8s infinite;
  }
  .eye.left {
    left: 28px;
  }
  .eye.right {
    right: 28px;
  }
  .nose {
    width: 14px;
    height: 10px;
    left: 40px;
    top: 49px;
    background: #1f2024;
    border-radius: 50% 50% 60% 60%;
  }
  .mouth {
    position: absolute;
    width: 18px;
    height: 10px;
    left: 38px;
    top: 59px;
    border-bottom: 2px solid #1f2024;
    border-radius: 0 0 999px 999px;
  }
  .blush {
    width: 10px;
    height: 6px;
    background: rgba(217, 119, 87, 0.28);
    top: 56px;
    border-radius: 999px;
  }
  .blush.left {
    left: 14px;
  }
  .blush.right {
    right: 14px;
  }
  .body {
    width: 84px;
    height: 76px;
    left: 16px;
    bottom: 10px;
    background: linear-gradient(180deg, #27292d 0%, #17191c 100%);
    border-radius: 36px 36px 24px 24px;
  }
  .body::before {
    content: "";
    position: absolute;
    left: 18px;
    right: 18px;
    top: 10px;
    bottom: 12px;
    border-radius: 28px;
    background: #fbf7ef;
  }
  .paw {
    width: 22px;
    height: 16px;
    background: #17191c;
    bottom: 2px;
    border-radius: 16px;
  }
  .paw.left {
    left: 18px;
  }
  .paw.right {
    right: 18px;
  }
  .panda-dock[data-mood="warning"] .glow {
    background: radial-gradient(circle, rgba(212, 148, 58, 0.42), transparent 70%);
  }
  .panda-dock[data-mood="feral"] .glow {
    background: radial-gradient(circle, rgba(196, 85, 77, 0.48), transparent 70%);
  }
  .panda-dock[data-mood="paused"] .glow {
    background: radial-gradient(circle, rgba(217, 119, 87, 0.4), transparent 70%);
  }
  @keyframes bob {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-6px); }
  }
  @keyframes blink {
    0%, 45%, 47%, 100% { transform: scaleY(1); }
    46% { transform: scaleY(0.15); }
  }
  @keyframes pulse {
    0%, 100% { transform: scaleX(0.92); opacity: 0.7; }
    50% { transform: scaleX(1.03); opacity: 1; }
  }
  @media (max-width: 1100px) {
    .panda-shell {
      grid-template-columns: 1fr;
      right: 16px;
      left: auto;
      bottom: 12px;
      max-width: 320px;
    }
    .panda-dock {
      justify-self: end;
    }
  }
</style>
