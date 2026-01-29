# Tauri-Based Game Architecture (React as Rendering Layer)

## Purpose

This document defines **architectural rules for games built with React + Tauri**
that target **Android, iOS, and Desktop** platforms.

This is **NOT a browser-based web game**.

It exists to ensure that:

* React is used as a rendering and UI layer only
* Game logic follows proper game architecture principles
* Performance and lifecycle constraints of mobile and desktop platforms are respected

This document refines the engine-agnostic game rules for a **Tauri runtime**.

---

## Fundamental Clarification

### This Is Not a Web Game

The agent MUST NOT treat this project as:

* A browser game
* A traditional SPA
* A web application with navigation-driven lifecycle

Key distinctions:

* Runtime is **Tauri WebView**, not a browser tab
* Application lifecycle is **native**, not page-based
* Reloading is exceptional, not normal behavior

---

## Runtime Model

### Game Loop Is Explicit

The game MUST have an explicit game loop.

The agent MUST:

* Treat gameplay updates as loop-driven
* Avoid tying gameplay progression to React renders

Typical loop mechanisms:

* `requestAnimationFrame`
* Custom scheduler driven by timestamps

React MUST NOT be treated as the game loop.

---

### Separation of Loop and Rendering

The agent MUST:

* Separate game update logic from rendering
* Ensure rendering observes state rather than mutates it

Game logic SHOULD:

* Advance state
* Emit render-relevant data

Rendering SHOULD:

* Reflect the current state
* Avoid side effects

---

## React Usage Rules

### React Is a View Layer

React MUST be used for:

* UI
* HUD
* Menus
* Developer tools

React MUST NOT:

* Contain core gameplay rules
* Control simulation timing
* Act as the authoritative source of game state

---

### State Management Discipline

React state SHOULD:

* Represent view state
* Mirror gameplay state only when necessary

Core gameplay state MUST:

* Live outside React
* Be owned by the game logic layer

Excessive React state updates in gameplay loops are prohibited.

---

## Rendering Layer Considerations

### Canvas / WebGL Discipline

When using Canvas or WebGL:

* Rendering MUST be decoupled from React reconciliation
* Draw calls SHOULD be driven by the game loop
* React SHOULD not re-render on every frame

The agent MUST:

* Avoid full component tree re-renders per frame
* Treat the renderer as a performance-critical subsystem

---

## Platform and Performance Constraints

### Mobile-First Assumptions

The agent MUST assume:

* Mobile-class CPUs and GPUs
* Limited memory
* Aggressive background throttling

Design decisions MUST:

* Favor predictable performance
* Avoid desktop-only assumptions

---

### Memory and GC Awareness

The agent MUST:

* Avoid per-frame allocations in JavaScript
* Avoid creating transient objects in hot paths
* Be conscious of GC pauses

This applies especially to:

* Game loops
* Input processing
* Rendering pipelines

---

## Tauri Integration

### IPC Is a Native Boundary

All native functionality MUST:

* Go through Tauri IPC
* Be treated as asynchronous
* Be considered expensive relative to in-process calls

The agent MUST NOT:

* Perform IPC calls every frame
* Treat IPC as a low-latency operation

---

### Native Responsibilities

Native (Rust / system) side SHOULD:

* Handle file system access
* Handle platform-specific services
* Handle long-running or blocking operations

Game logic SHOULD:

* Remain mostly in the game layer
* Avoid leaking native concerns upward

---

## Lifecycle and Suspension

### Backgrounding and Resuming

Mobile platforms may:

* Suspend the app
* Kill the process
* Reclaim resources

The agent MUST:

* Assume the game can be paused at any time
* Support state restoration where appropriate

Critical state MUST:

* Be saved explicitly
* Not rely on in-memory persistence alone

---

## Input Handling

Input MUST:

* Be normalized into domain-level actions
* Be decoupled from raw platform events

Avoid:

* Directly handling raw input inside gameplay logic
* Platform-specific assumptions in game rules

---

## Anti-Patterns to Avoid

* Treating React re-render as free
* Driving gameplay from UI events
* Browser-only APIs (Service Workers, DOM-specific hacks)
* SPA-style routing for game flow
* Frame-by-frame IPC calls

---

## Scope and Authority

* This document applies to all Tauri-based game projects.
* It refines engine-agnostic game rules.
* In case of conflict, game domain and performance rules take precedence.

---

## Expected Outcome

Following these rules results in Tauri-based games that:

* Behave like native games, not web apps
* Scale in complexity without collapsing performance
* Share architectural principles with Unity-based games
