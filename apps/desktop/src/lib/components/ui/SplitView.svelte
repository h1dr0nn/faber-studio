<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    direction?: "horizontal" | "vertical";
    initialSize?: string;
    minSize?: number;
    primary?: Snippet;
    children: Snippet;
  }

  let {
    direction = "horizontal",
    initialSize = "250px",
    minSize = 100,
    primary,
    children,
  }: Props = $props();
</script>

<div class="split-view {direction}">
  <div class="split-pane" style="flex-basis: {initialSize}">
    <!-- Primary Pane (Side Panel) -->
    {@render primary?.()}
  </div>

  <div class="resizer"></div>

  <div class="split-pane flex-1">
    <!-- Secondary Pane (Editor) -->
    {@render children()}
  </div>
</div>

<style>
  .split-view {
    display: flex;
    width: 100%;
    height: 100%;
  }

  .split-view.vertical {
    flex-direction: column;
  }

  .split-pane {
    position: relative;
    overflow: hidden;
  }

  .flex-1 {
    flex: 1;
  }

  .resizer {
    width: 5px; /* Visual width is 1px usually, but hit area larger */
    cursor: col-resize;
    position: relative;
    z-index: 10;
    margin-left: -2px; /* Center over border */
    border-left: 1px solid transparent;
  }

  .resizer:hover {
    border-left-color: var(--accent-primary);
  }
</style>
