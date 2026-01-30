<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, tick } from "svelte";
  import {
    Play,
    Hammer,
    Smartphone,
    Apple,
    Package,
    TestTube,
    FileSearch,
    Download,
    ChevronDown,
    ChevronRight,
    Loader2,
    Check,
    AlertCircle,
    Zap,
    ExternalLink,
    Terminal,
  } from "lucide-svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import TreeItem from "$lib/components/ui/TreeItem.svelte";
  import { uiState } from "$lib/ui-state.svelte";
  import { appConsole } from "$lib/stores/console.svelte";

  interface ProjectInfo {
    package_manager: string;
    project_type: string;
    has_tauri: boolean;
    has_android: boolean;
    has_ios: boolean;
    has_node_modules: boolean;
  }

  interface AndroidVersion {
    version_code: number;
    version_name: string;
  }

  let projectInfo = $state<ProjectInfo | null>(null);
  let androidVersion = $state<AndroidVersion | null>(null);
  let isLoading = $state(false);
  let runningCommand = $state<string | null>(null);

  let sections = $state({
    development: true,
    tauri: true,
    mobile: false,
    testing: false,
    dependencies: false,
  });

  function getPmRunCommand(): string {
    if (!projectInfo) return "npm";
    switch (projectInfo.package_manager) {
      case "bun":
        return "bun";
      case "pnpm":
        return "pnpm";
      case "yarn":
        return "yarn";
      default:
        return "npm";
    }
  }

  function getPmExecCommand(): string {
    if (!projectInfo) return "npx";
    switch (projectInfo.package_manager) {
      case "bun":
        return "bunx";
      case "pnpm":
        return "pnpm";
      case "yarn":
        return "yarn";
      default:
        return "npx";
    }
  }

  async function runCommand(
    name: string,
    command: string,
    args: string[] = [],
  ) {
    if (!uiState.projectRoot) {
      appConsole.warn("No project folder opened", "Quick Commands");
      return;
    }

    runningCommand = name;
    uiState.activeBottomPanelTab = "console";
    uiState.isBottomPanelVisible = true;

    appConsole.info(`Running: ${command} ${args.join(" ")}`, "Quick Commands");

    try {
      await uiState.runTask(command, args);
    } catch (err: any) {
      appConsole.error(`Failed: ${err}`, "Quick Commands");
    } finally {
      runningCommand = null;
    }
  }

  async function loadProjectInfo() {
    if (!uiState.projectRoot) return;

    isLoading = true;
    try {
      projectInfo = await invoke("detect_project_info", {
        path: uiState.projectRoot,
      });

      if (projectInfo?.has_android) {
        try {
          androidVersion = await invoke("read_android_version", {
            path: uiState.projectRoot,
          });
        } catch {
          androidVersion = null;
        }
      }
    } catch (err) {
      console.error("Failed to detect project info:", err);
    } finally {
      isLoading = false;
    }
  }

  async function buildAndroidRelease() {
    if (!uiState.projectRoot) return;

    try {
      // 1. Get Date Stamp (YYMMDD)
      const now = new Date();
      const dateStamp = now.toISOString().slice(2, 10).replace(/-/g, "");

      // 2. Read App Name from tauri.conf.json
      let appName = "app";
      try {
        const configPath = `${uiState.projectRoot}/src-tauri/tauri.conf.json`;
        const configContent = (await invoke("read_file", {
          path: configPath,
        })) as string;
        const config = JSON.parse(configContent);
        appName = config.productName || "app";
      } catch (e) {
        console.warn("Failed to read productName from tauri.conf.json", e);
      }

      // 3. Increment Android Version Code (based on count or logic in Rust)
      appConsole.info("Updating Android version code...", "Quick Commands");
      const newVersion = (await invoke("update_android_version", {
        path: uiState.projectRoot,
      })) as AndroidVersion;
      androidVersion = newVersion;

      appConsole.success(
        `Version set to ${newVersion.version_name} (${newVersion.version_code})`,
        "Quick Commands",
      );

      // 4. Run Tauri Build
      appConsole.info(
        `Starting production build for ${appName}...`,
        "Quick Commands",
      );
      await runCommand("android-release", getPmExecCommand(), [
        "tauri",
        "android",
        "build",
        "--release",
      ]);

      // 5. Discover Artifacts
      appConsole.info("Searching for built artifacts...", "Quick Commands");
      const artifacts = (await invoke("find_android_artifacts", {
        path: uiState.projectRoot,
      })) as any[];

      if (artifacts.length === 0) {
        appConsole.warn("No artifacts found after build.", "Quick Commands");
        return;
      }

      // 6. Manage Artifacts (Copy & Rename)
      const versionStr = newVersion.version_name;

      for (const artifact of artifacts) {
        // Filter for release artifacts (avoiding debug if possible)
        const lowerName = artifact.name.toLowerCase();
        if (!lowerName.includes("release")) continue;

        const targetName = `${appName.replace(/\s+/g, "")}_${dateStamp}_v${versionStr}.${artifact.extension}`;

        try {
          const finalPath = (await invoke("manage_build_artifact", {
            projectRoot: uiState.projectRoot,
            sourcePath: artifact.path,
            targetName: targetName,
          })) as string;

          appConsole.success(`Artifact saved: ${targetName}`, "Quick Commands");
          appConsole.info(`Path: ${finalPath}`, "Quick Commands");
        } catch (e) {
          appConsole.error(
            `Failed to move ${artifact.name}: ${e}`,
            "Quick Commands",
          );
        }
      }
    } catch (err: any) {
      appConsole.error(`Build failed: ${err}`, "Quick Commands");
    }
  }

  async function buildIOS() {
    if (!uiState.projectRoot) return;

    await runCommand("ios-build", getPmExecCommand(), [
      "tauri",
      "ios",
      "build",
    ]);

    try {
      await invoke("open_xcode_project", { path: uiState.projectRoot });
      appConsole.success("Opened Xcode project", "Quick Commands");
    } catch (err: any) {
      if (!err.toString().includes("NotSupported")) {
        appConsole.warn(`Could not open Xcode: ${err}`, "Quick Commands");
      }
    }
  }

  onMount(() => {
    loadProjectInfo();
  });

  $effect(() => {
    if (uiState.projectRoot) {
      loadProjectInfo();
    }
  });

  interface CommandItem {
    id: string;
    label: string;
    description?: string;
    icon: any;
    command: () => void;
    disabled?: boolean;
    badge?: string;
    color?: string;
  }

  const devCommands: CommandItem[] = [
    {
      id: "dev",
      label: "Start Dev Server",
      description: "Run pnpm dev",
      icon: Play,
      color: "#10b981",
      command: () => runCommand("dev", getPmRunCommand(), ["run", "dev"]),
    },
    {
      id: "build",
      label: "Build Project",
      description: "Compile for production",
      icon: Hammer,
      color: "#007acc",
      command: () => runCommand("build", getPmRunCommand(), ["run", "build"]),
    },
    {
      id: "preview",
      label: "Preview Build",
      description: "Serve built files locally",
      icon: ExternalLink,
      color: "#8b5cf6",
      command: () =>
        runCommand("preview", getPmRunCommand(), ["run", "preview"]),
    },
  ];

  const testCommands: CommandItem[] = [
    {
      id: "test",
      label: "Run Tests",
      description: "Execute unit tests",
      icon: TestTube,
      color: "#f59e0b",
      command: () => runCommand("test", getPmRunCommand(), ["run", "test"]),
    },
    {
      id: "lint",
      label: "Lint Check",
      description: "Check code style",
      icon: FileSearch,
      color: "#06b6d4",
      command: () => runCommand("lint", getPmRunCommand(), ["run", "lint"]),
    },
    {
      id: "check",
      label: "Type Check",
      description: "Verify TypeScript types",
      icon: Check,
      color: "#10b981",
      command: () => runCommand("check", getPmRunCommand(), ["run", "check"]),
    },
  ];
</script>

<div class="feature-container">
  <div class="header">
    <div class="header-main">
      <div class="title-group">
        <span class="title">QUICK COMMANDS</span>
        {#if isLoading}
          <Loader2 size={12} class="spin" />
        {/if}
      </div>

      {#if projectInfo}
        <div class="info-badges">
          <div class="badge pm">
            <Package size={10} />
            <span>{projectInfo.package_manager}</span>
          </div>
          {#if projectInfo.project_type !== "unknown"}
            <div class="badge type">
              <span>{projectInfo.project_type}</span>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>

  <div class="commands-viewport">
    {#if projectInfo && !projectInfo.has_node_modules}
      <div class="alert-banner">
        <div class="alert-content">
          <AlertCircle size={16} />
          <div class="alert-text">
            <strong>Missing dependencies</strong>
            <span>Run install to start development</span>
          </div>
        </div>
        <button
          class="alert-action"
          onclick={() => runCommand("install", getPmRunCommand(), ["install"])}
        >
          <Download size={14} />
          Install
        </button>
      </div>
    {/if}

    <div class="sections-list">
      <!-- Development Section -->
      <div class="section" class:expanded={sections.development}>
        <button
          class="section-trigger"
          onclick={() => (sections.development = !sections.development)}
        >
          <div class="trigger-left">
            <span class="chevron">
              {#if sections.development}<ChevronDown
                  size={14}
                />{:else}<ChevronRight size={14} />{/if}
            </span>
            <span class="label">DEVELOPMENT</span>
          </div>
          <span class="count">{devCommands.length}</span>
        </button>
        {#if sections.development}
          <div class="section-content">
            <div class="command-stack">
              {#each devCommands as cmd}
                <button
                  class="command-card"
                  style="--accent-color: {cmd.color}"
                  onclick={cmd.command}
                  disabled={runningCommand === cmd.id}
                >
                  <div class="card-icon">
                    <cmd.icon size={16} />
                    {#if runningCommand === cmd.id}
                      <div class="icon-sync">
                        <Loader2 size={10} class="spin" />
                      </div>
                    {/if}
                  </div>
                  <div class="card-info">
                    <span class="card-label">{cmd.label}</span>
                    <span class="card-desc">{cmd.description}</span>
                  </div>
                  {#if runningCommand === cmd.id}
                    <div class="running-indicator"></div>
                  {/if}
                </button>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- Tauri Section -->
      {#if projectInfo?.has_tauri || !projectInfo}
        <div class="section" class:expanded={sections.tauri}>
          <button
            class="section-trigger"
            onclick={() => (sections.tauri = !sections.tauri)}
          >
            <div class="trigger-left">
              <span class="chevron">
                {#if sections.tauri}<ChevronDown
                    size={14}
                  />{:else}<ChevronRight size={14} />{/if}
              </span>
              <span class="label">TAURI DESKTOP</span>
            </div>
            <span class="count">2</span>
          </button>
          {#if sections.tauri}
            <div class="section-content">
              <div class="command-grid">
                <button
                  class="command-card small"
                  style="--accent-color: #007acc"
                  onclick={() =>
                    runCommand("tauri-dev", getPmRunCommand(), [
                      "run",
                      "tauri",
                      "dev",
                    ])}
                  disabled={runningCommand === "tauri-dev"}
                >
                  <div class="card-icon"><Play size={14} /></div>
                  <span class="card-label">Dev</span>
                </button>
                <button
                  class="command-card small"
                  style="--accent-color: #f59e0b"
                  onclick={() =>
                    runCommand("tauri-build", getPmRunCommand(), [
                      "run",
                      "tauri",
                      "build",
                    ])}
                  disabled={runningCommand === "tauri-build"}
                >
                  <div class="card-icon"><Hammer size={14} /></div>
                  <span class="card-label">Build</span>
                </button>
              </div>
            </div>
          {/if}
        </div>
      {/if}

      <!-- Mobile Section -->
      {#if projectInfo?.has_android || projectInfo?.has_ios || !projectInfo}
        <div class="section" class:expanded={sections.mobile}>
          <button
            class="section-trigger"
            onclick={() => (sections.mobile = !sections.mobile)}
          >
            <div class="trigger-left">
              <span class="chevron">
                {#if sections.mobile}<ChevronDown
                    size={14}
                  />{:else}<ChevronRight size={14} />{/if}
              </span>
              <span class="label">MOBILE PLATFORMS</span>
            </div>
            <span class="count">
              {(projectInfo?.has_android ? 3 : 0) +
                (projectInfo?.has_ios ? 2 : 0) || 5}
            </span>
          </button>
          {#if sections.mobile}
            <div class="section-content mobile-content">
              {#if projectInfo?.has_android || !projectInfo}
                <div class="subsection">
                  <span class="subsection-title"
                    ><Smartphone size={10} /> Android</span
                  >
                  <div class="command-stack">
                    <button
                      class="command-card"
                      style="--accent-color: #10b981"
                      onclick={() =>
                        runCommand("android-dev", getPmRunCommand(), [
                          "run",
                          "tauri",
                          "android",
                          "dev",
                        ])}
                    >
                      <div class="card-icon"><Play size={16} /></div>
                      <div class="card-info">
                        <span class="card-label">Android Dev</span>
                        <span class="card-desc">Run on emulator/device</span>
                      </div>
                    </button>
                    <button
                      class="command-card"
                      style="--accent-color: #007acc"
                      onclick={() =>
                        runCommand("android-debug", getPmRunCommand(), [
                          "run",
                          "tauri",
                          "android",
                          "build",
                          "--debug",
                        ])}
                    >
                      <div class="card-icon"><Hammer size={16} /></div>
                      <div class="card-info">
                        <span class="card-label">Debug APK</span>
                        <span class="card-desc">Build debug version</span>
                      </div>
                    </button>
                    <button
                      class="command-card highlight"
                      style="--accent-color: #8b5cf6"
                      onclick={buildAndroidRelease}
                    >
                      <div class="card-icon"><Package size={16} /></div>
                      <div class="card-info">
                        <span class="card-label">Release APK</span>
                        <span class="card-desc"
                          >Build & Increment Version
                          {#if androidVersion}({androidVersion.version_name}){/if}</span
                        >
                      </div>
                    </button>
                  </div>
                </div>
              {/if}

              {#if projectInfo?.has_ios || !projectInfo}
                <div class="subsection">
                  <span class="subsection-title"><Apple size={10} /> iOS</span>
                  <div class="command-stack">
                    <button
                      class="command-card"
                      style="--accent-color: #10b981"
                      onclick={() =>
                        runCommand("ios-dev", getPmRunCommand(), [
                          "run",
                          "tauri",
                          "ios",
                          "dev",
                        ])}
                    >
                      <div class="card-icon"><Play size={16} /></div>
                      <div class="card-info">
                        <span class="card-label">iOS Dev</span>
                        <span class="card-desc">Run via Xcode</span>
                      </div>
                    </button>
                    <button
                      class="command-card"
                      style="--accent-color: #007acc"
                      onclick={buildIOS}
                    >
                      <div class="card-icon"><Hammer size={16} /></div>
                      <div class="card-info">
                        <span class="card-label">Build + Xcode</span>
                        <span class="card-desc">Export and open workspace</span>
                      </div>
                    </button>
                  </div>
                </div>
              {/if}
            </div>
          {/if}
        </div>
      {/if}

      <!-- Testing Section -->
      <div class="section" class:expanded={sections.testing}>
        <button
          class="section-trigger"
          onclick={() => (sections.testing = !sections.testing)}
        >
          <div class="trigger-left">
            <span class="chevron">
              {#if sections.testing}<ChevronDown
                  size={14}
                />{:else}<ChevronRight size={14} />{/if}
            </span>
            <span class="label">TESTING & QUALITY</span>
          </div>
          <span class="count">{testCommands.length}</span>
        </button>
        {#if sections.testing}
          <div class="section-content">
            <div class="command-stack">
              {#each testCommands as cmd}
                <button
                  class="command-card"
                  style="--accent-color: {cmd.color}"
                  onclick={cmd.command}
                  disabled={runningCommand === cmd.id}
                >
                  <div class="card-icon"><cmd.icon size={16} /></div>
                  <div class="card-info">
                    <span class="card-label">{cmd.label}</span>
                    <span class="card-desc">{cmd.description}</span>
                  </div>
                </button>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- Dependencies Section -->
      <div class="section" class:expanded={sections.dependencies}>
        <button
          class="section-trigger"
          onclick={() => (sections.dependencies = !sections.dependencies)}
        >
          <div class="trigger-left">
            <span class="chevron">
              {#if sections.dependencies}<ChevronDown
                  size={14}
                />{:else}<ChevronRight size={14} />{/if}
            </span>
            <span class="label">MANAGEMENT</span>
          </div>
          <span class="count">2</span>
        </button>
        {#if sections.dependencies}
          <div class="section-content">
            <div class="command-grid">
              <button
                class="command-card small"
                style="--accent-color: #10b981"
                onclick={() =>
                  runCommand("install", getPmRunCommand(), ["install"])}
              >
                <div class="card-icon"><Download size={14} /></div>
                <span class="card-label">Install</span>
              </button>
              <button
                class="command-card small"
                style="--accent-color: #007acc"
                onclick={() =>
                  runCommand("update", getPmRunCommand(), ["update"])}
              >
                <div class="card-icon"><Package size={14} /></div>
                <span class="card-label">Update</span>
              </button>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .feature-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--bg-sidebar);
    background: radial-gradient(
        circle at top right,
        rgba(var(--accent-rgb), 0.03) 0%,
        transparent 40%
      ),
      var(--bg-sidebar);
    color: var(--fg-primary);
  }

  /* Header Styling */
  .header {
    padding: 16px 12px 12px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .header-main {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .title-group {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .title {
    font-size: 10px;
    font-weight: 800;
    letter-spacing: 0.1em;
    color: var(--fg-secondary);
    text-transform: uppercase;
  }

  .info-badges {
    display: flex;
    gap: 6px;
  }

  .badge {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 3px 8px;
    background: var(--bg-hover);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    font-size: 9px;
    font-weight: 600;
    color: var(--fg-secondary);
  }

  .badge.pm {
    color: #fff;
    background: rgba(var(--accent-rgb), 0.1);
    border-color: rgba(var(--accent-rgb), 0.3);
  }

  /* Viewport & Scrolling */
  .commands-viewport {
    flex: 1;
    overflow-y: auto;
    padding: 12px 0;
  }

  /* Alert Banner */
  .alert-banner {
    margin: 0 12px 16px;
    padding: 10px;
    background: rgba(241, 76, 76, 0.1);
    border: 1px solid rgba(241, 76, 76, 0.3);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .alert-content {
    display: flex;
    gap: 10px;
    color: #f14c4c;
  }

  .alert-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .alert-text strong {
    font-size: 12px;
  }

  .alert-text span {
    font-size: 11px;
    opacity: 0.8;
  }

  .alert-action {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 6px;
    background: #f14c4c;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: filter 0.2s;
  }

  .alert-action:hover {
    filter: brightness(1.1);
  }

  /* Section Styling */
  .section {
    display: flex;
    flex-direction: column;
  }

  .section-trigger {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    background: transparent;
    border: none;
    color: var(--fg-secondary);
    cursor: pointer;
    transition: all 0.2s;
    width: 100%;
  }

  .section-trigger:hover {
    background: var(--bg-hover);
    color: var(--fg-primary);
  }

  .trigger-left {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .chevron {
    opacity: 0.5;
    display: flex;
    transform-origin: center;
  }

  .label {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.05em;
  }

  .count {
    font-size: 10px;
    opacity: 0.4;
    padding: 2px 6px;
    background: var(--bg-hover);
    border-radius: 10px;
  }

  .section-content {
    padding: 4px 12px 16px;
  }

  /* Command Cards */
  .command-stack {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .command-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 6px;
  }

  .command-card {
    position: relative;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px;
    background: var(--bg-app);
    background: linear-gradient(145deg, var(--bg-app), var(--bg-sidebar));
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    color: var(--fg-primary);
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    text-align: left;
    overflow: hidden;
    width: 100%;
  }

  .command-card::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    width: 3px;
    height: 100%;
    background: var(--accent-color, var(--accent-primary));
    opacity: 0.3;
    transition: opacity 0.2s;
  }

  .command-card:hover {
    background: var(--bg-hover);
    border-color: rgba(var(--accent-rgb), 0.5);
    transform: translateY(-2px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
  }

  .command-card:hover::before {
    opacity: 1;
  }

  .command-card:active {
    transform: translateY(0);
    filter: brightness(0.9);
  }

  .command-card:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
  }

  .card-icon {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background: rgba(var(--accent-color-rgb, 128, 128, 128), 0.1);
    background-color: var(--bg-active);
    border-radius: 6px;
    color: var(--accent-color, var(--accent-primary));
    flex-shrink: 0;
  }

  .icon-sync {
    position: absolute;
    top: -4px;
    right: -4px;
    background: var(--bg-app);
    border-radius: 50%;
    padding: 2px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
  }

  .card-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
    flex: 1;
  }

  .card-label {
    font-size: 13px;
    font-weight: 600;
  }

  .card-desc {
    font-size: 10px;
    color: var(--fg-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Small Card Variation */
  .command-card.small {
    padding: 8px;
    gap: 8px;
  }

  .command-card.small .card-icon {
    width: 24px;
    height: 24px;
    border-radius: 4px;
  }

  .command-card.small .card-label {
    font-size: 12px;
  }

  /* Highlight/Accent variation */
  .command-card.highlight {
    background: linear-gradient(
      135deg,
      rgba(var(--accent-rgb), 0.1) 0%,
      transparent 100%
    );
  }

  /* Progress Indicator */
  .running-indicator {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 2px;
    background: var(--accent-color, var(--accent-primary));
    animation: progress-pulse 1.5s infinite linear;
    transform-origin: left;
  }

  @keyframes progress-pulse {
    0% {
      transform: scaleX(0);
      opacity: 0;
    }
    50% {
      transform: scaleX(0.5);
      opacity: 1;
    }
    100% {
      transform: scaleX(1);
      opacity: 0;
    }
  }

  /* Subsection */
  .subsection {
    margin-bottom: 20px;
  }

  .subsection:last-child {
    margin-bottom: 0;
  }

  .subsection-title {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 10px;
    font-weight: 800;
    color: var(--fg-secondary);
    margin-bottom: 10px;
    padding-left: 4px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
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
