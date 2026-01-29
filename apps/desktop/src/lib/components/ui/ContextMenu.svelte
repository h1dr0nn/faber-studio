<script lang="ts">
  import { uiState } from "$lib/ui-state.svelte";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  let menuElement = $state<HTMLDivElement>();
  let adjustedX = $state(0);
  let adjustedY = $state(0);

  $effect(() => {
    if (uiState.contextMenu.visible && menuElement) {
      const { x, y } = uiState.contextMenu;
      const { innerWidth, innerHeight } = window;
      const rect = menuElement.getBoundingClientRect();

      adjustedX = x + rect.width > innerWidth ? innerWidth - rect.width - 5 : x;
      adjustedY =
        y + rect.height > innerHeight ? innerHeight - rect.height - 5 : y;
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      uiState.closeContextMenu();
    }
  }

  function handleOutsideClick(e: MouseEvent) {
    if (menuElement && !menuElement.contains(e.target as Node)) {
      uiState.closeContextMenu();
    }
  }
</script>

<svelte:window onmousedown={handleOutsideClick} onkeydown={handleKeydown} />

{#if uiState.contextMenu.visible}
  <div
    bind:this={menuElement}
    class="context-menu"
    style="left: {adjustedX}px; top: {adjustedY}px;"
    transition:fade={{ duration: 100 }}
  >
    {#each uiState.contextMenu.items as item}
      {#if item.separator}
        <div class="separator"></div>
      {:else}
        <button
          class="menu-item {item.danger ? 'danger' : ''} {item.disabled
            ? 'disabled'
            : ''}"
          onclick={() => {
            if (!item.disabled) {
              item.onclick?.();
              uiState.closeContextMenu();
            }
          }}
          disabled={item.disabled}
        >
          <div class="item-left">
            {#if item.icon}
              <item.icon size={14} class="icon" />
            {/if}
            <span class="label">{item.label}</span>
          </div>
          {#if item.shortcut}
            <span class="shortcut">{item.shortcut}</span>
          {/if}
        </button>
      {/if}
    {/each}
  </div>
{/if}

<style>
  .context-menu {
    position: fixed;
    z-index: 1000;
    background-color: var(--bg-sidebar);
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    padding: 4px;
    min-width: 160px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(10px);
  }

  .menu-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 4px 8px;
    border: none;
    background: none;
    color: var(--fg-secondary);
    font-size: 12px;
    border-radius: 4px;
    cursor: pointer;
    gap: 12px;
  }

  .menu-item:hover:not(.disabled) {
    background-color: var(--accent-primary);
    color: white !important;
  }

  .menu-item.danger {
    color: #c74e61;
  }

  .menu-item.disabled {
    opacity: 0.4;
    cursor: default;
  }

  .item-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .shortcut {
    font-size: 10px;
    opacity: 0.5;
  }

  .menu-item:hover .shortcut {
    color: white;
    opacity: 0.8;
  }

  .separator {
    height: 1px;
    background-color: var(--border-subtle);
    margin: 4px 6px;
  }

  :global(.icon) {
    opacity: 0.7;
  }

  .menu-item:hover :global(.icon) {
    opacity: 1;
    color: white !important;
  }
</style>
