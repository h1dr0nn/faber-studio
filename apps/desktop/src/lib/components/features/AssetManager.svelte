<script lang="ts">
  import Button from "$lib/components/ui/Button.svelte";
  import { Image } from "lucide-svelte";
</script>

<div class="feature-container">
  <div class="content-scroll">
    <div class="section-title">Source Image</div>
    <div class="drop-zone">
      <Image size={32} class="text-mute" />
      <span class="drop-text">Drop 1024x1024 icon</span>
      <Button variant="outline" size="sm">Browse</Button>
    </div>

    <div class="section-title mt-4">Platforms</div>
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
        <span>Desktop</span>
      </label>
    </div>
  </div>

  <div class="actions-sticky">
    <Button variant="primary" class="w-full">Generate Assets</Button>
  </div>
</div>

<style>
  .feature-container {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .content-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 16px 12px;
  }

  .section-title {
    font-size: 11px;
    font-weight: 600;
    color: var(--fg-secondary);
    text-transform: uppercase;
    margin-bottom: 8px;
  }

  .mt-4 {
    margin-top: 16px;
  }

  .drop-zone {
    border: 1px dashed var(--border-subtle);
    border-radius: 4px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    background-color: var(--bg-input);
    text-align: center;
  }

  .drop-text {
    color: var(--fg-secondary);
    font-size: 11px;
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
    font-size: 13px;
  }

  .secondary-text {
    font-size: 11px;
    color: var(--fg-tertiary);
  }

  .actions-sticky {
    padding: 12px;
    border-top: 1px solid var(--border-subtle);
  }
</style>
