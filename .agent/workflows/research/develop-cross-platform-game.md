---
description: Structured workflow for developing cross-platform games using Unity and/or React + Tauri, ensuring performance, lifecycle safety, and platform compliance.
---

# Workflow: Develop Cross-Platform Game (Unity / Tauri)

## Purpose

This workflow defines a **safe, structured process** for developing
**cross-platform games**, including:

* Unity-based games (C#)
* React + Tauri–based games (Android / iOS / Desktop)

It exists to prevent:

* Platform-specific performance failures
* Lifecycle and power bugs on mobile
* Architecture drift between game logic and UI
* Store compliance issues late in development

---

## When to Use This Workflow

Use this workflow when:

* Starting a new game project
* Adding a major gameplay system
* Porting a game between platforms
* Integrating SDKs (ads, IAP, analytics)

Do NOT use this workflow for:

* Small UI tweaks
* One-off prototypes

---

## Inputs

* Game platform targets (mobile, desktop, or both)
* Game engine choice (Unity, Tauri, or hybrid)
* Monetization requirements (ads, IAP)
* Online/offline requirements

---

## Step-by-Step Workflow

### Step 1: Declare Game Architecture

Explicitly declare:

* Game engine (Unity or Tauri)
* Separation between gameplay logic and UI
* Target platforms

Ambiguous architecture leads to unmaintainable code.

---

### Step 2: Apply Relevant Profiles

Apply:

* `profiles/unity-mobile.md` (if using Unity)
* `profiles/tauri-app.md` (if using Tauri)
* Both, if the project is hybrid

Profiles define non-negotiable constraints.

---

### Step 3: Define Core Game Loop and Time Model

Explicitly define:

* Simulation time vs frame time
* Determinism requirements
* Async boundaries

Frame-rate–dependent gameplay logic is prohibited.

---

### Step 4: Plan Platform-Specific Constraints

For **mobile**:

* Lifecycle and process death
* Power and thermal limits
* Store compliance
* Native SDK integration

For **desktop**:

* UI responsiveness
* Long-running stability
* File system usage (if any)

---

### Step 5: Integrate SDKs Safely

If monetization or analytics are required:

* Ads / IAP / attribution MUST be native on mobile
* Decisions MUST be gated by core policy (Rust or C#)
* UI MUST not call SDKs directly

SDK misuse leads to store rejection.

---

### Step 6: Network and Offline Planning

Explicitly plan:

* Online vs offline gameplay
* Sync strategy
* Failure and retry behavior

Network failure MUST not break gameplay.

---

### Step 7: Performance and Resource Discipline

Ensure:

* No heavy work on main/UI threads
* Memory usage remains bounded
* Background tasks are cancellable

Performance issues accumulate over long sessions.

---

### Step 8: Lifecycle and State Persistence

Ensure:

* Game state can survive mobile process death
* Save/load paths are explicit
* Partial progress is preserved safely

Memory-only progress is unacceptable on mobile.

---

### Step 9: Validation on Real Devices

Validate:

* Mobile power and thermal behavior
* Background / foreground transitions
* Desktop long-running stability
* UI responsiveness

Simulator-only validation is insufficient.

---

### Step 10: Prepare for Release

Before release:

* Enable crash / ANR observability
* Validate store compliance
* Test update and migration paths (if applicable)

---

## System Safety Checklist (Mandatory)

Before completing this workflow, the agent MUST verify:

### Game Loop & Time

* [ ] Simulation logic is frame-rate independent
* [ ] Async work does not mutate core state directly

### Mobile

* [ ] Lifecycle and process death are handled
* [ ] Power and thermal behavior are acceptable
* [ ] Native SDKs are isolated correctly
* [ ] Store compliance rules are enforced

### Desktop

* [ ] UI remains responsive
* [ ] Long-running sessions are stable
* [ ] No visible terminals or debug windows appear

### Cross-Cutting

* [ ] Data ownership is explicit
* [ ] Failures are isolated
* [ ] Observability is enabled

---

## Expected Outcome

Following this workflow results in games that:

* Run reliably across platforms
* Respect mobile and desktop constraints
* Avoid late-stage architectural rewrites
* Are ready for store submission
