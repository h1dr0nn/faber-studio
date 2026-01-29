<script lang="ts">
  import Panel from '$lib/components/ui/Panel.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { Image } from 'lucide-svelte';
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
    <Panel title="Platform Selection">
      <div class="checkbox-list">
        <label class="checkbox-item">
          <input type="checkbox" checked />
          <span>iOS</span>
        </label>
        <label class="checkbox-item">
          <input type="checkbox" checked />
          <span>Android</span>
        </label>
        <label class="checkbox-item">
          <input type="checkbox" checked />
          <span>Desktop (Windows, macOS, Linux)</span>
        </label>
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

  .upload-panel {
    margin-bottom: 16px;
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
  
  .checkbox-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  
  .checkbox-item {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    font-size: var(--text-sm);
  }
  
  .actions {
    margin-top: 16px;
  }
  
  /* Text mute utility reuse */
  .text-mute {
    color: var(--fg-secondary);
  }
</style>
