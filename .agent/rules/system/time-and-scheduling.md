# Time and Scheduling Rules

## Purpose

This document defines **non-negotiable rules** for **time modeling, scheduling,
and temporal reasoning** across all systems.

It exists to prevent:

* Frame-rate–dependent bugs
* Time drift and desynchronization
* Incorrect async timing assumptions
* Non-deterministic behavior that is hard to debug

These rules apply to:

* Unity games
* Tauri-based games and tools
* Rust backends
* Async systems and background jobs

---

## Fundamental Principles

### 1. Time Is a First-Class Concept

The agent MUST treat time as:

* Explicit
* Modeled
* Intentionally chosen

Using time implicitly or incidentally is prohibited.

---

### 2. Multiple Time Domains Exist

The agent MUST distinguish between the following time domains:

* **Frame Time**
  Time between rendered frames

* **Simulation Time**
  Logical game or system time

* **Wall-Clock Time**
  Real-world elapsed time

* **Async Completion Time**
  Time when async work finishes

Mixing time domains without explicit conversion is prohibited.

---

## Frame Time Rules

### Frame-Rate Independence

Gameplay and system logic MUST NOT:

* Depend directly on frame rate
* Assume fixed frame intervals

Frame-dependent behavior is a correctness bug.

---

### Frame Budget Discipline

The agent MUST:

* Respect fixed frame budgets (30 / 60 FPS)
* Treat long frames as errors, not edge cases

Frame overruns accumulate user-visible problems.

---

## Simulation Time Rules

### Simulation Drives Logic

Game logic SHOULD:

* Advance based on simulation time
* Be independent of rendering cadence

Simulation time MAY:

* Advance in fixed steps
* Advance in controlled variable steps

The chosen model MUST be explicit.

---

### Determinism Bias

Where feasible, systems SHOULD:

* Produce deterministic results given the same inputs
* Isolate randomness explicitly

Non-determinism MUST be justified.

---

## Wall-Clock Time Rules

Wall-clock time SHOULD be used only for:

* Real-world timestamps
* Persistence
* Analytics
* Scheduling outside gameplay logic

Wall-clock time MUST NOT:

* Drive gameplay mechanics
* Control simulation progression

---

## Async and Scheduling Rules

### Async Completion Is Not Time Progress

The agent MUST NOT:

* Assume async completion timing aligns with simulation time
* Trigger gameplay logic directly on async completion without mediation

Async results MUST be:

* Integrated explicitly
* Applied at controlled synchronization points

---

### Scheduling Discipline

Scheduled actions MUST:

* Declare which time domain they use
* Be cancellable
* Be lifecycle-aware

Implicit scheduling is prohibited.

---

## Pausing, Slow Motion, and Time Scaling

Time scaling MUST:

* Affect simulation time only
* Not affect wall-clock measurements
* Be explicitly applied

Pausing MUST:

* Freeze simulation time
* Allow background or IO work to continue if appropriate

---

## Cross-System Time Coordination

Across systems:

* Time domain boundaries MUST be explicit
* Data exchanged MUST carry timing semantics if relevant

Assuming shared understanding of “now” is prohibited.

---

## Error Handling and Time

On errors:

* Time progression MUST remain consistent
* Partial time advancement MUST be avoided

Temporal inconsistency leads to cascading bugs.

---

## Anti-Patterns (Explicitly Forbidden)

* Frame-rate–dependent logic
* Using `deltaTime` as a universal fix
* Async callbacks mutating simulation state directly
* Mixing wall-clock and simulation time
* Hidden timers or implicit delays

---

## Scope and Authority

* These rules apply globally.
* They override convenience-based timing logic.
* Stricter domain rules may apply.

---

## Expected Outcome

Following these rules results in systems that:

* Behave consistently across devices
* Are easier to debug and test
* Avoid time-related instability
