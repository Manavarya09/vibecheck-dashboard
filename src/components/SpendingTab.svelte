<script lang="ts">
  import { onMount } from "svelte";
  import { getSpendingRates, upsertSpendingRate, deleteSpendingRate } from "../lib/api";
  import type { SpendingRate } from "../lib/api";

  let rates = $state<SpendingRate[]>([]);
  let loaded = $state(false);
  let newTool = $state("");
  let newRate = $state(20);
  let newType = $state("subscription");
  let newPeriod = $state("monthly");

  onMount(async () => {
    rates = await getSpendingRates();
    loaded = true;
  });

  async function handleAdd() {
    if (!newTool.trim()) return;
    await upsertSpendingRate(newTool.trim(), newType, newRate, newPeriod);
    rates = await getSpendingRates();
    newTool = "";
  }

  async function handleDelete(id: number) {
    await deleteSpendingRate(id);
    rates = await getSpendingRates();
  }

  let monthlyTotal = $derived(
    rates.reduce((sum, r) => {
      if (r.rateType === "subscription") {
        return sum + (r.billingPeriod === "yearly" ? r.rateValue / 12 : r.rateValue);
      }
      return sum;
    }, 0)
  );
</script>

<div class="spending">
  <div class="section">
    <h3 class="section-title">Spending Rates</h3>
    <p class="section-desc">Configure your AI tool subscription costs</p>

    {#if loaded}
      <div class="rates-list">
        {#each rates as rate}
          <div class="rate-row">
            <span class="rate-name">{rate.toolName}</span>
            <span class="rate-value">${rate.rateValue}/{rate.billingPeriod ?? "mo"}</span>
            <button class="delete-btn" onclick={() => handleDelete(rate.id)} aria-label="Delete {rate.toolName}">x</button>
          </div>
        {/each}
      </div>

      <div class="add-form">
        <input class="input" placeholder="Tool name" bind:value={newTool} />
        <input class="input num" type="number" bind:value={newRate} min="0" step="0.01" />
        <select class="input" bind:value={newPeriod}>
          <option value="monthly">/mo</option>
          <option value="yearly">/yr</option>
        </select>
        <button class="add-btn" onclick={handleAdd}>Add</button>
      </div>

      <div class="monthly-total">
        <span class="total-label">Monthly total</span>
        <span class="total-value">${monthlyTotal.toFixed(2)}</span>
      </div>
    {:else}
      <p class="loading">Loading...</p>
    {/if}
  </div>
</div>

<style>
  .spending { max-width: 560px; }
  .section { margin-bottom: 32px; }
  .section-title { font-size: 15px; font-weight: 700; color: var(--text); margin-bottom: 4px; }
  .section-desc { font-size: 12px; color: var(--text-tertiary); margin-bottom: 16px; }
  .rates-list { display: flex; flex-direction: column; gap: 0; }
  .rate-row {
    display: flex; align-items: center; justify-content: space-between;
    padding: 10px 0; border-bottom: 1px solid var(--border); gap: 12px;
  }
  .rate-name { font-size: 13px; font-weight: 600; color: var(--text); flex: 1; }
  .rate-value { font-family: var(--font-mono); font-size: 13px; color: var(--primary); }
  .delete-btn {
    background: none; border: none; color: var(--text-tertiary); cursor: pointer;
    font-size: 14px; padding: 2px 6px;
  }
  .delete-btn:hover { color: var(--danger); }
  .add-form {
    display: flex; gap: 8px; margin-top: 12px; align-items: center;
  }
  .input {
    padding: 6px 10px; border: 1px solid var(--border); border-radius: var(--radius-sm);
    font-size: 12px; color: var(--text); background: var(--surface);
  }
  .input:focus { outline: none; border-color: var(--primary); }
  .input.num { width: 70px; text-align: right; font-family: var(--font-mono); }
  select.input { width: 60px; }
  .add-btn {
    padding: 6px 14px; background: var(--primary); color: white; border: none;
    border-radius: var(--radius-sm); font-size: 12px; font-weight: 600; cursor: pointer;
  }
  .add-btn:hover { background: var(--primary-hover); }
  .monthly-total {
    display: flex; justify-content: space-between; margin-top: 16px;
    padding-top: 12px; border-top: 1px solid var(--border);
  }
  .total-label { font-size: 12px; font-weight: 600; color: var(--text-tertiary); text-transform: uppercase; letter-spacing: 0.06em; }
  .total-value { font-family: var(--font-mono); font-size: 16px; font-weight: 700; color: var(--primary); }
  .loading { font-size: 13px; color: var(--text-tertiary); }
</style>
