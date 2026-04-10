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
    gap: 24px;
    border-bottom: 1px solid var(--border);
    margin-bottom: 20px;
    padding-bottom: 0;
  }
  .tab {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-tertiary);
    background: none;
    border: none;
    padding: 8px 0;
    cursor: pointer;
    position: relative;
    transition: color 0.15s;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
  .tab:hover {
    color: var(--text);
  }
  .tab.active {
    color: var(--primary);
  }
  .tab.active::after {
    content: "";
    position: absolute;
    bottom: -1px;
    left: 0;
    right: 0;
    height: 2px;
    background: var(--primary);
    border-radius: 1px 1px 0 0;
  }
</style>
