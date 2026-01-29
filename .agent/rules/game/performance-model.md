# Game Performance Model (Cross-Platform)

## Purpose

This document defines a **shared performance model for game development**
that applies across engines and platforms, including:

* Unity (C#)
* React + Tauri (WebView-based renderer)
* Android / iOS / Desktop targets

Its goal is to ensure performance decisions are:

* Intentional
* Predictable
* Aligned with real device constraints

---

## Core Performance Mindset

### Frame Budget Is the Primary Constraint

Games operate under a fixed **frame budget**.

Typical targets:

* 60 FPS → ~16.6 ms per frame
* 30 FPS → ~33.3 ms per frame

All gameplay, rendering, and system work MUST fit within this budget.

The agent MUST:

* Reason in terms of frame time, not abstract performance
* Treat budget overruns as design issues, not minor bugs

---

### Mobile-First Performance Assumption

Unless explicitly stated otherwise, the agent MUST assume:

* Mobile-class CPUs
* Limited thermal headroom
* Aggressive background throttling

Desktop-class assumptions MUST NOT be the default.

---

## Update Frequency Discipline

### Not All Logic Runs Every Frame

The agent MUST:

* Distinguish between per-frame logic and lower-frequency logic
* Avoid running heavy systems every frame without justification

Examples:

* AI decisions
* Pathfinding
* Economy simulation

These SHOULD:

* Run at fixed intervals
* Be amortized over multiple frames
* Be event-driven when possible

---

### Fixed vs Variable Updates

Gameplay logic SHOULD:

* Be robust to variable frame rates
* Use explicit delta time where appropriate

Critical simulation logic MAY:

* Use fixed-step updates
* Be decoupled from rendering

---

## Allocation and Memory Discipline

### Avoid Per-Frame Allocations

The agent MUST:

* Avoid allocating memory in hot paths
* Avoid per-frame object creation

This applies to:

* C# (GC pressure)
* JavaScript / TypeScript (GC pauses)
* Rust (allocation overhead)

Pooling or reuse SHOULD be preferred where justified.

---

### Memory Growth Awareness

Long-running game sessions MUST:

* Avoid unbounded memory growth
* Release resources explicitly

Memory leaks in games are unacceptable.

---

## Rendering Cost Awareness

### Rendering Is Not Free

The agent MUST:

* Treat rendering as a major cost
* Avoid unnecessary redraws or state changes

For React-based games:

* Re-rendering large trees is expensive
* React reconciliation is not a game loop

For Unity:

* Draw calls
* Overdraw
* Shader complexity matter

---

## Asynchronous Work

### Async Does Not Eliminate Cost

Async or background work:

* Still consumes CPU
* Still competes for resources

The agent MUST:

* Avoid unbounded async work
* Ensure async tasks do not starve the main loop

---

## Instrumentation and Measurement

### Measure Before Optimizing

The agent SHOULD:

* Encourage profiling on target devices
* Use frame timing metrics
* Identify real bottlenecks

Speculative optimization without measurement is discouraged.

---

## Performance Trade-Off Transparency

When proposing optimizations, the agent MUST:

* State the trade-off clearly
* Explain what is gained and what is sacrificed

Performance at the cost of correctness or clarity requires justification.

---

## Anti-Patterns to Avoid

* Treating mobile devices as “small desktops”
* Running all systems every frame
* Excessive dynamic allocation
* UI-driven game logic
* Ignoring thermal throttling

---

## Scope and Authority

* This document applies to all game performance considerations.
* Engine-specific rules may refine implementation details.
* In case of conflict, platform constraints take precedence.

---

## Expected Outcome

Following this performance model results in games that:

* Run smoothly across platforms
* Scale in complexity without collapsing performance
* Behave predictably on real devices
