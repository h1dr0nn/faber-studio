<script lang="ts">
  let { text = "", children, position = "top" } = $props();
  let isVisible = $state(false);

  function show() {
    isVisible = true;
  }
  function hide() {
    isVisible = false;
  }
</script>

<div
  class="tooltip-trigger"
  onmouseenter={show}
  onmouseleave={hide}
  role="tooltip"
>
  {@render children()}

  {#if isVisible}
    <div class="tooltip {position}">
      {text}
    </div>
  {/if}
</div>

<style>
  .tooltip-trigger {
    position: relative;
    display: inline-block;
  }

  .tooltip {
    position: absolute;
    background-color: var(--bg-hover);
    color: var(--fg-primary);
    padding: 4px 8px;
    border-radius: 4px;
    font-size: var(--text-xs);
    white-space: nowrap;
    z-index: 50;
    border: 1px solid var(--border-subtle);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    animation: fade-in 0.15s ease;
  }

  .tooltip.top {
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%) translateY(-4px);
  }

  .tooltip.bottom {
    top: 100%;
    left: 50%;
    transform: translateX(-50%) translateY(4px);
  }

  @keyframes fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>
