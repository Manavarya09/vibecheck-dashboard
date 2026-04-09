<script lang="ts">
  let { status = "idle" }: { status: string } = $props();

  const config: Record<string, { label: string; color: string }> = {
    active: { label: "ACTIVE", color: "var(--success)" },
    paused: { label: "PAUSED", color: "var(--warning)" },
    idle: { label: "IDLE", color: "var(--text-tertiary)" },
    completed: { label: "DONE", color: "var(--text-secondary)" },
  };

  let current = $derived(config[status] ?? config.idle);
</script>

<span class="status" style="color: {current.color}">
  {current.label}
</span>

<style>
  .status {
    font-family: var(--font-mono);
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.08em;
  }
</style>
