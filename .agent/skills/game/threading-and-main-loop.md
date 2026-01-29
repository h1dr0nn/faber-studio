# Skill: Threading and Main Loop Design (Game)

## Purpose

This skill defines how the agent should **design and reason about threading**
in relation to the **game main loop**, ensuring:

* Main thread safety
* Predictable frame timing
* Correct use of background threads
* No accidental performance collapse

It applies to:

* Unity-based games
* Tauri-based games (React as rendering layer)

---

## When to Use This Skill

Use this skill when:

* Implementing gameplay systems with non-trivial CPU cost
* Offloading work from the main loop
* Designing background jobs, async tasks, or simulations
* Debugging frame drops, stutters, or long-frame spikes

Do NOT use this skill for:

* UI-only animation
* IO-only async operations
* Tooling or editor-only scripts

---

## Inputs

The agent SHOULD expect:

* Description of the gameplay or system logic
* Execution frequency (per-frame, per-tick, event-driven)
* Target platform (Unity / Tauri, desktop / mobile)
* Observed or expected performance constraints

If execution frequency is unclear, the agent MUST ask.

---

## Core Principle

> **The main loop is sacred.**

The main loop exists to:

* Advance simulation deterministically
* Process input
* Produce the next frame

Anything that threatens its timing is a critical design error.

---

## Unity: Main Thread Rules

### What MUST Run on the Unity Main Thread

The agent MUST ensure the following stay on the main thread:

* Rendering
* Scene graph changes
* Transform updates
* Physics step coordination
* Unity API calls (unless explicitly thread-safe)

The Unity main thread MUST NOT be blocked.

---

### What MUST NOT Run on the Unity Main Thread

The agent MUST NOT execute:

* Heavy computation
* Long-running loops
* Pathfinding, AI planning, or simulations
* Data processing or aggregation

inside:

* `Update`
* `LateUpdate`
* `FixedUpdate`

---

## Unity: Background Work Strategy

When offloading work, the agent MUST:

1. Isolate **pure computation**
2. Run it in background context (e.g. UniTask background)
3. Return results to the main thread explicitly
4. Apply results in a controlled step

Background work MUST:

* Be cancellable
* Respect scene and object lifetimes
* Avoid touching Unity APIs

---

## Tauri Game: Main Loop Rules

### Rendering Thread Discipline

In Tauri-based games:

* The WebView / render thread is equivalent to a UI main thread
* React render must remain lightweight and predictable

The agent MUST NOT:

* Run gameplay simulation in React
* Drive game state via render cycles
* Perform heavy computation in JS event handlers

---

### Native Worker Threads

For Tauri games:

* CPU-heavy logic SHOULD live in native code (Rust)
* Background threads SHOULD be owned and managed natively
* Results MAY be sent to the frontend asynchronously

IPC MUST NOT be used as a per-frame synchronization mechanism.

---

## Frame Budget Awareness

The agent MUST:

* Respect fixed frame budgets (30 / 60 FPS)
* Treat long frames as correctness bugs
* Avoid “sometimes slow” logic in the main loop

A single long frame is worse than several small slowdowns.

---

## Synchronization Rules

The agent MUST avoid:

* Blocking waits on the main loop
* Mutex contention on frame-critical paths
* Frequent cross-thread synchronization

Synchronization SHOULD be:

* Coarse-grained
* Predictable
* Outside hot paths

---

## Background Job Design

Background jobs SHOULD be:

* Coarse-grained
* Low-frequency
* Interruptible
* Result-oriented

Avoid:

* Per-frame job spawning
* Tiny tasks with high overhead
* Fine-grained parallelism

---

## Mobile-Specific Constraints

On mobile platforms, the agent MUST assume:

* Fewer effective CPU cores
* Aggressive thermal throttling
* Lower tolerance for sustained parallelism

The agent SHOULD prefer:

* Burst computation
* Incremental processing across frames
* Early exits and degradation strategies

---

## Validation and Debugging

When threading issues appear, the agent SHOULD:

* Inspect main thread timing
* Look for blocking or synchronization points
* Verify cancellation and cleanup paths

Threading bugs often appear as:

* Frame spikes
* Input lag
* Inconsistent behavior

---

## Output

The output of this skill SHOULD include:

* Identification of main-thread vs background responsibilities
* Clear ownership of background jobs
* Synchronization strategy
* Cancellation and lifecycle handling

---

## Constraints

The agent MUST:

* Follow CPU threading rules
* Follow game architecture rules
* Respect engine-specific thread safety constraints

---

## Anti-Patterns to Avoid

* “Just move it to a thread”
* Background threads touching engine APIs
* Frame-by-frame job spawning
* Blocking waits in the main loop

---

## Expected Outcome

Following this skill results in games that:

* Maintain stable frame timing
* Use background threads intentionally
* Scale safely across platforms and devices
