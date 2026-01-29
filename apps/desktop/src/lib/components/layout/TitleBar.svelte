<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    Minus,
    Square,
    X,
    Settings,
    PanelLeft,
    PanelRight,
    PanelBottom,
  } from "lucide-svelte";
  import { uiState } from "$lib/ui-state.svelte";

  interface MenuItem {
    label?: string;
    sub?: MenuItem[];
    onclick?: () => void;
    separator?: boolean;
  }

  interface Props {}
  let {}: Props = $props();

  let appWindow: any;

  onMount(async () => {
    if (typeof window !== "undefined") {
      try {
        const { getCurrentWindow } = await import("@tauri-apps/api/window");
        appWindow = getCurrentWindow();
      } catch (e) {
        console.error("Failed to load Tauri window API", e);
      }
    }
  });

  function minimize() {
    console.log("Minimizing window...");
    appWindow
      ?.minimize()
      .catch((e: any) => console.error("Failed to minimize", e));
  }

  function toggleMaximize() {
    console.log("Toggling maximize...");
    appWindow
      ?.toggleMaximize()
      .catch((e: any) => console.error("Failed to maximize", e));
  }

  function close() {
    console.log("Closing window...");
    appWindow?.close().catch((e: any) => console.error("Failed to close", e));
  }

  // Updated menu items with actions
  let menuItems: MenuItem[] = [
    {
      label: "File",
      sub: [
        { label: "New File", onclick: () => {} },
        { label: "Open File", onclick: () => {} },
        { label: "Open Folder", onclick: () => uiState.openFolder() },
        { separator: true },
        { label: "Save", onclick: () => {} },
        { label: "Exit", onclick: () => close() },
      ],
    },
    {
      label: "Edit",
      sub: [
        { label: "Undo", onclick: () => {} },
        { label: "Redo", onclick: () => {} },
      ],
    },
    {
      label: "View",
      sub: [
        { label: "Appearance", onclick: () => {} },
        { separator: true },
        { label: "Settings", onclick: () => uiState.openSettings() },
      ],
    },
    { label: "Run", sub: [{ label: "Start", onclick: () => {} }] },
    { label: "Help", sub: [{ label: "About", onclick: () => {} }] },
  ];

  let activeMenu: string | null = $state(null);

  function toggleMenu(label: string) {
    if (activeMenu === label) {
      activeMenu = null;
    } else {
      activeMenu = label;
    }
  }

  function handleOutsideClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest(".menubar")) {
      activeMenu = null;
    }
  }

  function handleMouseEnter(label: string) {
    if (activeMenu) {
      activeMenu = label;
    }
  }

  async function startDrag(e: MouseEvent) {
    // Ignore if clicking interactive elements
    const target = e.target as HTMLElement;
    if (
      target.closest(
        'button, [role="button"], .menu-item, .dropdown, .control-btn, .win-btn',
      )
    ) {
      return;
    }

    try {
      await appWindow?.startDragging();
    } catch (e) {
      console.error("Failed to start dragging", e);
    }
  }

  onMount(() => {
    document.addEventListener("click", handleOutsideClick);
    return () => document.removeEventListener("click", handleOutsideClick);
  });
</script>

<header class="title-bar" onmousedown={startDrag}>
  <div class="left-section">
    <div class="app-icon">
      <div class="icon-placeholder"></div>
    </div>

    <nav class="menubar">
      {#each menuItems as item}
        <div class="menu-container">
          <div
            class="menu-item {activeMenu === item.label ? 'active' : ''}"
            onclick={(e) => {
              e.stopPropagation();
              toggleMenu(item.label || "");
            }}
            onmouseenter={() => handleMouseEnter(item.label || "")}
            role="button"
            tabindex="0"
            onkeydown={(e) => e.key === "Enter" && toggleMenu(item.label || "")}
          >
            {item.label}
          </div>

          {#if activeMenu === item.label}
            <div class="dropdown">
              {#each item.sub as subItem}
                {#if subItem.separator}
                  <div class="separator"></div>
                {:else}
                  <div
                    class="dropdown-item"
                    onclick={(e) => {
                      e.stopPropagation();
                      subItem.onclick?.();
                      activeMenu = null;
                    }}
                    role="button"
                    tabindex="0"
                    onkeydown={(e) =>
                      e.key === "Enter" &&
                      (subItem.onclick?.(), (activeMenu = null))}
                  >
                    {subItem.label}
                  </div>
                {/if}
              {/each}
            </div>
          {/if}
        </div>
      {/each}
    </nav>
  </div>

  <div class="center-section">
    <!-- Clean center section as requested -->
    <span class="app-title">Faber Studio</span>
  </div>

  <div class="right-section">
    <div class="layout-controls">
      <button
        class="control-btn {uiState.isSidePanelVisible ? 'active' : ''}"
        onclick={() => uiState.toggleSidePanel()}
        title="Toggle Primary Side Bar (Ctrl+B)"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
          <path d="M9 3v18" />
          {#if uiState.isSidePanelVisible}
            <path
              d="M3 5 a2 2 0 0 1 2 -2 h4 v18 h-4 a2 2 0 0 1 -2 -2 z"
              fill="currentColor"
              stroke="none"
            />
          {/if}
        </svg>
      </button>

      <button
        class="control-btn {uiState.isBottomPanelVisible ? 'active' : ''}"
        onclick={() => uiState.toggleBottomPanel()}
        title="Toggle Panel (Ctrl+J)"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
          <path d="M3 15h18" />
          {#if uiState.isBottomPanelVisible}
            <path
              d="M3 15 h18 v4 a2 2 0 0 1 -2 2 h-14 a2 2 0 0 1 -2 -2 z"
              fill="currentColor"
              stroke="none"
            />
          {/if}
        </svg>
      </button>

      <button
        class="control-btn {uiState.isRightPanelVisible ? 'active' : ''}"
        onclick={() => uiState.toggleRightPanel()}
        title="Toggle Secondary Side Bar"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <rect width="18" height="18" x="3" y="3" rx="2" ry="2" />
          <path d="M15 3v18" />
          {#if uiState.isRightPanelVisible}
            <path
              d="M15 3 h4 a2 2 0 0 1 2 2 v14 a2 2 0 0 1 -2 2 h-4 z"
              fill="currentColor"
              stroke="none"
            />
          {/if}
        </svg>
      </button>
    </div>

    <div class="window-controls">
      <button class="win-btn" onclick={minimize} title="Minimize">
        <Minus size={14} />
      </button>
      <button class="win-btn" onclick={toggleMaximize} title="Maximize">
        <Square size={12} />
      </button>
      <button class="win-btn close" onclick={close} title="Close">
        <X size={14} />
      </button>
    </div>
  </div>
</header>

<style>
  .title-bar {
    height: 35px; /* Slightly taller for standard feel */
    background-color: var(--bg-title-bar, #181818);
    display: flex;
    justify-content: space-between;
    align-items: center;
    user-select: none;
    font-size: 12px; /* Standard IDE font size */
    color: var(--fg-secondary);
    border-bottom: 1px solid var(--border-subtle);
    position: relative;
    z-index: 100;
  }

  /* drag-region removed as it's now on header */

  .left-section,
  .center-section,
  .right-section {
    display: flex;
    align-items: center;
    height: 100%;
    position: relative;
    z-index: 1; /* Place above drag region */
    pointer-events: none; /* Let clicks pass through */
  }

  .left-section {
    padding-left: 8px;
  }

  .icon-placeholder {
    width: 16px;
    height: 16px;
    background-color: var(--accent-primary);
    margin-right: 12px;
    /* icon placeholder should be draggable, so let it be none/through */
  }

  .menubar {
    display: flex;
    height: 100%;
    pointer-events: none; /* Container itself shouldn't capture clicks */
  }

  .menu-container {
    position: relative;
    height: 100%;
    display: flex;
    align-items: center;
    pointer-events: none;
  }

  .menu-item {
    padding: 0 8px;
    cursor: pointer;
    color: var(--fg-secondary);
    display: flex;
    align-items: center;
    height: 100%;
    transition: background-color 0.1s;
    font-size: 13px;
    pointer-events: auto; /* Buttons are clickable */
  }

  .menu-item:hover,
  .menu-item.active {
    background-color: var(--bg-hover);
    color: var(--fg-primary);
  }

  .dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    min-width: 200px;
    background-color: var(--bg-panel); /* Assuming bg-panel is a solid color */
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
    padding: 4px 0;
    z-index: 200;
    pointer-events: auto; /* Dropdown content must be interactive */
  }

  .dropdown-item {
    padding: 6px 12px;
    cursor: pointer;
    font-size: 13px;
    color: var(--fg-primary);
  }

  .dropdown-item:hover {
    background-color: var(--bg-selection);
    color: white;
  }

  .separator {
    height: 1px;
    background-color: var(--border-subtle);
    margin: 4px 0;
  }

  .center-section {
    flex: 1;
    justify-content: center;
    pointer-events: none;
  }

  .app-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--fg-secondary);
    pointer-events: none; /* Make title text simple drag handle */
  }

  .right-section {
    padding-right: 0;
  }

  .layout-controls {
    display: flex;
    margin-right: 12px;
    pointer-events: none;
  }

  .control-btn {
    background: transparent;
    border: none;
    color: inherit;
    padding: 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    transition: all 0.1s;
    border-radius: 4px;
    margin: 0 1px;
    pointer-events: auto; /* Buttons are clickable */
  }

  .control-btn:hover {
    color: var(--fg-primary);
    background-color: var(--bg-hover);
  }

  .control-btn.active {
    color: var(--fg-primary); /* Use primary foreground for active state */
    background-color: var(--bg-hover); /* Keep humble background like hover */
  }

  .window-controls {
    display: flex;
    height: 100%;
    pointer-events: none;
  }

  .win-btn {
    background: transparent;
    border: none;
    color: var(--fg-secondary);
    width: 46px;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: default;
    transition:
      background-color 0.1s,
      color 0.1s;
    pointer-events: auto; /* Buttons are clickable */
  }

  .win-btn:hover {
    background-color: rgba(255, 255, 255, 0.1);
    color: var(--fg-primary);
  }

  .win-btn.close:hover {
    background-color: #e81123;
    color: white;
  }

  .win-btn:active {
    background-color: rgba(255, 255, 255, 0.2);
  }
</style>
