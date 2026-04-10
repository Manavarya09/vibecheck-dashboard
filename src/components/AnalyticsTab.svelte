<script lang="ts">
  import Heatmap from "./Heatmap.svelte";
</script>

<div class="analytics">
  <div class="section">
    <h3 class="section-title">Activity Heatmap</h3>
    <p class="section-desc">Your coding activity over the past year</p>
    <Heatmap />
  </div>
</div>

<style>
  .analytics {
    max-width: 840px;
    margin: 0 auto;
  }
  .section {
    margin-bottom: 32px;
  }
  .section-title {
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
    margin-bottom: 4px;
  }
  .section-desc {
    font-size: 12px;
    color: var(--text-tertiary);
    margin-bottom: 16px;
  }
</style>
