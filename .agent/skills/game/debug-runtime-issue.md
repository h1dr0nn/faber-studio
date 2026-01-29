# Skill: Debug Game Runtime Issues

## Purpose

This skill defines how the agent should **systematically debug runtime issues**
in game projects, including:

* Unity-based games
* Tauri-based games (React + Canvas / WebGL)

It exists to ensure debugging is:

* Structured
* Root-cause driven
* Safe (does not introduce new instability)

---

## When to Use This Skill

Use this skill when:

* The game crashes or freezes at runtime
* Gameplay behavior is incorrect or inconsistent
* State becomes corrupted over time
* Issues appear only on certain platforms or devices
* Bugs occur only after long play sessions

Do NOT use this skill for:

* Compile-time or build errors
* Pure UI layout issues (use UI debug skills instead)

---

## Inputs

The agent SHOULD expect:

* Description of the observed issue
* Platform(s) affected (Unity, Tauri, Android, iOS, Desktop)
* Reproduction steps (if known)
* Logs, error messages, or stack traces (if available)

If reproduction steps are unclear, the agent MUST ask.

---

## Preconditions

Before debugging, the agent MUST:

1. Classify the issue type:

   * Crash
   * Freeze / hang
   * Logic error
   * Performance degradation
2. Identify when the issue occurs:

   * Immediately
   * After a specific action
   * After long runtime
3. Determine whether the issue is:

   * Deterministic
   * Intermittent
   * Platform-specific

---

## Core Debugging Principles

### 1. Reproduce Before Fixing

The agent MUST:

* Attempt to reproduce the issue
* Avoid applying fixes without understanding the cause

Fixes without reproduction are speculative and risky.

---

### 2. Minimize the Scope

The agent SHOULD:

* Narrow down the smallest context where the issue occurs
* Disable unrelated systems temporarily to isolate the problem

Broad changes obscure root causes.

---

## Debugging Strategies

### State and Logic Inspection

The agent SHOULD:

* Inspect game state before and after the failure
* Verify invariants and assumptions
* Check for unexpected state mutations

Corrupted state is a common source of runtime bugs.

---

### Lifecycle and Timing Issues

The agent MUST:

* Inspect lifecycle boundaries (scene load/unload, mount/unmount)
* Check async task cancellation and cleanup
* Verify that logic does not run after disposal

Many runtime bugs originate from lifecycle misuse.

---

### Async and Concurrency Bugs

For async-related issues, the agent SHOULD:

* Verify cancellation tokens are respected
* Check for race conditions
* Ensure async tasks do not outlive their owners

Fire-and-forget logic is a frequent culprit.

---

### Performance and Resource Issues

For freezes or degradation, the agent SHOULD:

* Check for unbounded loops or per-frame allocations
* Monitor memory growth over time
* Look for GC spikes or thread starvation

Long-session bugs often hide here.

---

## Platform-Specific Considerations

### Unity

The agent SHOULD:

* Inspect MonoBehaviour lifecycles
* Check scene transitions and object persistence
* Verify DI scope alignment (VContainer)

---

### Tauri-Based Games

The agent SHOULD:

* Inspect game loop timing
* Check React render frequency
* Verify IPC usage and async boundaries

Browser-like assumptions often cause bugs here.

---

## Logging and Instrumentation

The agent SHOULD:

* Add targeted logging around suspected areas
* Log state transitions and lifecycle events

Logging MUST:

* Be temporary and scoped
* Avoid spamming or performance degradation

---

## Fix Validation

After applying a fix, the agent MUST:

* Reproduce the original scenario
* Confirm the issue is resolved
* Verify no regressions were introduced

---

## Postconditions

After executing this skill:

* The root cause should be identified
* The fix should be minimal and targeted
* The game should behave predictably under the same conditions

---

## Constraints

The agent MUST:

* Follow game architecture and performance rules
* Avoid broad refactors during debugging
* Avoid masking issues instead of fixing them

---

## Anti-Patterns to Avoid

* “Fixing” by adding delays or retries blindly
* Resetting state to hide bugs
* Disabling features without understanding why
* Shipping debug code to production

---

## Expected Outcome

Following this skill results in runtime debugging that:

* Identifies real root causes
* Produces stable, durable fixes
* Improves long-term game reliability
