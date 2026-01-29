<script lang="ts">
  import {
    Github,
    FolderOpen,
    Loader2,
    CheckCircle2,
    AlertCircle,
  } from "lucide-svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { uiState } from "$lib/ui-state.svelte";
  import { open } from "@tauri-apps/plugin-dialog";

  let repoUrl = $state("");
  let parentDir = $state("");
  let error = $state("");
  let success = $state(false);

  async function selectParentDir() {
    try {
      const selected = await open({ directory: true, multiple: false });
      if (selected && typeof selected === "string") {
        parentDir = selected;
      }
    } catch (err) {
      console.error("Failed to select directory:", err);
    }
  }

  async function handleClone() {
    error = "";
    success = false;

    if (!repoUrl) {
      error = "Please enter a repository URL.";
      return;
    }
    if (!parentDir) {
      error = "Please select a target directory.";
      return;
    }

    try {
      await uiState.cloneRepository(repoUrl, parentDir);
      success = true;
      repoUrl = "";
    } catch (err: any) {
      error = err.message || "An unexpected error occurred while cloning.";
    }
  }
</script>

<div class="feature-container">
  <div class="clone-header">
    <Github size={20} />
    <span>CLONE REPOSITORY</span>
  </div>

  <div class="clone-content">
    <div class="input-group">
      <label for="repo-url">Repository URL</label>
      <Input
        id="repo-url"
        placeholder="https://github.com/username/repo.git"
        bind:value={repoUrl}
        disabled={uiState.isCloning}
      />
    </div>

    <div class="input-group">
      <label for="target-dir">Target Directory</label>
      <div class="dir-input">
        <Input
          id="target-dir"
          placeholder="Select a parent folder"
          bind:value={parentDir}
          disabled={uiState.isCloning}
          readonly
        />
        <button
          class="dir-btn"
          onclick={selectParentDir}
          disabled={uiState.isCloning}
          title="Select Directory"
        >
          <FolderOpen size={16} />
        </button>
      </div>
    </div>

    <div class="actions">
      <Button
        variant="primary"
        class="w-full"
        onclick={handleClone}
        disabled={uiState.isCloning}
      >
        {#if uiState.isCloning}
          <Loader2 size={16} class="spin" />
          <span>Cloning...</span>
        {:else}
          <span>Clone Repository</span>
        {/if}
      </Button>
    </div>

    {#if uiState.isCloning}
      <div class="progress-box">
        <p>{uiState.cloningProgress}</p>
      </div>
    {/if}

    {#if error}
      <div class="message error">
        <AlertCircle size={16} />
        <span>{error}</span>
      </div>
    {/if}

    {#if success}
      <div class="message success">
        <CheckCircle2 size={16} />
        <span>Successfully cloned and opened!</span>
      </div>
    {/if}

    <div class="info-section">
      <p>
        Tip: You can clone from any Git provider (GitHub, GitLab, Bitbucket,
        etc.).
      </p>
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

  .clone-header {
    padding: 12px 16px;
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 11px;
    font-weight: 700;
    border-bottom: 1px solid var(--border-subtle);
    color: var(--accent-primary);
  }

  .clone-content {
    padding: 20px 16px;
    display: flex;
    flex-direction: column;
    gap: 20px;
    overflow-y: auto;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .input-group label {
    font-size: 11px;
    font-weight: 600;
    color: var(--fg-secondary);
    text-transform: uppercase;
  }

  .dir-input {
    display: flex;
    gap: 8px;
  }

  .dir-input :global(input) {
    flex: 1;
  }

  .dir-btn {
    background: var(--bg-active);
    border: 1px solid var(--border-subtle);
    color: var(--fg-primary);
    border-radius: 4px;
    padding: 0 10px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .dir-btn:hover {
    background-color: var(--bg-hover);
  }

  .dir-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .actions {
    margin-top: 8px;
  }

  .progress-box {
    padding: 12px;
    background-color: var(--bg-active);
    border-radius: 4px;
    font-size: 12px;
    color: var(--fg-primary);
    text-align: center;
  }

  .message {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px;
    border-radius: 4px;
    font-size: 13px;
  }

  .message.error {
    background-color: rgba(241, 76, 76, 0.1);
    color: var(--accent-error);
    border: 1px solid var(--accent-error);
  }

  .message.success {
    background-color: rgba(22, 130, 93, 0.1);
    color: var(--accent-success);
    border: 1px solid var(--accent-success);
  }

  .info-section {
    margin-top: auto;
    padding-top: 20px;
    font-size: 11px;
    opacity: 0.6;
    font-style: italic;
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
</style>
