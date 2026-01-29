<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Button from "$lib/components/ui/Button.svelte";

  // Real Diagnostic state
  let loading = $state(false);
  let checks = $state<any[]>([]);

  async function runCheck() {
    loading = true;
    try {
      checks = await invoke("check_environment");
    } catch (err) {
      console.error("Failed to run diagnostics:", err);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    runCheck();
  });
</script>

<div class="feature-container">
  <div class="header-compact">
    <Button
      variant="primary"
      size="sm"
      onclick={runCheck}
      disabled={loading}
      class="w-full"
    >
      {loading ? "Scanning..." : "Run Diagnostics"}
    </Button>
  </div>

  <div class="content-scroll">
    <div class="check-list">
      {#each checks as check}
        <div class="check-item">
          <div class="check-icon {check.status}"></div>
          <div class="check-info">
            <span class="check-name">{check.name}</span>
            {#if check.version}
              <span class="check-meta mono">{check.version}</span>
            {/if}
            {#if check.message}
              <span class="check-meta text-error">{check.message}</span>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .feature-container {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .header-compact {
    padding: 12px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .content-scroll {
    flex: 1;
    overflow-y: auto;
  }

  .check-list {
    display: flex;
    flex-direction: column;
    padding: 8px 12px;
    gap: 8px;
  }

  .check-item {
    display: flex;
    align-items: center;
    padding: 8px;
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
  }

  .check-icon {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    margin-right: 10px;
    flex-shrink: 0;
  }

  .check-icon.success {
    background-color: var(--accent-success);
  }
  .check-icon.warning {
    background-color: var(--accent-warning);
  }
  .check-icon.error {
    background-color: var(--accent-error);
  }

  .check-info {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .check-name {
    font-weight: 500;
    font-size: 13px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .check-meta {
    font-size: 11px;
    color: var(--fg-secondary);
  }

  .text-error {
    color: var(--accent-error);
  }

  .mono {
    font-family: var(--font-mono);
  }
</style>
