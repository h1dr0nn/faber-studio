---
description: Controlled process for preparing, building, and releasing game builds for target platforms.
---

# Workflow: Release Game Build (Unity & Tauri)

## Purpose

This workflow defines the **controlled process for preparing, building, and releasing**
a game to target platforms, including:

- Unity games (Android / iOS)
- Tauri-based games (Android / iOS / Desktop)

It exists to ensure releases are:

- Stable
- Performant
- Secure
- Reproducible

---

## When to Use This Workflow

Use this workflow when:

- Preparing a public or internal game release
- Creating QA, beta, or production builds
- Shipping updates to existing players

Do NOT use this workflow for:

- Local development builds
- Experimental or prototype builds

---

## Inputs

The agent SHOULD expect:

- Target platforms (Android, iOS, Desktop)
- Release type (QA, Beta, Production)
- Target FPS and performance requirements
- Security or compliance constraints

If release scope or target platforms are unclear, the agent MUST ask.

---

## Step-by-Step Workflow

### Step 1: Freeze Scope and Features

- Lock the feature set for this release
- Identify known issues explicitly
- Define release-blocking vs non-blocking bugs

**Output**

- Release scope definition
- Known issues list

---

### Step 2: Validate Gameplay and Logic Stability

- Run through core gameplay flows
- Validate state transitions and invariants
- Ensure no debug-only logic remains enabled

**Skills Used**

- `debug-runtime-issue`

---

### Step 3: Performance Validation

- Validate FPS targets on representative devices
- Run long-session tests (thermal, memory)
- Verify no performance regressions

**Skills Used**

- `optimize-mobile-fps`
- `backend-performance-profiling`

**Output**

- Performance validation notes
- Pass / fail status

---

### Step 4: Security and Boundary Review

- Review client trust assumptions
- Validate IPC exposure (for Tauri)
- Disable debug commands and dev tools

**Rules Applied**

- Security philosophy
- Tauri or Unity security rules

**Output**

- Security checklist
- Mitigation confirmations

---

### Step 5: Build Configuration Verification

#### Unity

- Verify build settings
- Confirm correct scripting backend
- Confirm stripping and optimization settings
- Disable development build flags

#### Tauri

- Verify permissions and capabilities
- Verify IPC allowlist
- Verify environment separation (dev vs prod)

---

### Step 6: Build Generation

- Produce builds for each target platform
- Ensure builds are reproducible
- Tag builds with version and metadata

**Output**

- Release artifacts
- Build metadata

---

### Step 7: Smoke Testing

- Install builds on real devices
- Validate startup, core flows, and shutdown
- Check save/load behavior

Smoke tests MUST pass before release.

---

### Step 8: Final Sign-Off

- Review release checklist
- Confirm all blockers resolved
- Approve release explicitly

---

## Failure Handling

If at any step:

- Critical bugs are found
- Performance targets are missed
- Security risks are identified

The agent MUST:

- Halt the release
- Surface issues clearly
- Recommend next actions

---

## Deliverables

By the end of this workflow, the agent SHOULD produce:

- Verified release builds
- Performance and security notes
- Release checklist and sign-off record

---

## Anti-Patterns to Avoid

- Shipping untested builds
- Ignoring long-session performance
- Leaving debug features enabled
- Rushing release without sign-off

---

## Expected Outcome

Following this workflow results in game releases that:

- Are stable on target platforms
- Meet performance expectations
- Avoid common release-time failures
