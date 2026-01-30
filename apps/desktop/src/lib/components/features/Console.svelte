<script lang="ts">
  import { appConsole } from "$lib/stores/console.svelte";
  import { uiState } from "$lib/ui-state.svelte";
  import { onMount, onDestroy } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { fly } from "svelte/transition";
  import { cn } from "$lib/utils";
  import {
    Play,
    Trash2,
    Terminal,
    Plus,
    X,
    Clock,
    Command,
  } from "lucide-svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";

  let command = $state("");
  let unlisten: any[] = [];
  let logContainer: HTMLElement;
  let inputElement: HTMLInputElement;

  // Suggestion system
  let showSuggestions = $state(false);
  let selectedIndex = $state(0);
  let commandHistory = $state<string[]>([]);
  let detectedPM = $state<string>("npm");

  // Detect package manager when project changes
  $effect(() => {
    if (uiState.projectRoot) {
      invoke("detect_package_manager", { path: uiState.projectRoot })
        .then((pm) => {
          detectedPM = (pm as string) || "npm";
        })
        .catch(() => {
          detectedPM = "npm";
        });
    }
  });

  // Generate commands based on detected package manager
  let predefinedCommands = $derived.by(() => {
    const pm = detectedPM;
    const pmCommands = [
      { cmd: `${pm} run dev`, desc: "Start development server" },
      { cmd: `${pm} run build`, desc: "Build for production" },
      { cmd: `${pm} run tauri dev`, desc: "Run Tauri in dev mode" },
      { cmd: `${pm} run tauri build`, desc: "Build Tauri app" },
      { cmd: `${pm} install`, desc: "Install dependencies" },
      { cmd: `${pm} run test`, desc: "Run tests" },
    ];

    const otherCommands = [
      { cmd: "cargo build", desc: "Build Rust project" },
      { cmd: "cargo run", desc: "Run Rust project" },
      { cmd: "cargo test", desc: "Run Rust tests" },
      { cmd: "cargo check", desc: "Check Rust project" },
      { cmd: "git status", desc: "Show git status" },
      { cmd: "git pull", desc: "Pull from remote" },
      { cmd: "git push", desc: "Push to remote" },
      { cmd: "git log --oneline -10", desc: "Show last 10 commits" },
      { cmd: "ls", desc: "List files" },
      { cmd: "dir", desc: "List files (Windows)" },
      { cmd: "cd ..", desc: "Go to parent directory" },
      { cmd: "clear", desc: "Clear terminal" },
    ];

    return [...pmCommands, ...otherCommands];
  });

  // Filtered suggestions based on input
  let suggestions = $derived.by(() => {
    const query = command.toLowerCase().trim();
    if (!query) {
      // Show history first, then some common commands
      const historyItems = commandHistory.slice(0, 5).map((cmd) => ({
        cmd,
        desc: "Recent",
        isHistory: true,
      }));
      const commonItems = predefinedCommands
        .slice(0, 5 - historyItems.length)
        .map((c) => ({
          ...c,
          isHistory: false,
        }));
      return [...historyItems, ...commonItems];
    }

    // Filter commands that match
    const historyMatches = commandHistory
      .filter((cmd) => cmd.toLowerCase().includes(query))
      .slice(0, 3)
      .map((cmd) => ({ cmd, desc: "Recent", isHistory: true }));

    const predefinedMatches = predefinedCommands
      .filter((c) => c.cmd.toLowerCase().includes(query))
      .slice(0, 5 - historyMatches.length)
      .map((c) => ({ ...c, isHistory: false }));

    return [...historyMatches, ...predefinedMatches];
  });

  function runCommand() {
    if (!command) return;
    const parts = command.trim().split(/\s+/);
    if (parts.length === 0) return;

    // Add to history
    const trimmedCmd = command.trim();
    if (trimmedCmd && !commandHistory.includes(trimmedCmd)) {
      commandHistory = [trimmedCmd, ...commandHistory].slice(0, 20);
    }

    const cmd = parts[0];
    const args = parts.slice(1);

    appConsole.info(`> ${command}`, "Terminal");
    uiState.runTask(cmd, args);
    showSuggestions = false;
  }

  function selectSuggestion(suggestion: { cmd: string }) {
    command = suggestion.cmd;
    showSuggestions = false;
    inputElement?.focus();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (showSuggestions && suggestions.length > 0) {
      if (e.key === "ArrowDown") {
        e.preventDefault();
        selectedIndex = (selectedIndex + 1) % suggestions.length;
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        selectedIndex =
          (selectedIndex - 1 + suggestions.length) % suggestions.length;
      } else if (e.key === "Enter" && selectedIndex >= 0) {
        e.preventDefault();
        selectSuggestion(suggestions[selectedIndex]);
        runCommand();
      } else if (e.key === "Tab" && selectedIndex >= 0) {
        e.preventDefault();
        selectSuggestion(suggestions[selectedIndex]);
      } else if (e.key === "Escape") {
        showSuggestions = false;
      }
    } else if (e.key === "Enter") {
      runCommand();
    }
  }

  function handleFocus() {
    showSuggestions = true;
    selectedIndex = 0;
  }

  function handleBlur() {
    // Delay to allow click on suggestion
    setTimeout(() => {
      showSuggestions = false;
    }, 150);
  }

  function createNewSession() {
    appConsole.createSession(`Terminal ${appConsole.sessions.length + 1}`);
  }

  onMount(async () => {
    unlisten.push(
      await listen("runner-stdout", (event: any) => {
        appConsole.info(event.payload.message, "Runner");
      }),
    );
    unlisten.push(
      await listen("runner-stderr", (event: any) => {
        appConsole.warn(event.payload.message, "Runner");
      }),
    );
    unlisten.push(
      await listen("runner-error", (event: any) => {
        appConsole.error(event.payload.error, "Runner");
      }),
    );
    unlisten.push(
      await listen("runner-exit", (event: any) => {
        appConsole.success(
          `Process exited with code ${event.payload.code}`,
          "Runner",
        );
      }),
    );
  });

  onDestroy(() => {
    unlisten.forEach((u) => u());
  });

  $effect(() => {
    if (appConsole.entries && logContainer) {
      setTimeout(() => {
        logContainer.scrollTop = logContainer.scrollHeight;
      }, 0);
    }
  });
</script>

<div class="console-container">
  <div class="console-tabs">
    {#each appConsole.sessions as session}
      <div
        class="console-tab {appConsole.activeSessionId === session.id
          ? 'active'
          : ''}"
        onclick={() => (appConsole.activeSessionId = session.id)}
        role="button"
        tabindex="0"
        onkeydown={(e) =>
          e.key === "Enter" && (appConsole.activeSessionId = session.id)}
      >
        <span class="tab-label">{session.name}</span>
        {#if appConsole.sessions.length > 1}
          <button
            class="close-tab"
            onclick={(e) => {
              e.stopPropagation();
              appConsole.closeSession(session.id);
            }}
            title="Close Tab"
          >
            <X size={10} />
          </button>
        {/if}
      </div>
    {/each}
    <button class="add-tab" onclick={createNewSession} title="New Terminal">
      <Plus size={14} />
    </button>
  </div>

  <div class="console-output selectable" bind:this={logContainer}>
    {#each appConsole.entries as entry (entry.id)}
      <div
        class={cn("log-entry", entry.level)}
        transition:fly={{ x: -10, duration: 100 }}
      >
        <span class="timestamp"
          >{entry.timestamp.toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit",
            hour12: false,
          })}</span
        >
        <span class="source">[{entry.source}]</span>
        <span class="message">{entry.message}</span>
      </div>
    {/each}
    {#if appConsole.entries.length === 0}
      <div class="empty-state">Console Ready.</div>
    {/if}
  </div>

  <div class="console-input-bar">
    {#if uiState.projectRoot}
      <div class="cwd-indicator" title={uiState.projectRoot}>
        {uiState.projectRoot}
      </div>
    {/if}
    <div class="input-row">
      <div class="input-inner">
        <Terminal size={12} class="prompt-icon" />
        <input
          type="text"
          bind:this={inputElement}
          bind:value={command}
          placeholder="Type command here..."
          class="cmd-input-native"
          onkeydown={handleKeydown}
          onfocus={handleFocus}
          onblur={handleBlur}
        />

        {#if showSuggestions && suggestions.length > 0}
          <div
            class="suggestions-dropdown"
            transition:fly={{ y: -5, duration: 100 }}
          >
            {#each suggestions as suggestion, i}
              <button
                class="suggestion-item {selectedIndex === i ? 'selected' : ''}"
                onmousedown={() => selectSuggestion(suggestion)}
                onmouseenter={() => (selectedIndex = i)}
              >
                <span class="suggestion-icon">
                  {#if suggestion.isHistory}
                    <Clock size={12} />
                  {:else}
                    <Command size={12} />
                  {/if}
                </span>
                <span class="suggestion-cmd">{suggestion.cmd}</span>
                <span class="suggestion-desc">{suggestion.desc}</span>
              </button>
            {/each}
          </div>
        {/if}
      </div>
      <div class="bar-actions">
        <Button variant="primary" size="sm" onclick={runCommand}>
          <Play size={12} class="mr-1" /> Run
        </Button>
      </div>
    </div>
  </div>
</div>

<style>
  .console-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-app);
  }

  .console-tabs {
    display: flex;
    flex-wrap: nowrap;
    height: 30px;
    background-color: var(--bg-tab-inactive);
    border-bottom: 1px solid var(--border-subtle);
    padding: 0 4px;
    gap: 1px;
    align-items: flex-end;
    overflow-x: auto;
    overflow-y: hidden;
  }

  .console-tabs::-webkit-scrollbar {
    height: 3px;
  }

  .console-tabs::-webkit-scrollbar-thumb {
    background: var(--bg-active);
    border-radius: 2px;
  }

  .console-tab {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 12px;
    height: 26px;
    background-color: transparent;
    color: var(--fg-tertiary);
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    border-radius: 4px 4px 0 0;
    position: relative;
    user-select: none;
    flex-shrink: 0;
    white-space: nowrap;
  }

  .console-tab:hover {
    color: var(--fg-secondary);
    background-color: var(--bg-hover);
  }

  .console-tab.active {
    background-color: var(--bg-app);
    color: var(--fg-primary);
    border: 1px solid var(--border-subtle);
    border-bottom: none;
  }

  .close-tab {
    background: transparent;
    border: none;
    color: inherit;
    opacity: 0.5;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2px;
    border-radius: 2px;
  }

  .close-tab:hover {
    opacity: 1;
    background-color: var(--bg-active);
  }

  .add-tab {
    background: transparent;
    border: none;
    color: var(--fg-tertiary);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    margin-bottom: 4px;
    cursor: pointer;
    border-radius: 4px;
  }

  .add-tab:hover {
    color: var(--fg-primary);
    background-color: var(--bg-hover);
  }

  .console-output {
    flex: 1;
    overflow-y: auto;
    padding: 8px 12px;
    font-family: var(--font-mono);
    font-size: 12px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .selectable {
    user-select: text !important;
    -webkit-user-select: text !important;
  }

  .console-input-bar {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 8px 12px;
    border-top: 1px solid var(--border-subtle);
    background-color: var(--bg-panel);
  }

  .cwd-indicator {
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--fg-tertiary);
    opacity: 0.6;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    padding-left: 30px;
  }

  .input-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .input-inner {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
  }

  :global(.prompt-icon) {
    position: absolute;
    left: 10px;
    color: var(--accent-success);
    z-index: 2;
    margin-right: 8px;
  }

  :global(.cmd-input) {
    padding-left: 30px !important;
    font-family: var(--font-mono) !important;
    height: 28px !important;
    font-size: 12px !important;
    background: var(--bg-input) !important;
  }

  .bar-actions {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .log-entry {
    display: flex;
    gap: 12px;
    line-height: 1.5;
    white-space: pre-wrap;
    word-break: break-all;
  }

  .timestamp {
    color: var(--fg-tertiary);
    min-width: 65px;
    font-variant-numeric: tabular-nums;
  }

  .source {
    color: var(--accent-primary);
    min-width: 80px;
    font-weight: 600;
  }

  .log-entry.info .message {
    color: var(--fg-secondary);
  }
  .log-entry.success .message {
    color: var(--accent-success);
  }
  .log-entry.warn .message {
    color: var(--accent-warning);
  }
  .log-entry.error .message {
    color: var(--accent-error);
  }

  .empty-state {
    color: var(--fg-tertiary);
    font-style: italic;
    padding: 20px;
    text-align: center;
  }

  /* Scrollbar styling */
  .console-output::-webkit-scrollbar {
    width: 6px;
  }
  .console-output::-webkit-scrollbar-thumb {
    background: var(--bg-active);
    border-radius: 3px;
  }

  /* Native input styling */
  .cmd-input-native {
    flex: 1;
    padding: 6px 10px 6px 30px;
    font-family: var(--font-mono);
    font-size: 12px;
    height: 28px;
    background: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    color: var(--fg-primary);
    outline: none;
    width: 100%;
  }

  .cmd-input-native:focus {
    border-color: var(--accent-primary);
  }

  .cmd-input-native::placeholder {
    color: var(--fg-tertiary);
  }

  /* Suggestion dropdown */
  .suggestions-dropdown {
    position: absolute;
    bottom: 100%;
    left: 0;
    right: 0;
    background: var(--bg-panel);
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    margin-bottom: 4px;
    box-shadow: 0 -4px 12px rgba(0, 0, 0, 0.3);
    overflow: hidden;
    z-index: 100;
    max-height: 200px;
    overflow-y: auto;
  }

  .suggestion-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    cursor: pointer;
    font-size: 12px;
    background: transparent;
    border: none;
    width: 100%;
    text-align: left;
    color: var(--fg-secondary);
  }

  .suggestion-item:hover,
  .suggestion-item.selected {
    background-color: var(--bg-hover);
    color: var(--fg-primary);
  }

  .suggestion-icon {
    color: var(--fg-tertiary);
    display: flex;
    align-items: center;
  }

  .suggestion-item.selected .suggestion-icon,
  .suggestion-item:hover .suggestion-icon {
    color: var(--accent-primary);
  }

  .suggestion-cmd {
    font-family: var(--font-mono);
    color: var(--fg-primary);
    flex: 1;
  }

  .suggestion-desc {
    font-size: 11px;
    color: var(--fg-tertiary);
    opacity: 0.7;
  }
</style>
