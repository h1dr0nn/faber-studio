<script lang="ts">
  import {
    Play,
    ChevronDown,
    ChevronRight,
    Settings,
    MoreHorizontal,
    Bug,
    Shield,
  } from "lucide-svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import TreeItem from "$lib/components/ui/TreeItem.svelte";
  import { uiState } from "$lib/ui-state.svelte";

  let sections = $state({
    variables: true,
    watch: true,
    callStack: true,
    breakpoints: true,
  });
</script>

<div class="feature-container">
  <div class="debug-header">
    <div class="header-actions">
      <span>RUN AND DEBUG</span>
      <div class="actions"></div>
    </div>

    <div class="launch-box">
      <Button
        variant="primary"
        size="sm"
        class="w-full launch-btn"
        onclick={() => {
          uiState.activeBottomPanelTab = "matrix";
          uiState.isBottomPanelVisible = true;
          uiState.runTask(
            "echo 'Starting Debug Session... (Authentication required for real debugging)'",
          );
        }}
      >
        <Play size={14} fill="currentColor" />
        Run and Debug
      </Button>
      <div class="config-text">
        To customize Run and Debug, <button
          class="link-btn"
          onclick={() => {
            if (uiState.projectRoot) {
              uiState.createFile(
                `${uiState.projectRoot}/.vscode`,
                "launch.json",
              );
              setTimeout(
                () =>
                  uiState.openFile(
                    `${uiState.projectRoot}/.vscode/launch.json`,
                    "launch.json",
                  ),
                100,
              );
            }
          }}>create a launch.json file</button
        >.
      </div>
    </div>
  </div>

  <div class="debug-content">
    <!-- Variables Section -->
    <div class="debug-section">
      <div
        class="section-header"
        onclick={() => (sections.variables = !sections.variables)}
        role="button"
        tabindex="0"
        onkeydown={(e) =>
          e.key === "Enter" && (sections.variables = !sections.variables)}
      >
        <TreeItem
          label="VARIABLES"
          isFolder={true}
          bind:expanded={sections.variables}
        />
      </div>
      {#if sections.variables}
        <div class="section-content empty">Not paused on a breakpoint</div>
      {/if}
    </div>

    <!-- Watch Section -->
    <div class="debug-section">
      <div
        class="section-header"
        onclick={() => (sections.watch = !sections.watch)}
        role="button"
        tabindex="0"
        onkeydown={(e) =>
          e.key === "Enter" && (sections.watch = !sections.watch)}
      >
        <TreeItem
          label="WATCH"
          isFolder={true}
          bind:expanded={sections.watch}
        />
      </div>
      {#if sections.watch}
        <div class="section-content empty">No expressions to watch</div>
      {/if}
    </div>

    <!-- Call Stack Section -->
    <div class="debug-section">
      <div
        class="section-header"
        onclick={() => (sections.callStack = !sections.callStack)}
        role="button"
        tabindex="0"
        onkeydown={(e) =>
          e.key === "Enter" && (sections.callStack = !sections.callStack)}
      >
        <TreeItem
          label="CALL STACK"
          isFolder={true}
          bind:expanded={sections.callStack}
        />
      </div>
      {#if sections.callStack}
        <div class="section-content empty">Not running</div>
      {/if}
    </div>

    <!-- Breakpoints Section -->
    <div class="debug-section">
      <div
        class="section-header"
        onclick={() => (sections.breakpoints = !sections.breakpoints)}
        role="button"
        tabindex="0"
        onkeydown={(e) =>
          e.key === "Enter" && (sections.breakpoints = !sections.breakpoints)}
      >
        <TreeItem
          label="BREAKPOINTS"
          isFolder={true}
          bind:expanded={sections.breakpoints}
        />
      </div>
      {#if sections.breakpoints}
        <div class="section-content">
          <div class="breakpoint-item">
            <input type="checkbox" checked />
            <span class="bp-icon"
              ><Shield size={10} fill="#c74e61" color="#c74e61" /></span
            >
            <span class="bp-label">Uncaught Exceptions</span>
          </div>
          <div class="breakpoint-item">
            <input type="checkbox" />
            <span class="bp-icon"
              ><Shield size={10} fill="#555" color="#555" /></span
            >
            <span class="bp-label">All Exceptions</span>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .feature-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    color: var(--fg-secondary);
  }

  .debug-header {
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .header-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 11px;
    font-weight: 700;
    opacity: 0.8;
  }

  .icon-btn {
    background: none;
    border: none;
    color: var(--fg-tertiary);
    cursor: pointer;
    padding: 2px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon-btn:hover {
    background-color: var(--bg-hover);
    color: var(--fg-secondary);
  }

  .launch-box {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  :global(.launch-btn) {
    gap: 8px !important;
  }

  .config-text {
    font-size: 11px;
    line-height: 1.4;
    opacity: 0.7;
  }

  .link-btn {
    background: none;
    border: none;
    color: var(--accent-primary);
    padding: 0;
    font: inherit;
    cursor: pointer;
    text-decoration: none;
  }

  .link-btn:hover {
    text-decoration: underline;
  }

  .debug-content {
    flex: 1;
    overflow-y: auto;
  }

  .debug-section {
    display: flex;
    flex-direction: column;
  }

  .section-header {
    display: flex;
    align-items: center;
    cursor: pointer;
  }

  .section-content {
    padding: 4px 0;
  }

  .section-content.empty {
    padding: 8px 24px;
    font-size: 11px;
    opacity: 0.5;
    font-style: italic;
  }

  .breakpoint-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 12px 4px 24px;
    cursor: pointer;
    font-size: 12px;
  }

  .breakpoint-item:hover {
    background-color: var(--bg-hover);
  }

  .breakpoint-item input {
    margin: 0;
  }

  .bp-icon {
    display: flex;
    align-items: center;
  }

  .bp-label {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
