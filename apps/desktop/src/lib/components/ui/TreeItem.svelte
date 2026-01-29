<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    ChevronRight,
    ChevronDown,
    File,
    Folder,
    FolderOpen,
    X,
  } from "lucide-svelte";

  interface Props {
    label: string;
    depth?: number;
    isFolder?: boolean;
    expanded?: boolean;
    selected?: boolean;
    icon?: any;
    onclick?: () => void;
    children?: import("svelte").Snippet;
    [key: string]: any;
  }

  let {
    label,
    depth = 0,
    isFolder = false,
    expanded = $bindable(false),
    selected = false,
    icon: IconProp = undefined,
    onclick = undefined,
    oncontextmenu = undefined,
    isRenaming = false,
    onRename = undefined,
    onclose = undefined,
    children,
    ...rest
  }: Props & { onclose?: () => void } = $props();

  function toggle() {
    if (isRenaming) return;
    if (isFolder) {
      expanded = !expanded;
    }
    if (onclick) {
      onclick();
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      e.stopPropagation();
      const input = e.target as HTMLInputElement;
      if (onRename && input.value) {
        onRename(input.value);
      }
    } else if (e.key === "Escape") {
      e.preventDefault();
      e.stopPropagation();
      if (onRename) {
        onRename(null); // Cancel
      }
    }
  }

  function handleBlur(e: FocusEvent) {
    const input = e.target as HTMLInputElement;
    if (onRename) {
      // Optional: Commit on blur, or cancel. VS Code confirms on blur if changed.
      // For now let's cancel to be safe or just commit.
      // Let's commit if value is valid.
      if (input.value) onRename(input.value);
      else onRename(null);
    }
  }

  function autofocus(node: HTMLElement) {
    node.focus();
    // Select all text
    if (node instanceof HTMLInputElement) {
      node.select();
    }
  }
</script>

<div class="tree-item-container">
  <div
    class="tree-row {selected ? 'selected' : ''}"
    style="padding-left: {depth * 12 + 8}px"
    onclick={toggle}
    {oncontextmenu}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === "Enter" && toggle()}
  >
    <span class="twistie {isFolder ? 'visible' : ''}">
      {#if isFolder}
        {#if expanded}
          <ChevronDown size={14} />
        {:else}
          <ChevronRight size={14} />
        {/if}
      {/if}
    </span>

    <span class="icon">
      {#if IconProp}
        <IconProp size={14} />
      {:else if isFolder}
        {#if expanded}
          <FolderOpen size={14} color="var(--fg-secondary)" />
        {:else}
          <Folder size={14} color="var(--fg-secondary)" />
        {/if}
      {:else}
        <File size={14} color="var(--fg-secondary)" />
      {/if}
    </span>

    {#if isRenaming}
      <input
        value={label}
        class="rename-input"
        onclick={(e) => e.stopPropagation()}
        onkeydown={handleKeyDown}
        onblur={handleBlur}
        use:autofocus
      />
    {:else}
      <span class="label">{label}</span>
    {/if}

    {#if onclose}
      <button
        class="close-btn"
        onclick={(e) => {
          e.stopPropagation();
          onclose();
        }}
        title="Close"
      >
        <X size={12} />
      </button>
    {/if}
  </div>

  {#if isFolder && expanded}
    <div class="tree-children">
      {@render children?.()}
    </div>
  {/if}
</div>

<style>
  .tree-row {
    display: flex;
    align-items: center;
    height: 22px;
    cursor: pointer;
    font-size: 13px;
    color: var(--fg-secondary);
    user-select: none;
    border: 1px solid transparent; /* Reserve space for border */
  }

  .tree-row:hover {
    background-color: var(--bg-hover);
    color: var(--fg-primary);
  }

  .tree-row.selected {
    background-color: var(--bg-selection);
    color: #fff;
  }

  .tree-row.selected:focus {
    outline: 1px solid var(--accent-focus-border);
    outline-offset: -1px;
  }

  .twistie {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    margin-right: 4px;
    opacity: 0.7;
    visibility: hidden;
  }

  .twistie.visible {
    visibility: visible;
  }

  .icon {
    display: flex;
    align-items: center;
    margin-right: 6px;
  }

  .label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  .close-btn {
    opacity: 0;
    background: transparent;
    border: none;
    color: var(--fg-secondary);
    padding: 2px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 4px;
  }

  .tree-row:hover .close-btn {
    opacity: 1;
  }

  .close-btn:hover {
    background-color: var(--bg-active);
    color: var(--fg-primary);
  }

  .rename-input {
    background: var(--bg-input);
    color: var(--fg-primary);
    border: 1px solid var(--accent-focus-border);
    border-radius: 2px;
    height: 18px;
    font-size: 13px;
    padding: 0 4px;
    outline: none;
    min-width: 0;
    flex: 1;
  }
</style>
