<script lang="ts">
  import { uiState } from "$lib/ui-state.svelte";
  import { FolderOpen, PlusSquare, Github, Command } from "lucide-svelte";
</script>

<div class="welcome-container">
  <div class="welcome-hero">
    <div class="logo-area">
      <div class="app-logo"></div>
      <h1 class="app-name">Faber Studio</h1>
      <p class="app-tagline">Advanced Agentic Coding Environment</p>
    </div>

    <div class="welcome-grid">
      <div class="welcome-section">
        <h3>Start</h3>
        <button class="welcome-button" onclick={() => uiState.openFolder()}>
          <FolderOpen size={18} />
          <span>Open Folder...</span>
        </button>
        <button
          class="welcome-button"
          onclick={() => (uiState.activeRightActivityId = "init")}
        >
          <PlusSquare size={18} />
          <span>New Project...</span>
        </button>
        <button class="welcome-button">
          <Github size={18} />
          <span>Clone Repository...</span>
        </button>
      </div>

      <div class="welcome-section">
        <h3>Recent</h3>
        <div class="recent-empty">No recent folders</div>
      </div>
    </div>

    <div class="shortcuts-footer">
      <div class="shortcut-item">
        <span class="label">Show All Commands</span>
        <span class="key-hint">Ctrl+Shift+P</span>
      </div>
      <div class="shortcut-item">
        <span class="label">Go to File</span>
        <span class="key-hint">Ctrl+P</span>
      </div>
      <div class="shortcut-item">
        <span class="label">Find in Files</span>
        <span class="key-hint">Ctrl+Shift+F</span>
      </div>
    </div>
  </div>
</div>

<style>
  .welcome-container {
    height: 100%;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--bg-editor);
    color: var(--fg-primary);
  }

  .welcome-hero {
    max-width: 800px;
    width: 100%;
    padding: 40px;
  }

  .logo-area {
    margin-bottom: 64px;
  }

  .app-logo {
    width: 64px;
    height: 64px;
    background-color: var(--accent-primary);
    border-radius: 12px;
    margin-bottom: 16px;
  }

  .app-name {
    font-size: 42px;
    font-weight: 200;
    margin: 0;
    color: var(--fg-primary);
    letter-spacing: -0.5px;
  }

  .app-tagline {
    font-size: 16px;
    color: var(--fg-secondary);
    margin: 4px 0 0 0;
  }

  .welcome-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 48px;
    margin-bottom: 64px;
  }

  .welcome-section h3 {
    font-size: 14px;
    font-weight: 600;
    color: var(--fg-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 16px;
  }

  .welcome-button {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 8px 0;
    background: transparent;
    border: none;
    color: var(--accent-primary);
    font-size: 14px;
    cursor: pointer;
    text-align: left;
    transition: color 0.1s;
  }

  .welcome-button:hover {
    color: var(--fg-primary);
    text-decoration: underline;
  }

  .recent-empty {
    font-size: 13px;
    color: var(--fg-tertiary);
    font-style: italic;
  }

  .shortcuts-footer {
    display: flex;
    flex-wrap: wrap;
    gap: 24px;
    padding-top: 32px;
    border-top: 1px solid var(--border-subtle);
  }

  .shortcut-item {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .shortcut-item .label {
    font-size: 12px;
    color: var(--fg-secondary);
  }

  .shortcut-item .key-hint {
    font-size: 11px;
    background-color: var(--bg-active);
    padding: 2px 6px;
    border-radius: 4px;
    color: var(--fg-primary);
    font-family: var(--font-mono);
  }
</style>
