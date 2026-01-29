<script lang="ts">
  import {
    GitBranch,
    RefreshCw,
    AlertTriangle,
    XCircle,
    Check,
  } from "lucide-svelte";

  let branch = "main";
  let errors = 0;
  let warnings = 2;
  let isSyncing = false;
</script>

<footer class="status-bar">
  <div class="left-items">
    <div class="status-item remote-indicator">
      <span>><span class="remote-text">Local</span></span>
    </div>
    <div class="status-item clickable">
      <GitBranch size={12} />
      <span class="label">{branch}</span>
    </div>
    <div class="status-item clickable">
      <RefreshCw size={12} class={isSyncing ? "spin" : ""} />
    </div>
    <div class="status-item clickable error-stat">
      <XCircle size={12} />
      <span class="label">{errors}</span>
      <AlertTriangle size={12} />
      <span class="label">{warnings}</span>
    </div>
  </div>

  <div class="right-items">
    <div class="status-item">
      <span class="label">Ln 12, Col 34</span>
    </div>
    <div class="status-item">
      <span class="label">UTF-8</span>
    </div>
    <div class="status-item">
      <span class="label">TypeScript React</span>
    </div>
    <div class="status-item clickable">
      <Check size={12} />
      <span class="label">Prettier</span>
    </div>
  </div>
</footer>

<style>
  .status-bar {
    height: 22px;
    background-color: var(--bg-status-bar);
    color: var(--fg-on-accent);
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 11px;
    user-select: none;
    font-family: var(--font-sans);
  }

  .left-items,
  .right-items {
    display: flex;
    height: 100%;
  }

  .status-item {
    display: flex;
    align-items: center;
    padding: 0 8px;
    cursor: default;
    gap: 4px;
    opacity: 0.9;
  }

  .status-item.clickable:hover {
    background-color: rgba(255, 255, 255, 0.12);
    cursor: pointer;
    opacity: 1;
  }

  .remote-indicator {
    background-color: var(--bg-status-bar-remote);
    color: #fff;
    padding: 0 10px;
    font-weight: 600;
  }

  .error-stat {
    gap: 2px;
  }

  .error-stat .label {
    margin-right: 6px;
  }

  .spin {
    animation: spin 2s linear infinite;
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
