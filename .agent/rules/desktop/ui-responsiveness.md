# UI Responsiveness and Main Thread Discipline (Desktop)

## Purpose

This document defines **non-negotiable rules** for maintaining **UI responsiveness**
and protecting the **main/UI thread** in desktop applications on:

* Windows
* macOS
* Linux

It exists to prevent:

* UI freezes and perceived “app hangs”
* OS-level “Not Responding” states
* Forced quits and data loss
* Unrecoverable user trust damage

These rules apply to:

* Tauri desktop applications
* Unity desktop tools or games
* Any desktop app with a graphical UI

---

## Fundamental Principle

> **A frozen UI is a functional failure, even if the app has not crashed.**

Desktop users interpret UI unresponsiveness as a broken application.

---

## UI Thread Is Sacred

The agent MUST treat the following as **protected threads**:

* UI / event loop thread
* Render thread
* Windowing / message pump thread

Protected threads MUST remain responsive at all times.

---

## Prohibited Work on UI Thread

The agent MUST NOT execute the following on the UI thread:

* Blocking IO (file, network, IPC)
* CPU-heavy computation
* Long-running loops
* Synchronous waits on background tasks

Even short blocking operations can accumulate into freezes.

---

## Asynchronous Execution Discipline

When work cannot complete immediately, the agent MUST:

* Offload work to background threads or tasks
* Provide progress reporting where appropriate
* Allow cancellation by the user

“Fire-and-forget” background work without visibility is discouraged.

---

## IPC and UI Responsiveness (Tauri)

For Tauri desktop applications:

* IPC calls MUST be treated as potentially blocking
* IPC MUST NOT be executed synchronously from UI handlers
* Long-running IPC operations MUST be async and cancellable

IPC-induced UI freezes are unacceptable.

---

## Feedback and Progress

For operations longer than a short threshold:

* UI MUST indicate work is in progress
* UI MUST remain interactive where possible
* The user MUST retain control (cancel, close, defer)

Silent blocking operations are prohibited.

---

## Event Loop Discipline

The agent MUST:

* Avoid flooding the event loop
* Avoid tight loops driven by UI events
* Throttle or debounce high-frequency UI-driven actions

Unbounded event handling leads to responsiveness collapse.

---

## Deadlock and Re-Entrancy Awareness

The agent MUST avoid:

* Waiting on background tasks from the UI thread
* Re-entrant calls that block event processing
* Circular dependencies between UI and background work

Deadlocks on desktop are often perceived as freezes, not crashes.

---

## Cross-Platform Considerations

The agent MUST assume:

* UI responsiveness expectations vary slightly by OS
* macOS is especially sensitive to main-thread blocking
* Windows may display “Not Responding” quickly
* Linux window managers vary widely

Designs MUST satisfy the strictest expectations.

---

## Error Handling and UI Stability

On errors:

* UI MUST remain responsive
* Errors MUST be surfaced non-blockingly
* Recovery paths MUST not block the UI thread

Crashing is preferable to freezing silently.

---

## Anti-Patterns (Explicitly Forbidden)

* “Just run it synchronously, it’s fast enough”
* Blocking IPC from UI callbacks
* Long file operations on UI thread
* Busy-wait loops in UI handlers
* UI freezes masked as loading states

---

## Scope and Authority

* These rules apply to all desktop builds.
* They override convenience-based UI logic.
* Stricter OS-specific rules may apply.

---

## Expected Outcome

Following these rules results in desktop apps that:

* Remain responsive under load
* Avoid OS-level hang detection
* Preserve user trust and data
* Feel stable and professional
