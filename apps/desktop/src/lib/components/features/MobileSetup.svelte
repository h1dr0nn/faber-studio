<script lang="ts">
  import Button from "$lib/components/ui/Button.svelte";
  import Tabs from "$lib/components/ui/Tabs.svelte";

  let activeTab = $state("android");
</script>

<div class="feature-container">
  <div class="tabs-wrapper">
    <Tabs
      bind:activeTabId={activeTab}
      tabs={[
        { id: "android", label: "Android" },
        { id: "ios", label: "iOS" },
      ]}
    />
  </div>

  <div class="content-scroll">
    {#if activeTab === "android"}
      <div class="empty-state">
        <span class="primary-text">No Android devices</span>
        <span class="secondary-text">Enable USB debugging</span>
        <Button variant="outline" size="sm" class="mt-4">Refresh</Button>
      </div>
    {:else}
      <div class="empty-state">
        <span class="primary-text">No iOS devices</span>
        <span class="secondary-text">Check Xcode connection</span>
        <Button variant="outline" size="sm" class="mt-4">Refresh</Button>
      </div>
    {/if}
  </div>
</div>

<style>
  .feature-container {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .tabs-wrapper {
    border-bottom: 1px solid var(--border-subtle);
  }

  .content-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 16px 12px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    min-height: 200px;
    text-align: center;
    color: var(--fg-secondary);
  }

  .primary-text {
    font-size: 13px;
    font-weight: 500;
  }

  .secondary-text {
    font-size: 11px;
    color: var(--fg-tertiary);
  }
</style>
