<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    class?: string;
    variant?: "default" | "flat" | "bordered";
    title?: string;
    children: Snippet;
    action?: Snippet;
    [key: string]: any;
  }

  let {
    class: className = "",
    variant = "default",
    title = "",
    children,
    action = undefined,
    ...rest
  }: Props = $props();
</script>

<div class="panel {variant} {className}" {...rest}>
  {#if title}
    <div class="panel-header">
      <span class="panel-title">{title}</span>
      {#if action}
        {@render action()}
      {/if}
    </div>
  {/if}
  <div class="panel-content">
    {@render children()}
  </div>
</div>

<style>
  .panel {
    background-color: var(--bg-panel);
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .panel.default {
    background-color: var(--bg-panel);
  }

  .panel.flat {
    background-color: transparent;
    border: none;
  }

  .panel.bordered {
    background-color: transparent;
    border: 1px solid var(--border-subtle);
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 12px;
    background-color: var(--bg-panel);
    border-bottom: 1px solid var(--border-subtle);
  }

  .panel-title {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--fg-primary);
  }

  .panel-content {
    padding: 12px;
    flex: 1;
    overflow: auto;
  }
</style>
