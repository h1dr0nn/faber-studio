<script lang="ts">
  import {
    ChevronDown,
    ChevronRight,
    Type,
    CaseSensitive,
    WholeWord,
    Regex,
    Replace,
    ListFilter,
    X,
    Loader2,
  } from "lucide-svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Button from "$lib/components/ui/Button.svelte";

  import { uiState } from "$lib/ui-state.svelte";

  let searchQuery = $state("");
  let replaceQuery = $state("");
  let isReplaceVisible = $state(false);
  let isIncludesVisible = $state(false);
  let isSearching = $state(false);

  let searchOptions = $state({
    matchCase: false,
    wholeWord: false,
    useRegex: false,
  });

  // Debounce timer
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Auto-search with debounce when query changes
  $effect(() => {
    if (debounceTimer) clearTimeout(debounceTimer);

    if (searchQuery.trim().length >= 2) {
      isSearching = true;
      debounceTimer = setTimeout(async () => {
        await uiState.searchProject(searchQuery);
        isSearching = false;
      }, 300);
    } else {
      isSearching = false;
    }
  });

  // Group flat results by file
  let groupedResults = $derived.by(() => {
    const map = new Map();
    for (const res of uiState.searchResults) {
      if (!res) continue; // Safety check
      if (!map.has(res.file)) {
        map.set(res.file, { file: res.file, matches: [] });
      }
      map.get(res.file).matches.push({ line: res.line, text: res.content });
    }
    return Array.from(map.values());
  });

  function handleSearch(e: KeyboardEvent) {
    if (e.key === "Enter") {
      if (debounceTimer) clearTimeout(debounceTimer);
      uiState.searchProject(searchQuery);
    }
  }
</script>

<div class="feature-container">
  <div class="search-header">
    <div class="search-input-group">
      <button
        class="toggle-replace {isReplaceVisible ? 'active' : ''}"
        onclick={() => (isReplaceVisible = !isReplaceVisible)}
        title="Toggle Replace"
      >
        <ChevronRight size={16} class={isReplaceVisible ? "rotated" : ""} />
      </button>
      <div class="input-wrapper">
        <Input
          placeholder="Search"
          bind:value={searchQuery}
          autofocus
          onkeydown={handleSearch}
        />

        <div class="input-actions">
          <button
            class="action-icon {searchOptions.matchCase ? 'active' : ''}"
            onclick={() => (searchOptions.matchCase = !searchOptions.matchCase)}
            title="Match Case"
          >
            <CaseSensitive size={16} />
          </button>
          <button
            class="action-icon {searchOptions.wholeWord ? 'active' : ''}"
            onclick={() => (searchOptions.wholeWord = !searchOptions.wholeWord)}
            title="Match Whole Word"
          >
            <WholeWord size={16} />
          </button>
          <button
            class="action-icon {searchOptions.useRegex ? 'active' : ''}"
            onclick={() => (searchOptions.useRegex = !searchOptions.useRegex)}
            title="Use Regular Expression"
          >
            <Regex size={16} />
          </button>
        </div>
      </div>
    </div>

    {#if isReplaceVisible}
      <div class="replace-input-group">
        <div class="input-wrapper">
          <Input placeholder="Replace" bind:value={replaceQuery} />
          <div class="input-actions">
            <button class="action-icon" title="Replace All">
              <Replace size={16} />
            </button>
          </div>
        </div>
      </div>
    {/if}

    <div class="filter-toggle">
      <button
        class="text-btn"
        onclick={() => (isIncludesVisible = !isIncludesVisible)}
      >
        <ListFilter size={14} />
        <span>Files to include/exclude</span>
      </button>
    </div>

    {#if isIncludesVisible}
      <div class="filters-group">
        <div class="filter-item">
          <label for="search-include">files to include</label>
          <Input id="search-include" placeholder="e.g. *.ts, src/**" />
        </div>
        <div class="filter-item">
          <label for="search-exclude">files to exclude</label>
          <Input id="search-exclude" placeholder="e.g. node_modules, dist" />
        </div>
      </div>
    {/if}
  </div>

  <div class="results-area">
    {#if searchQuery}
      <div class="results-header">
        {#if isSearching}
          <Loader2 size={12} class="spin" />
          <span>Searching...</span>
        {:else}
          {groupedResults.length} files found
        {/if}
      </div>
      <div class="results-list">
        {#each groupedResults as result}
          <div class="file-group">
            <button
              class="file-header"
              onclick={() =>
                uiState.openFile(
                  result.file,
                  result.file.split(/[\\/]/).pop() || "",
                )}
            >
              <ChevronDown size={14} />
              <span class="file-name">{result.file.split(/[\\/]/).pop()}</span>
              <span class="file-path"
                >{result.file.split(/[\\/]/).slice(0, -1).join("/")}</span
              >
              <span class="match-count">{result.matches.length}</span>
            </button>
            <div class="file-matches">
              {#each result.matches as match}
                <button
                  class="match-item"
                  onclick={() =>
                    uiState.openFile(
                      result.file,
                      result.file.split(/[\\/]/).pop() || "",
                    )}
                >
                  <span class="line-number">{match.line}</span>
                  <span class="match-text">
                    {@html match.text.replace(
                      searchQuery,
                      `<span class="highlight">${searchQuery}</span>`,
                    )}
                  </span>
                </button>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="empty-results">
        <p>Type to search in project</p>
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

  .search-header {
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .search-input-group,
  .replace-input-group {
    display: flex;
    align-items: flex-start;
    gap: 4px;
  }

  .toggle-replace {
    background: none;
    border: none;
    color: var(--fg-tertiary);
    cursor: pointer;
    padding: 4px 0;
    margin-top: 4px;
  }

  :global(.rotated) {
    transform: rotate(90deg);
  }

  .input-wrapper {
    flex: 1;
    position: relative;
  }

  .input-actions {
    position: absolute;
    right: 4px;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    gap: 2px;
  }

  .action-icon {
    background: none;
    border: none;
    color: var(--fg-tertiary);
    cursor: pointer;
    padding: 4px;
    border-radius: 3px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .action-icon:hover {
    background-color: var(--bg-hover);
    color: var(--fg-secondary);
  }

  .action-icon.active {
    background-color: var(--accent-primary);
    color: #fff;
  }

  .filter-toggle {
    margin-top: 4px;
  }

  .text-btn {
    background: none;
    border: none;
    color: var(--fg-tertiary);
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    padding: 0;
  }

  .text-btn:hover {
    color: var(--fg-secondary);
  }

  .filters-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 4px 0;
  }

  .filter-item label {
    display: block;
    font-size: 10px;
    margin-bottom: 4px;
    text-transform: uppercase;
    opacity: 0.7;
  }

  .results-area {
    flex: 1;
    overflow-y: auto;
  }

  .results-header {
    padding: 8px 12px;
    font-size: 11px;
    opacity: 0.7;
    border-bottom: 1px solid var(--border-subtle);
  }

  .file-group {
    display: flex;
    flex-direction: column;
  }

  .file-header {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 12px;
    cursor: pointer;
    font-size: 13px;
    background-color: var(--bg-hover);
    border: none;
    width: 100%;
    text-align: left;
    color: inherit;
  }

  .file-name {
    color: var(--fg-primary);
  }

  .file-path {
    font-size: 11px;
    opacity: 0.5;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .match-count {
    background-color: var(--border-subtle);
    padding: 0 6px;
    border-radius: 10px;
    font-size: 11px;
  }

  .match-item {
    display: flex;
    gap: 8px;
    padding: 2px 12px 2px 32px;
    cursor: pointer;
    font-size: 12px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    background: none;
    border: none;
    width: 100%;
    text-align: left;
    color: inherit;
  }

  .match-item:hover {
    background-color: var(--bg-hover);
  }

  .line-number {
    color: var(--accent-primary);
    min-width: 20px;
    text-align: right;
  }

  .match-text {
    overflow: hidden;
    text-overflow: ellipsis;
  }

  :global(.highlight) {
    background-color: rgba(255, 255, 0, 0.3);
    color: var(--fg-primary);
  }

  .empty-results {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    opacity: 0.5;
    font-size: 13px;
  }
</style>
