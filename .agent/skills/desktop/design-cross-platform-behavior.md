# Skill: Design Cross-Platform Behavior (Desktop)

## Purpose

This skill defines how the agent should **design and reason about cross-platform behavior**
for desktop applications targeting:

* Windows
* macOS
* Linux

It exists to prevent:

* OS-specific bugs caused by incorrect assumptions
* “Works on my machine” failures
* Hidden platform coupling
* Fragile conditional logic scattered throughout the codebase

This skill focuses on **design-time decisions**, not platform-specific coding syntax.

---

## When to Use This Skill

Use this skill when:

* Designing new desktop features
* Handling file system, process, or OS integration
* Adding native extensions or helpers
* Refactoring shared logic used across platforms

Do NOT use this skill for:

* Single-platform prototypes
* Throwaway internal tools
* Platform-specific experiments

---

## Core Principle

> **Cross-platform correctness comes from explicit design, not conditional hacks.**

Assuming platform similarity is the primary source of desktop bugs.

---

## Step 1: Identify Platform-Sensitive Areas

The agent MUST identify whether a feature touches:

* File system paths
* Permissions or access control
* Process spawning
* Windowing or focus behavior
* Packaging and installation layout
* System integrations (tray, notifications)

If yes, the feature is platform-sensitive.

---

## Step 2: Separate Policy from Mechanism

For platform-sensitive features, the agent SHOULD:

* Define **what** the feature does (policy)
* Separate **how** it is implemented (mechanism)

Policy MUST remain platform-agnostic.
Mechanism MAY differ per OS.

---

## Step 3: Prefer Capability Detection Over OS Detection

The agent SHOULD prefer:

* Detecting capabilities or availability
* Checking APIs or permissions

Over:

* Hard-coded OS checks
* Version string comparisons

Assuming OS behavior from version alone is fragile.

---

## Step 4: Define Platform Boundaries Explicitly

The agent SHOULD:

* Isolate platform-specific code
* Avoid scattering `if (windows)` / `if (mac)` logic
* Centralize platform adaptations

Cross-platform code should read cleanly.

---

## Step 5: Define Failure and Fallback Behavior

For each platform-sensitive feature, the agent MUST define:

* What happens if the feature is unavailable
* How the app degrades gracefully
* What the user is informed about

Undefined fallback behavior is unacceptable.

---

## Step 6: Packaging and Distribution Awareness

The agent MUST consider:

* Different install locations per OS
* Different permission models
* Differences in update mechanisms

Assuming identical packaging behavior is invalid.

---

## Step 7: Validation Strategy

The agent SHOULD:

* Test features on real target OSes
* Validate assumptions explicitly
* Avoid relying solely on CI or emulators

Cross-platform issues often appear only in real environments.

---

## Common Design Mistakes (Avoid)

* Assuming POSIX behavior everywhere
* Hard-coding path separators
* Relying on terminal availability
* Assuming identical window focus behavior
* Treating Linux as “just another Unix”

---

## Output

The output of this skill SHOULD include:

* Identified platform-sensitive areas
* Policy vs mechanism separation
* Platform adaptation strategy
* Defined fallback behaviors

---

## Postconditions

After executing this skill:

* Platform differences should be explicit
* Cross-platform bugs should be reduced
* Platform-specific logic should be contained

---

## Constraints

The agent MUST:

* Follow desktop system rules
* Avoid speculative abstractions
* Keep platform logic auditable

---

## Expected Outcome

Following this skill results in desktop applications that:

* Behave correctly across OSes
* Are easier to maintain
* Avoid subtle platform-specific failures
