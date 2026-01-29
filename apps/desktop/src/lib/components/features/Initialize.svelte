<script lang="ts">
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

<div class="feature-container">
  <div class="form-scroll">
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

    <div class="form-group">
      <label for="git-init">Initialize Git</label>
      <div class="switch-row">
        <Switch bind:checked={initGit} />
        <span class="switch-label">{initGit ? "Yes" : "No"}</span>
      </div>
    </div>
  </div>

  <div class="actions-sticky">
    <Button variant="primary" onclick={initProject} class="w-full"
      >Initialize Tauri</Button
    >
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

  .form-group {
    margin-bottom: 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  label {
    font-size: 11px;
    font-weight: 500;
    color: var(--fg-secondary);
  }

  .switch-row {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 32px;
  }

  .switch-label {
    font-size: 11px;
    color: var(--fg-secondary);
  }

  .actions-sticky {
    padding: 12px;
    border-top: 1px solid var(--border-subtle);
  }
</style>
