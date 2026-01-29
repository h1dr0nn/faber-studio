<script lang="ts">
  import { notifications } from "$lib/stores/notifications.svelte";
  import {
    X,
    CheckCircle,
    AlertTriangle,
    Info,
    AlertCircle,
  } from "lucide-svelte";
  import { fly } from "svelte/transition";

  // Icons map
  const icons = {
    info: Info,
    success: CheckCircle,
    warning: AlertTriangle,
    error: AlertCircle,
  };
</script>

<div class="toast-viewport">
  {#each notifications.items as toast (toast.id)}
    {@const Icon = icons[toast.type]}
    <div
      class="toast {toast.type}"
      transition:fly={{ y: 20, duration: 300 }}
      role="alert"
    >
      <Icon size={18} class="toast-icon" />
      <span class="toast-msg">{toast.message}</span>
      <button
        class="toast-close"
        onclick={() => notifications.remove(toast.id)}
      >
        <X size={14} />
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-viewport {
    position: fixed;
    bottom: 24px;
    right: 24px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    z-index: 200;
    width: 350px;
    max-width: 100%;
    pointer-events: none; /* Let clicks pass through gaps */
  }

  .toast {
    pointer-events: auto;
    display: flex;
    align-items: flex-start;
    padding: 12px;
    background-color: var(--bg-panel);
    border: 1px solid var(--border-subtle);
    border-left-width: 4px;
    border-radius: 4px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    color: var(--fg-primary);
  }

  .toast.info {
    border-left-color: var(--accent-primary);
  }
  .toast.success {
    border-left-color: var(--accent-success);
  }
  .toast.warning {
    border-left-color: var(--accent-warning);
  }
  .toast.error {
    border-left-color: var(--accent-error);
  }

  .toast-icon {
    margin-right: 12px;
    flex-shrink: 0;
    margin-top: 2px;
  }

  .toast.info .toast-icon {
    color: var(--accent-primary);
  }
  .toast.success .toast-icon {
    color: var(--accent-success);
  }
  .toast.warning .toast-icon {
    color: var(--accent-warning);
  }
  .toast.error .toast-icon {
    color: var(--accent-error);
  }

  .toast-msg {
    flex: 1;
    font-size: var(--text-sm);
    line-height: 1.4;
  }

  .toast-close {
    background: transparent;
    border: none;
    color: var(--fg-tertiary);
    cursor: pointer;
    margin-left: 8px;
    padding: 2px;
  }

  .toast-close:hover {
    color: var(--fg-primary);
  }
</style>
