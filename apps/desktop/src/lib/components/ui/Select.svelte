<script lang="ts">
  import { ChevronDown } from "lucide-svelte";

  let {
    value = $bindable(),
    options = [],
    class: className = "",
    disabled = false,
    ...rest
  }: {
    value: any;
    options: { label: string; value: any }[];
    class?: string;
    disabled?: boolean;
  } = $props();
</script>

<div class="select-container {className} {disabled ? 'disabled' : ''}">
  <select bind:value {disabled} {...rest} class="select">
    {#each options as option}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
  <div class="icon">
    <ChevronDown size={14} />
  </div>
</div>

<style>
  .select-container {
    position: relative;
    width: 100%;
    background-color: var(--bg-input);
    border: 1px solid var(--bg-input);
    border-radius: 2px;
    height: 25px; /* Compact height */
  }

  .select-container:hover {
    border-color: var(--border-subtle);
  }

  .select-container:focus-within {
    border-color: var(--accent-focus-border);
  }

  .select {
    width: 100%;
    height: 100%;
    background: transparent;
    border: none;
    color: var(--fg-primary);
    font-family: inherit;
    font-size: 13px;
    padding: 0 24px 0 8px;
    appearance: none;
    outline: none;
    cursor: pointer;
  }

  .icon {
    position: absolute;
    right: 6px;
    top: 50%;
    transform: translateY(-50%);
    pointer-events: none;
    display: flex;
    color: var(--fg-secondary);
  }

  .disabled {
    opacity: 0.6;
    pointer-events: none;
  }
</style>
