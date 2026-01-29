# Long-Running Stability Rules (Desktop)

## Purpose

This document defines **non-negotiable rules** for ensuring **stability over long-running
sessions** in desktop applications on:

* Windows
* macOS
* Linux

It exists to prevent:

* Gradual memory leaks
* File handle and resource exhaustion
* Performance degradation over time
* Failures that appear only after hours or days of use

These rules apply to:

* Tauri desktop applications
* Unity desktop tools or games
* Any desktop app expected to run for long periods

---

## Fundamental Principle

> **Desktop applications are expected to run for hours or days without degradation.**

Small leaks and unmanaged resources accumulate into fatal failures over time.

---

## Resource Lifetime Discipline

The agent MUST ensure:

* Every acquired resource has a defined release point
* Resource ownership is explicit
* Cleanup occurs deterministically when possible

Resources include:

* Memory allocations
* File handles
* Network sockets
* OS handles
* GPU or native resources

Implicit cleanup assumptions are prohibited.

---

## Memory Stability Rules

The agent MUST:

* Avoid unbounded memory growth
* Avoid retaining references unnecessarily
* Periodically validate steady-state memory usage

Memory usage SHOULD plateau during normal operation.

---

## Handle and Descriptor Management

The agent MUST:

* Close file handles explicitly
* Release OS resources promptly
* Avoid leaking descriptors in loops or repeated operations

Handle leaks often surface only after extended use.

---

## Background Task Discipline

Background tasks MUST:

* Have clear lifetimes
* Be cancellable
* Terminate cleanly when no longer needed

Orphaned background tasks are prohibited.

---

## Event Listener and Subscription Cleanup

The agent MUST:

* Unsubscribe event listeners when no longer needed
* Avoid accumulating observers or callbacks
* Ensure subscriptions align with object lifetime

Listener leaks are a common source of gradual degradation.

---

## Periodic Cleanup and Housekeeping

For long-running apps, the agent SHOULD:

* Perform periodic cleanup
* Reset caches where appropriate
* Release unused resources proactively

Neglecting housekeeping leads to slow failure.

---

## Detection and Validation

The agent SHOULD:

* Monitor memory and resource usage over time
* Test extended sessions intentionally
* Detect growth trends early

Stability issues must be observable.

---

## Error Handling and Stability

On errors:

* Resources MUST still be released
* Cleanup paths MUST run reliably
* Partial failures MUST NOT leak resources

Error paths are frequent leak sources.

---

## Cross-Layer Responsibilities

### React / UI

* MUST avoid accumulating listeners
* MUST clean up subscriptions on unmount

---

### Rust / Native Core

* MUST enforce RAII or explicit cleanup
* MUST audit long-lived structures

---

## Anti-Patterns (Explicitly Forbidden)

* “It’s a small leak, it won’t matter”
* Relying solely on process restart for cleanup
* Fire-and-forget tasks without cancellation
* Never-cleared caches
* Ignoring long-session testing

---

## Scope and Authority

* These rules apply to all desktop builds.
* They override convenience-based resource handling.
* Stricter OS-specific rules may apply.

---

## Expected Outcome

Following these rules results in desktop applications that:

* Remain stable over long sessions
* Avoid gradual performance collapse
* Use system resources responsibly
* Feel reliable and professional
