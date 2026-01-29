<script lang="ts">
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import { Play } from "lucide-svelte";
  import { appConsole } from "$lib/stores/console.svelte";

  import { uiState } from "$lib/ui-state.svelte";

  let command = $state("npm run dev");

  function runCommand() {
    if (!command) return;
    const parts = command.trim().split(/\s+/);
    if (parts.length === 0) return;

    const cmd = parts[0];
    const args = parts.slice(1);

    appConsole.info(`> ${command}`, "Runner");
    uiState.runTask(cmd, args);
  }
</script>

<div class="command-runner">
  <div class="runner-bar">
    <div class="input-wrapper">
      <span class="prompt">$</span>
      <Input
        bind:value={command}
        placeholder="Enter command..."
        class="cmd-input"
        onkeydown={(e: KeyboardEvent) => e.key === "Enter" && runCommand()}
      />
    </div>
    <Button variant="primary" size="sm" onclick={runCommand}>
      <Play size={14} class="mr-2" /> Run
    </Button>
  </div>
  <div class="runner-hint">Output will be streamed to the Matrix Log.</div>
</div>

<style>
  .command-runner {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 12px;
    height: 100%;
  }

  .runner-bar {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .input-wrapper {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
  }

  .prompt {
    position: absolute;
    left: 10px;
    color: var(--accent-success);
    font-family: var(--font-mono);
    font-weight: bold;
    z-index: 2;
    font-size: 13px;
  }

  :global(.cmd-input) {
    padding-left: 28px !important;
    font-family: var(--font-mono);
    height: 32px !important;
    font-size: 13px !important;
  }

  .runner-hint {
    font-size: 11px;
    color: var(--fg-tertiary);
    font-style: italic;
  }
</style>
