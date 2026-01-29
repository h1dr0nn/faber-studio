<script lang="ts">
  import Panel from "$lib/components/ui/Panel.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import { Play } from "lucide-svelte";
  import { appConsole } from "$lib/stores/console.svelte";

  let command = $state("cargo tauri info");

  function runCommand() {
    if (!command) return;
    appConsole.info(`$ ${command}`, "Runner");

    // Mock run
    setTimeout(() => {
      appConsole.info("Running command...", "Runner");
      setTimeout(() => {
        appConsole.success("Command executed successfully.", "Runner");
      }, 500);
    }, 200);
  }
</script>

<div class="page-container">
  <div class="header">
    <h1 class="page-title">Command Runner</h1>
  </div>

  <div class="runner-bar">
    <div class="input-wrapper">
      <span class="prompt">$</span>
      <Input
        bind:value={command}
        placeholder="Enter command..."
        class="cmd-input"
      />
    </div>
    <Button variant="primary" onclick={runCommand}>
      <Play size={16} class="mr-2" /> Run
    </Button>
  </div>

  <!-- Runner specific logs or use global matrix? 
       Design decision: use this panel for specific output, matrix for global events.
       For now, let's just show a tip.
  -->
  <Panel title="Output" class="logs-panel inverted">
    <div class="empty-state">
      Check the <strong>Matrix Log Stream</strong> below for output.
    </div>
  </Panel>
</div>

<style>
  .page-container {
    padding: 24px;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-bottom: 60px;
  }

  .header {
    border-bottom: 1px solid var(--border-subtle);
    padding-bottom: 16px;
  }

  .page-title {
    font-size: 18px;
    font-weight: 500;
    margin: 0;
  }

  .runner-bar {
    display: flex;
    gap: 12px;
  }

  .input-wrapper {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
  }

  .prompt {
    position: absolute;
    left: 12px;
    color: var(--accent-success);
    font-family: var(--font-mono);
    font-weight: bold;
    z-index: 2;
  }

  :global(.cmd-input) {
    padding-left: 32px !important;
    font-family: var(--font-mono);
  }

  .logs-panel {
    flex: 1;
  }

  .empty-state {
    color: var(--fg-secondary);
    text-align: center;
    padding: 24px;
    font-size: var(--text-sm);
  }

  .mr-2 {
    margin-right: 8px;
  }
</style>
