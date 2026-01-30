<script lang="ts">
  import { appConsole } from "$lib/stores/console.svelte";
  import { uiState } from "$lib/ui-state.svelte";
  import { onMount, onDestroy } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { fly } from "svelte/transition";
  import { cn } from "$lib/utils";
  import { Play, Trash2, Terminal, Plus, X } from "lucide-svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";

  let command = $state("pnpm run dev");
  let unlisten: any[] = [];
  let logContainer: HTMLElement;

  function runCommand() {
    if (!command) return;
    const parts = command.trim().split(/\s+/);
    if (parts.length === 0) return;

    const cmd = parts[0];
    const args = parts.slice(1);

    appConsole.info(`> ${command}`, "Terminal");
    uiState.runTask(cmd, args);
  }

  function createNewSession() {
    appConsole.createSession(`Terminal ${appConsole.sessions.length + 1}`);
  }

  onMount(async () => {
    unlisten.push(
      await listen("runner-stdout", (event: any) => {
        appConsole.info(event.payload.message, "Runner");
      }),
    );
    unlisten.push(
      await listen("runner-stderr", (event: any) => {
        appConsole.warn(event.payload.message, "Runner");
      }),
    );
    unlisten.push(
      await listen("runner-error", (event: any) => {
        appConsole.error(event.payload.error, "Runner");
      }),
    );
    unlisten.push(
      await listen("runner-exit", (event: any) => {
        appConsole.success(
          `Process exited with code ${event.payload.code}`,
          "Runner",
        );
      }),
    );
  });

  onDestroy(() => {
    unlisten.forEach((u) => u());
  });

  $effect(() => {
    if (appConsole.entries && logContainer) {
      setTimeout(() => {
        logContainer.scrollTop = logContainer.scrollHeight;
      }, 0);
    }
  });
</script>

<div class="console-container">
  <div class="console-tabs">
    {#each appConsole.sessions as session}
      <div
        class="console-tab {appConsole.activeSessionId === session.id
          ? 'active'
          : ''}"
        onclick={() => (appConsole.activeSessionId = session.id)}
        role="button"
        tabindex="0"
        onkeydown={(e) =>
          e.key === "Enter" && (appConsole.activeSessionId = session.id)}
      >
        <span class="tab-label">{session.name}</span>
        {#if appConsole.sessions.length > 1}
          <button
            class="close-tab"
            onclick={(e) => {
              e.stopPropagation();
              appConsole.closeSession(session.id);
            }}
            title="Close Tab"
          >
            <X size={10} />
          </button>
        {/if}
      </div>
    {/each}
    <button class="add-tab" onclick={createNewSession} title="New Terminal">
      <Plus size={14} />
    </button>
  </div>

  <div class="console-output selectable" bind:this={logContainer}>
    {#each appConsole.entries as entry (entry.id)}
      <div
        class={cn("log-entry", entry.level)}
        transition:fly={{ x: -10, duration: 100 }}
      >
        <span class="timestamp"
          >{entry.timestamp.toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit",
            hour12: false,
          })}</span
        >
        <span class="source">[{entry.source}]</span>
        <span class="message">{entry.message}</span>
      </div>
    {/each}
    {#if appConsole.entries.length === 0}
      <div class="empty-state">Console Ready.</div>
    {/if}
  </div>

  <div class="console-input-bar">
    {#if uiState.projectRoot}
      <div class="cwd-indicator" title={uiState.projectRoot}>
        {uiState.projectRoot}
      </div>
    {/if}
    <div class="input-row">
      <div class="input-inner">
        <Terminal size={12} class="prompt-icon" />
        <Input
          bind:value={command}
          placeholder="Type command here..."
          class="cmd-input"
          onkeydown={(e: KeyboardEvent) => e.key === "Enter" && runCommand()}
        />
      </div>
      <div class="bar-actions">
        <Button
          variant="ghost"
          size="sm"
          onclick={() => appConsole.clear()}
          title="Clear Console"
        >
          <Trash2 size={14} />
        </Button>
        <Button variant="primary" size="sm" onclick={runCommand}>
          <Play size={12} class="mr-1" /> Run
        </Button>
      </div>
    </div>
  </div>
</div>

<style>
  .console-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-app);
  }

  .console-tabs {
    display: flex;
    height: 30px;
    background-color: var(--bg-tab-inactive);
    border-bottom: 1px solid var(--border-subtle);
    padding: 0 4px;
    gap: 1px;
    align-items: flex-end;
  }

  .console-tab {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 12px;
    height: 26px;
    background-color: transparent;
    color: var(--fg-tertiary);
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    border-radius: 4px 4px 0 0;
    position: relative;
    user-select: none;
  }

  .console-tab:hover {
    color: var(--fg-secondary);
    background-color: var(--bg-hover);
  }

  .console-tab.active {
    background-color: var(--bg-app);
    color: var(--fg-primary);
    border: 1px solid var(--border-subtle);
    border-bottom: none;
  }

  .close-tab {
    background: transparent;
    border: none;
    color: inherit;
    opacity: 0.5;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2px;
    border-radius: 2px;
  }

  .close-tab:hover {
    opacity: 1;
    background-color: var(--bg-active);
  }

  .add-tab {
    background: transparent;
    border: none;
    color: var(--fg-tertiary);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    margin-bottom: 4px;
    cursor: pointer;
    border-radius: 4px;
  }

  .add-tab:hover {
    color: var(--fg-primary);
    background-color: var(--bg-hover);
  }

  .console-output {
    flex: 1;
    overflow-y: auto;
    padding: 8px 12px;
    font-family: var(--font-mono);
    font-size: 12px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .selectable {
    user-select: text !important;
    -webkit-user-select: text !important;
  }

  .console-input-bar {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 8px 12px;
    border-top: 1px solid var(--border-subtle);
    background-color: var(--bg-panel);
  }

  .cwd-indicator {
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--fg-tertiary);
    opacity: 0.6;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    padding-left: 30px;
  }

  .input-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .input-inner {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
  }

  :global(.prompt-icon) {
    position: absolute;
    left: 10px;
    color: var(--accent-success);
    z-index: 2;
    margin-right: 8px;
  }

  :global(.cmd-input) {
    padding-left: 30px !important;
    font-family: var(--font-mono) !important;
    height: 28px !important;
    font-size: 12px !important;
    background: var(--bg-input) !important;
  }

  .bar-actions {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .log-entry {
    display: flex;
    gap: 12px;
    line-height: 1.5;
    white-space: pre-wrap;
    word-break: break-all;
  }

  .timestamp {
    color: var(--fg-tertiary);
    min-width: 65px;
    font-variant-numeric: tabular-nums;
  }

  .source {
    color: var(--accent-primary);
    min-width: 80px;
    font-weight: 600;
  }

  .log-entry.info .message {
    color: var(--fg-secondary);
  }
  .log-entry.success .message {
    color: var(--accent-success);
  }
  .log-entry.warn .message {
    color: var(--accent-warning);
  }
  .log-entry.error .message {
    color: var(--accent-error);
  }

  .empty-state {
    color: var(--fg-tertiary);
    font-style: italic;
    padding: 20px;
    text-align: center;
  }

  /* Scrollbar styling */
  .console-output::-webkit-scrollbar {
    width: 6px;
  }
  .console-output::-webkit-scrollbar-thumb {
    background: var(--bg-active);
    border-radius: 3px;
  }
</style>
