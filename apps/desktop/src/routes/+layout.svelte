<script lang="ts">
  import "../app.css";
  import ActivityBar from "$lib/components/layout/ActivityBar.svelte";
  import SidePanel from "$lib/components/layout/SidePanel.svelte";
  import StatusBar from "$lib/components/layout/StatusBar.svelte";
  import TitleBar from "$lib/components/layout/TitleBar.svelte";
  import RightActivityBar from "$lib/components/layout/RightActivityBar.svelte";
  import SplitView from "$lib/components/ui/SplitView.svelte";
  import TreeItem from "$lib/components/ui/TreeItem.svelte";
  import Tabs from "$lib/components/ui/Tabs.svelte";
  import Toast from "$lib/components/ui/Toast.svelte";
  import MatrixLog from "$lib/components/features/MatrixLog.svelte";
  import CommandRunner from "$lib/components/features/CommandRunner.svelte";
  import {
    FileCode,
    Search,
    GitBranch,
    Bug,
    Terminal,
    Activity,
    Trash2,
    X,
    Box as BoxIcon,
    Smartphone as SmartphoneIcon,
    Stethoscope as StethoscopeIcon,
  } from "lucide-svelte";
  import { uiState } from "$lib/ui-state.svelte";
  import { preferences } from "$lib/stores/preferences.svelte";
  import { appConsole } from "$lib/stores/console.svelte";
  import { onMount } from "svelte";

  import Doctor from "$lib/components/features/Doctor.svelte";
  import Initialize from "$lib/components/features/Initialize.svelte";
  import AssetManager from "$lib/components/features/AssetManager.svelte";
  import MobileSetup from "$lib/components/features/MobileSetup.svelte";
  import ResizeHandle from "$lib/components/ui/ResizeHandle.svelte";
  import SearchFeature from "$lib/components/features/Search.svelte";
  import SourceControl from "$lib/components/features/SourceControl.svelte";
  import RunAndDebug from "$lib/components/features/RunAndDebug.svelte";
  import AppSettings from "$lib/components/features/AppSettings.svelte";
  import CloneRepo from "$lib/components/features/CloneRepo.svelte";
  import Chat from "$lib/components/features/Chat.svelte";
  import ContextMenu from "$lib/components/ui/ContextMenu.svelte";
  import Editor from "$lib/components/features/Editor.svelte";

  let { children } = $props();
  let activeTab = $state("page");

  onMount(() => {
    uiState.init();
  });

  $effect(() => {
    document.documentElement.setAttribute("data-theme", preferences.theme);
  });

  $effect(() => {
    if (uiState.activeTabId) {
      const activeTab = uiState.openTabs.find(
        (t) => t.id === uiState.activeTabId,
      );
      if (activeTab && activeTab.path) {
        const ext = activeTab.path.split(".").pop()?.toLowerCase();
        let lang = "Plain Text";
        const map: Record<string, string> = {
          js: "JavaScript",
          ts: "TypeScript",
          svelte: "Svelte",
          rs: "Rust",
          json: "JSON",
          css: "CSS",
          html: "HTML",
          md: "Markdown",
          py: "Python",
          go: "Go",
          cpp: "C++",
          c: "C",
          sql: "SQL",
        };
        lang = map[ext || ""] || "Plain Text";
        uiState.setEditorTelemetry(lang);
      }
    }
  });
</script>

<div class="app-shell">
  <TitleBar />

  <div class="main-container">
    <ActivityBar />

    {#if uiState.isSidePanelVisible}
      <div class="side-panel-wrapper" style="width: {uiState.sidebarWidth}px">
        <SidePanel
          title={uiState.activeSidePanelTitle}
          borderSide="right"
          width="100%"
          onmore={(e) => {
            e.preventDefault();
            const items = [];

            if (uiState.activeActivityId === "explorer") {
              items.push(
                {
                  label: "New File...",
                  onclick: () => {
                    const name = prompt("Enter file name:");
                    if (name && uiState.projectRoot)
                      uiState.createFile(uiState.projectRoot, name);
                  },
                },
                {
                  label: "New Folder...",
                  onclick: () => {
                    const name = prompt("Enter folder name:");
                    if (name && uiState.projectRoot)
                      uiState.createFolder(uiState.projectRoot, name);
                  },
                },
                { separator: true },
                {
                  label: "Refresh Explorer",
                  onclick: () => uiState.refreshFileTree(),
                },
                { label: "Collapse All", onclick: () => {} },
              );
            } else if (uiState.activeActivityId === "search") {
              items.push({
                label: "Clear Search Results",
                onclick: () => uiState.clearSearchResults(),
              });
            } else if (uiState.activeActivityId === "git") {
              items.push(
                {
                  label: "Refresh Status",
                  onclick: () => uiState.refreshGitStatus(),
                },
                {
                  label: "Commit Staged",
                  onclick: () => {
                    if (uiState.commitMessage)
                      uiState.commitChanges(uiState.commitMessage);
                    else alert("Please enter a commit message first.");
                  },
                },
                { label: "Push", onclick: () => uiState.pushChanges() },
                { label: "Pull", onclick: () => uiState.pullChanges() },
              );
            } else if (uiState.activeActivityId === "debug") {
              items.push(
                {
                  label: "Debug Console",
                  onclick: () => {
                    uiState.activeBottomPanelTab = "matrix";
                    uiState.isBottomPanelVisible = true;
                  },
                },
                { separator: true },
                {
                  label: "Open launch.json",
                  onclick: () =>
                    uiState.openFile(
                      `${uiState.projectRoot}/.vscode/launch.json`,
                      "launch.json",
                    ),
                },
              );
            }

            if (items.length > 0) {
              uiState.showContextMenu(e.clientX, e.clientY, items);
            }
          }}
        >
          {#if uiState.activeActivityId === "explorer"}
            <div class="explorer-content">
              <div class="explorer-section">
                <span class="section-title">Open Editors</span>
                <div class="tree-root">
                  {#each uiState.openTabs as tab}
                    <TreeItem
                      label={tab.label}
                      depth={0}
                      selected={tab.id === uiState.activeTabId}
                      onclick={() => (uiState.activeTabId = tab.id)}
                      onclose={() => uiState.closeTab(tab.id)}
                      oncontextmenu={(e: MouseEvent) => {
                        e.preventDefault();
                        e.stopPropagation();
                        uiState.showContextMenu(e.clientX, e.clientY, [
                          { label: "Open to the Side", onclick: () => {} },
                          { separator: true },
                          {
                            label: "Cut",
                            shortcut: "Ctrl+X",
                            onclick: () => {},
                          },
                          {
                            label: "Copy",
                            shortcut: "Ctrl+C",
                            onclick: () => {},
                          },
                          { separator: true },
                          { label: "Copy Path", onclick: () => {} },
                          {
                            label: "Reveal in File Explorer",
                            onclick: () => {},
                          },
                          { separator: true },
                          {
                            label: "Rename",
                            shortcut: "F2",
                            onclick: () => {},
                          },
                          {
                            label: "Delete",
                            shortcut: "Del",
                            danger: true,
                            onclick: () => {},
                          },
                        ]);
                      }}
                    />
                  {/each}
                </div>
              </div>

              <div class="explorer-section">
                <span class="section-title"
                  >Project: {uiState.projectRoot?.split(/[\\/]/).pop() ||
                    "No Folder Opened"}</span
                >
                <div class="tree-root" role="none">
                  {#if uiState.fileTree}
                    {@render RenderTree(uiState.fileTree, 0)}
                  {:else}
                    <div class="empty-state">
                      <p>You have not yet opened a folder.</p>
                      <button
                        class="open-btn"
                        onclick={() => uiState.openFolder()}>Open Folder</button
                      >

                      {#if uiState.recentProjects.length > 0}
                        <div class="recent-projects">
                          <span class="recent-title">Recent</span>
                          {#each uiState.recentProjects as project}
                            <button
                              class="recent-item"
                              onclick={() => uiState.setProjectRoot(project)}
                            >
                              <span class="recent-name"
                                >{project.split(/[\\/]/).pop()}</span
                              >
                              <span class="recent-path">{project}</span>
                            </button>
                          {/each}
                        </div>
                      {/if}
                    </div>
                  {/if}
                </div>
              </div>

              {#snippet RenderTree(node: any, depth: number)}
                {#if node && node.children}
                  {#each node.children as child}
                    <TreeItem
                      label={child.name}
                      isFolder={child.is_dir}
                      {depth}
                      onclick={() => {
                        if (!child.is_dir) {
                          uiState.openFile(child.path, child.name);
                        }
                      }}
                      isRenaming={uiState.renamingPath === child.path}
                      onRename={(newName: string | null) => {
                        if (newName && newName !== child.name) {
                          const newPath = child.path.replace(
                            child.name,
                            newName,
                          );
                          uiState.renamePath(child.path, newPath);
                        }
                        uiState.renamingPath = null;
                      }}
                      oncontextmenu={(e: MouseEvent) => {
                        e.preventDefault();
                        e.stopPropagation();
                        uiState.showContextMenu(e.clientX, e.clientY, [
                          {
                            label: "New File",
                            onclick: () => {
                              const name = prompt("Enter file name:");
                              if (name)
                                uiState.createFile(
                                  child.is_dir
                                    ? child.path
                                    : child.path
                                        .split(/[\\/]/)
                                        .slice(0, -1)
                                        .join("/"),
                                  name,
                                );
                            },
                          },
                          {
                            label: "New Folder",
                            onclick: () => {
                              const name = prompt("Enter folder name:");
                              if (name)
                                uiState.createFolder(
                                  child.is_dir
                                    ? child.path
                                    : child.path
                                        .split(/[\\/]/)
                                        .slice(0, -1)
                                        .join("/"),
                                  name,
                                );
                            },
                          },
                          { separator: true },
                          {
                            label: "Rename",
                            shortcut: "F2",
                            onclick: () => {
                              uiState.renamingPath = child.path;
                            },
                          },
                          {
                            label: "Delete",
                            shortcut: "Del",
                            danger: true,
                            onclick: () => {
                              if (
                                confirm(
                                  `Are you sure you want to delete ${child.name}?`,
                                )
                              ) {
                                uiState.deletePath(child.path);
                              }
                            },
                          },
                        ]);
                      }}
                    >
                      {#if child.is_dir && child.children}
                        {@render RenderTree(child, depth + 1)}
                      {/if}
                    </TreeItem>
                  {/each}
                {/if}
              {/snippet}
            </div>
          {:else if uiState.activeActivityId === "search"}
            <SearchFeature />
          {:else if uiState.activeActivityId === "git"}
            <SourceControl />
          {:else if uiState.activeActivityId === "debug"}
            <RunAndDebug />
          {/if}
        </SidePanel>

        <ResizeHandle
          direction="horizontal"
          side="right"
          onresize={(delta: number) => {
            uiState.sidebarWidth = Math.max(
              150,
              Math.min(600, uiState.sidebarWidth + delta),
            );
          }}
        />
      </div>
    {/if}

    <div class="editor-stack">
      <div class="editor-area">
        {#if uiState.openTabs.length > 0}
          <Tabs
            tabs={uiState.openTabs}
            bind:activeTabId={uiState.activeTabId}
            onclose={(id) => uiState.closeTab(id)}
          />
          <main class="editor-content">
            {#each uiState.openTabs as tab}
              {#if tab.id === uiState.activeTabId}
                {#if tab.type === "file"}
                  <div class="code-editor">
                    <Editor {tab} />
                  </div>
                {:else if tab.type === "settings"}
                  <AppSettings />
                {/if}
              {/if}
            {/each}
          </main>
        {:else}
          <div class="editor-placeholder">
            {@render children()}
          </div>
        {/if}
      </div>

      {#if uiState.isBottomPanelVisible}
        <div class="bottom-panel" style="height: {uiState.bottomPanelHeight}px">
          <ResizeHandle
            direction="vertical"
            side="top"
            onresize={(delta: number) => {
              uiState.bottomPanelHeight = Math.max(
                100,
                Math.min(600, uiState.bottomPanelHeight - delta),
              );
            }}
          />
          <div
            class="bottom-panel-header"
            role="none"
            oncontextmenu={(e) => {
              if (e.target !== e.currentTarget) return;
              e.preventDefault();
              uiState.showContextMenu(e.clientX, e.clientY, [
                { label: "Align Panel Left", onclick: () => {} },
                { label: "Align Panel Right", onclick: () => {} },
                {
                  label: "Align Panel Bottom",
                  onclick: () => {},
                  disabled: true,
                },
                { separator: true },
                {
                  label: "Hide Panel",
                  onclick: () => (uiState.isBottomPanelVisible = false),
                },
              ]);
            }}
          >
            <div class="bottom-tabs">
              <button
                class="bottom-tab {uiState.activeBottomPanelTab === 'runner'
                  ? 'active'
                  : ''}"
                onclick={() => (uiState.activeBottomPanelTab = "runner")}
                oncontextmenu={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  uiState.showContextMenu(e.clientX, e.clientY, [
                    { label: "Move to Side Bar", onclick: () => {} },
                    { label: "Close", onclick: () => {} },
                  ]);
                }}
              >
                <Terminal size={14} />
                COMMAND RUNNER
              </button>
              <button
                class="bottom-tab {uiState.activeBottomPanelTab === 'matrix'
                  ? 'active'
                  : ''}"
                onclick={() => (uiState.activeBottomPanelTab = "matrix")}
                oncontextmenu={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  uiState.showContextMenu(e.clientX, e.clientY, [
                    { label: "Move to Side Bar", onclick: () => {} },
                    { label: "Close", onclick: () => {} },
                  ]);
                }}
              >
                <Activity size={14} />
                MATRIX LOG
              </button>
            </div>

            <div class="bottom-actions">
              {#if uiState.activeBottomPanelTab === "matrix"}
                <button
                  class="action-btn"
                  title="Clear Logs"
                  onclick={() => appConsole.clear()}
                >
                  <Trash2 size={14} />
                </button>
              {/if}
              <button
                class="action-btn"
                title="Close Panel"
                onclick={() => (uiState.isBottomPanelVisible = false)}
              >
                <X size={14} />
              </button>
            </div>
          </div>
          <div class="bottom-panel-content">
            {#if uiState.activeBottomPanelTab === "runner"}
              <CommandRunner />
            {:else if uiState.activeBottomPanelTab === "matrix"}
              <MatrixLog />
            {/if}
          </div>
        </div>
      {/if}
    </div>

    {#if uiState.isRightPanelVisible}
      <div
        class="right-panel-wrapper"
        style="width: {uiState.rightSidebarWidth}px"
      >
        <ResizeHandle
          direction="horizontal"
          side="left"
          onresize={(delta: number) => {
            uiState.rightSidebarWidth = Math.max(
              360,
              Math.min(600, uiState.rightSidebarWidth - delta),
            );
          }}
        />
        <SidePanel
          title={uiState.activeRightSidePanelTitle}
          borderSide="left"
          width="100%"
          onmore={(e) => {
            e.preventDefault();
            // Generic options for now as right panel is mostly specific tools
            uiState.showContextMenu(e.clientX, e.clientY, [
              {
                label: "Hide Side Bar",
                onclick: () => uiState.toggleRightPanel(),
              },
              { label: "Move to Left", onclick: () => {} }, // Placeholder
            ]);
          }}
        >
          <div class="right-panel-content">
            {#if uiState.activeRightActivityId === "doctor"}
              <Doctor />
            {:else if uiState.activeRightActivityId === "init"}
              <Initialize />
            {:else if uiState.activeRightActivityId === "assets"}
              <AssetManager />
            {:else if uiState.activeRightActivityId === "mobile"}
              <MobileSetup />
            {:else if uiState.activeRightActivityId === "clone"}
              <CloneRepo />
            {:else if uiState.activeRightActivityId === "chat"}
              <Chat />
            {/if}
          </div>
        </SidePanel>
      </div>
    {/if}

    <RightActivityBar />
  </div>

  <StatusBar />
  <Toast />
</div>

<ContextMenu />

<style>
  .app-shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    background-color: var(--bg-app);
    color: var(--fg-primary);
    overflow: hidden;
  }

  .main-container {
    flex: 1;
    display: flex;
    overflow: hidden;
    width: 100%;
  }

  .editor-stack {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    min-width: 0;
    position: relative;
    background-color: var(--bg-editor);
  }

  .editor-area {
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  .explorer-content {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }

  .empty-state {
    padding: 20px;
    text-align: center;
    color: var(--fg-secondary);
    opacity: 0.8;
  }

  .empty-state p {
    margin-bottom: 12px;
    font-size: 13px;
  }

  .open-btn {
    background-color: var(--accent-primary);
    color: white;
    border: none;
    padding: 6px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 13px;
    width: 100%;
  }

  .open-btn:hover {
    filter: brightness(1.1);
  }

  .recent-projects {
    margin-top: 24px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    text-align: left;
    width: 100%;
  }

  .recent-title {
    font-size: 11px;
    font-weight: 700;
    color: var(--fg-secondary);
    text-transform: uppercase;
    margin-bottom: 8px;
    padding-left: 8px;
  }

  .recent-item {
    display: flex;
    flex-direction: column;
    background: transparent;
    border: none;
    padding: 6px 8px;
    cursor: pointer;
    text-align: left;
    border-radius: 4px;
    color: var(--fg-primary);
  }

  .recent-item:hover {
    background-color: var(--bg-hover);
  }

  .recent-name {
    font-size: 13px;
    color: var(--accent-primary);
  }

  .recent-path {
    font-size: 11px;
    color: var(--fg-secondary);
    opacity: 0.7;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
  }

  .code-editor {
    width: 100%;
    height: 100%;
    background-color: var(--bg-app);
    overflow: hidden;
  }

  .editor-content {
    flex: 1;
    overflow: hidden;
    position: relative;
    padding: 0;
  }

  .side-panel-wrapper {
    height: 100%;
    display: flex;
    overflow: visible;
    position: relative;
  }

  .right-panel-wrapper {
    height: 100%;
    display: flex;
    overflow: visible;
    position: relative;
  }

  .right-panel-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .bottom-panel {
    background-color: var(--bg-panel);
    border-top: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    overflow: visible;
    position: relative;
    z-index: 20;
  }

  .bottom-panel-header {
    height: 35px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px;
    background-color: var(--bg-tab-inactive);
    border-bottom: 1px solid var(--border-subtle);
  }

  .bottom-tabs {
    display: flex;
    height: 100%;
    gap: 4px;
  }

  .bottom-tab {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 12px;
    height: 100%;
    background: transparent;
    border: none;
    color: var(--fg-tertiary);
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    border-top: 1px solid transparent;
  }

  .bottom-tab:hover {
    color: var(--fg-secondary);
  }

  .bottom-tab.active {
    color: var(--fg-primary);
    border-top-color: var(--accent-primary);
    background-color: var(--bg-panel);
  }

  .bottom-actions {
    display: flex;
    gap: 8px;
  }

  .action-btn {
    background: transparent;
    border: none;
    color: var(--fg-tertiary);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .action-btn:hover {
    color: var(--fg-primary);
    background-color: var(--bg-hover);
  }

  .bottom-panel-content {
    flex: 1;
    overflow: hidden;
  }

  .explorer-content {
    padding: 0;
  }

  .explorer-section {
    padding: 8px 0;
  }

  .section-title {
    padding: 0 12px;
    font-size: 11px;
    font-weight: 700;
    color: var(--fg-secondary);
    text-transform: uppercase;
  }

  .tree-root {
    display: flex;
    flex-direction: column;
  }
</style>
