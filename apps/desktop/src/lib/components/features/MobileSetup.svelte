<script lang="ts">
  import Button from "$lib/components/ui/Button.svelte";
  import Tabs from "$lib/components/ui/Tabs.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { Smartphone, RefreshCw, Loader2 } from "lucide-svelte";

  interface MobileDevice {
    id: string;
    model: string;
    manufacturer: string;
    platform: string;
  }

  let activeTab = $state("android");
  let devices = $state<MobileDevice[]>([]);
  let isLoading = $state(false);
  let error = $state("");

  onMount(() => {
    fetchDevices();
  });

  async function fetchDevices() {
    isLoading = true;
    error = "";
    try {
      const result = await invoke<MobileDevice[]>("get_devices");
      devices = result;
    } catch (err: any) {
      error = String(err);
      console.error("Failed to fetch devices:", err);
    } finally {
      isLoading = false;
    }
  }

  let androidDevices = $derived(
    devices.filter((d) => d.platform === "android"),
  );
  let iosDevices = $derived(devices.filter((d) => d.platform === "ios"));
</script>

<div class="feature-container">
  <div class="tabs-wrapper">
    <Tabs
      bind:activeTabId={activeTab}
      tabs={[
        { id: "android", label: `Android (${androidDevices.length})` },
        { id: "ios", label: `iOS (${iosDevices.length})` },
      ]}
    />
  </div>

  <div class="content-scroll">
    {#if isLoading}
      <div class="empty-state">
        <Loader2 size={24} class="spin" />
        <span class="secondary-text">Scanning devices...</span>
      </div>
    {:else if error}
      <div class="empty-state">
        <span class="error-text">{error}</span>
        <Button variant="outline" size="sm" onclick={fetchDevices}>Retry</Button
        >
      </div>
    {:else if activeTab === "android"}
      {#if androidDevices.length === 0}
        <div class="empty-state">
          <Smartphone size={32} class="text-mute" />
          <span class="primary-text">No Android devices</span>
          <span class="secondary-text"
            >Enable USB debugging and connect device</span
          >
          <Button variant="outline" size="sm" onclick={fetchDevices}>
            <RefreshCw size={14} />
            Refresh
          </Button>
        </div>
      {:else}
        <div class="device-list">
          {#each androidDevices as device}
            <div class="device-item">
              <Smartphone size={18} />
              <div class="device-info">
                <span class="device-model">{device.model}</span>
                <span class="device-id">{device.id}</span>
              </div>
            </div>
          {/each}
        </div>
        <Button
          variant="ghost"
          size="sm"
          onclick={fetchDevices}
          class="refresh-btn"
        >
          <RefreshCw size={14} />
          Refresh
        </Button>
      {/if}
    {:else}
      <div class="empty-state">
        <Smartphone size={32} class="text-mute" />
        <span class="primary-text">No iOS devices</span>
        <span class="secondary-text">iOS detection coming soon</span>
        <Button variant="outline" size="sm" onclick={fetchDevices}>
          <RefreshCw size={14} />
          Refresh
        </Button>
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
    gap: 8px;
  }

  .primary-text {
    font-size: 13px;
    font-weight: 500;
  }

  .secondary-text {
    font-size: 11px;
    color: var(--fg-tertiary);
  }

  .error-text {
    font-size: 11px;
    color: #ff4646;
  }

  .text-mute {
    color: var(--fg-tertiary);
  }

  .device-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .device-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
  }

  .device-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .device-model {
    font-size: 13px;
    font-weight: 500;
    color: var(--fg-primary);
  }

  .device-id {
    font-size: 10px;
    color: var(--fg-tertiary);
    font-family: monospace;
  }

  .refresh-btn {
    margin-top: 12px;
  }

  :global(.spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
