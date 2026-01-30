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

  // Theme definitions matching app themes
  const themeDefinitions: Record<
    string,
    { base: "vs" | "vs-dark"; colors: Record<string, string> }
  > = {
    dark: {
      base: "vs-dark",
      colors: {
        "editor.background": "#1f1f1f",
        "editor.foreground": "#cccccc",
        "editorLineNumber.foreground": "#666666",
        "editor.selectionBackground": "#394160",
        "editor.lineHighlightBackground": "#282828",
      },
    },
    light: {
      base: "vs",
      colors: {
        "editor.background": "#ffffff",
        "editor.foreground": "#2c2c2c",
        "editorLineNumber.foreground": "#888888",
        "editor.selectionBackground": "#add6ff",
        "editor.lineHighlightBackground": "#f5f5f5",
      },
    },
    midnight: {
      base: "vs-dark",
      colors: {
        "editor.background": "#282a36",
        "editor.foreground": "#f8f8f2",
        "editorLineNumber.foreground": "#6272a4",
        "editor.selectionBackground": "#44475a",
        "editor.lineHighlightBackground": "#313340",
      },
    },
    iceberg: {
      base: "vs-dark",
      colors: {
        "editor.background": "#2e3440",
        "editor.foreground": "#d8dee9",
        "editorLineNumber.foreground": "#4c566a",
        "editor.selectionBackground": "#434c5e",
        "editor.lineHighlightBackground": "#363d4a",
      },
    },
    "solar-light": {
      base: "vs",
      colors: {
        "editor.background": "#fdf6e3",
        "editor.foreground": "#657b83",
        "editorLineNumber.foreground": "#93a1a1",
        "editor.selectionBackground": "#eee8d5",
        "editor.lineHighlightBackground": "#f5efdc",
      },
    },
    monolith: {
      base: "vs-dark",
      colors: {
        "editor.background": "#272822",
        "editor.foreground": "#f8f8f2",
        "editorLineNumber.foreground": "#75715e",
        "editor.selectionBackground": "#49483e",
        "editor.lineHighlightBackground": "#31322c",
      },
    },
    forest: {
      base: "vs-dark",
      colors: {
        "editor.background": "#0b0f0f",
        "editor.foreground": "#a8b5a0",
        "editorLineNumber.foreground": "#3d4a3d",
        "editor.selectionBackground": "#1a2a1a",
        "editor.lineHighlightBackground": "#111818",
      },
    },
    "emerald-pro": {
      base: "vs-dark",
      colors: {
        "editor.background": "#0b0f0f",
        "editor.foreground": "#f8f8f2",
        "editorLineNumber.foreground": "#404040",
        "editor.selectionBackground": "#064e3b",
        "editor.lineHighlightBackground": "#111818",
      },
    },
  };

  function getMonacoTheme(theme: string) {
    // Return custom theme name, fallback to vs-dark
    if (themeDefinitions[theme]) {
      return `faber-${theme}`;
    }
    return theme === "light" ? "vs" : "vs-dark";
  }

  function registerThemes(monacoInstance: any) {
    for (const [name, def] of Object.entries(themeDefinitions)) {
      monacoInstance.editor.defineTheme(`faber-${name}`, {
        base: def.base,
        inherit: true,
        rules: [],
        colors: def.colors,
      });
    }
  }

  onMount(async () => {
    // Import monaco only in browser
    monaco = await import("monaco-editor");

    // Register custom themes
    registerThemes(monaco);

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
