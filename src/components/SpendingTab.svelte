<script lang="ts">
  import { onMount } from "svelte";
  import SpendingChart from "./SpendingChart.svelte";
  import {
    getSpendingRates,
    upsertSpendingRate,
    deleteSpendingRate,
    getBudgetConfigs,
    setBudget,
  } from "../lib/api";
  import type { BudgetConfig, SpendingRate } from "../lib/api";

  let rates = $state<SpendingRate[]>([]);
  let budgets = $state<BudgetConfig[]>([]);
  let loaded = $state(false);
  let newTool = $state("");
  let newRate = $state(20);
  let newType = $state("subscription");
  let newPeriod = $state("monthly");
  let monthlyBudget = $state(150);

  onMount(async () => {
    const [loadedRates, loadedBudgets] = await Promise.all([
      getSpendingRates(),
      getBudgetConfigs(),
    ]);
    rates = loadedRates;
    budgets = loadedBudgets;
    const monthly = loadedBudgets.find((budget) => budget.period === "monthly");
    if (monthly) monthlyBudget = monthly.limitAmount;
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

  async function handleSaveBudget() {
    await setBudget("monthly", monthlyBudget);
    budgets = await getBudgetConfigs();
  }

  let monthlyTotal = $derived(
    rates.reduce((sum, r) => {
      if (r.rateType === "subscription") {
        return sum + (r.billingPeriod === "yearly" ? r.rateValue / 12 : r.rateValue);
      }
      return sum;
    }, 0)
  );
  let projectedStatus = $derived(
    monthlyBudget > 0 ? Math.round((monthlyTotal / monthlyBudget) * 100) : 0
  );
</script>

<div class="spending">
  <div class="section">
    <h3 class="section-title">Spending Overview</h3>
    <p class="section-desc">Your 30-day AI tool spending</p>
    <SpendingChart />
  </div>

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

      <div class="budget-panel">
        <div class="budget-copy">
          <span class="budget-title">Monthly budget</span>
          <span class="budget-desc">Set the limit where Panko starts shaming your subscription stack.</span>
        </div>
        <div class="budget-actions">
          <input class="input num" type="number" bind:value={monthlyBudget} min="0" step="1" />
          <button class="add-btn" onclick={handleSaveBudget}>Save Budget</button>
        </div>
        <div class="budget-status">
          <span class="total-label">Projected load</span>
          <span class:danger={projectedStatus >= 100} class="total-value">{projectedStatus}%</span>
        </div>
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
  .budget-panel {
    margin-top: 14px;
    padding-top: 14px;
    border-top: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .budget-copy {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }
  .budget-title { font-size: 13px; font-weight: 700; color: var(--text); }
  .budget-desc { font-size: 12px; color: var(--text-tertiary); }
  .budget-actions {
    display: flex;
    gap: 8px;
    align-items: center;
  }
  .budget-status {
    display: flex;
    justify-content: space-between;
  }
  .total-label { font-size: 12px; font-weight: 600; color: var(--text-tertiary); text-transform: uppercase; letter-spacing: 0.06em; }
  .total-value { font-family: var(--font-mono); font-size: 16px; font-weight: 700; color: var(--primary); }
  .total-value.danger { color: var(--danger); }
  .loading { font-size: 13px; color: var(--text-tertiary); }
</style>
