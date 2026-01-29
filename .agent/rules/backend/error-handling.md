# Error Handling and Failure Management

## Purpose

This document defines **how errors, failures, and exceptional conditions**
must be handled across backend and systems code assisted by the agent.

It applies to:

* Rust backend code
* Python scripts and automation
* Cross-language boundaries (IPC, FFI)
* Tooling and long-running services

The goal is to ensure failures are:

* Visible
* Actionable
* Non-destructive

---

## Core Philosophy

### Errors Are Expected, Not Exceptional

Errors are a normal part of system behavior.

The agent MUST:

* Design for failure explicitly
* Treat error handling as a first-class concern

Ignoring or hiding errors is unacceptable.

---

## Error Classification

Errors SHOULD be classified into clear categories:

* **Recoverable errors**

  * Invalid input
  * Missing resources
  * Transient I/O issues

* **Non-recoverable errors**

  * Corrupted state
  * Broken invariants
  * Programmer errors

The handling strategy MUST reflect the category.

---

## Rust Error Handling Rules

### Prefer `Result` Over Panics

The agent MUST:

* Use `Result<T, E>` for fallible operations
* Propagate errors intentionally

The agent MUST NOT:

* Use `panic!` in production code paths
* Use `unwrap()` or `expect()` without strong justification

---

### Preserve Error Context

Errors SHOULD:

* Include meaningful context
* Preserve underlying causes

Avoid:

* Replacing structured errors with generic messages
* Losing stack or source information

---

## Python Error Handling Rules

### Explicit Exception Handling

Python code MUST:

* Catch exceptions where recovery is possible
* Let unrecoverable exceptions terminate execution clearly

The agent MUST NOT:

* Swallow exceptions silently
* Catch broad exceptions without rethrowing or logging

---

### Exit Codes Matter

Scripts MUST:

* Exit with non-zero status on failure
* Communicate failure clearly to calling systems

---

## Cross-Language Boundaries

### Validate All Inputs

All data crossing boundaries MUST:

* Be validated
* Be treated as untrusted

Never assume:

* Correct type
* Valid range
* Well-formed structure

---

### Error Translation

Errors crossing language boundaries MUST:

* Be translated into appropriate representations
* Preserve intent and severity

Avoid leaking:

* Internal implementation details
* Language-specific abstractions

---

## Logging and Observability

Errors MUST:

* Be logged with sufficient context
* Include relevant identifiers or state

Logging SHOULD:

* Be structured where possible
* Avoid excessive noise

---

## User-Facing Errors

User-visible errors SHOULD:

* Be clear and actionable
* Avoid technical jargon
* Avoid exposing internal details

Internal diagnostics SHOULD remain internal.

---

## Anti-Patterns to Avoid

* Silent failures
* Catch-all error handling
* Panic-driven control flow
* Logging without context

---

## Scope and Authority

* This document applies to all backend error handling.
* Security rules may impose stricter requirements.
* In case of conflict, safety and correctness take precedence.

---

## Expected Outcome

Following these rules ensures systems that:

* Fail predictably
* Are easier to debug
* Inspire confidence in long-running use
