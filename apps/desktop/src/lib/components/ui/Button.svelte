<script lang="ts">
  import { cn } from "$lib/utils";
  import type { HTMLButtonAttributes } from "svelte/elements";

  interface Props extends HTMLButtonAttributes {
    variant?: "default" | "ghost" | "primary" | "destructive" | "outline";
    size?: "default" | "sm" | "icon";
    children: import("svelte").Snippet;
  }

  let {
    variant = "default",
    size = "default",
    class: className = "",
    children,
    ...rest
  }: Props = $props();

  const variants = {
    default: "bg-panel text-primary hover:bg-hover border border-subtle",
    ghost: "bg-transparent hover:bg-hover text-secondary hover:text-primary",
    primary: "bg-accent text-white hover:opacity-90",
    destructive:
      "bg-error/10 text-error hover:bg-error/20 border border-error/50",
    outline: "border border-subtle hover:bg-hover",
  };

  const sizes = {
    default: "h-9 px-4 py-2",
    sm: "h-7 px-3 text-xs",
    icon: "h-9 w-9 p-0 flex items-center justify-center",
  };

  // Setup simple class mapping until we have a real utility for it
  function resolveClass() {
    // Using style attribute injection mostly for now given we don't have full tailwind
    // relying on app.css global classes + inline styles
    return className;
  }
</script>

<button class="btn {variant} {size} {className}" {...rest}>
  {@render children()}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    font-weight: 500;
    transition: all 0.15s ease;
    cursor: pointer;
    border: 1px solid transparent;
    outline: none;
    background: transparent;
    color: var(--fg-primary);
  }

  .btn:focus-visible {
    box-shadow:
      0 0 0 2px var(--bg-app),
      0 0 0 4px var(--accent-primary);
  }

  .btn.default {
    background-color: var(--bg-panel);
    border-color: var(--border-subtle);
  }
  .btn.default:hover {
    background-color: var(--bg-hover);
    border-color: var(--fg-tertiary);
  }

  .btn.ghost {
    color: var(--fg-secondary);
  }
  .btn.ghost:hover {
    background-color: var(--bg-hover);
    color: var(--fg-primary);
  }

  .btn.primary {
    background-color: var(--accent-primary);
    color: #fff;
    border-color: var(--accent-primary);
  }

  .btn.destructive {
    color: var(--accent-error);
    border-color: var(--accent-error);
    background-color: rgba(248, 81, 73, 0.1);
  }
  .btn.destructive:hover {
    background-color: rgba(248, 81, 73, 0.2);
  }

  .btn.sm {
    height: 28px;
    padding: 0 12px;
    font-size: var(--text-xs);
  }

  .btn.icon {
    width: 32px;
    height: 32px;
    padding: 0;
  }
</style>
