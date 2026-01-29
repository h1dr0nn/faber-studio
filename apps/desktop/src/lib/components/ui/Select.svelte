<script lang="ts">
  import { ChevronDown, Check } from "lucide-svelte";
  import { fade, fly } from "svelte/transition";
  import { onMount } from "svelte";

  let {
    value = $bindable(),
    options = [],
    class: className = "",
    disabled = false,
    placeholder = "Select...",
    searchable = false,
    onchange,
  }: {
    value: any;
    options: { label: string; value: any; disabled?: boolean }[];
    class?: string;
    disabled?: boolean;
    placeholder?: string;
    searchable?: boolean;
    onchange?: (value: any) => void;
  } = $props();

  let isOpen = $state(false);
  let searchQuery = $state("");
  let container: HTMLDivElement;
  let searchInput = $state<HTMLInputElement>();

  function toggle() {
    if (!disabled) {
      isOpen = !isOpen;
      if (isOpen) searchQuery = "";
    }
  }

  function selectOption(optValue: any) {
    if (options.find((o) => o.value === optValue)?.disabled) return;
    value = optValue;
    isOpen = false;
    searchQuery = "";
    if (onchange) onchange(optValue);
  }

  const filteredOptions = $derived(
    searchQuery
      ? options.filter((o) =>
          o.label.toLowerCase().includes(searchQuery.toLowerCase()),
        )
      : options,
  );

  const selectedLabel = $derived(
    options.find((o) => o.value === value)?.label || placeholder,
  );

  // Close when clicking outside
  function handleClickOutside(event: MouseEvent) {
    if (container && !container.contains(event.target as Node)) {
      isOpen = false;
    }
  }

  onMount(() => {
    window.addEventListener("click", handleClickOutside);
    return () => window.removeEventListener("click", handleClickOutside);
  });

  $effect(() => {
    if (isOpen && searchable && searchInput) {
      setTimeout(() => searchInput?.focus(), 50);
    }
  });
</script>

<div
  class="select-container {className} {disabled ? 'disabled' : ''}"
  bind:this={container}
>
  <button
    type="button"
    class="select-trigger"
    class:active={isOpen}
    onclick={toggle}
    {disabled}
  >
    <span class="label">{selectedLabel}</span>
    <div class="icon" class:rotated={isOpen}>
      <ChevronDown size={14} />
    </div>
  </button>

  {#if isOpen}
    <div
      class="dropdown-menu"
      in:fly={{ y: 5, duration: 150 }}
      out:fade={{ duration: 100 }}
    >
      <div class="options-list">
        {#if searchable}
          <div class="search-wrapper">
            <input
              bind:this={searchInput}
              type="text"
              class="search-input"
              placeholder="Search..."
              bind:value={searchQuery}
              onclick={(e) => e.stopPropagation()}
            />
          </div>
        {/if}
        {#each filteredOptions as option}
          <button
            type="button"
            class="option-item"
            class:selected={value === option.value}
            class:disabled={option.disabled}
            onclick={() => selectOption(option.value)}
            disabled={option.disabled}
          >
            <span>{option.label}</span>
            {#if value === option.value}
              <Check size={12} class="check-icon" />
            {/if}
          </button>
        {:else}
          <div class="no-options">No results found</div>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .select-container {
    position: relative;
    width: 100%;
  }

  .select-trigger {
    width: 100%;
    height: 30px;
    box-sizing: border-box;
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 10px;
    cursor: pointer;
    color: var(--fg-primary);
    font-family: inherit;
    font-size: 13px;
    transition: all 0.2s ease;
    outline: none;
    user-select: none;
  }

  .select-trigger:hover:not(:disabled) {
    background-color: var(--bg-hover);
    border-color: var(--fg-secondary);
  }

  .select-trigger.active {
    border-color: var(--accent-primary);
    box-shadow: 0 0 0 1px var(--accent-primary);
  }

  .label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .icon {
    display: flex;
    align-items: center;
    color: var(--fg-primary);
    transition: transform 0.2s ease;
    margin-left: 8px;
    flex-shrink: 0;
    opacity: 0.8;
  }

  .icon.rotated {
    transform: rotate(180deg);
  }

  .dropdown-menu {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    right: 0;
    background-color: var(--bg-dropdown, var(--bg-active));
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.5);
    z-index: 1000;
    max-height: 250px;
    overflow-y: auto;
    padding: 4px;
  }

  .options-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .option-item {
    width: 100%;
    height: 28px;
    background: transparent;
    border: none;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px;
    color: var(--fg-secondary);
    font-size: 13px;
    cursor: pointer;
    text-align: left;
    transition: all 0.1s ease;
  }

  .option-item:hover {
    background-color: var(--accent-primary);
    color: white;
  }

  .option-item.selected {
    color: var(--fg-primary);
    font-weight: 500;
    background-color: var(--bg-hover);
  }

  .option-item.selected:hover {
    color: white;
  }

  .check-icon {
    flex-shrink: 0;
    margin-left: 8px;
  }

  .search-wrapper {
    padding: 4px;
    border-bottom: 1px solid var(--border-subtle);
    margin-bottom: 4px;
  }

  .search-input {
    width: 100%;
    height: 24px;
    background-color: var(--bg-hover);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    padding: 0 8px;
    font-size: 11px;
    color: var(--fg-primary);
    outline: none;
  }

  .search-input:focus {
    border-color: var(--accent-primary);
  }

  .no-options {
    padding: 8px;
    font-size: 11px;
    color: var(--fg-secondary);
    text-align: center;
  }

  .disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  /* Custom Scrollbar for dropdown */
  .dropdown-menu::-webkit-scrollbar {
    width: 4px;
  }
  .dropdown-menu::-webkit-scrollbar-track {
    background: transparent;
  }
  .dropdown-menu::-webkit-scrollbar-thumb {
    background: var(--border-subtle);
    border-radius: 10px;
  }
</style>
