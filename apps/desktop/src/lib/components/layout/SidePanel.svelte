<script lang="ts">
  import { MoreHorizontal } from "lucide-svelte";

  interface Props {
    title?: string;
    children: import("svelte").Snippet;
    borderSide?: "left" | "right" | "none";
    width?: string;
  }

  let {
    title = "EXPLORER",
    children,
    borderSide = "right",
    width = "250px",
  }: Props = $props();
</script>

<div
  class="side-panel"
  style="width: {width}; border-{borderSide}: {borderSide === 'none'
    ? 'none'
    : '1px solid var(--border-subtle)'}"
>
  <div class="panel-header">
    <span class="title">{title}</span>
    <div class="actions">
      <button class="icon-btn">
        <MoreHorizontal size={16} />
      </button>
    </div>
  </div>
  <div class="panel-content">
    {@render children()}
  </div>
</div>

<style>
  .side-panel {
    height: 100%;
    background-color: var(--bg-sidebar);
    display: flex;
    flex-direction: column;
    /* transition: width 0.1s; */
  }

  .panel-header {
    height: 35px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px 0 20px;
    color: var(--fg-secondary);
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .title {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .icon-btn {
    background: none;
    border: none;
    color: inherit;
    cursor: pointer;
    display: flex;
    padding: 4px;
    border-radius: 4px;
  }

  .icon-btn:hover {
    background-color: var(--bg-hover);
    color: var(--fg-primary);
  }

  .panel-content {
    flex: 1;
    overflow-y: auto;
  }
</style>
