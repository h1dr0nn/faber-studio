<script lang="ts">
  import { uiState } from "$lib/ui-state.svelte";
  import { X } from "lucide-svelte";

  /* 
    Simple Tabs component.
    In a real app, this would likely interact with a central "Editor Service" 
    store to manage open files. For now, it manages its own state via props. 
  */

  interface Props {
    tabs?: { id: string; label: string; icon?: any; isDirty?: boolean }[];
    activeTabId?: string | null;
    onclose?: (id: string) => void;
    onselect?: (id: string) => void;
  }

  let {
    tabs = [],
    activeTabId = $bindable(null),
    onclose = undefined,
    onselect = undefined,
  }: Props = $props();

  function handleSelect(id: string) {
    activeTabId = id;
    if (onselect) onselect(id);
  }

  function handleClose(e: Event, id: string) {
    e.stopPropagation();
    if (onclose) onclose(id);
  }
</script>

<div
  class="tabs-container"
  role="none"
  oncontextmenu={(e) => {
    if (e.target !== e.currentTarget) return;
    e.preventDefault();
    uiState.showContextMenu(e.clientX, e.clientY, [
      { label: "New File", onclick: () => {} },
      { label: "Close All", onclick: () => {} },
    ]);
  }}
>
  <div class="tabs-scroll-area">
    {#each tabs as tab (tab.id)}
      <div
        class="tab {activeTabId === tab.id ? 'active' : ''}"
        onclick={() => handleSelect(tab.id)}
        oncontextmenu={(e) => {
          e.preventDefault();
          e.stopPropagation();
          uiState.showContextMenu(e.clientX, e.clientY, [
            { label: "Close", onclick: () => handleClose(e, tab.id) },
            { label: "Close Others", onclick: () => {} },
            { label: "Close to the Right", onclick: () => {} },
            { label: "Close Saved", onclick: () => {} },
            { separator: true },
            { label: "Pin Tab", onclick: () => {} },
            { separator: true },
            { label: "Split Right", onclick: () => {} },
            { label: "Split Down", onclick: () => {} },
          ]);
        }}
        role="button"
        tabindex="0"
        onkeydown={(e) => e.key === "Enter" && handleSelect(tab.id)}
      >
        <div class="tab-label-area">
          {#if tab.icon}
            {@const TabIcon = tab.icon}
            <span class="tab-icon">
              <TabIcon size={13} />
            </span>
          {/if}
          <span class="tab-label {tab.isDirty ? 'dirty' : ''}">{tab.label}</span
          >
        </div>

        <button
          class="tab-close {tab.isDirty ? 'dirty-icon' : ''}"
          onclick={(e) => handleClose(e, tab.id)}
          title="Close"
        >
          {#if tab.isDirty}
            <div class="circle-icon"></div>
          {:else}
            <X size={12} />
          {/if}
        </button>
      </div>
    {/each}
  </div>
</div>

<style>
  .tabs-container {
    height: 35px;
    background-color: var(--bg-app);
    display: flex;
    overflow-x: auto;
    width: 100%;
  }

  /* Hide scrollbar but keep functionality */
  .tabs-container::-webkit-scrollbar {
    height: 3px;
  }
  .tabs-container::-webkit-scrollbar-thumb {
    background: transparent;
  }
  .tabs-container:hover::-webkit-scrollbar-thumb {
    background: var(--bg-active);
  }

  .tabs-scroll-area {
    display: flex;
  }

  .tab {
    display: flex;
    align-items: center;
    background-color: var(--tab-inactive-bg);
    border-right: 1px solid var(--tab-border);
    padding: 0 10px;
    min-width: 120px;
    max-width: 200px;
    height: 100%;
    cursor: pointer;
    font-size: 13px;
    color: var(--fg-secondary);
    user-select: none;
    position: relative;
  }

  .tab:hover {
    background-color: var(--bg-hover);
  }

  .tab.active {
    background-color: var(--tab-active-bg);
    color: var(--fg-primary);
    border-top: 1px solid var(--accent-primary);
  }

  .tab-icon {
    margin-right: 6px;
    display: flex;
  }

  .tab-label-area {
    display: flex;
    align-items: center;
    flex: 1;
    overflow: hidden;
  }

  .tab-label {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
  }

  /* .tab-label.dirty { color: var(--accent-warning); } */

  .tab-close {
    margin-left: 8px;
    border-radius: 4px;
    padding: 2px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: inherit;
    opacity: 0; /* Hidden by default similar to VS Code */
    width: 18px;
    height: 18px;
  }

  .tab:hover .tab-close,
  .tab.active .tab-close {
    opacity: 1;
  }

  .tab-close:hover {
    background-color: rgba(255, 255, 255, 0.1);
    color: var(--fg-primary);
  }

  .circle-icon {
    width: 8px;
    height: 8px;
    background-color: var(--fg-secondary);
    border-radius: 50%;
  }

  .tab.active .circle-icon {
    background-color: #fff;
  }
</style>
