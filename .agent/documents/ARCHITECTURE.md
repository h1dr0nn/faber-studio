# Faber Studio – Architecture Overview

Faber Studio is an open-source desktop application that provides a graphical
interface for common Tauri development workflows. It does not replace the Tauri
CLI. Instead, it orchestrates and visualizes CLI operations in a transparent,
stateful, and developer-friendly way.

---

## Core Principles

1. CLI-first  
   All real work is delegated to existing CLI tools (cargo, tauri, node).

2. Transparency  
   Every action maps to an explicit command with visible logs and exit codes.

3. Separation of Concerns
   - Rust backend: environment access, process execution, validation
   - Frontend: orchestration, state, visualization

4. Local-only  
   No cloud services, no telemetry.

---

## Backend (Rust)

Responsibilities:

- Detect environment and installed tools
- Execute CLI commands via a unified runner
- Stream stdout / stderr to frontend
- Normalize errors and results

---

## Frontend (Svelte)

Responsibilities:

- Drive workflows
- Display diagnostics and logs
- Maintain explicit UI states:
  idle → running → success / error

Logs are treated as first-class UI content.

---

## Feature Modules

- Environment Doctor
- Project Initialization
- Asset & Icon Generation
- Mobile Targets
- Command Runner

All modules reuse the same execution primitives.

---

## Non-goals

- Re-implementing Tauri internals
- Hiding CLI behavior
- Acting as an official Tauri tool
