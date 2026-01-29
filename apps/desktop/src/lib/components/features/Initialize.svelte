<script lang="ts">
  import Input from "$lib/components/ui/Input.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import Switch from "$lib/components/ui/Switch.svelte";
  import { uiState } from "$lib/ui-state.svelte";
  import { appConsole } from "$lib/stores/console.svelte";
  import { notifications } from "$lib/stores/notifications.svelte";

  let projectName = $state("my-tauri-app");
  let bundleIdentifier = $state("com.example.app");
  let packageManager = $state("pnpm");
  let framework = $state("svelte");
  let language = $state("typescript");
  let styling = $state("tailwind");
  let mobileSupport = $state(true);
  let agentEnabled = $state(true);
  let template = $state("full");
  let database = $state("sqlite");
  let cloud = $state("none");
  let extraFeatures = $state({
    auth: true,
    analytics: false,
    logging: true,
  });

  function initProject() {
    if (!uiState.projectRoot) {
      notifications.error("Please open a folder first in the Explorer.");
      return;
    }

    appConsole.info(
      `Initializing project ${projectName} in ${uiState.projectRoot}...`,
      "Initialize",
    );
    appConsole.info(
      `Template: ${template}, Stack: ${framework}+${styling}+${database}, Cloud: ${cloud}`,
      "Initialize",
    );
    appConsole.info(
      `Enabled: Mobile=${mobileSupport}, Agent=${agentEnabled}, Auth=${extraFeatures.auth}`,
      "Initialize",
    );

    // Mock implementation for UI feedback
    setTimeout(() => {
      notifications.success("Project initialized successfully!");
      appConsole.success(
        "Initialization complete. Happy coding!",
        "Initialize",
      );
      uiState.refreshFileTree();
    }, 2000);
  }
</script>

<div class="feature-container">
  <div class="form-scroll">
    <div class="current-path-info">
      <span class="path-label">Active Folder:</span>
      <span class="path-value"
        >{uiState.projectRoot?.split(/[\\/]/).pop() || "None"}</span
      >
    </div>

    <!-- Basic Config Section -->
    <div class="section-group">
      <div class="section-header">Project Identity</div>
      <div class="form-group">
        <label for="project-name">Project Name</label>
        <Input
          id="project-name"
          bind:value={projectName}
          placeholder="e.g. my-awesome-app"
        />
      </div>

      <div class="form-group">
        <label for="bundle-id">Bundle Identifier</label>
        <Input
          id="bundle-id"
          bind:value={bundleIdentifier}
          placeholder="e.g. com.domain.app"
        />
      </div>
    </div>

    <!-- Technical Stack Section -->
    <div class="section-group">
      <div class="section-header">Technical Stack</div>
      <div class="form-group">
        <label for="template">Project Template</label>
        <Select
          bind:value={template}
          options={[
            { label: "Faber Full (Sidebars + Matrix)", value: "full" },
            { label: "Minimal (Clean Slate)", value: "minimal" },
            { label: "AI-Ready Agentic Template", value: "agentic" },
            { label: "Mobile-First MobileApp", value: "mobile" },
          ]}
        />
      </div>

      <div class="grid-2">
        <div class="form-group">
          <label for="framework">Framework</label>
          <Select
            bind:value={framework}
            options={[
              { label: "Svelte 5", value: "svelte" },
              { label: "React", value: "react" },
              { label: "Vue", value: "vue" },
              { label: "Vanilla", value: "vanilla" },
            ]}
          />
        </div>

        <div class="form-group">
          <label for="language">Language</label>
          <Select
            bind:value={language}
            options={[
              { label: "TypeScript", value: "typescript" },
              { label: "JavaScript", value: "javascript" },
            ]}
          />
        </div>

        <div class="form-group">
          <label for="styling">Styling</label>
          <Select
            bind:value={styling}
            options={[
              { label: "Tailwind CSS", value: "tailwind" },
              { label: "Vanilla CSS", value: "vanilla" },
              { label: "Sass (SCSS)", value: "sass" },
            ]}
          />
        </div>

        <div class="form-group">
          <label for="database">Database</label>
          <Select
            bind:value={database}
            options={[
              { label: "SQLite (Local)", value: "sqlite" },
              { label: "PostgreSQL", value: "postgres" },
              { label: "MongoDB", value: "mongo" },
              { label: "None", value: "none" },
            ]}
          />
        </div>
      </div>
    </div>

    <!-- Infrastructure Section -->
    <div class="section-group">
      <div class="section-header">Infrastructure</div>
      <div class="form-group">
        <label for="cloud">Cloud Integration</label>
        <Select
          bind:value={cloud}
          options={[
            { label: "None (Local Only)", value: "none" },
            { label: "Vercel / Netlify", value: "vercel" },
            { label: "AWS / Google Cloud", value: "cloud" },
            { label: "Supabase / Firebase", value: "backend-as-service" },
          ]}
        />
      </div>

      <div class="form-group">
        <label for="pkg-manager">Package Manager</label>
        <Select
          bind:value={packageManager}
          options={[
            { label: "pnpm", value: "pnpm" },
            { label: "npm", value: "npm" },
            { label: "yarn", value: "yarn" },
            { label: "bun", value: "bun" },
          ]}
        />
      </div>
    </div>

    <!-- Capabilities Section -->
    <div class="section-group last">
      <div class="section-header">Core Capabilities</div>
      <div class="capabilities-grid">
        <div class="capability-item">
          <Switch bind:checked={mobileSupport} />
          <span class="cap-label">Mobile Support</span>
        </div>
        <div class="capability-item">
          <Switch bind:checked={agentEnabled} />
          <span class="cap-label">Agentic AI</span>
        </div>
        <div class="capability-item">
          <Switch bind:checked={extraFeatures.auth} />
          <span class="cap-label">Authentication</span>
        </div>
        <div class="capability-item">
          <Switch bind:checked={extraFeatures.logging} />
          <span class="cap-label">Error Logging</span>
        </div>
      </div>
    </div>
  </div>

  <div class="actions-sticky">
    <Button
      variant="primary"
      onclick={initProject}
      class="w-full"
      disabled={!uiState.projectRoot}
    >
      Initialize Tauri Project
    </Button>
  </div>
</div>

<style>
  .feature-container {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .form-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 16px 12px;
  }

  .current-path-info {
    background: var(--bg-active);
    padding: 8px 12px;
    border-radius: 6px;
    margin-bottom: 16px;
    display: flex;
    align-items: center;
    font-size: 11px;
    border: 1px solid var(--border-subtle);
  }

  .path-label {
    color: var(--fg-secondary);
    margin-right: 6px;
  }

  .path-value {
    color: var(--accent-primary);
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .section-group {
    margin-bottom: 10px;
    padding-bottom: 10px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .section-group.last {
    margin-bottom: 0;
    padding-bottom: 0;
    border-bottom: none;
  }

  .section-header {
    font-size: 10px;
    font-weight: 800;
    color: var(--fg-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 8px;
  }

  .form-group {
    margin-bottom: 8px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
  }

  .grid-2 {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .capabilities-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 8px;
    margin-top: 8px;
  }

  .capability-item {
    display: flex;
    align-items: center;
    gap: 8px;
    background-color: var(--bg-hover);
    padding: 6px 10px;
    border-radius: 4px;
  }

  .cap-label {
    font-size: 11px;
    color: var(--fg-secondary);
  }

  label {
    font-size: 11px;
    font-weight: 500;
    color: var(--fg-secondary);
  }

  .actions-sticky {
    padding: 12px;
    border-top: 1px solid var(--border-subtle);
  }
</style>
