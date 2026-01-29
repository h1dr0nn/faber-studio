<script lang="ts">
  import Panel from '$lib/components/ui/Panel.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  
  // Placeholder for real Doctor logic
  let loading = $state(false);
  let checks = $state([
    { name: 'Node.js', status: 'success', version: 'v20.10.0' },
    { name: 'Rust', status: 'success', version: '1.75.0' },
    { name: 'Cargo', status: 'success', version: '1.75.0' },
    { name: 'Tauri CLI', status: 'warning', version: 'v1.5.0 (outdated)' },
    { name: 'Android SDK', status: 'error', message: 'Not found' },
  ]);

  function runCheck() {
    loading = true;
    setTimeout(() => {
      loading = false;
    }, 1000);
  }
</script>

<div class="page-container">
  <div class="header">
    <h1 class="page-title">Environment Doctor</h1>
    <Button variant="primary" size="sm" onclick={runCheck} disabled={loading}>
      {loading ? 'Scanning...' : 'Run Checks'}
    </Button>
  </div>

  <div class="grid">
    <Panel title="System Status" class="col-span-1 h-full">
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
    </Panel>
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
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .page-title {
    font-size: 18px;
    font-weight: 500;
    margin: 0;
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 16px;
    flex: 1;
    min-height: 0; 
  }

  .check-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .check-item {
    display: flex;
    align-items: center;
    padding: 8px;
    background-color: var(--bg-app);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
  }

  .check-icon {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    margin-right: 12px;
  }
  
  .check-icon.success { background-color: var(--accent-success); }
  .check-icon.warning { background-color: var(--accent-warning); }
  .check-icon.error { background-color: var(--accent-error); }

  .check-info {
    display: flex;
    flex-direction: column;
  }

  .check-name {
    font-weight: 500;
  }

  .check-meta {
    font-size: var(--text-xs);
    color: var(--fg-secondary);
  }
</style>
