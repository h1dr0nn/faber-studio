# Failure Isolation and Error Containment Rules

## Purpose

This document defines **non-negotiable rules** for **failure isolation, error propagation,
and containment** across all systems.

It exists to prevent:

* Cascading failures
* Total system crashes from localized faults
* Silent corruption after partial failure
* Unrecoverable states in long-running applications

These rules apply to:

* Unity games
* Tauri applications and games
* Rust backends
* Shared libraries
* Tooling and automation

---

## Fundamental Principles

### 1. Failure Is Inevitable, Collapse Is Optional

The agent MUST assume:

* Errors will occur
* External systems will fail
* Unexpected states will be reached

System design MUST focus on **containing failure**, not pretending it will not happen.

---

### 2. Failures Must Be Localized

A failure MUST:

* Affect the smallest possible scope
* Not automatically propagate across system boundaries

A failure in one subsystem MUST NOT crash unrelated subsystems.

---

## Failure Domains

The agent MUST explicitly identify **failure domains**, such as:

* A gameplay feature
* A background job
* A native module
* An IPC command
* A UI component or screen

Failure domains MUST have clear boundaries.

---

## Error Propagation Rules

### Explicit Error Surfaces

Errors MUST:

* Be surfaced explicitly
* Be represented as structured results or states

Silent failures are prohibited.

---

### Boundary Translation

At system boundaries:

* Errors MUST be translated into boundary-appropriate representations
* Internal error details MUST NOT leak outward

Each boundary defines what error information is safe to expose.

---

## Main Loop Protection Rule

The following loops MUST be protected from fatal failure:

* Unity game loop
* Tauri render / UI loop
* Core scheduling loops

The agent MUST NOT:

* Allow unhandled exceptions to escape into main loops
* Let background task failure crash the main loop

Main loop termination is a catastrophic failure.

---

## Background Work Isolation

Background jobs MUST:

* Fail independently
* Report failure without blocking the main loop
* Be cancellable and restartable where appropriate

A failed background job MUST NOT:

* Deadlock the system
* Leave shared state partially mutated

---

## State Integrity on Failure

On failure, the agent MUST ensure:

* State remains valid or is rolled back
* Partial updates are not committed
* Invariants are preserved

Corrupted state is worse than lost work.

---

## Retry and Degradation Discipline

Retries MAY be used only when:

* Failure is transient
* Retry cost is bounded
* Backoff strategy exists

Automatic retries without limits are prohibited.

Graceful degradation SHOULD be preferred over retries when possible.

---

## User-Facing Error Handling

User-facing systems MUST:

* Fail visibly but safely
* Avoid freezing or crashing
* Preserve user control where possible

Hiding failure leads to worse outcomes.

---

## Logging and Observation

Failures MUST:

* Be observable
* Be logged at appropriate severity
* Avoid excessive noise

Logging MUST NOT:

* Become a bottleneck
* Leak sensitive information

---

## Cross-Language Failure Rules

Across Rust ↔ TypeScript ↔ C# boundaries:

* Failures MUST be serialized explicitly
* Panics or crashes MUST NOT cross boundaries
* Each language layer must regain control after failure

Undefined behavior across language boundaries is unacceptable.

---

## Anti-Patterns (Explicitly Forbidden)

* Letting one feature crash the whole app
* Catching errors and continuing blindly
* Global try/catch around the entire system
* Retrying infinitely
* Corrupting state to “keep going”

---

## Scope and Authority

* These rules apply globally.
* They override convenience-based error handling.
* Stricter domain-specific rules may apply.

---

## Expected Outcome

Following these rules results in systems that:

* Survive partial failures
* Remain responsive under error conditions
* Are easier to debug and recover
* Protect long-running stability
