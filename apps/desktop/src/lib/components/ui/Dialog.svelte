<script lang="ts">
  import { cn } from "$lib/utils";
  import Button from "./Button.svelte";
  import Panel from "./Panel.svelte";
  import { X } from "lucide-svelte";

  let {
    open = $bindable(false),
    title = "",
    description = "",
    children,
    onConfirm,
    confirmText = "Confirm",
    cancelText = "Cancel",
    variant = "default", // default | destructive
  } = $props();

  function close() {
    open = false;
  }

  function handleConfirm() {
    if (onConfirm) onConfirm();
    close();
  }
</script>

{#if open}
  <div class="dialog-overlay" onclick={close} role="presentation">
    <div
      class="dialog-content"
      onclick={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
    >
      <div class="dialog-header">
        <h3 class="dialog-title">{title}</h3>
        <button class="close-btn" onclick={close}>
          <X size={16} />
        </button>
      </div>

      {#if description}
        <p class="dialog-desc">{description}</p>
      {/if}

      <div class="dialog-body">
        {@render children?.()}
      </div>

      <div class="dialog-footer">
        <Button variant="ghost" onclick={close}>{cancelText}</Button>
        <Button
          variant={variant === "destructive" ? "destructive" : "primary"}
          onclick={handleConfirm}
        >
          {confirmText}
        </Button>
      </div>
    </div>
  </div>
{/if}

<style>
  .dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(2px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    animation: fade-in 0.2s ease-out;
  }

  .dialog-content {
    background-color: var(--bg-panel);
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    width: 100%;
    max-width: 450px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    animation: scale-in 0.2s ease-out;
    display: flex;
    flex-direction: column;
  }

  .dialog-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .dialog-title {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--fg-secondary);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .close-btn:hover {
    background-color: var(--bg-hover);
    color: var(--fg-primary);
  }

  .dialog-desc {
    padding: 16px 16px 0 16px;
    font-size: var(--text-sm);
    color: var(--fg-secondary);
    margin: 0;
  }

  .dialog-body {
    padding: 16px;
  }

  .dialog-footer {
    padding: 16px;
    border-top: 1px solid var(--border-subtle);
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    background-color: var(--bg-app);
    border-bottom-left-radius: 8px;
    border-bottom-right-radius: 8px;
  }

  @keyframes fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes scale-in {
    from {
      transform: scale(0.95);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }
</style>
