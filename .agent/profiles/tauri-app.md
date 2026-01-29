# Profile: Tauri Application (Desktop & Mobile)

## Scope

This profile defines **architectural constraints, responsibilities, and expectations**
for applications built with **Tauri + React**, targeting:

* Desktop: Windows / macOS / Linux
* Mobile: Android / iOS

This profile applies to:

* Tools
* Desktop apps
* Mobile apps
* React-based games built on Tauri

---

## Core Architectural Model

Tauri applications are **native applications with a WebView UI**, not web apps.

The system MUST be understood as a **multi-layer architecture**:

* React: UI and interaction
* Rust: core logic, policy, validation
* Native (Kotlin / Swift): platform authority (mobile only)

---

## Trust Model

### React (UI Layer)

* Treated as **untrusted**
* Handles rendering and user interaction
* Must not own critical decisions or side effects

React MUST NOT:

* Access OS or SDKs directly
* Make compliance-sensitive decisions
* Perform file system or process operations

---

### Rust (Core / Policy Layer)

* Treated as **trusted**
* Owns business rules and validation
* Coordinates IPC and native bridges

Rust MUST:

* Enforce ownership, lifetime, and threading rules
* Gate sensitive actions (ads, IAP, filesystem, network)
* Remain platform-agnostic where possible

---

### Native Layer (Mobile Only: Kotlin / Swift)

* Treated as **platform authority**
* Owns all native SDK integrations

Native code MUST:

* Integrate Ads, IAP, analytics, attribution, ATT
* Handle OS lifecycle and permissions
* Remain minimal and free of business logic

---

## Desktop-Specific Constraints

For desktop targets, the agent MUST enforce:

* UI thread responsiveness at all times
* No blocking IO or IPC on UI thread
* Hidden execution of bundled tools (no terminal popups)
* Safe file system access (no silent overwrite)
* Stability over long-running sessions
* Safe auto-update and data migration

Desktop apps are expected to run for long periods
and interact with real user data.

---

## Mobile-Specific Constraints

For mobile targets, the agent MUST enforce:

* Explicit lifecycle and process-death handling
* Power and thermal budget discipline
* Network variability and offline tolerance
* Store compliance as a system rule
* Native SDK integration only via Kotlin / Swift
* Controlled background execution
* Crash and ANR observability

Mobile apps MUST assume:

* Process death is normal
* Resources are constrained
* OS policies override app intent

---

## IPC Discipline

IPC boundaries MUST be treated as:

* Ownership boundaries
* Serialization boundaries
* Failure isolation boundaries

Shared mutable state across IPC is prohibited.

---

## Update and Release Discipline

All builds MUST:

* Be version-aware
* Handle migration explicitly
* Fail safely on update errors
* Preserve user data across updates

---

## Intended Use

This profile is intended to:

* Guide system design
* Constrain agent behavior
* Align architecture across projects
* Prevent platform-specific anti-patterns

It is **not** a coding guide,
but a **system-level contract**.

---

## Key Takeaway

> **Tauri apps are native systems with a web UI, not web apps with native glue.**

Respecting this distinction is essential for
stability, performance, and compliance.
