<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { uiState } from "$lib/ui-state.svelte";
  import { preferences } from "$lib/stores/preferences.svelte";

  let { tab } = $props<{ tab: any }>();
  let editorContainer: HTMLElement;
  let monaco: any;
  let editor: any;

  // Language mapping
  function getLanguage(path: string) {
    const ext = path.split(".").pop()?.toLowerCase();
    const map: Record<string, string> = {
      js: "javascript",
      jsx: "javascript",
      ts: "typescript",
      tsx: "typescript",
      svelte: "html",
      rs: "rust",
      json: "json",
      css: "css",
      html: "html",
      md: "markdown",
      py: "python",
      go: "go",
      cpp: "cpp",
      c: "c",
      sql: "sql",
    };
    return map[ext || ""] || "plaintext";
  }

  function getMonacoTheme(theme: string) {
    if (theme === "light") return "vs";
    return "vs-dark";
  }

  onMount(async () => {
    // Import monaco only in browser
    monaco = await import("monaco-editor");

    editor = monaco.editor.create(editorContainer, {
      value: tab.content,
      language: getLanguage(tab.path),
      theme: getMonacoTheme(preferences.theme),
      automaticLayout: true,
      fontSize: 14,
      fontFamily: "var(--font-mono)",
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      lineNumbers: "on",
      roundedSelection: false,
      padding: { top: 10, bottom: 10 },
      bracketPairColorization: { enabled: true },
    });

    editor.onDidChangeModelContent(() => {
      tab.content = editor.getValue();
    });

    editor.onDidChangeCursorPosition((e: any) => {
      uiState.cursorPosition = {
        ln: e.position.lineNumber,
        col: e.position.column,
      };
    });

    // Update telemetry once on mount
    const model = editor.getModel();
    if (model) {
      uiState.setEditorTelemetry(getLanguage(tab.path).toUpperCase(), "UTF-8");
    }
  });

  onDestroy(() => {
    if (editor) {
      editor.dispose();
    }
  });

  // Keep editor content in sync if it changes externally (like Prettier)
  $effect(() => {
    if (editor && editor.getValue() !== tab.content) {
      editor.setValue(tab.content);
    }
  });

  // Sync theme
  $effect(() => {
    if (editor && monaco) {
      monaco.editor.setTheme(getMonacoTheme(preferences.theme));
    }
  });

  // Sync language when tab path changes
  $effect(() => {
    if (editor && monaco) {
      const model = editor.getModel();
      if (model) {
        monaco.editor.setModelLanguage(model, getLanguage(tab.path));
      }
    }
  });
</script>

<div class="editor-wrapper" bind:this={editorContainer}></div>

<style>
  .editor-wrapper {
    width: 100%;
    height: 100%;
  }
</style>
