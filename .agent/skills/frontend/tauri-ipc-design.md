# Skill: Design Tauri IPC Interfaces

## Purpose

This skill defines how the agent should **design, evaluate, and use IPC interfaces**
between the frontend (React) and the native backend (Rust / system code)
in Tauri-based applications and games.

The goal is to ensure IPC usage is:

* Secure
* Performant
* Intentional
* Maintainable over time

---

## When to Use This Skill

Use this skill when:

* Defining new IPC commands
* Refactoring existing IPC boundaries
* Evaluating IPC performance or security issues
* Designing frontend–native interaction for tools or games

Do NOT use this skill for:

* Pure frontend logic
* In-process communication

---

## Inputs

The agent SHOULD expect:

* Description of the required native capability
* Data that must cross the IPC boundary
* Performance expectations (frequency, latency)
* Security considerations

If IPC frequency or sensitivity is unclear, the agent MUST ask.

---

## Preconditions

Before executing this skill, the agent MUST:

1. Confirm that IPC is truly required
2. Identify the trust boundary being crossed
3. Classify the IPC call as:

   * Occasional
   * Interactive
   * Performance-sensitive

---

## Core IPC Design Principles

### 1. IPC Is Expensive

The agent MUST treat IPC calls as:

* High-latency relative to in-process calls
* Potential performance bottlenecks

IPC MUST NOT:

* Be used in per-frame loops
* Be triggered excessively by UI events

---

### 2. Coarse-Grained Commands

IPC commands SHOULD:

* Represent meaningful domain operations
* Perform substantial work per call

Avoid:

* Fine-grained “getter/setter” style IPC
* Chatty interfaces

---

### 3. Explicit Data Contracts

IPC payloads MUST:

* Be explicitly structured
* Have clear ownership and semantics
* Avoid loosely-typed or ad-hoc shapes

All fields MUST:

* Be validated on the native side
* Be treated as untrusted input

---

## Security Discipline

### Frontend Is Untrusted

The agent MUST assume:

* IPC inputs can be forged
* IPC calls can be replayed
* Payloads can be malformed

Therefore:

* Validation MUST occur in native code
* Sensitive logic MUST NOT live in the frontend

---

### Least Privilege IPC

Each IPC command MUST:

* Expose the minimal necessary capability
* Avoid generic or overly powerful actions

The agent MUST NOT:

* Expose arbitrary file system access
* Expose command execution APIs

---

## Performance Guidelines

### Frequency Awareness

The agent MUST:

* Minimize IPC call frequency
* Batch operations when possible
* Cache results on the frontend when appropriate

---

### Async and Feedback

IPC interactions MUST:

* Be asynchronous
* Provide clear loading or progress feedback
* Avoid blocking UI or game loops

---

## Error Handling

IPC errors MUST:

* Be surfaced clearly to the frontend
* Be sanitized
* Avoid leaking native implementation details

Frontend code MUST:

* Handle IPC failures gracefully
* Avoid assuming success

---

## Evolution and Maintainability

IPC interfaces SHOULD:

* Be versionable
* Be backward-compatible when feasible
* Avoid breaking changes without justification

The agent SHOULD:

* Consider future extensibility
* Avoid overfitting to current UI needs

---

## Postconditions

After applying this skill:

* IPC interfaces should be explicit and understandable
* Security boundaries should be clear
* Performance risks should be minimized

---

## Constraints

The agent MUST:

* Follow global and security rules
* Respect Tauri-specific security constraints
* Avoid speculative IPC design

---

## Anti-Patterns to Avoid

* Per-frame IPC calls
* Overly generic commands
* Trusting frontend input
* IPC-driven application architecture

---

## Expected Outcome

Following this skill results in IPC designs that:

* Are secure by default
* Scale as the application grows
* Do not become performance bottlenecks
