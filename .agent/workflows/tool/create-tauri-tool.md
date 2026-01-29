---
description: End-to-end process for creating cross-platform desktop and mobile tools using React and Tauri, while enforcing system, platform, and compliance constraints.
---

# Workflow: Create Tauri Tool (Desktop & Mobile)

## Purpose

This workflow defines a **safe, end-to-end process** for creating a new
**cross-platform tool or application** using **React + Tauri**, targeting:

* Desktop (Windows / macOS / Linux)
* Mobile (Android / iOS)

It ensures that new projects:

* Start with correct architecture
* Respect system and platform constraints
* Avoid common Tauri, mobile, and desktop anti-patterns

---

## When to Use This Workflow

Use this workflow when:

* Creating a new Tauri-based tool or app
* Targeting desktop, mobile, or both
* Planning long-term maintenance and releases

Do NOT use this workflow for:

* Prototypes or throwaway demos
* Web-only applications

---

## Inputs

* Target platforms (desktop, mobile, or both)
* Intended functionality
* Expected lifespan (short-term vs long-term)
* Required integrations (ads, IAP, native SDKs, filesystem, etc.)

---

## Step-by-Step Workflow

### Step 1: Select Target Platforms

Explicitly declare:

* Desktop only
* Mobile only
* Desktop + Mobile

Platform selection determines applicable rules and constraints.

---

### Step 2: Apply Project Profile

Apply:

* `profiles/tauri-app.md`

This profile defines:

* Trust boundaries
* Layer responsibilities
* Desktop vs mobile differences

Proceeding without a profile is prohibited.

---

### Step 3: Define Architectural Layers

Explicitly define:

* React responsibilities (UI only)
* Rust responsibilities (core logic, policy)
* Native responsibilities (mobile SDKs only)

Layer ambiguity is prohibited.

---

### Step 4: Identify Platform-Specific Requirements

For **mobile**:

* Ads / IAP / analytics
* Native SDK integration (Kotlin / Swift)
* Lifecycle, power, and store compliance

For **desktop**:

* File system access
* Process spawning
* Auto-update behavior
* Long-running stability expectations

Each requirement MUST map to existing rules.

---

### Step 5: Plan IPC and Data Boundaries

Define:

* IPC commands and data shapes
* Ownership and serialization boundaries
* Failure handling across IPC

Shared mutable state across IPC is prohibited.

---

### Step 6: Plan Native Extensions (Mobile Only)

If targeting mobile:

* Design native SDK bridges
* Keep native code minimal
* Route all decisions through Rust policy layer

Calling SDKs from React is prohibited.

---

### Step 7: Plan Build, Packaging, and Distribution

Explicitly plan:

* Desktop packaging per OS
* Mobile build pipelines
* Update and migration strategy
* Store submission requirements

Assuming “default Tauri behavior is enough” is invalid.

---

### Step 8: Implement Incrementally

Implementation MUST:

* Respect UI thread discipline
* Avoid blocking IPC
* Respect power and lifecycle constraints
* Follow file system safety rules

Shortcuts that violate rules are prohibited.

---

### Step 9: Validate Platform Constraints

Validate explicitly:

* Desktop UI responsiveness
* No terminal windows from bundled tools
* File system safety
* Mobile lifecycle handling
* Power and thermal behavior
* Store compliance readiness

Skipping validation is unacceptable.

---

### Step 10: Prepare for Release

Before release:

* Enable crash / ANR observability
* Validate update and rollback paths
* Test long-running sessions
* Test poor network conditions (mobile)

---

## System Safety Checklist (Mandatory)

Before completing this workflow, the agent MUST verify:

### Desktop

* [ ] UI thread is never blocked by IO, IPC, or computation
* [ ] No child process spawns visible terminal windows
* [ ] File system operations are explicit and non-destructive
* [ ] App remains stable over long-running sessions
* [ ] Update and migration paths are validated

### Mobile

* [ ] App tolerates lifecycle changes and process death
* [ ] Power and thermal usage are bounded
* [ ] Network variability and offline states are handled
* [ ] Native SDKs are implemented only in Kotlin / Swift
* [ ] Store compliance requirements are enforced
* [ ] Background execution (if any) follows OS rules

### Cross-Cutting

* [ ] Data ownership and IPC boundaries are explicit
* [ ] Failure in one subsystem does not crash the whole app
* [ ] Observability is enabled before release

---

## Expected Outcome

Following this workflow results in Tauri applications that:

* Are architecturally sound from day one
* Behave correctly across desktop and mobile
* Avoid common production failures
* Scale safely over time
