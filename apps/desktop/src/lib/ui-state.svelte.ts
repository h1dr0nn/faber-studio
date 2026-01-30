import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { appConsole } from "./stores/console.svelte";

export function createUIState() {
  // --- Persistent State Variables ---
  let activeActivityId = $state("explorer");
  let activeRightActivityId = $state("doctor");
  let isSidePanelVisible = $state(true);
  let isBottomPanelVisible = $state(false);
  let isRightPanelVisible = $state(false);
  let activeSidePanelTitle = $state("EXPLORER");
  let activeRightSidePanelTitle = $state("DOCTOR");
  let activeBottomPanelTab = $state("console");

  let sidebarWidth = $state(250);
  let rightSidebarWidth = $state(360);
  let bottomPanelHeight = $state(200);

  let projectRoot = $state<string | null>(null);
  let recentProjects = $state<string[]>([]);
  let openTabs = $state<{ id: string; label: string; path: string; content: string; type?: "file" | "settings" | "welcome" }[]>([]);
  let activeTabId = $state<string | null>(null);
  let commitMessage = $state("");

  // AI Settings
  let aiSettings = $state({
    activeProvider: 'gemini',
    instructions: `You are a professional software engineer. Generate a concise and meaningful git commit message based on the provided diff.\n\nStrictly follow the Conventional Commits specification:\n\nFormat: <type>(<scope>): <subject>\n\n[Types]: feat, fix, docs, style, refactor, perf, test, chore, ci, build, revert.\n[Scope]: Optional. Use keywords like ui, api, auth, db, config, deps, or a component name.\n[Subject]:\n- Use imperative, present tense (e.g., 'add' not 'added').\n- Do not capitalize the first letter.\n- No trailing period.\n- Maximum 50 characters.\n\n[Body/Footer]: Optional. Only include if the diff is complex. Explain 'what' and 'why', not 'how'. Wrap at 72 characters. Use 'BREAKING CHANGE: description' for breaking changes.`,
    providers: {
      gemini: { authMethod: 'apiKey', apiKey: '', model: 'gemini-1.5-flash' },
      openai: { authMethod: 'apiKey', apiKey: '', model: 'gpt-4o-mini', orgId: '' },
      anthropic: { authMethod: 'apiKey', apiKey: '', model: 'claude-3-5-sonnet-latest' },
      grok: { authMethod: 'apiKey', apiKey: '', model: 'grok-beta' },
      deepseek: { authMethod: 'apiKey', apiKey: '', model: 'deepseek-chat' },
      mistral: { authMethod: 'apiKey', apiKey: '', model: 'mistral-tiny' },
      cohere: { authMethod: 'apiKey', apiKey: '', model: 'command-r-plus' },
      azure: { authMethod: 'apiKey', apiKey: '', model: '', baseUrl: '', deploymentName: '', apiVersion: '2024-02-15-preview' },
      openrouter: { authMethod: 'apiKey', apiKey: '', model: '' },
      ollama: { authMethod: 'apiKey', apiKey: '', model: '', baseUrl: 'http://localhost:11434' },
      custom: { authMethod: 'apiKey', apiKey: '', model: '', baseUrl: '' }
    } as Record<string, any>
  });

  // --- Non-Persistent State ---
  let contextMenu = $state({ x: 0, y: 0, visible: false, items: [] as any[] });
  let renamingPath = $state<string | null>(null);
  let fileTree = $state<any>(null);
  let searchResults = $state<any[]>([]);
  let gitChanges = $state<{ staged: any[]; unstaged: any[] }>({ staged: [], unstaged: [] });
  let gitBranch = $state("main");
  let activeTaskId = $state<string | null>(null);
  let _availableModels = $state<Record<string, { id: string; label: string; description?: string; isFree?: boolean }[]>>({});
  let _isFetchingModels = $state(false);
  let _isGeneratingCommitMessage = $state(false);
  let _isCloning = $state(false);
  let _cloningProgress = $state("");
  let chatMessages = $state<{ role: "user" | "assistant"; content: string }[]>([]);
  let isContinuousChat = $state(true);
  let _isSendingChat = $state(false);
  
  let cursorPosition = $state({ ln: 1, col: 1 });
  let editorTelemetry = $state({ language: "Plain Text", encoding: "UTF-8" });

  // --- Persistence Logic ---
  async function saveConfig(key: string, value: any) {
    try {
      await invoke("save_config", { key, value });
    } catch (err) {
      console.error(`Failed to save ${key} to Rust:`, err);
    }
  }

  function saveUIState() {
    saveConfig("ui_state", {
      activeActivityId,
      activeRightActivityId,
      isSidePanelVisible,
      isBottomPanelVisible,
      isRightPanelVisible,
      activeSidePanelTitle,
      activeRightSidePanelTitle,
      activeBottomPanelTab,
      sidebarWidth,
      rightSidebarWidth,
      bottomPanelHeight
    });
  }

  function saveChat() {
    saveConfig("chat_data", {
      chatMessages,
      isContinuousChat
    });
  }

  return {
    async init() {
      try {
        const ui = await invoke("load_config", { key: "ui_state" }) as any;
        if (ui) {
          activeActivityId = ui.activeActivityId || activeActivityId;
          activeRightActivityId = ui.activeRightActivityId || activeRightActivityId;
          isSidePanelVisible = ui.isSidePanelVisible ?? isSidePanelVisible;
          isBottomPanelVisible = ui.isBottomPanelVisible ?? isBottomPanelVisible;
          isRightPanelVisible = ui.isRightPanelVisible ?? isRightPanelVisible;
          activeSidePanelTitle = ui.activeSidePanelTitle || activeSidePanelTitle;
          activeRightSidePanelTitle = ui.activeRightSidePanelTitle || activeRightSidePanelTitle;
          activeBottomPanelTab = ui.activeBottomPanelTab || activeBottomPanelTab;
          sidebarWidth = ui.sidebarWidth || sidebarWidth;
          rightSidebarWidth = ui.rightSidebarWidth || rightSidebarWidth;
          bottomPanelHeight = ui.bottomPanelHeight || bottomPanelHeight;
        }

        const ai = await invoke("load_config", { key: "ai_settings" }) as any;
        if (ai) aiSettings = { ...aiSettings, ...ai };

        // const root = await invoke("load_config", { key: "project_root" }) as string | null;
        // if (root) this.setProjectRoot(root);

        const recent = await invoke("load_config", { key: "recent_projects" }) as string[] | null;
        if (recent) recentProjects = recent;

        const tabs = await invoke("load_config", { key: "open_tabs" }) as any[];
        if (tabs) {
          openTabs = await Promise.all(tabs.map(async (t) => {
            if (t.type === "file" && t.path) {
              try {
                const content = await invoke("read_file", { path: t.path }) as string;
                return { ...t, content };
              } catch (err) {
                console.error(`Failed to reload content for ${t.path}:`, err);
                return { ...t, content: "" };
              }
            }
            return t;
          }));
        }

        const activeTab = await invoke("load_config", { key: "active_tab_id" }) as string | null;
        if (activeTab) activeTabId = activeTab;

        const chat = await invoke("load_config", { key: "chat_data" }) as any;
        if (chat) {
          chatMessages = chat.chatMessages || [];
          isContinuousChat = chat.isContinuousChat ?? true;
        }

      } catch (err) {
        console.error("Failed to hydrate UI state from Rust:", err);
      }
    },

    get activeActivityId() { return activeActivityId; },
    set activeActivityId(value: string) {
      activeActivityId = value;
      activeSidePanelTitle = value.toUpperCase();
      saveUIState();
    },
    get activeRightActivityId() { return activeRightActivityId; },
    set activeRightActivityId(value: string) {
      activeRightActivityId = value;
      const titles: Record<string, string> = {
        'init': 'INITIALIZE', 'doctor': 'DOCTOR', 'assets': 'ASSET MANAGER', 'mobile': 'MOBILE SETUP', 'clone': 'CLONE REPOSITORY', 'chat': 'AI CHAT'
      };
      activeRightSidePanelTitle = titles[value] || value.toUpperCase();
      saveUIState();
    },
    get isSidePanelVisible() { return isSidePanelVisible; },
    set isSidePanelVisible(value: boolean) { isSidePanelVisible = value; saveUIState(); },
    get isBottomPanelVisible() { return isBottomPanelVisible; },
    set isBottomPanelVisible(value: boolean) { isBottomPanelVisible = value; saveUIState(); },
    get isRightPanelVisible() { return isRightPanelVisible; },
    set isRightPanelVisible(value: boolean) { isRightPanelVisible = value; saveUIState(); },
    get activeBottomPanelTab() { return activeBottomPanelTab; },
    set activeBottomPanelTab(value: string) { activeBottomPanelTab = value; saveUIState(); },
    get activeSidePanelTitle() { return activeSidePanelTitle; },
    set activeSidePanelTitle(value: string) { activeSidePanelTitle = value; saveUIState(); },
    get activeRightSidePanelTitle() { return activeRightSidePanelTitle; },
    set activeRightSidePanelTitle(value: string) { activeRightSidePanelTitle = value; saveUIState(); },
    get sidebarWidth() { return sidebarWidth; },
    set sidebarWidth(value: number) { sidebarWidth = value; saveUIState(); },
    get rightSidebarWidth() { return rightSidebarWidth; },
    set rightSidebarWidth(value: number) { rightSidebarWidth = value; saveUIState(); },
    get bottomPanelHeight() { return bottomPanelHeight; },
    set bottomPanelHeight(value: number) { bottomPanelHeight = value; saveUIState(); },

    get contextMenu() { return contextMenu; },
    showContextMenu(x: number, y: number, items: any[]) {
      contextMenu.x = x; contextMenu.y = y; contextMenu.items = items; contextMenu.visible = true;
    },
    closeContextMenu() { contextMenu.visible = false; },
    get renamingPath() { return renamingPath; },
    set renamingPath(value: string | null) { renamingPath = value; },

    get projectRoot() { return projectRoot; },
    async setProjectRoot(path: string) {
      projectRoot = path;
      saveConfig("project_root", path);
      recentProjects = [path, ...recentProjects.filter(p => p !== path)].slice(0, 10);
      saveConfig("recent_projects", recentProjects);
      await this.refreshFileTree();
      await this.refreshGitStatus();
    },
    get recentProjects() { return recentProjects; },

    get openTabs() { return openTabs; },
    get activeTabId() { return activeTabId; },
    set activeTabId(value: string | null) {
      activeTabId = value;
      saveConfig("active_tab_id", value);
    },
    saveTabs() {
      const metadata = openTabs.map(t => ({ id: t.id, label: t.label, path: t.path, type: t.type }));
      saveConfig("open_tabs", metadata);
    },

    async openFile(path: string, name: string) {
      const existing = openTabs.find(t => t.path === path);
      if (existing) { activeTabId = existing.id; return; }
      try {
        const content = await invoke("read_file", { path }) as string;
        const newTab = { id: Math.random().toString(36).substring(2, 11), label: name, path, content, type: "file" as const };
        openTabs.push(newTab);
        activeTabId = newTab.id;
        this.saveTabs();
      } catch (err) { console.error("Failed to read file:", err); }
    },
    async createFile(parentPath: string, name: string) {
      try {
        const path = `${parentPath}/${name}`;
        await invoke("create_file", { path });
        await this.refreshFileTree();
      } catch (err) { console.error("Failed to create file:", err); }
    },
    async createFolder(parentPath: string, name: string) {
      try {
        const path = `${parentPath}/${name}`;
        await invoke("create_dir", { path });
        await this.refreshFileTree();
      } catch (err) { console.error("Failed to create folder:", err); }
    },
    async deletePath(path: string) {
      try {
        await invoke("delete_path", { path });
        const tab = openTabs.find(t => t.path === path);
        if (tab) this.closeTab(tab.id);
        await this.refreshFileTree();
      } catch (err) { console.error("Failed to delete path:", err); }
    },
    async renamePath(oldPath: string, newPath: string) {
      try {
        await invoke("rename_path", { oldPath, newPath });
        const tab = openTabs.find(t => t.path === oldPath);
        if (tab) {
          tab.path = newPath;
          tab.label = newPath.split(/[\\/]/).pop() || tab.label;
        }
        await this.refreshFileTree();
      } catch (err) { console.error("Failed to rename path:", err); }
    },
    openSettings() {
      const existing = openTabs.find(t => t.id === "settings-tab");
      if (existing) { activeTabId = "settings-tab"; return; }
      const settingsTab = { id: "settings-tab", label: "Settings", path: "settings://internal", content: "", type: "settings" as const };
      openTabs.push(settingsTab);
      activeTabId = "settings-tab";
      this.saveTabs();
    },
    closeTab(id: string) {
      const index = openTabs.findIndex(t => t.id === id);
      if (index !== -1) {
        openTabs.splice(index, 1);
        if (activeTabId === id) activeTabId = openTabs.length > 0 ? openTabs[openTabs.length - 1].id : null;
        this.saveTabs();
      }
    },
    async saveActiveFile() {
      const activeTab = openTabs.find(t => t.id === activeTabId);
      if (activeTab && projectRoot) {
        try {
          await invoke("write_file", { path: activeTab.path, content: activeTab.content });
        } catch (err) { console.error("Failed to save file:", err); }
      }
    },
    async refreshProject() {
      if (projectRoot) {
        fileTree = await invoke("scan_project", { path: projectRoot });
      }
    },
    toggleSidePanel() { isSidePanelVisible = !isSidePanelVisible; saveUIState(); },
    toggleBottomPanel() { isBottomPanelVisible = !isBottomPanelVisible; saveUIState(); },
    toggleRightPanel() { isRightPanelVisible = !isRightPanelVisible; saveUIState(); },

    get fileTree() { return fileTree; },
    async refreshFileTree() {
      if (!projectRoot) return;
      try { fileTree = await invoke("scan_project", { path: projectRoot }); } catch (err) { console.error(err); }
    },
    async openFolder() {
      try {
        const selected = await open({ directory: true, multiple: false });
        if (selected && typeof selected === "string") await this.setProjectRoot(selected);
      } catch (err) { console.error(err); }
    },
    get searchResults() { return searchResults; },
    async searchProject(query: string) {
      if (!projectRoot || !query) return;
      try {
        searchResults = await invoke("search_in_files", { query, path: projectRoot, include: null, exclude: null });
        if (activeActivityId !== "search") { activeActivityId = "search"; activeSidePanelTitle = "SEARCH"; saveUIState(); }
      } catch (err) { console.error(err); }
    },
    clearSearchResults() {
      searchResults = [];
    },
    get gitChanges() { return gitChanges; },
    async refreshGitStatus() {
      if (!projectRoot) return;
      try { 
        gitChanges = await invoke("git_status", { path: projectRoot }) as any; 
        await this.refreshGitBranch();
      } catch (err) { console.error(err); }
    },
    get gitBranch() { return gitBranch; },
    async refreshGitBranch() {
      if (!projectRoot) return;
      try { gitBranch = await invoke("git_branch", { path: projectRoot }) as string; } catch (err) { console.error(err); }
    },
    async stageFile(file: string) {
      if (!projectRoot) return;
      try { await invoke("git_stage", { path: projectRoot, file }); await this.refreshGitStatus(); } catch (err) { console.error(err); }
    },
    async unstageFile(file: string) {
      if (!projectRoot) return;
      try { await invoke("git_unstage", { path: projectRoot, file }); await this.refreshGitStatus(); } catch (err) { console.error(err); }
    },
    get commitMessage() { return commitMessage; },
    set commitMessage(value: string) { commitMessage = value; },
    async commitChanges(message: string) {
      if (!projectRoot) return;
      try { await invoke("git_commit", { path: projectRoot, message }); await this.refreshGitStatus(); commitMessage = ""; } catch (err) { console.error(err); }
    },
    async pushChanges() {
      if (!projectRoot) return;
      try { await invoke("git_push", { path: projectRoot }); } catch (err) { console.error(err); }
    },
    async pullChanges() {
      if (!projectRoot) return;
      try { await invoke("git_pull", { path: projectRoot }); await this.refreshGitStatus(); } catch (err) { console.error(err); }
    },
    async discardChanges(file: string) {
      if (!projectRoot) return;
      try { await invoke("git_discard_changes", { path: projectRoot, file }); await this.refreshGitStatus(); } catch (err) { console.error(err); }
    },

    get aiSettings() { return aiSettings; },
    set aiSettings(value: any) {
      aiSettings = value;
      saveConfig("ai_settings", aiSettings);
    },
    get isGeneratingCommitMessage() { return _isGeneratingCommitMessage; },
    async generateCommitMessage() {
      if (!projectRoot) return "";
      const { activeProvider, instructions, providers } = aiSettings;
      const config = providers[activeProvider];
      if (!config.apiKey && activeProvider !== 'custom') return "Please configure API Key in Settings.";
      _isGeneratingCommitMessage = true;
      try {
        const diff = await invoke("git_diff_staged", { path: projectRoot }) as string;
        if (!diff || diff.trim() === "") return "No staged changes found. Please stage changes first.";
        const prompt = `${instructions}\n\nStaged changes diff:\n${diff}`;
        let res = "";
        if (activeProvider === 'gemini') {
          const url = `https://generativelanguage.googleapis.com/v1beta/models/${config.model}:generateContent?key=${config.apiKey}`;
          const response = await fetch(url, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ contents: [{ parts: [{ text: prompt }] }], generationConfig: { temperature: 0.1, maxOutputTokens: 800 } }) });
          const data = await response.json();
          if (data.error) throw new Error(data.error.message);
          res = data.candidates?.[0]?.content?.parts?.[0]?.text?.trim() || "Failed to generate message.";
        } else if (activeProvider === 'anthropic') {
          const response = await fetch("https://api.anthropic.com/v1/messages", { method: 'POST', headers: { 'Content-Type': 'application/json', 'x-api-key': config.apiKey, 'anthropic-version': '2023-06-01', 'dangerously-allow-browser': 'true' }, body: JSON.stringify({ model: config.model, max_tokens: 800, messages: [{ role: "user", content: prompt }], system: instructions, temperature: 0.1 }) });
          const data = await response.json();
          if (data.error) throw new Error(data.error.message);
          res = data.content?.[0]?.text?.trim() || "Failed to generate message.";
        } else {
          let baseUrl = "https://api.openai.com/v1/chat/completions";
          if (activeProvider === 'grok') baseUrl = "https://api.x.ai/v1/chat/completions";
          else if (activeProvider === 'deepseek') baseUrl = "https://api.deepseek.com/v1/chat/completions";
          else if (activeProvider === 'mistral') baseUrl = "https://api.mistral.ai/v1/chat/completions";
          else if (activeProvider === 'cohere') baseUrl = "https://api.cohere.ai/v1/chat/completions";
          else if (activeProvider === 'openrouter') baseUrl = "https://openrouter.ai/api/v1/chat/completions";
          else if (activeProvider === 'ollama') baseUrl = `${config.baseUrl}/v1/chat/completions`;
          else if (activeProvider === 'azure') baseUrl = `${config.baseUrl}/openai/deployments/${config.deploymentName}/chat/completions?api-version=${config.apiVersion}`;
          else if (activeProvider === 'custom') baseUrl = config.baseUrl;
          const headers: Record<string, string> = { 'Content-Type': 'application/json', 'Authorization': `Bearer ${config.apiKey}` };
          if (activeProvider === 'azure') { delete headers['Authorization']; headers['api-key'] = config.apiKey; }
          if (activeProvider === 'openai' && config.orgId) headers['OpenAI-Organization'] = config.orgId;
          const response = await fetch(baseUrl, { method: 'POST', headers, body: JSON.stringify({ model: config.model, messages: [{ role: "system", content: instructions }, { role: "user", content: `Generate a commit message for these staged changes:\n${diff}` }], temperature: 0.1 }) });
          const data = await response.json();
          if (data.error) throw new Error(data.error.message);
          res = data.choices?.[0]?.message?.content?.trim() || "Failed to generate message.";
        }
        return res;
      } catch (err: any) {
        console.error(err);
        let msg = err.message || err;
        if (msg.includes("quota") || msg.includes("limit")) msg = `Quota Exceeded: ${msg}. If you're on a free tier, try a different model (like Gemini 1.5 Flash) or check your billing.`;
        return `Error: ${msg}`;
      } finally { _isGeneratingCommitMessage = false; }
    },
    async fetchModels(providerId: string) {
      const config = aiSettings.providers[providerId];
      if (!config.apiKey && providerId !== 'custom') return;
      _isFetchingModels = true;
      try {
        if (providerId === 'gemini') {
          const response = await fetch(`https://generativelanguage.googleapis.com/v1beta/models?key=${config.apiKey}`);
          const data = await response.json();
          if (data.models) _availableModels[providerId] = data.models.filter((m: any) => m.supportedGenerationMethods.includes("generateContent")).map((m: any) => ({ id: m.name.replace("models/", ""), label: m.displayName || m.name.replace("models/", ""), description: m.description, isFree: m.description?.toLowerCase().includes("free") || m.name.includes("flash") }));
        } else if (providerId === 'anthropic') {
          const response = await fetch("https://api.anthropic.com/v1/models", { headers: { 'x-api-key': config.apiKey, 'anthropic-version': '2023-06-01' } });
          const data = await response.json();
          if (data.data) _availableModels[providerId] = data.data.map((m: any) => ({ id: m.id, label: m.display_name || m.id, isFree: m.id.includes("haiku") }));
        } else {
          let baseUrl = "https://api.openai.com/v1/models";
          if (providerId === 'grok') baseUrl = "https://api.x.ai/v1/models";
          else if (providerId === 'deepseek') baseUrl = "https://api.deepseek.com/models";
          else if (providerId === 'mistral') baseUrl = "https://api.mistral.ai/v1/models";
          else if (providerId === 'cohere') baseUrl = "https://api.cohere.ai/v1/models";
          else if (providerId === 'custom') {
            if (!config.baseUrl) return;
            baseUrl = config.baseUrl.replace(/\/chat\/completions$/, "").replace(/\/completions$/, "") + "/models";
          }
          const response = await fetch(baseUrl, { headers: providerId === 'azure' ? { 'api-key': config.apiKey } : { 'Authorization': `Bearer ${config.apiKey}` } });
          const data = await response.json();
          let rawModels: any[] = data.data || data.models || [];
          if (rawModels.length > 0) _availableModels[providerId] = rawModels.map((m: any) => ({ id: m.id || m.name, label: m.id || m.name, isFree: (m.id || m.name).toLowerCase().includes("mini") || (m.id || m.name).toLowerCase().includes("tiny") }));
        }
        const currentModels = _availableModels[providerId] || [];
        if (currentModels.length > 0 && !config.model) config.model = currentModels[0].id;
      } catch (err) { console.error(err); } finally { _isFetchingModels = false; }
    },

    saveAISettings() {
      saveConfig("ai_settings", aiSettings);
    },

    get isCloning() { return _isCloning; },
    get cloningProgress() { return _cloningProgress; },
    async cloneRepository(url: string, parentDir: string) {
      if (_isCloning) return;
      _isCloning = true;
      _cloningProgress = "Cloning repository...";
      try {
        const repoName = url.split('/').pop()?.replace('.git', '') || 'repository';
        const targetPath = `${parentDir}/${repoName}`;
        await invoke("git_clone", { url, path: targetPath });
        await this.setProjectRoot(targetPath);
        _cloningProgress = "Successfully cloned!";
      } catch (err: any) {
        console.error("Failed to clone repository:", err);
        _cloningProgress = `Error: ${err.message || err}`;
        throw err;
      } finally {
        _isCloning = false;
      }
    },

    get availableModels() { return _availableModels; },
    get isFetchingModels() { return _isFetchingModels; },
    get activeTaskId() { return activeTaskId; },
    async runTask(command: string, args: string[] = []) {
      if (!projectRoot) return;
      try { 
        const taskId = await invoke("run_command", { command, args, cwd: projectRoot }); 
        activeTaskId = taskId as string; 
      } catch (err) { console.error(err); }
    },

    get chatMessages() { return chatMessages; },
    get isContinuousChat() { return isContinuousChat; },
    set isContinuousChat(value: boolean) { 
      isContinuousChat = value; 
      saveChat();
    },
    get isSendingChat() { return _isSendingChat; },
    clearChat() { 
      chatMessages = []; 
      saveChat();
    },
    async sendChatMessage(content: string) {
      if (!content.trim() || _isSendingChat) return;
      
      const userMessage = { role: "user" as const, content };
      chatMessages.push(userMessage);
      _isSendingChat = true;

      try {
        const { activeProvider, providers } = aiSettings;
        const config = providers[activeProvider];
        if (!config.apiKey && activeProvider !== 'custom') {
           chatMessages.push({ role: "assistant", content: "Please configure API Key in Settings." });
           return;
        }

        const history = isContinuousChat ? chatMessages : [userMessage];
        const instructions = "You are a helpful AI assistant integrated into Faber Studio, an agentic IDE.";
        
        let res = "";
        if (activeProvider === 'gemini') {
          const url = `https://generativelanguage.googleapis.com/v1beta/models/${config.model}:generateContent?key=${config.apiKey}`;
          const contents = history.map(m => ({
            role: m.role === "user" ? "user" : "model",
            parts: [{ text: m.content }]
          }));
          const response = await fetch(url, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ contents, systemInstruction: { parts: [{ text: instructions }] } }) });
          const data = await response.json();
          res = data.candidates?.[0]?.content?.parts?.[0]?.text || "No response from Gemini.";
        } else if (activeProvider === 'anthropic') {
           const response = await fetch("https://api.anthropic.com/v1/messages", { method: 'POST', headers: { 'Content-Type': 'application/json', 'x-api-key': config.apiKey, 'anthropic-version': '2023-06-01', 'dangerously-allow-browser': 'true' }, body: JSON.stringify({ model: config.model, max_tokens: 1024, messages: history, system: instructions }) });
           const data = await response.json();
           res = data.content?.[0]?.text || "No response from Anthropic.";
        } else {
          let baseUrl = "https://api.openai.com/v1/chat/completions";
          // Mapping other providers (simplified for now, reusing logic from generateCommitMessage)
          if (activeProvider === 'grok') baseUrl = "https://api.x.ai/v1/chat/completions";
          else if (activeProvider === 'deepseek') baseUrl = "https://api.deepseek.com/v1/chat/completions";
          else if (activeProvider === 'ollama') baseUrl = `${config.baseUrl}/v1/chat/completions`;
          
          const headers: Record<string, string> = { 'Content-Type': 'application/json', 'Authorization': `Bearer ${config.apiKey}` };
          const response = await fetch(baseUrl, { method: 'POST', headers, body: JSON.stringify({ model: config.model, messages: [{ role: "system", content: instructions }, ...history] }) });
          const data = await response.json();
          res = data.choices?.[0]?.message?.content || "No response from provider.";
        }

        chatMessages.push({ role: "assistant", content: res });
        saveChat();
      } catch (err: any) {
        console.error("Chat error:", err);
        chatMessages.push({ role: "assistant", content: `Error: ${err.message || err}` });
      } finally {
        _isSendingChat = false;
      }
    },

    get cursorPosition() { return cursorPosition; },
    set cursorPosition(value: { ln: number; col: number }) { cursorPosition = value; },
    updateCursor(textarea: HTMLTextAreaElement) {
      const text = textarea.value.substring(0, textarea.selectionStart);
      const lines = text.split("\n");
      cursorPosition = {
        ln: lines.length,
        col: lines[lines.length - 1].length + 1
      };
    },
    get editorTelemetry() { return editorTelemetry; },
    setEditorTelemetry(language: string, encoding: string = "UTF-8") {
      editorTelemetry = { language, encoding };
    },
    formatActiveFile() {
      const index = openTabs.findIndex(t => t.id === activeTabId);
      if (index !== -1 && openTabs[index].type === "file") {
        const tab = openTabs[index];
        // More visible formatting: trim lines, remove multiple newlines, add final newline
        const originalContent = tab.content;
        const formatted = tab.content
          .split("\n")
          .map(l => l.trimEnd())
          .join("\n")
          .replace(/\n{3,}/g, "\n\n")
          .trim() + "\n";
        
        if (originalContent !== formatted) {
          openTabs[index].content = formatted;
          appConsole.success(`Formatted ${tab.label}`, "Prettier");
        } else {
          appConsole.info(`File ${tab.label} is already formatted`, "Prettier");
        }
      }
    }
  };
}

export const uiState = createUIState();
