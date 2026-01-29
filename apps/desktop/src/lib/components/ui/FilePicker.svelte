<script lang="ts">
  import Button from "./Button.svelte";
  import Input from "./Input.svelte";
  import { FolderOpen } from "lucide-svelte";
  import { open } from "@tauri-apps/plugin-dialog"; // Requires plugin

  let {
    value = $bindable(),
    placeholder = "Select path...",
    directory = true,
    class: className = "",
  } = $props();

  async function pick() {
    try {
      const selected = await open({
        directory,
        multiple: false,
        defaultPath: value || undefined,
      });

      if (selected && typeof selected === "string") {
        value = selected;
      }
    } catch (e) {
      console.error("Failed to open dialog", e);
      // Fallback for web mode if needed, or toast error
    }
  }
</script>

<div class="file-picker {className}">
  <Input bind:value {placeholder} class="flex-1" />
  <Button variant="outline" class="icon-btn" onclick={pick} title="Browse">
    <FolderOpen size={16} />
  </Button>
</div>

<style>
  .file-picker {
    display: flex;
    gap: 8px;
    width: 100%;
  }

  /* Make button same height as input */
  :global(.icon-btn) {
    height: 36px;
    width: 36px;
    padding: 0 !important;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
