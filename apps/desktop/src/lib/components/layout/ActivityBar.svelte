<script lang="ts">
  import { uiState } from "$lib/ui-state.svelte";
  import {
    Files,
    Search,
    GitGraph,
    Bug,
    Settings,
    UserCircle,
    Terminal,
  } from "lucide-svelte";

  let activities = [
    { id: "explorer", icon: Files, label: "Explorer", href: "/" },
    { id: "search", icon: Search, label: "Search", href: "/search" },
    { id: "git", icon: GitGraph, label: "Source Control", href: "/git" },
    { id: "debug", icon: Bug, label: "Run and Debug", href: "/debug" },
    {
      id: "commands",
      icon: Terminal,
      label: "Quick Commands",
      href: "/commands",
    },
  ];

  let bottomActivities = [
    {
      id: "settings",
      icon: Settings,
      label: "Settings",
      onclick: () => uiState.openSettings(),
    },
  ];
</script>

<aside
  class="activity-bar"
  oncontextmenu={(e) => {
    if (e.target !== e.currentTarget) return;
    e.preventDefault();
    uiState.showContextMenu(e.clientX, e.clientY, [
      {
        label: "Explorer",
        onclick: () => (uiState.activeActivityId = "explorer"),
      },
      { label: "Search", onclick: () => (uiState.activeActivityId = "search") },
      {
        label: "Source Control",
        onclick: () => (uiState.activeActivityId = "git"),
      },
      {
        label: "Run and Debug",
        onclick: () => (uiState.activeActivityId = "debug"),
      },
      {
        label: "Quick Commands",
        onclick: () => (uiState.activeActivityId = "commands"),
      },
      { separator: true },
      { label: "Accounts", onclick: () => {}, disabled: true },
      { label: "Manage", onclick: () => uiState.openSettings() },
    ]);
  }}
>
  <div class="top-actions">
    {#each activities as action}
      <button
        class="action-item {uiState.activeActivityId === action.id
          ? 'active'
          : ''}"
        onclick={() => (uiState.activeActivityId = action.id)}
        oncontextmenu={(e) => {
          e.preventDefault();
          e.stopPropagation();
          uiState.showContextMenu(e.clientX, e.clientY, [
            {
              label: `Hide '${action.label}'`,
              onclick: () => {},
              disabled: true,
            },
            { label: "Reset Location", onclick: () => {}, disabled: true },
          ]);
        }}
        title={action.label}
      >
        <action.icon size={24} strokeWidth={1.5} />
      </button>
    {/each}
  </div>

  <div class="bottom-actions">
    {#each bottomActivities as action}
      <button
        class="action-item {uiState.activeActivityId === action.id
          ? 'active'
          : ''}"
        onclick={() => {
          if (action.onclick) action.onclick();
          else uiState.activeActivityId = action.id;
        }}
        oncontextmenu={(e) => {
          e.preventDefault();
          e.stopPropagation();
          uiState.showContextMenu(e.clientX, e.clientY, [
            {
              label: `Hide '${action.label}'`,
              onclick: () => {},
              disabled: true,
            },
          ]);
        }}
        title={action.label}
      >
        <action.icon size={24} strokeWidth={1.5} />
      </button>
    {/each}
  </div>
</aside>

<style>
  .activity-bar {
    width: 48px;
    height: 100%;
    background-color: var(--bg-activity-bar);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    border-right: 1px solid var(--border-subtle);
  }

  .action-item {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: var(--fg-secondary);
    cursor: pointer;
    position: relative;
    opacity: 0.7;
    transition:
      opacity 0.1s,
      color 0.1s;
    outline: none;
  }

  .action-item:hover {
    color: var(--fg-primary);
    opacity: 1;
  }

  .action-item.active {
    color: var(--fg-primary);
    opacity: 1;
  }

  .action-item.active::before {
    content: "";
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 2px;
    background-color: var(--accent-primary);
  }
</style>
