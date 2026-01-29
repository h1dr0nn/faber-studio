<script lang="ts">
  import {
    Palette,
    Type,
    Settings2,
    Keyboard,
    Globe,
    Monitor,
    ChevronRight,
    Search,
    Sparkles,
    RefreshCw,
    Shield,
    Key,
    Info,
  } from "lucide-svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { uiState } from "$lib/ui-state.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Switch from "$lib/components/ui/Switch.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import { onMount } from "svelte";
  import { preferences } from "$lib/stores/preferences.svelte";

  $effect(() => {
    // Auto-fetch models when provider changes or API key is loaded/hydrated
    const activeProvider = uiState.aiSettings.activeProvider;
    const config = uiState.aiSettings.providers[activeProvider];
    // We check if we have an API key (or it's custom), no models are loaded yet, and we aren't already fetching
    if (
      (config.apiKey || activeProvider === "custom") &&
      (!uiState.availableModels[activeProvider] ||
        uiState.availableModels[activeProvider].length === 0) &&
      !uiState.isFetchingModels
    ) {
      uiState.fetchModels(activeProvider);
    }
  });

  $effect(() => {
    // Track deep changes to AI settings
    const _ = JSON.stringify(uiState.aiSettings);
    // Debounce or just save directly? Debounce might be better but direct is fine for local
    uiState.saveAISettings();
  });

  let searchQuery = $state("");
  let settings = $state({
    // Editor
    theme: "dark",
    fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
    fontSize: 13,
    lineHeight: 1.5,
    tabSize: 4,
    insertSpaces: true,
    renderWhitespace: "none",
    lineNumbers: true,
    minimap: false,
    wordWrap: "off",
    bracketPairColorization: true,
    guides: true,

    // Files
    autoSave: true,
    autoSaveDelay: 1000,
    exclude: "**/node_modules, **/target, **/.git",
    defaultEOL: "\n",

    // Terminal
    terminalFontSize: 12,
    terminalCursorStyle: "block",
    terminalCursorBlinking: true,

    // Performance & Telemetry
    telemetry: false,
    gpuAcceleration: true,
  });

  let appVersion = $state("...");

  onMount(async () => {
    try {
      appVersion = await getVersion();
    } catch (e) {
      console.error("Failed to get app version", e);
      appVersion = "0.0.1"; // Fallback
    }
  });

  const categories = [
    { id: "editor", label: "Text Editor", icon: Type },
    { id: "files", label: "Files", icon: Settings2 },
    { id: "terminal", label: "Terminal", icon: Keyboard },
    { id: "appearance", label: "Appearance", icon: Palette },
    { id: "ai", label: "AI", icon: Sparkles },
    { id: "accessibility", label: "Accessibility", icon: Monitor },
    { id: "about", label: "About", icon: Info },
  ];
</script>

<div class="feature-container">
  <div class="settings-header">
    <div class="search-box">
      <div class="search-icon-wrapper">
        <Search size={14} />
      </div>
      <Input
        placeholder="Search settings"
        bind:value={searchQuery}
        class="with-icon"
      />
    </div>
  </div>

  <div class="settings-content">
    <div class="category-list">
      {#each categories as cat}
        <button
          class="category-item"
          onclick={() => {
            const el = document.getElementById(`settings-group-${cat.id}`);
            if (el) {
              el.scrollIntoView({ behavior: "smooth" });
            }
          }}
        >
          <cat.icon size={16} />
          <span>{cat.label}</span>
        </button>
      {/each}
    </div>

    {#if !searchQuery || "Editor"
        .toLowerCase()
        .includes(searchQuery.toLowerCase()) || "font size line numbers word wrap tab size bracket pair colorization".includes(searchQuery.toLowerCase())}
      <div class="settings-group" id="settings-group-editor">
        <span class="group-title">Editor</span>

        {#if !searchQuery || "Font Family"
            .toLowerCase()
            .includes(searchQuery.toLowerCase())}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Font Family</span>
              <span class="desc">Controls the font family.</span>
            </div>
            <div class="setting-control">
              <Input bind:value={settings.fontFamily} class="large-input" />
            </div>
          </div>
        {/if}

        {#if !searchQuery || "Font Size"
            .toLowerCase()
            .includes(searchQuery.toLowerCase())}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Font Size</span>
              <span class="desc">Controls the font size in pixels.</span>
            </div>
            <div class="setting-control">
              <Input
                type="number"
                bind:value={settings.fontSize}
                class="small-input"
              />
            </div>
          </div>
        {/if}

        {#if !searchQuery || "Word Wrap"
            .toLowerCase()
            .includes(searchQuery.toLowerCase())}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Word Wrap</span>
              <span class="desc">Controls how lines should wrap.</span>
            </div>
            <div class="setting-control">
              <Select
                options={[
                  { value: "off", label: "Off" },
                  { value: "on", label: "On" },
                  { value: "wordWrapColumn", label: "Word Wrap Column" },
                  { value: "bounded", label: "Bounded" },
                ]}
                bind:value={settings.wordWrap}
                class="ai-select"
              />
            </div>
          </div>
        {/if}

        {#if !searchQuery || "Line Numbers"
            .toLowerCase()
            .includes(searchQuery.toLowerCase())}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Line Numbers</span>
              <span class="desc">Controls the display of line numbers.</span>
            </div>
            <div class="setting-control">
              <Switch
                bind:checked={settings.lineNumbers}
                label="Line Numbers"
              />
            </div>
          </div>
        {/if}

        {#if !searchQuery || "Tab Size"
            .toLowerCase()
            .includes(searchQuery.toLowerCase())}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Tab Size</span>
              <span class="desc">The number of spaces a tab is equal to.</span>
            </div>
            <div class="setting-control">
              <Input
                type="number"
                bind:value={settings.tabSize}
                class="small-input"
              />
            </div>
          </div>
        {/if}

        {#if !searchQuery || "Bracket Pair Colorization"
            .toLowerCase()
            .includes(searchQuery.toLowerCase())}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Bracket Pair Colorization</span>
              <span class="desc"
                >Controls whether bracket pair colorization is enabled.</span
              >
            </div>
            <div class="setting-control">
              <Switch
                bind:checked={settings.bracketPairColorization}
                label="Bracket Pairs"
              />
            </div>
          </div>
        {/if}
      </div>
    {/if}

    {#if !searchQuery || "Files"
        .toLowerCase()
        .includes(searchQuery.toLowerCase()) || "auto save exclude".includes(searchQuery.toLowerCase())}
      <div class="settings-group" id="settings-group-files">
        <span class="group-title">Files</span>

        {#if !searchQuery || "Auto Save"
            .toLowerCase()
            .includes(searchQuery.toLowerCase())}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Auto Save</span>
              <span class="desc">Automatically save dirty files.</span>
            </div>
            <div class="setting-control">
              <Switch bind:checked={settings.autoSave} label="Auto Save" />
            </div>
          </div>
        {/if}

        {#if !searchQuery || "Exclude"
            .toLowerCase()
            .includes(searchQuery.toLowerCase())}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Files: Exclude</span>
              <span class="desc"
                >Configure glob patterns for excluding files and folders.</span
              >
            </div>
            <div class="setting-control">
              <Input bind:value={settings.exclude} class="large-input" />
            </div>
          </div>
        {/if}
      </div>
    {/if}

    {#if !searchQuery || "Terminal"
        .toLowerCase()
        .includes(searchQuery.toLowerCase()) || "font size cursor style".includes(searchQuery.toLowerCase())}
      <div class="settings-group" id="settings-group-terminal">
        <span class="group-title">Terminal</span>

        {#if !searchQuery || "Font Size"
            .toLowerCase()
            .includes(searchQuery.toLowerCase())}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Terminal: Font Size</span>
              <span class="desc"
                >Controls the font size in pixels of the terminal.</span
              >
            </div>
            <div class="setting-control">
              <Input
                type="number"
                bind:value={settings.terminalFontSize}
                class="small-input"
              />
            </div>
          </div>
        {/if}

        {#if !searchQuery || "Cursor Style"
            .toLowerCase()
            .includes(searchQuery.toLowerCase())}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Terminal: Cursor Style</span>
              <span class="desc"
                >Controls the style of the terminal cursor.</span
              >
            </div>
            <div class="setting-control">
              <Select
                options={[
                  { value: "block", label: "Block" },
                  { value: "line", label: "Line" },
                  { value: "underline", label: "Underline" },
                ]}
                bind:value={settings.terminalCursorStyle}
                class="ai-select"
              />
            </div>
          </div>
        {/if}
      </div>
    {/if}
    {#if !searchQuery || "Appearance"
        .toLowerCase()
        .includes(searchQuery.toLowerCase())}
      <div class="settings-group" id="settings-group-appearance">
        <span class="group-title">Appearance</span>
        <div class="setting-row">
          <div class="setting-info">
            <span class="label">Color Theme</span>
            <span class="desc">The theme used in the workbench.</span>
          </div>
          <div class="setting-control">
            <Select
              options={[
                { value: "dark", label: "Dark Modern" },
                { value: "emerald-pro", label: "Emerald Pro" },
                { value: "forest", label: "Forest Dark" },
                { value: "light", label: "Light Modern" },
                { value: "midnight", label: "Midnight Pro" },
                { value: "iceberg", label: "Arctic Ice" },
                { value: "solar-light", label: "Solar Flare" },
                { value: "monolith", label: "Monolith" },
              ]}
              bind:value={preferences.theme}
              class="ai-select"
              onchange={() => preferences.save()}
            />
          </div>
        </div>
      </div>
    {/if}

    {#if !searchQuery || "AI"
        .toLowerCase()
        .includes(searchQuery.toLowerCase()) || "provider api key instructions model base url organization authentication".includes(searchQuery.toLowerCase())}
      {@const activeProvider = uiState.aiSettings.activeProvider}
      {@const config = uiState.aiSettings.providers[activeProvider]}
      <div class="settings-group" id="settings-group-ai">
        <span class="group-title">AI Configuration</span>

        <div class="setting-row">
          <div class="setting-info">
            <span class="label">AI Provider</span>
            <span class="desc"
              >Select the AI service to use for code assistance and commit
              generation.</span
            >
          </div>
          <div class="setting-control">
            <Select
              class="ai-select"
              options={[
                { value: "gemini", label: "Google Gemini" },
                { value: "openai", label: "OpenAI / ChatGPT" },
                { value: "anthropic", label: "Anthropic Claude" },
                { value: "grok", label: "xAI Grok" },
                { value: "deepseek", label: "DeepSeek" },
                { value: "mistral", label: "Mistral AI" },
                { value: "cohere", label: "Cohere" },
                { value: "azure", label: "Azure OpenAI" },
                { value: "openrouter", label: "OpenRouter" },
                { value: "ollama", label: "Ollama (Local)" },
                { value: "custom", label: "Custom (OpenAI Compatible)" },
              ]}
              bind:value={uiState.aiSettings.activeProvider}
              onchange={() =>
                uiState.fetchModels(uiState.aiSettings.activeProvider)}
            />
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <span class="label">Authentication</span>
            <span class="desc"
              >Choose how to authenticate with this provider.</span
            >
          </div>
          <div class="setting-control">
            <Select
              class="ai-select"
              options={[
                { value: "apiKey", label: "API Key" },
                {
                  value: "oauth",
                  label: "OAuth (Coming Soon)",
                  disabled: true,
                },
              ]}
              bind:value={config.authMethod}
            />
          </div>
        </div>

        {#if activeProvider === "azure"}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Endpoint URL</span>
              <span class="desc"
                >Your Azure OpenAI resource endpoint (e.g.
                https://res-name.openai.azure.com/).</span
              >
            </div>
            <div class="setting-control">
              <Input
                bind:value={config.baseUrl}
                placeholder="https://your-resource.openai.azure.com/"
                class="large-input"
              />
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Deployment Name</span>
              <span class="desc">The name of the model deployment.</span>
            </div>
            <div class="setting-control">
              <Input
                bind:value={config.deploymentName}
                placeholder="gpt-4o"
                class="large-input"
              />
            </div>
          </div>
        {/if}

        {#if activeProvider === "ollama" || activeProvider === "custom"}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Base URL</span>
              <span class="desc"
                >{activeProvider === "ollama"
                  ? "The Ollama API endpoint."
                  : "The API endpoint (e.g. http://localhost:11434/v1)."}</span
              >
            </div>
            <div class="setting-control">
              <Input
                bind:value={config.baseUrl}
                placeholder={activeProvider === "ollama"
                  ? "http://localhost:11434"
                  : "https://api.example.com/v1"}
                class="large-input"
                onchange={() => uiState.fetchModels(activeProvider)}
              />
            </div>
          </div>
        {/if}

        {#if config.authMethod === "oauth"}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Client ID</span>
              <span class="desc">The application client ID for OAuth.</span>
            </div>
            <div class="setting-control">
              <Input
                placeholder="Coming soon..."
                disabled={true}
                class="large-input"
              />
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Client Secret</span>
              <span class="desc">The application client secret.</span>
            </div>
            <div class="setting-control">
              <Input
                type="password"
                placeholder="Coming soon..."
                disabled={true}
                class="large-input"
              />
            </div>
          </div>
        {/if}

        {#if config.authMethod === "apiKey" && activeProvider !== "ollama"}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">API Key</span>
              <span class="desc"
                >Your personal API key for {activeProvider.toUpperCase()}.</span
              >
            </div>
            <div class="setting-control">
              <Input
                type="password"
                bind:value={config.apiKey}
                placeholder={activeProvider === "custom"
                  ? "Optional"
                  : "sk-..."}
                class="large-input"
                onchange={() => uiState.fetchModels(activeProvider)}
              />
            </div>
          </div>
        {/if}

        {#if activeProvider === "openai"}
          <div class="setting-row">
            <div class="setting-info">
              <span class="label">Organization ID</span>
              <span class="desc">Optional OpenAI Organization ID.</span>
            </div>
            <div class="setting-control">
              <Input
                bind:value={config.orgId}
                placeholder="org-..."
                class="large-input"
              />
            </div>
          </div>
        {/if}

        <div class="setting-row">
          <div class="setting-info">
            <span class="label">Model</span>
            <span class="desc">Select the specific model architecture.</span>
          </div>
          <div class="setting-control">
            <div class="model-select-wrapper full-width">
              <Select
                class="ai-select full-width"
                searchable={true}
                placeholder={uiState.isFetchingModels
                  ? "Fetching models..."
                  : "Enter API key to load models..."}
                options={uiState.availableModels[activeProvider]?.map((m) => ({
                  value: m.id,
                  label: m.isFree ? `${m.label} (Free Tier)` : m.label,
                })) || []}
                bind:value={config.model}
              />
            </div>
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <span class="label">Instructions</span>
            <span class="desc"
              >Custom instructions for generating commit messages.</span
            >
          </div>
          <div class="setting-control flex-col align-end">
            <textarea
              class="custom-textarea"
              bind:value={uiState.aiSettings.instructions}
              placeholder="e.g. Use conventional commits format..."
            ></textarea>
          </div>
        </div>
      </div>
    {/if}

    {#if !searchQuery || "Accessibility"
        .toLowerCase()
        .includes(searchQuery.toLowerCase())}
      <div class="settings-group" id="settings-group-accessibility">
        <span class="group-title">Accessibility</span>
        <div class="setting-row">
          <div class="setting-info">
            <span class="label">GPU Acceleration</span>
            <span class="desc">Controls whether to use GPU acceleration.</span>
          </div>
          <div class="setting-control">
            <Switch
              bind:checked={settings.gpuAcceleration}
              label="GPU Acceleration"
            />
          </div>
        </div>
      </div>
    {/if}

    {#if !searchQuery || "About"
        .toLowerCase()
        .includes(searchQuery.toLowerCase())}
      <div class="settings-group" id="settings-group-about">
        <div class="about-branding">
          <img src="/app-logo.png" alt="Logo" class="about-logo" />
          <div class="about-title-info">
            <span class="about-title-name">Faber Studio</span>
            <span class="about-title-tag"
              >Advanced Agentic Coding Environment</span
            >
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <span class="label">Version</span>
            <span class="desc"
              >The currently installed version of Faber Studio.</span
            >
          </div>
          <div class="setting-control">
            <span class="value-text">{appVersion}</span>
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <span class="label">Developer</span>
            <span class="desc">Created and maintained by h1dr0n.</span>
          </div>
          <div class="setting-control">
            <span class="value-text">h1dr0n</span>
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <span class="label">Repository</span>
            <span class="desc">View source code and contribute on GitHub.</span>
          </div>
          <div class="setting-control">
            <a
              href="https://github.com/h1dr0nn/faber-studio"
              target="_blank"
              class="standard-link">GitHub</a
            >
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <span class="label">Check for Updates</span>
            <span class="desc"
              >Check if there is a newer version available.</span
            >
          </div>
          <div class="setting-control">
            <button
              class="action-btn"
              onclick={() =>
                alert(
                  "Checking for updates...\nYou are on the latest version.",
                )}
            >
              <RefreshCw size={14} />
              <span>Check Update</span>
            </button>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .feature-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    color: var(--fg-secondary);
  }

  .settings-header {
    padding: 12px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .search-box {
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-icon-wrapper {
    position: absolute;
    left: 8px;
    z-index: 1;
    pointer-events: none;
    opacity: 0.5;
    display: flex;
  }

  :global(.with-icon) {
    padding-left: 28px !important;
  }

  .settings-content {
    flex: 1;
    overflow-y: auto;
    padding-bottom: 50px; /* Space at bottom */
  }

  /* Custom scrollbar */
  .settings-content::-webkit-scrollbar {
    width: 12px;
  }
  .settings-content::-webkit-scrollbar-thumb {
    background-color: var(--bg-active);
    border: 3px solid transparent;
    background-clip: content-box;
    border-radius: 10px;
  }

  .category-list {
    padding: 8px 0;
    border-bottom: 1px solid var(--border-subtle);
  }

  .category-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 16px;
    width: 100%;
    background: none;
    border: none;
    color: var(--fg-secondary);
    cursor: pointer;
    font-size: 13px;
    text-align: left;
  }

  .category-item:hover {
    background-color: var(--bg-hover);
    color: var(--fg-primary);
  }

  .settings-group {
    padding: 16px 0 24px 0;
    display: flex;
    flex-direction: column;
    gap: 16px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .settings-group:last-child {
    border-bottom: none;
  }

  .group-title {
    padding: 0 16px;
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    color: var(--accent-primary);
    letter-spacing: 0.5px;
  }

  .setting-row {
    padding: 0 16px;
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 16px;
  }

  .setting-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .label {
    font-size: 13px;
    color: var(--fg-primary);
  }

  .desc {
    font-size: 11px;
    opacity: 0.6;
    line-height: 1.4;
  }

  .setting-control {
    display: flex;
    align-items: center;
  }

  :global(.large-input) {
    width: 260px !important;
    height: 30px !important;
    box-sizing: border-box !important;
  }

  :global(.small-input) {
    width: 120px !important;
  }

  :global(.ai-select) {
    width: 260px !important;
  }

  .model-select-wrapper {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 260px;
  }

  .model-select-wrapper.full-width {
    width: 260px;
  }

  :global(.ai-select.full-width) {
    width: 260px !important;
  }

  :global(.spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .custom-textarea {
    width: 260px;
    height: 100px;
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    color: var(--fg-primary);
    padding: 8px;
    font-size: 12px;
    resize: vertical;
    outline: none;
  }

  .custom-textarea:focus {
    border-color: var(--accent-primary);
  }

  .flex-col {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .align-end {
    align-items: flex-end;
  }

  .align-end {
    align-items: flex-end;
  }

  /* About Section Refinement */
  .about-branding {
    padding: 12px 16px 24px 16px;
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 4px;
  }

  .about-logo {
    width: 48px;
    height: 48px;
    object-fit: contain;
    filter: drop-shadow(0 0 15px rgba(var(--accent-rgb), 0.2));
  }

  .about-title-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .about-title-name {
    font-size: 20px;
    font-weight: 200;
    color: var(--fg-primary);
    letter-spacing: -0.5px;
  }

  .about-title-tag {
    font-size: 11px;
    color: var(--fg-secondary);
    opacity: 0.7;
  }

  .value-text {
    font-size: 13px;
    color: var(--fg-primary);
    background-color: var(--bg-active);
    padding: 2px 8px;
    border-radius: 4px;
    font-family: var(--font-mono);
  }

  .standard-link {
    color: var(--accent-primary);
    text-decoration: none;
    font-size: 13px;
  }

  .standard-link:hover {
    text-decoration: underline;
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px;
    background-color: var(--bg-active);
    color: var(--fg-primary);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.1s;
  }

  .action-btn:hover {
    background-color: var(--bg-hover);
    border-color: var(--accent-primary);
  }

  :global(.mr-2) {
    margin-right: 4px;
  }
</style>
