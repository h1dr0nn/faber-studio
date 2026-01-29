# Skill: Design Rust Module (Production-Oriented)

## Purpose

This skill defines how the agent should **design and structure a Rust module**
for backend, systems, or native layers used in:

* Tauri backends
* Native tooling
* Performance-critical components
* Cross-language boundaries (IPC / FFI)

The goal is to produce Rust modules that are:

* Safe by default
* Easy to reason about
* Maintainable over time
* Suitable for long-running use

---

## When to Use This Skill

Use this skill when:

* Creating a new Rust module or subsystem
* Refactoring existing Rust code into clearer boundaries
* Designing native functionality exposed to frontend or tools

Do NOT use this skill for:

* One-off scripts
* Trivial glue code that does not warrant a module boundary

---

## Inputs

The agent SHOULD expect:

* The responsibility of the module
* Public API requirements
* Performance or safety constraints
* Whether the module is exposed externally (IPC / FFI)

If the module’s responsibility is unclear, the agent MUST ask for clarification.

---

## Preconditions

Before executing this skill, the agent MUST:

1. Clearly define the module’s single responsibility
2. Identify what the module OWNS vs USES
3. Determine whether the module is:

   * Internal
   * Public
   * Boundary-facing (IPC / FFI)

---

## Core Design Principles

### 1. Explicit Module Responsibility

Each Rust module MUST:

* Have a clear, narrow purpose
* Avoid owning unrelated concerns

The agent MUST NOT:

* Create “utility” modules without clear scope
* Combine unrelated logic for convenience

---

### 2. Public API Discipline

Public APIs MUST:

* Be minimal
* Be intention-revealing
* Hide internal implementation details

The agent SHOULD:

* Prefer small public surfaces
* Use `pub(crate)` where appropriate

---

### 3. Ownership and Data Flow

The agent MUST:

* Design APIs with clear ownership transfer
* Avoid ambiguous borrowing patterns at boundaries

Prefer:

* Owned types in public APIs
* Borrowing for internal implementation details

---

## Error Handling

### Result-Oriented APIs

Fallible operations MUST:

* Return `Result<T, E>`
* Use structured error types
* Preserve context

The agent MUST NOT:

* Panic in normal execution paths
* Expose raw internal errors across boundaries

---

### Error Translation at Boundaries

When exposed externally:

* Errors MUST be translated into stable representations
* Internal details MUST NOT leak

---

## Async and Concurrency Considerations

If the module is async-aware:

* Async boundaries MUST be explicit
* Blocking operations MUST be isolated

The agent MUST:

* Avoid mixing sync and async logic implicitly
* Be clear about runtime expectations

---

## State and Mutability

The agent SHOULD:

* Minimize shared mutable state
* Prefer explicit state transitions

When mutation is required:

* Scope it tightly
* Make it visible in the API

---

## Performance Awareness

The agent MUST:

* Be conscious of allocations
* Avoid unnecessary cloning
* Justify optimizations

Performance-sensitive paths SHOULD:

* Be documented
* Be measurable

---

## Documentation Requirements

Each module MUST include:

* A module-level doc comment explaining purpose
* Documentation for all public APIs
* Notes on invariants or constraints

---

## Testing Considerations

Modules SHOULD:

* Be testable in isolation
* Avoid reliance on global state

The agent SHOULD:

* Encourage unit tests for core logic
* Avoid designs that require full application context to test

---

## Postconditions

After executing this skill:

* The module should have a clear boundary
* Public APIs should be minimal and safe
* Internal complexity should be hidden

---

## Constraints

The agent MUST:

* Follow Rust safety and async rules
* Respect security boundaries
* Avoid speculative abstraction

---

## Anti-Patterns to Avoid

* Overly generic modules
* Exposing internal data structures
* Panic-driven control flow
* Lifetime-heavy public APIs without necessity

---

## Expected Outcome

Following this skill results in Rust modules that:

* Are safe and predictable
* Scale with system complexity
* Integrate cleanly with Tauri and other layers
