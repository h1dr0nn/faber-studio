<script lang="ts">
  import Panel from "$lib/components/ui/Panel.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import Switch from "$lib/components/ui/Switch.svelte";
  import FilePicker from "$lib/components/ui/FilePicker.svelte";
  import { appConsole } from "$lib/stores/console.svelte";
  import { notifications } from "$lib/stores/notifications.svelte";

  let projectName = $state("my-tauri-app");
  let bundleIdentifier = $state("com.example.app");
  let projectPath = $state("");
  let packageManager = $state("pnpm");
  let initGit = $state(true);

  function initProject() {
    if (!projectPath) {
      notifications.error("Please select a project path.");
      return;
    }

    appConsole.info(
      `Initializing project ${projectName} in ${projectPath}...`,
      "Init",
    );
    appConsole.info(`Using ${packageManager}, Git: ${initGit}`, "Init");

    // Mock
    setTimeout(() => {
      notifications.success("Project initialized successfully!");
      appConsole.success("Initialization complete.", "Init");
    }, 1500);
  }
</script>

<div class="page-container">
  <div class="header">
    <h1 class="page-title">Initialize Project</h1>
  </div>

  <div class="grid">
    <Panel title="Configuration" class="col-span-1">
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

      <div class="form-group">
        <label for="project-path">Location</label>
        <FilePicker
          bind:value={projectPath}
          placeholder="Select destination..."
        />
      </div>

      <div class="row">
        <div class="form-group flex-1">
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

        <div class="form-group flex-1">
          <label for="git-init">Initialize Git</label>
          <div class="switch-row">
            <Switch bind:checked={initGit} />
            <span class="switch-label">{initGit ? "Yes" : "No"}</span>
          </div>
        </div>
      </div>

      <div class="actions">
        <Button variant="primary" onclick={initProject}>Initialize Tauri</Button
        >
      </div>
    </Panel>
  </div>
</div>

<style>
  .page-container {
    padding: 24px;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-bottom: 60px; /* Space for Matrix */
  }

  .header {
    border-bottom: 1px solid var(--border-subtle);
    padding-bottom: 16px;
  }

  .page-title {
    font-size: 18px;
    font-weight: 500;
    margin: 0;
  }

  .grid {
    display: grid;
    grid-template-columns: minmax(300px, 600px);
    gap: 16px;
  }

  .form-group {
    margin-bottom: 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .row {
    display: flex;
    gap: 16px;
  }

  label {
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--fg-primary);
  }

  .switch-row {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 36px; /* align with inputs */
  }

  .switch-label {
    font-size: var(--text-sm);
    color: var(--fg-secondary);
  }

  .actions {
    margin-top: 24px;
    display: flex;
    justify-content: flex-end;
  }
</style>
