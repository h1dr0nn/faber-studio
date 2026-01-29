<script lang="ts">
  import Button from "$lib/components/ui/Button.svelte";
  import {
    Image,
    FolderOpen,
    CheckCircle2,
    AlertCircle,
    Loader2,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import { uiState } from "$lib/ui-state.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { convertFileSrc } from "@tauri-apps/api/core";

  let sourcePath = $state("");
  let targetDir = $state("");
  let platforms = $state({
    ios: true,
    android: true,
    desktop: true,
  });

  let isGenerating = $state(false);
  let progress = $state(0);
  let message = $state({ type: "", text: "" });

  // Icon sizes configuration
  const iconSizes = {
    desktop: [16, 24, 32, 48, 64, 128, 256],
    ios: [20, 29, 40, 58, 60, 76, 80, 87, 120, 152, 167, 180, 1024],
    android: [36, 48, 72, 96, 144, 192, 512],
  };

  // Get preview URL for selected image
  let previewUrl = $derived(sourcePath ? convertFileSrc(sourcePath) : "");

  // Auto-detected icons directory from backend
  let detectedIconsDir = $state("");

  // Find icons directory when component mounts and project root is available
  onMount(() => {
    if (uiState.projectRoot) {
      findIconsDirectory();
    }
  });

  // Also find when project root changes
  $effect(() => {
    if (uiState.projectRoot && !detectedIconsDir) {
      findIconsDirectory();
    }
  });

  async function findIconsDirectory() {
    if (!uiState.projectRoot) return;

    try {
      console.log("Finding icons dir for:", uiState.projectRoot);
      const result = await invoke<string | null>("find_icons_dir", {
        projectRoot: uiState.projectRoot,
      });
      console.log("Found icons dir:", result);
      if (result) {
        detectedIconsDir = result;
      }
    } catch (err) {
      console.error("Failed to find icons dir:", err);
    }
  }

  // Calculate total icons to generate
  let totalIcons = $derived(() => {
    let count = 0;
    if (platforms.desktop) count += iconSizes.desktop.length + 2; // +2 for ico/icns
    if (platforms.ios) count += iconSizes.ios.length;
    if (platforms.android) count += iconSizes.android.length;
    return count;
  });

  async function browseSource() {
    const selected = await open({
      filters: [{ name: "Images", extensions: ["png"] }],
      multiple: false,
    });
    if (selected && typeof selected === "string") {
      sourcePath = selected;
      message = { type: "", text: "" };
    }
  }

  async function browseTarget() {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected && typeof selected === "string") {
      targetDir = selected;
    }
  }

  async function generateIcons() {
    if (!sourcePath) {
      message = { type: "error", text: "Please select a source image" };
      return;
    }

    const selectedPlatforms = Object.entries(platforms)
      .filter(([_, enabled]) => enabled)
      .map(([id]) => id);

    if (selectedPlatforms.length === 0) {
      message = { type: "error", text: "Please select at least one platform" };
      return;
    }

    const finalTargetDir = targetDir || detectedIconsDir || uiState.projectRoot;
    if (!finalTargetDir) {
      message = { type: "error", text: "Please select a target directory" };
      return;
    }

    isGenerating = true;
    progress = 0;
    message = { type: "info", text: "Generating icons..." };

    // Simulate progress animation
    const progressInterval = setInterval(() => {
      if (progress < 90) {
        progress += Math.random() * 15;
      }
    }, 200);

    try {
      await invoke("generate_icons", {
        sourcePath,
        targetDir: finalTargetDir,
        platforms: selectedPlatforms,
      });
      progress = 100;
      message = { type: "success", text: "Generated icons successfully!" };
    } catch (err: any) {
      message = { type: "error", text: `Error: ${err}` };
    } finally {
      clearInterval(progressInterval);
      isGenerating = false;
    }
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    const files = e.dataTransfer?.files;
    if (files && files.length > 0) {
      const file = files[0];
      if (file.type === "image/png") {
        message = {
          type: "info",
          text: "Dropped: " + file.name + ". Use Browse for full path.",
        };
      }
    }
  }
</script>

<div class="feature-container">
  <div class="content-scroll">
    <div class="section-title">Source Image (PNG)</div>
    <div
      class="drop-zone"
      class:has-file={!!sourcePath}
      role="button"
      tabindex="0"
      ondragover={(e) => e.preventDefault()}
      ondrop={handleDrop}
    >
      {#if sourcePath}
        <div class="preview-container">
          <img src={previewUrl} alt="Preview" class="preview-image" />
          <div class="preview-info">
            <span class="file-name">{sourcePath.split(/[\\/]/).pop()}</span>
            <Button variant="ghost" size="sm" onclick={() => (sourcePath = "")}
              >Clear</Button
            >
          </div>
        </div>
      {:else}
        <Image size={32} class="text-mute" />
        <span class="drop-text">Drop 1024x1024 PNG icon here</span>
        <Button variant="outline" size="sm" onclick={browseSource}
          >Browse</Button
        >
      {/if}
    </div>

    <div class="section-title mt-4">Target Directory</div>
    <div class="path-selector">
      <div class="current-path">
        {targetDir || detectedIconsDir || "Not selected"}
      </div>
      <Button variant="ghost" size="sm" onclick={browseTarget}>
        <FolderOpen size={16} />
      </Button>
    </div>

    <div class="section-title mt-4">Platforms</div>
    <div class="checkbox-list">
      <label class="checkbox-item">
        <input type="checkbox" bind:checked={platforms.ios} />
        <span>iOS ({iconSizes.ios.length} sizes)</span>
      </label>
      <label class="checkbox-item">
        <input type="checkbox" bind:checked={platforms.android} />
        <span>Android ({iconSizes.android.length} sizes)</span>
      </label>
      <label class="checkbox-item">
        <input type="checkbox" bind:checked={platforms.desktop} />
        <span>Desktop (.ico + .icns)</span>
      </label>
    </div>

    <!-- Icon Sizes Preview -->
    <div class="section-title mt-4">Output Sizes</div>
    <div class="sizes-preview">
      {#if platforms.desktop}
        <div class="size-group">
          <span class="size-label">Desktop:</span>
          <span class="size-values">{iconSizes.desktop.join(", ")}px</span>
        </div>
      {/if}
      {#if platforms.ios}
        <div class="size-group">
          <span class="size-label">iOS:</span>
          <span class="size-values"
            >{iconSizes.ios.slice(0, 5).join(", ")}... ({iconSizes.ios.length} total)</span
          >
        </div>
      {/if}
      {#if platforms.android}
        <div class="size-group">
          <span class="size-label">Android:</span>
          <span class="size-values">{iconSizes.android.join(", ")}px</span>
        </div>
      {/if}
    </div>

    <!-- Progress Bar -->
    {#if isGenerating || progress > 0}
      <div class="progress-container mt-4">
        <div class="progress-bar">
          <div
            class="progress-fill"
            style="width: {Math.min(progress, 100)}%"
          ></div>
        </div>
        <span class="progress-text">{Math.round(progress)}%</span>
      </div>
    {/if}

    {#if message.text}
      <div class="message-box {message.type}">
        {#if message.type === "error"}
          <AlertCircle size={14} />
        {:else if message.type === "success"}
          <CheckCircle2 size={14} />
        {:else if message.type === "info" && isGenerating}
          <Loader2 size={14} class="spin" />
        {/if}
        <span>{message.text}</span>
      </div>
    {/if}
  </div>

  <div class="actions-sticky">
    <Button
      variant="primary"
      class="w-full"
      onclick={generateIcons}
      disabled={isGenerating}
    >
      {isGenerating ? "Generating..." : "Generate Assets"}
    </Button>
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
    transition: all 0.2s ease;
  }

  .drop-zone:hover {
    border-color: var(--accent-primary);
    background-color: var(--bg-hover);
  }

  .drop-zone.has-file {
    border-style: solid;
    border-color: var(--accent-primary);
    background-color: var(--bg-active);
  }

  .file-info {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .file-name {
    font-size: 13px;
    font-weight: 600;
    color: var(--fg-primary);
    max-width: 200px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .path-selector {
    display: flex;
    align-items: center;
    gap: 8px;
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    padding: 4px 8px;
  }

  .current-path {
    flex: 1;
    font-size: 11px;
    color: var(--fg-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .message-box {
    margin-top: 16px;
    padding: 10px 12px;
    border-radius: 4px;
    font-size: 12px;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .message-box.info {
    background-color: rgba(0, 150, 255, 0.1);
    color: #0096ff;
    border: 1px solid rgba(0, 150, 255, 0.2);
  }

  .message-box.error {
    background-color: rgba(255, 70, 70, 0.1);
    color: #ff4646;
    border: 1px solid rgba(255, 70, 70, 0.2);
  }

  .message-box.success {
    background-color: rgba(0, 200, 100, 0.1);
    color: #00c864;
    border: 1px solid rgba(0, 200, 100, 0.2);
  }

  .text-success {
    color: #00c864;
  }
  .text-mute {
    color: var(--fg-tertiary);
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

  /* Preview styles */
  .preview-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
  }

  .preview-image {
    width: 80px;
    height: 80px;
    border-radius: 8px;
    object-fit: cover;
    border: 2px solid var(--accent-primary);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .preview-info {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  /* Sizes preview */
  .sizes-preview {
    background-color: var(--bg-input);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    padding: 8px 10px;
    font-size: 11px;
  }

  .size-group {
    display: flex;
    gap: 6px;
    margin-bottom: 4px;
  }

  .size-group:last-child {
    margin-bottom: 0;
  }

  .size-label {
    color: var(--fg-secondary);
    font-weight: 600;
    min-width: 60px;
  }

  .size-values {
    color: var(--fg-tertiary);
  }

  /* Progress bar */
  .progress-container {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .progress-bar {
    flex: 1;
    height: 6px;
    background-color: var(--bg-hover);
    border-radius: 3px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(
      90deg,
      var(--accent-primary),
      var(--accent-secondary, #00c864)
    );
    border-radius: 3px;
    transition: width 0.2s ease;
  }

  .progress-text {
    font-size: 11px;
    color: var(--fg-secondary);
    min-width: 36px;
    text-align: right;
  }

  /* Spinner animation */
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
