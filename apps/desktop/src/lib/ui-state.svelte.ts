import { invoke } from "@tauri-apps/api/core";

import { open } from "@tauri-apps/plugin-dialog";

export function createUIState() {
  let activeActivityId = $state("explorer");
  let activeRightActivityId = $state("doctor");
  let isSidePanelVisible = $state(true);
  let isBottomPanelVisible = $state(false);
  let isRightPanelVisible = $state(false);
  let activeSidePanelTitle = $state("EXPLORER");
  let activeRightSidePanelTitle = $state("DOCTOR");
  let activeBottomPanelTab = $state("runner");

  let sidebarWidth = $state(250);
  let rightSidebarWidth = $state(250);
  let bottomPanelHeight = $state(200);

  let contextMenu = $state({
    x: 0,
    y: 0,
    visible: false,
    items: [] as any[]
  });

  let renamingPath = $state<string | null>(null);

  let projectRoot = $state<string | null>(null);

  let fileTree = $state<any>(null);

  let openTabs = $state<{ id: string; label: string; path: string; content: string; type?: "file" | "settings" | "welcome" }[]>([]);
  let activeTabId = $state<string | null>(null);
  let searchResults = $state<any[]>([]);
  let gitChanges = $state<{ staged: any[]; unstaged: any[] }>({ staged: [], unstaged: [] });
  let activeTaskId = $state<string | null>(null);

  return {


    get activeActivityId() {
      return activeActivityId;
    },
    set activeActivityId(value) {
      activeActivityId = value;
      // Automatically update side panel title based on activity
      activeSidePanelTitle = value.toUpperCase();
    },
    get activeRightActivityId() {
      return activeRightActivityId;
    },
    set activeRightActivityId(value) {
      activeRightActivityId = value;
      activeRightSidePanelTitle = value.toUpperCase();
    },
    get isSidePanelVisible() {
      return isSidePanelVisible;
    },
    set isSidePanelVisible(value) {
      isSidePanelVisible = value;
    },
    get isBottomPanelVisible() {
      return isBottomPanelVisible;
    },
    set isBottomPanelVisible(value) {
      isBottomPanelVisible = value;
    },
    get isRightPanelVisible() {
      return isRightPanelVisible;
    },
    set isRightPanelVisible(value) {
      isRightPanelVisible = value;
    },
    get activeBottomPanelTab() {
      return activeBottomPanelTab;
    },
    set activeBottomPanelTab(value) {
      activeBottomPanelTab = value;
    },
    get activeSidePanelTitle() {
      return activeSidePanelTitle;
    },
    set activeSidePanelTitle(value) {
      activeSidePanelTitle = value;
    },
    get activeRightSidePanelTitle() {
      return activeRightSidePanelTitle;
    },
    set activeRightSidePanelTitle(value) {
      activeRightSidePanelTitle = value;
    },
    get sidebarWidth() {
      return sidebarWidth;
    },
    set sidebarWidth(value) {
      sidebarWidth = value;
    },
    get rightSidebarWidth() {
      return rightSidebarWidth;
    },
    set rightSidebarWidth(value) {
      rightSidebarWidth = value;
    },
    get bottomPanelHeight() {
      return bottomPanelHeight;
    },
    set bottomPanelHeight(value) {
      bottomPanelHeight = value;
    },
    get contextMenu() {
      return contextMenu;
    },
    showContextMenu(x: number, y: number, items: any[]) {
      contextMenu.x = x;
      contextMenu.y = y;
      contextMenu.items = items;
      contextMenu.visible = true;
    },
    closeContextMenu() {
      contextMenu.visible = false;
    },
    get renamingPath() {
      return renamingPath;
    },
    set renamingPath(value) {
      renamingPath = value;
    },
    get projectRoot() {

      return projectRoot;
    },
    get fileTree() {
      return fileTree;
    },
    async openFolder() {
      try {
        const selected = await open({
          directory: true,
          multiple: false,
        });
        if (selected && typeof selected === "string") {
          projectRoot = selected;
          await this.refreshFileTree();
        }
      } catch (err) {
        console.error("Failed to open folder:", err);
      }
    },
    async refreshFileTree() {
      if (!projectRoot) return;
      try {
        fileTree = await invoke("scan_project", { path: projectRoot });
      } catch (err) {
        console.error("Failed to refresh file tree:", err);
      }
    },
    async createFile(parentPath: string, name: string) {
      try {
        const path = `${parentPath}/${name}`;
        await invoke("create_file", { path });
        await this.refreshFileTree();
      } catch (err) {
        console.error("Failed to create file:", err);
      }
    },
    async createFolder(parentPath: string, name: string) {
      try {
        const path = `${parentPath}/${name}`;
        await invoke("create_dir", { path });
        await this.refreshFileTree();
      } catch (err) {
        console.error("Failed to create folder:", err);
      }
    },
    async deletePath(path: string) {
      try {
        await invoke("delete_path", { path });
        // Close tab if open
        const tab = openTabs.find(t => t.path === path);
        if (tab) this.closeTab(tab.id);
        await this.refreshFileTree();
      } catch (err) {
        console.error("Failed to delete path:", err);
      }
    },
    async renamePath(oldPath: string, newPath: string) {
      try {
        await invoke("rename_path", { oldPath, newPath });
        // Update tab if open
        const tab = openTabs.find(t => t.path === oldPath);
        if (tab) {
          tab.path = newPath;
          tab.label = newPath.split(/[\\/]/).pop() || tab.label;
        }
        await this.refreshFileTree();
      } catch (err) {
        console.error("Failed to rename path:", err);
      }
    },
    get openTabs() {
      return openTabs;
    },
    get activeTabId() {
      return activeTabId;
    },
    set activeTabId(value) {
      activeTabId = value;
    },
    async openFile(path: string, name: string) {
      // Check if already open
      const existing = openTabs.find(t => t.path === path);
      if (existing) {
        activeTabId = existing.id;
        return;
      }

      try {
        const content = await invoke("read_file", { path }) as string;
        const newTab = {
          id: Math.random().toString(36).substring(2, 11),
          label: name,
          path,
          content,
          type: "file" as const
        };
        openTabs.push(newTab);
        activeTabId = newTab.id;
      } catch (err) {
        console.error("Failed to read file:", err);
      }
    },
    openSettings() {
      const existing = openTabs.find(t => t.id === "settings-tab");
      if (existing) {
        activeTabId = "settings-tab";
        return;
      }

      const settingsTab = {
        id: "settings-tab",
        label: "Settings",
        path: "settings://internal",
        content: "",
        type: "settings" as const
      };
      openTabs.push(settingsTab);
      activeTabId = "settings-tab";
    },
    closeTab(id: string) {
      const index = openTabs.findIndex(t => t.id === id);
      if (index !== -1) {
        openTabs.splice(index, 1);
        if (activeTabId === id) {
          activeTabId = openTabs.length > 0 ? openTabs[openTabs.length - 1].id : null;
        }
      }
    },
    async saveActiveFile() {
      const activeTab = openTabs.find(t => t.id === activeTabId);
      if (activeTab && projectRoot) {
        try {
          await invoke("write_file", { path: activeTab.path, content: activeTab.content });
          console.log("File saved:", activeTab.path);
        } catch (err) {
          console.error("Failed to save file:", err);
        }
      }
    },
    async refreshProject() {
      if (projectRoot) {
        const result = await invoke("scan_project", { path: projectRoot });
        fileTree = result;
      }
    },
    toggleSidePanel() {
      isSidePanelVisible = !isSidePanelVisible;
    },
    toggleBottomPanel() {
      isBottomPanelVisible = !isBottomPanelVisible;
    },
    toggleRightPanel() {
      isRightPanelVisible = !isRightPanelVisible;
    },
    get searchResults() {
      return searchResults;
    },
    async searchProject(query: string) {
      if (!projectRoot || !query) return;
      try {
        searchResults = await invoke("search_in_files", { query, path: projectRoot, include: null, exclude: null });
        if (activeActivityId !== "search") {
          activeActivityId = "search";
          activeSidePanelTitle = "SEARCH";
        }
      } catch (err) {
        console.error("Failed to search project:", err);
      }
    },
    get gitChanges() {
      return gitChanges;
    },
    async refreshGitStatus() {
      if (!projectRoot) return;
      try {
        const status = await invoke("git_status", { path: projectRoot });
        gitChanges = status as any;
      } catch (err) {
        console.error("Failed to refresh git status:", err);
      }
    },
    async stageFile(file: string) {
      if (!projectRoot) return;
      try {
        await invoke("git_stage", { path: projectRoot, file });
        await this.refreshGitStatus();
      } catch (err) {
        console.error("Failed to stage file:", err);
      }
    },
    async unstageFile(file: string) {
      if (!projectRoot) return;
      try {
        await invoke("git_unstage", { path: projectRoot, file });
        await this.refreshGitStatus();
      } catch (err) {
        console.error("Failed to unstage file:", err);
      }
    },
    async commitChanges(message: string) {
      if (!projectRoot) return;
      try {
        await invoke("git_commit", { path: projectRoot, message });
        await this.refreshGitStatus();
      } catch (err) {
        console.error("Failed to commit changes:", err);
      }
    },
    get activeTaskId() {
        return activeTaskId;
    },
    async runTask(command: string, args: string[] = []) {
      if (!projectRoot) return;
      try {
        const taskId = await invoke("run_command", { command, args, cwd: projectRoot });
        activeTaskId = taskId as string;
        // Optionally switch to Matrix panel?
        activeActivityId = "matrix"; // Assuming matrix is a valid ID or bottom panel
        // Wait, matrix is in bottom panel. Need to ensure bottom panel is visible?
        // For now, just setting ID.
      } catch (err) {
        console.error("Failed to run command:", err);
      }
    },
  };
}




export const uiState = createUIState();
