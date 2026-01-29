<script lang="ts">
  import { cn } from "$lib/utils";

  let {
    checked = $bindable(false),
    disabled = false,
    class: className = "",
    label = "Toggle",
  } = $props();

  function toggle() {
    if (!disabled) checked = !checked;
  }
</script>

<button
  type="button"
  role="switch"
  aria-checked={checked}
  aria-label={label}
  {disabled}
  class={cn(
    "switch",
    checked ? "checked" : "unchecked",
    disabled && "disabled",
    className,
  )}
  onclick={toggle}
>
  <span class="thumb"></span>
</button>

<style>
  .switch {
    width: 36px;
    height: 20px;
    border-radius: 999px;
    position: relative;
    cursor: pointer;
    border: 1px solid transparent;
    transition: background-color 0.2s;
    padding: 2px;
    background-color: var(--bg-active);
  }

  .switch:focus-visible {
    box-shadow:
      0 0 0 2px var(--bg-app),
      0 0 0 4px var(--accent-primary);
    outline: none;
  }

  .switch.checked {
    background-color: var(--accent-primary, #007acc);
  }

  .switch.unchecked {
    background-color: var(--bg-active);
  }

  .switch.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .thumb {
    display: block;
    width: 14px;
    height: 14px;
    background-color: white;
    border-radius: 50%;
    transition: transform 0.2s;
    transform: translateX(0);
  }

  .switch.checked .thumb {
    transform: translateX(16px);
  }
</style>
