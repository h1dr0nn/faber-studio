<script lang="ts">
  import Panel from "$lib/components/ui/Panel.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Tabs from "$lib/components/ui/Tabs.svelte";

  let activeTab = $state("android");
</script>

<div class="page-container">
  <div class="header">
    <h1 class="page-title">Mobile Targets</h1>
  </div>

  <Tabs
    bind:activeTabId={activeTab}
    tabs={[
      { id: "android", label: "Android" },
      { id: "ios", label: "iOS" },
    ]}
  />

  <div class="content">
    {#if activeTab === "android"}
      <Panel title="Android Targets" class="h-full">
        <div class="empty-state">
          <span>No Android devices connected.</span>
          <br />
          <span class="text-mute text-xs">Ensure USB debugging is enabled.</span
          >

          <div class="actions">
            <Button variant="outline" size="sm">Refresh Devices</Button>
          </div>
        </div>
      </Panel>
    {:else}
      <Panel title="iOS Targets" class="h-full">
        <div class="empty-state">
          <span>No iOS devices found.</span>

          <div class="actions">
            <Button variant="outline" size="sm">Refresh Devices</Button>
          </div>
        </div>
      </Panel>
    {/if}
  </div>
</div>

<style>
  .page-container {
    padding: 24px;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .header {
    /* No bottom border here since tabs do it */
    padding-bottom: 8px;
  }

  .page-title {
    font-size: 18px;
    font-weight: 500;
    margin: 0;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
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

  .actions {
    margin-top: 16px;
  }

  .text-mute {
    color: var(--fg-secondary);
  }
  .text-xs {
    font-size: var(--text-xs);
  }
</style>
