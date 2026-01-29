<script lang="ts">
  import {
    Stethoscope as StethoscopeIcon,
    Box as BoxIcon,
    Activity as ActivityIcon,
    Smartphone as SmartphoneIcon,
    MessageSquare as MessageSquareIcon,
    Download as DownloadIcon,
  } from "lucide-svelte";
  import { uiState } from "$lib/ui-state.svelte";

  let tools = [
    { id: "doctor", icon: StethoscopeIcon, label: "Doctor" },
    { id: "init", icon: BoxIcon, label: "Initialize" },
    { id: "assets", icon: ActivityIcon, label: "Assets" },
    { id: "mobile", icon: SmartphoneIcon, label: "Mobile" },
    { id: "clone", icon: DownloadIcon, label: "Clone" },
    { id: "chat", icon: MessageSquareIcon, label: "AI Chat" },
  ];

  function toggleTool(id: string) {
    console.log(
      "Toggling right tool:",
      id,
      "current:",
      uiState.activeRightActivityId,
      "visible:",
      uiState.isRightPanelVisible,
    );
    if (uiState.activeRightActivityId === id && uiState.isRightPanelVisible) {
      uiState.isRightPanelVisible = false;
    } else {
      uiState.activeRightActivityId = id;
      uiState.isRightPanelVisible = true;
    }
  }
</script>

<aside class="right-activity-bar">
  <div class="top-actions">
    {#each tools as tool}
      <button
        class="tool-item {uiState.activeRightActivityId === tool.id &&
        uiState.isRightPanelVisible
          ? 'active'
          : ''}"
        onclick={() => toggleTool(tool.id)}
        title={tool.label}
      >
        <tool.icon size={24} strokeWidth={1.5} />
      </button>
    {/each}
  </div>

  <div class="bottom-actions">
    <!-- Placeholder for symmetry -->
  </div>
</aside>

<style>
  .right-activity-bar {
    width: 48px;
    background-color: var(--bg-activity-bar);
    border-left: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    height: 100%;
    z-index: 10;
  }

  .top-actions {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .tool-item {
    width: 48px;
    height: 48px;
    display: flex;
    justify-content: center;
    align-items: center;
    color: var(--fg-secondary);
    cursor: pointer;
    background: transparent;
    border: none;
    position: relative;
    opacity: 0.7;
    transition:
      opacity 0.1s,
      color 0.1s;
    outline: none;
  }

  .tool-item:hover {
    color: var(--fg-primary);
    opacity: 1;
  }

  .tool-item.active {
    color: var(--fg-primary);
    opacity: 1;
  }

  .tool-item.active::after {
    content: "";
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    width: 2px;
    background-color: var(--accent-primary);
  }
</style>
