<script lang="ts">
  import { getSettings, updateSetting, resetSettings, exportData, getDbPath } from "../lib/api";
  import { onMount } from "svelte";

  let settings = $state<Record<string, string>>({});
  let saving = $state<string | null>(null);
  let loaded = $state(false);

  onMount(async () => {
    settings = await getSettings();
    loaded = true;
  });

  async function handleToggle(key: string) {
    const current = settings[key] === "true";
    const next = (!current).toString();
    saving = key;
    await updateSetting(key, next);
    settings[key] = next;
    saving = null;
  }

  async function handleNumber(key: string, value: string) {
    const num = parseInt(value, 10);
    if (isNaN(num) || num < 1) return;
    saving = key;
    await updateSetting(key, num.toString());
    settings[key] = num.toString();
    saving = null;
  }

  async function handleReset() {
    if (!confirm("Reset all settings to defaults?")) return;
    settings = await resetSettings();
  }

  interface SettingDef {
    key: string;
    label: string;
    description: string;
    type: "toggle" | "number";
    min?: number;
    max?: number;
    unit?: string;
  }

  const SETTING_DEFS: SettingDef[] = [
    {
      key: "break_enforcer_enabled",
      label: "Break Enforcer",
      description: "Show break reminders during long AI coding sessions",
      type: "toggle",
    },
    {
      key: "break_interval_mins",
      label: "Break Interval",
      description: "Minutes of continuous AI coding before a break reminder",
      type: "number",
      min: 5,
      max: 120,
      unit: "min",
    },
    {
      key: "break_duration_mins",
      label: "Break Duration",
      description: "Suggested break length",
      type: "number",
      min: 1,
      max: 30,
      unit: "min",
    },
    {
      key: "idle_threshold_mins",
      label: "Idle Threshold",
      description: "Minutes of inactivity before auto-pausing",
      type: "number",
      min: 1,
      max: 30,
      unit: "min",
    },
    {
      key: "polling_interval_secs",
      label: "Polling Interval",
      description: "How often to check the active window",
      type: "number",
      min: 1,
      max: 30,
      unit: "sec",
    },
    {
      key: "auto_start_on_coding",
      label: "Auto-start Sessions",
      description: "Automatically start a session when a coding app is detected",
      type: "toggle",
    },
    {
      key: "auto_stop_on_idle",
      label: "Auto-stop on Idle",
      description: "Stop sessions after extended inactivity",
      type: "toggle",
    },
    {
      key: "startup_at_login",
      label: "Start at Login",
      description: "Launch VibeCheck when you log in to your Mac",
      type: "toggle",
    },
  ];

  let exporting = $state(false);
  let dbPath = $state("");

  async function showDbPath() {
    dbPath = await getDbPath();
  }

  async function handleExport(format: "json" | "csv") {
    exporting = true;
    try {
      const data = await exportData(format);
      const blob = new Blob([data], { type: format === "json" ? "application/json" : "text/csv" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `vibecheck-export.${format}`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (e) {
      console.error("Export failed:", e);
    }
    exporting = false;
  }
</script>

{#if !loaded}
  <div class="loading">Loading settings...</div>
{:else}
  <div class="settings">
    <div class="settings-header">
      <h2 class="settings-title">Settings</h2>
      <button class="reset-btn" onclick={handleReset}>Reset Defaults</button>
    </div>

    <div class="settings-list">
      {#each SETTING_DEFS as def}
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-label">{def.label}</span>
            <span class="setting-desc">{def.description}</span>
          </div>
          <div class="setting-control">
            {#if def.type === "toggle"}
              <button
                class="toggle"
                class:active={settings[def.key] === "true"}
                class:saving={saving === def.key}
                onclick={() => handleToggle(def.key)}
                aria-label="Toggle {def.label}"
              >
                <span class="toggle-knob"></span>
              </button>
            {:else if def.type === "number"}
              <div class="number-input">
                <input
                  type="number"
                  value={settings[def.key]}
                  min={def.min}
                  max={def.max}
                  onchange={(e) => handleNumber(def.key, (e.target as HTMLInputElement).value)}
                />
                {#if def.unit}
                  <span class="unit">{def.unit}</span>
                {/if}
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>

    <div class="export-section">
      <h3 class="export-title">Database</h3>
      <p class="export-desc">Your data is stored locally. Back up the database file to keep it safe.</p>
      <button class="export-btn" onclick={showDbPath}>Show Database Location</button>
    </div>

    <div class="export-section">
      <h3 class="export-title">Data Export</h3>
      <p class="export-desc">Export all sessions and activity data.</p>
      <div class="export-actions">
        <button class="export-btn" onclick={() => handleExport("json")} disabled={exporting}>
          Export JSON
        </button>
        <button class="export-btn" onclick={() => handleExport("csv")} disabled={exporting}>
          Export CSV
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .settings {
    padding: 24px;
    max-width: 560px;
  }
  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }
  .settings-title {
    font-size: 18px;
    font-weight: 700;
    color: var(--text);
  }
  .reset-btn {
    font-size: 12px;
    color: var(--text-tertiary);
    background: none;
    border: 1px solid var(--border);
    padding: 4px 12px;
    border-radius: var(--radius-sm);
    cursor: pointer;
  }
  .reset-btn:hover {
    color: var(--danger);
    border-color: var(--danger);
  }
  .settings-list {
    display: flex;
    flex-direction: column;
    gap: 0;
  }
  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 0;
    border-bottom: 1px solid var(--border);
    gap: 16px;
  }
  .setting-row:last-child {
    border-bottom: none;
  }
  .setting-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .setting-label {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
  }
  .setting-desc {
    font-size: 11px;
    color: var(--text-tertiary);
    line-height: 1.4;
  }
  .setting-control {
    flex-shrink: 0;
  }
  .toggle {
    width: 40px;
    height: 22px;
    border-radius: 11px;
    background: var(--border);
    border: none;
    cursor: pointer;
    position: relative;
    transition: background 0.2s;
  }
  .toggle.active {
    background: var(--primary);
  }
  .toggle.saving {
    opacity: 0.6;
  }
  .toggle-knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: white;
    transition: transform 0.2s;
  }
  .toggle.active .toggle-knob {
    transform: translateX(18px);
  }
  .number-input {
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .number-input input {
    width: 56px;
    padding: 4px 8px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: 13px;
    text-align: right;
    color: var(--text);
    background: var(--surface);
  }
  .number-input input:focus {
    outline: none;
    border-color: var(--primary);
  }
  .unit {
    font-size: 11px;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
  }
  .loading {
    padding: 24px;
    color: var(--text-tertiary);
    font-size: 13px;
  }
  .export-section {
    margin-top: 24px;
    padding-top: 24px;
    border-top: 1px solid var(--border);
  }
  .export-title {
    font-size: 14px;
    font-weight: 700;
    color: var(--text);
    margin-bottom: 4px;
  }
  .export-desc {
    font-size: 12px;
    color: var(--text-tertiary);
    margin-bottom: 12px;
  }
  .export-actions {
    display: flex;
    gap: 8px;
  }
  .export-btn {
    padding: 8px 16px;
    font-size: 12px;
    font-weight: 600;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text);
    cursor: pointer;
    transition: background 0.15s;
  }
  .export-btn:hover:not(:disabled) {
    background: var(--surface-hover);
  }
  .export-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
