<script lang="ts">
  interface Tab {
    id: string;
    label: string;
  }

  interface Props {
    tabs: Tab[];
    active: string;
    onSelect: (id: string) => void;
  }

  let { tabs, active, onSelect }: Props = $props();
</script>

<div class="tab-bar" role="tablist">
  {#each tabs as tab}
    <button
      class="tab"
      class:active={active === tab.id}
      role="tab"
      aria-selected={active === tab.id}
      onclick={() => onSelect(tab.id)}
    >
      {tab.label}
    </button>
  {/each}
</div>

<style>
  .tab-bar {
    display: flex;
    gap: 10px;
    margin: 0 auto 20px;
    max-width: 1120px;
    padding: 6px;
    border: 1px solid rgba(116, 78, 55, 0.12);
    border-radius: 999px;
    background: rgba(255, 251, 246, 0.72);
    backdrop-filter: blur(12px);
    width: fit-content;
  }
  .tab {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-tertiary);
    background: none;
    border: none;
    padding: 10px 14px;
    cursor: pointer;
    position: relative;
    transition: color 0.15s;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    border-radius: 999px;
  }
  .tab:hover {
    color: var(--text);
  }
  .tab.active {
    color: white;
    background: linear-gradient(135deg, var(--primary), #e69163);
    box-shadow: 0 10px 24px rgba(219, 118, 85, 0.2);
  }
</style>
