<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { checkScreenRecordingPermission } from "../lib/api";

  let visible = $state(false);
  let dismissed = $state(false);

  onMount(async () => {
    try {
      const hasPermission = await checkScreenRecordingPermission();
      if (!hasPermission && !dismissed) {
        visible = true;
      }
    } catch (e) {
      console.error("Permission check failed:", e);
    }

    listen("permission-missing", () => {
      if (!dismissed) {
        visible = true;
      }
    });
  });

  function dismiss() {
    visible = false;
    dismissed = true;
  }
</script>

{#if visible}
  <div class="prompt-banner" role="alert">
    <div class="prompt-content">
      <strong>Screen Recording permission needed</strong>
      <p>
        VibeCheck needs Screen Recording access to read window titles for accurate
        activity classification. Without it, classification works by app name only.
      </p>
      <p class="prompt-path">
        System Settings &rarr; Privacy &amp; Security &rarr; Screen Recording &rarr; VibeCheck
      </p>
    </div>
    <button class="prompt-dismiss" onclick={dismiss} aria-label="Dismiss">
      Dismiss
    </button>
  </div>
{/if}

<style>
  .prompt-banner {
    background: var(--surface, #f5f3ef);
    border-bottom: 1px solid var(--warning, #d4943a);
    padding: 12px 20px;
    display: flex;
    align-items: flex-start;
    gap: 16px;
    font-size: 12px;
    color: var(--text);
  }
  .prompt-content {
    flex: 1;
  }
  .prompt-content strong {
    display: block;
    margin-bottom: 4px;
    font-size: 13px;
  }
  .prompt-content p {
    margin: 4px 0;
    color: var(--text-tertiary);
    line-height: 1.4;
  }
  .prompt-path {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--primary) !important;
  }
  .prompt-dismiss {
    flex-shrink: 0;
    background: none;
    border: 1px solid var(--border);
    padding: 4px 12px;
    border-radius: var(--radius-sm);
    font-size: 11px;
    cursor: pointer;
    color: var(--text-tertiary);
  }
  .prompt-dismiss:hover {
    color: var(--text);
  }
</style>
