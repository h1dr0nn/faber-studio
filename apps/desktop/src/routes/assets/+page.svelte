<script lang="ts">
  import Panel from "$lib/components/ui/Panel.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Switch from "$lib/components/ui/Switch.svelte";
  import { Image } from "lucide-svelte";

  let desktopCheckbox: HTMLInputElement;
  let applySquircle = $state(true);
</script>

<div class="page-container">
  <div class="header">
    <h1 class="page-title">Asset Generator</h1>
  </div>

  <Panel title="Source Image" class="upload-panel">
    <div class="drop-zone">
      <Image size={48} class="text-mute" />
      <span class="drop-text">Drag & Drop your 1024x1024 icon here</span>
      <Button variant="outline" size="sm">Choose File</Button>
    </div>
  </Panel>

  <div class="grid">
    <Panel title="Platforms & Options">
      <div class="toggle-row">
        <span class="toggle-label">iOS Icons</span>
        <Switch label="iOS" checked />
      </div>
      <div class="toggle-row">
        <span class="toggle-label">Android Icons</span>
        <Switch label="Android" checked />
      </div>
      <div class="toggle-row">
        <span class="toggle-label">Desktop Icons (.ico + .icns)</span>
        <Switch label="Desktop" checked />
      </div>

      <div class="toggle-row standalone">
        <span class="toggle-label">Apply native macOS squircle mask</span>
        <Switch bind:checked={applySquircle} label="Apply native squircle" />
      </div>

      <div class="actions">
        <Button variant="primary">Generate Icons</Button>
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
  .drop-zone {
    border: 2px dashed var(--border-subtle);
    border-radius: 6px;
    height: 200px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    background-color: var(--bg-input);
    transition: border-color 0.2s;
  }

  .drop-zone:hover {
    border-color: var(--accent-primary);
  }

  .drop-text {
    color: var(--fg-secondary);
    font-size: var(--text-sm);
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 16px;
  }

  .actions {
    margin-top: 16px;
  }

  .toggle-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 0;
  }

  .toggle-row.standalone {
    margin-top: 16px;
    padding-top: 16px;
    border-top: 1px solid var(--border-subtle);
    border-bottom: none;
  }

  .toggle-label {
    font-size: var(--text-sm);
    color: var(--fg-secondary);
  }
</style>
