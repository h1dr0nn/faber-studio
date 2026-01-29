<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  interface Props extends HTMLInputAttributes {
    value?: string | number | null;
  }

  let {
    value = $bindable(),
    class: className = "",
    type = "text",
    ...rest
  }: Props = $props();
</script>

<input bind:value {type} class="input {className}" {...rest} />

<style>
  .input {
    background-color: var(--bg-input);
    color: var(--fg-primary);
    border: 1px solid var(--bg-input);
    border-radius: 2px;
    padding: 6px 8px;
    font-family: inherit;
    font-size: 13px;
    outline: none;
    width: 100%;
    transition: border-color 0.1s;
  }

  .input:focus {
    border-color: var(--accent-focus-border);
  }

  .input:hover {
    border-color: var(--border-subtle);
  }

  .input::placeholder {
    color: var(--fg-secondary);
    opacity: 0.6;
  }
</style>
