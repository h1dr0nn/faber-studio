<script lang="ts">
  import { appConsole } from "$lib/stores/console.svelte";

  import { fly } from "svelte/transition";
  import { cn } from "$lib/utils";
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";

  interface Props {
    class?: string;
  }

  let { class: className }: Props = $props();

  let unlisten: any[] = [];

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
</script>

<div class={cn("matrix-panel", className)}>
  <div class="matrix-content">
    {#each appConsole.entries as entry (entry.id)}
      <div
        class={cn("log-entry", entry.level)}
        transition:fly={{ x: -10, duration: 100 }}
      >
        <span class="timestamp">{entry.timestamp.toLocaleTimeString()}</span>
        <span class="source">[{entry.source}]</span>
        <span class="message">{entry.message}</span>
      </div>
    {/each}
    {#if appConsole.entries.length === 0}
      <div class="empty-log">System Ready. Awaiting signals...</div>
    {/if}
  </div>
</div>

<style>
  .matrix-panel {
    height: 100%;
    background-color: transparent;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .matrix-content {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
    font-family: "JetBrains Mono", monospace;
    font-size: 11px;
    display: flex;
    flex-direction: column-reverse; /* Pin to bottom/newest */
    gap: 2px;
  }

  .log-entry {
    display: flex;
    gap: 8px;
    line-height: 1.4;
  }

  .timestamp {
    color: var(--fg-tertiary);
    min-width: 60px;
  }
  .source {
    color: var(--accent-primary);
    min-width: 80px;
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

  .empty-log {
    color: var(--fg-tertiary);
    font-style: italic;
    padding: 8px;
    text-align: center;
  }
</style>
