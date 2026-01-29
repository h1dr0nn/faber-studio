# Faber Studio – UI Style Guide (Code Theme)

Faber Studio uses a code-oriented visual style inspired by terminals,
editors, and developer tools.

---

## Design Philosophy

- UI should feel like an extension of the editor
- Logs and commands are first-class citizens
- No decorative visuals
- Information density is preferred over whitespace

---

## Color Scheme

- Dark-first theme
- Near-black background (not pure black)
- High-contrast foreground text
- Semantic colors only:
  - green: success
  - yellow: warning
  - red: error
  - blue/cyan: active / focus

Avoid gradients and brand-heavy colors.

---

## Typography

- Primary font: monospace or code-adjacent
- Consistent line height
- Clear distinction between:
  - commands
  - logs
  - UI labels

---

## Layout

- Panel-based layout
- Resizable sections where applicable
- Vertical log streams
- Minimal borders, subtle separators

---

## Components

- Buttons: flat, no elevation
- Inputs: editor-like
- Tabs: understated, functional
- Modals: rare, blocking only when required

---

## UX Rules

- Always show the command being executed
- Never hide errors
- Logs should be copyable
- Prefer inline feedback over popups

---

## Non-goals

- Marketing visuals
- Animations for decoration
- Theme customizers (initially)
