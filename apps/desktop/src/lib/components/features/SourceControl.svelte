<script lang="ts">
  import {
    GitCommit,
    RefreshCw,
    Check,
    MoreHorizontal,
    Plus,
    Minus,
    Undo2,
    CheckCheck,
  } from "lucide-svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import TreeItem from "$lib/components/ui/TreeItem.svelte";

  import { uiState } from "$lib/ui-state.svelte";
  import { onMount } from "svelte";

  let commitMessage = $state("");

  async function handleCommit() {
    if (!commitMessage) return;
    await uiState.commitChanges(commitMessage);
    commitMessage = "";
  }

  function handleStage(file: string) {
    uiState.stageFile(file);
  }

  function handleUnstage(file: string) {
    uiState.unstageFile(file);
  }

  onMount(() => {
    uiState.refreshGitStatus();
  });

  let isStagedExpanded = $state(true);
  let isChangesExpanded = $state(true);
</script>

<div class="feature-container">
  <div class="scm-header">
    <div class="header-actions">
      <span>SOURCE CONTROL</span>
      <div class="actions">
        <button
          class="icon-btn"
          title="Refresh"
          onclick={() => uiState.refreshGitStatus()}
        >
          <RefreshCw size={14} />
        </button>

        <button class="icon-btn" title="Commit & Push">
          <CheckCheck size={14} />
        </button>
        <button class="icon-btn" title="More Actions">
          <MoreHorizontal size={14} />
        </button>
      </div>
    </div>

    <div class="commit-box">
      <Input
        placeholder="Message (Ctrl+Enter to commit on 'main')"
        bind:value={commitMessage}
        onkeydown={(e) => {
          if ((e.ctrlKey || e.metaKey) && e.key === "Enter") handleCommit();
        }}
      />
      <div class="primary-action">
        <Button
          variant="primary"
          size="sm"
          class="w-full"
          onclick={handleCommit}>Commit</Button
        >
      </div>
    </div>
  </div>

  <div class="scm-content">
    <div class="scm-section">
      <div
        class="section-header"
        onclick={() => (isStagedExpanded = !isStagedExpanded)}
        role="button"
        tabindex="0"
        onkeydown={(e) =>
          e.key === "Enter" && (isStagedExpanded = !isStagedExpanded)}
      >
        <TreeItem
          label="STAGED CHANGES"
          isFolder={true}
          bind:expanded={isStagedExpanded}
        />
        <span class="badge">{uiState.gitChanges.staged.length}</span>
      </div>

      {#if isStagedExpanded}
        <div class="section-items">
          {#each uiState.gitChanges.staged as item}
            <div class="scm-item">
              <span class="item-icon {item.status.toLowerCase()}"
                >{item.status}</span
              >
              <span class="item-label">{item.path.split(/[\\/]/).pop()}</span>
              <span class="item-path"
                >{item.path.split(/[\\/]/).slice(0, -1).join("/")}</span
              >
              <div class="item-actions">
                <button
                  class="icon-btn"
                  title="Unstage Changes"
                  onclick={() => handleUnstage(item.path)}
                >
                  <Minus size={14} />
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <div class="scm-section">
      <div
        class="section-header"
        onclick={() => (isChangesExpanded = !isChangesExpanded)}
        role="button"
        tabindex="0"
        onkeydown={(e) =>
          e.key === "Enter" && (isChangesExpanded = !isChangesExpanded)}
      >
        <TreeItem
          label="CHANGES"
          isFolder={true}
          bind:expanded={isChangesExpanded}
        />
        <span class="badge">{uiState.gitChanges.unstaged.length}</span>
      </div>

      {#if isChangesExpanded}
        <div class="section-items">
          {#each uiState.gitChanges.unstaged as item}
            <div class="scm-item">
              <span class="item-icon {item.status.toLowerCase()}"
                >{item.status}</span
              >
              <span class="item-label">{item.path.split(/[\\/]/).pop()}</span>
              <span class="item-path"
                >{item.path.split(/[\\/]/).slice(0, -1).join("/")}</span
              >
              <div class="item-actions">
                <button class="icon-btn" title="Discard Changes">
                  <Undo2 size={14} />
                </button>
                <button
                  class="icon-btn"
                  title="Stage Changes"
                  onclick={() => handleStage(item.path)}
                >
                  <Plus size={14} />
                </button>
              </div>
            </div>
          {/each}
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

  .scm-header {
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

  .actions {
    display: flex;
    gap: 4px;
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

  .commit-box {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  :global(.w-full) {
    width: 100%;
  }

  .scm-content {
    flex: 1;
    overflow-y: auto;
  }

  .scm-section {
    display: flex;
    flex-direction: column;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-right: 12px;
    cursor: pointer;
  }

  .section-header :global(.tree-row) {
    flex: 1;
  }

  .badge {
    background-color: var(--border-subtle);
    padding: 0 6px;
    border-radius: 10px;
    font-size: 11px;
    min-width: 18px;
    text-align: center;
  }

  .scm-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 12px 4px 28px;
    cursor: pointer;
    font-size: 13px;
  }

  .scm-item:hover {
    background-color: var(--bg-hover);
  }

  .scm-item:hover .item-actions {
    display: flex;
  }

  .item-icon {
    font-size: 10px;
    font-weight: 800;
    width: 14px;
    text-align: center;
  }

  .item-icon.m {
    color: #e2c08d;
  } /* Modified - Yellowish */
  .item-icon.a {
    color: #81b88b;
  } /* Added - Greenish */
  .item-icon.d {
    color: #c74e61;
  } /* Deleted - Reddish */

  .item-label {
    color: var(--fg-primary);
  }

  .item-path {
    font-size: 11px;
    opacity: 0.5;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
  }

  .item-actions {
    display: none;
    gap: 2px;
  }
</style>
