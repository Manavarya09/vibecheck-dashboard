<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { formatDuration } from "../lib/utils";

  let visible = $state(false);
  let continuousSecs = $state(0);
  let breakDurationMins = $state(5);
  let countdownSecs = $state(0);
  let countdownInterval: ReturnType<typeof setInterval> | null = null;

  onMount(() => {
    listen<{ continuousSecs: number; breakDurationMins: number }>(
      "break-reminder",
      (event) => {
        continuousSecs = event.payload.continuousSecs;
        breakDurationMins = event.payload.breakDurationMins;
        countdownSecs = breakDurationMins * 60;
        visible = true;
        startCountdown();
      }
    );
  });

  function startCountdown() {
    if (countdownInterval) clearInterval(countdownInterval);
    countdownInterval = setInterval(() => {
      countdownSecs -= 1;
      if (countdownSecs <= 0) {
        dismiss();
      }
    }, 1000);
  }

  function dismiss() {
    visible = false;
    if (countdownInterval) {
      clearInterval(countdownInterval);
      countdownInterval = null;
    }
  }

  function snooze() {
    dismiss();
    // Snooze is handled by resetting the counter on the backend --
    // the next break will trigger after another full interval
  }

  let countdownDisplay = $derived(formatDuration(countdownSecs));
  let codingDisplay = $derived(formatDuration(continuousSecs));
</script>

{#if visible}
  <div class="overlay" role="dialog" aria-label="Break reminder">
    <div class="overlay-card">
      <h2 class="overlay-title">Time for a Break</h2>
      <p class="overlay-subtitle">
        You've been AI coding for <strong>{codingDisplay}</strong> straight.
      </p>

      <div class="countdown">
        <span class="countdown-value">{countdownDisplay}</span>
        <span class="countdown-label">suggested break</span>
      </div>

      <p class="overlay-tip">
        Step away from the screen. Stretch. Hydrate. Your code will be here when you get back.
      </p>

      <div class="overlay-actions">
        <button class="btn btn-primary" onclick={dismiss}>
          I'm Back
        </button>
        <button class="btn btn-secondary" onclick={snooze}>
          Snooze 5 min
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(26, 26, 46, 0.85);
    backdrop-filter: blur(8px);
    animation: fadeIn 0.3s ease;
  }
  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
  .overlay-card {
    background: var(--bg);
    border-radius: var(--radius-lg, 16px);
    padding: 48px;
    max-width: 420px;
    text-align: center;
    box-shadow: 0 24px 48px rgba(0, 0, 0, 0.3);
    animation: slideUp 0.4s ease;
  }
  @keyframes slideUp {
    from { transform: translateY(20px); opacity: 0; }
    to { transform: translateY(0); opacity: 1; }
  }
  .overlay-title {
    font-size: 24px;
    font-weight: 700;
    color: var(--text);
    margin-bottom: 8px;
  }
  .overlay-subtitle {
    font-size: 14px;
    color: var(--text-secondary, var(--text-tertiary));
    margin-bottom: 24px;
  }
  .overlay-subtitle strong {
    color: var(--primary);
    font-family: var(--font-mono);
  }
  .countdown {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: 24px;
  }
  .countdown-value {
    font-size: 48px;
    font-weight: 700;
    font-family: var(--font-mono);
    color: var(--primary);
    letter-spacing: -0.02em;
  }
  .countdown-label {
    font-size: 11px;
    color: var(--text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    font-weight: 600;
    margin-top: 4px;
  }
  .overlay-tip {
    font-size: 13px;
    color: var(--text-tertiary);
    margin-bottom: 32px;
    line-height: 1.5;
  }
  .overlay-actions {
    display: flex;
    gap: 8px;
    justify-content: center;
  }
  .btn {
    padding: 10px 24px;
    border-radius: var(--radius-md);
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s;
  }
  .btn-primary {
    background: var(--primary);
    color: white;
    border: none;
  }
  .btn-primary:hover {
    background: var(--primary-hover);
  }
  .btn-secondary {
    background: none;
    color: var(--text-tertiary);
    border: 1px solid var(--border);
  }
  .btn-secondary:hover {
    color: var(--text);
    border-color: var(--text-tertiary);
  }
</style>
