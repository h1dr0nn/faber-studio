# Rust Safety and Correctness Rules

## Purpose

This document defines **mandatory safety, correctness, and design rules**
for Rust code produced or assisted by the agent.

It applies to:

* Tauri backend code
* Systems utilities
* Performance-critical components
* Cross-language boundaries (IPC, FFI)

The goal is to leverage Rust’s strengths without:

* Overcomplicating design
* Fighting the borrow checker
* Introducing unnecessary `unsafe` code

---

## Safety as a Default

### Zero `unsafe` by Default

The agent MUST assume:

* All Rust code is written in **safe Rust** by default.

`unsafe` code is ONLY allowed when:

* There is a concrete, documented reason
* Performance or FFI requirements demand it
* The unsafe boundary is tightly scoped

Every use of `unsafe` MUST:

* Be justified
* Be isolated
* Include a safety comment explaining invariants

---

## Ownership and Borrowing Discipline

### Prefer Clear Ownership Models

The agent MUST:

* Design APIs with explicit ownership semantics
* Prefer ownership transfer over complex borrowing when clarity improves

Avoid:

* Overly complex lifetime annotations
* Designing APIs that are hard to reason about

---

### Borrowing Over Cloning, But With Judgment

The agent SHOULD:

* Prefer borrowing to avoid unnecessary allocation

The agent MUST NOT:

* Obsessively eliminate cloning at the expense of clarity
* Introduce complex lifetimes solely to avoid a cheap clone

Clarity > micro-optimizations.

---

## Error Handling

### No Panics in Production Paths

The agent MUST NOT:

* Use `panic!` in normal execution paths
* Rely on `unwrap()` or `expect()` outside of clearly safe contexts

Acceptable uses:

* Tests
* Prototyping (explicitly marked)
* Truly unrecoverable states (with justification)

---

### Use Idiomatic Error Types

Errors SHOULD:

* Use `Result<T, E>`
* Prefer structured error types
* Preserve error context

The agent SHOULD:

* Avoid stringly-typed errors when structure is meaningful
* Propagate errors intentionally

---

## Concurrency and Async Safety

### Explicit Async Boundaries

Async code MUST:

* Clearly define async boundaries
* Avoid mixing blocking and async code

The agent MUST NOT:

* Block the async runtime
* Assume single-threaded execution unless enforced

---

### Thread Safety Awareness

When sharing data across threads:

* Types MUST be `Send` and `Sync` where required
* Synchronization primitives MUST be used intentionally

Avoid:

* Overusing `Arc<Mutex<...>>` as a default solution

---

## API Design Guidelines

### Prefer Small, Composable Functions

Functions SHOULD:

* Do one thing
* Be easy to test
* Have explicit inputs and outputs

Large monolithic functions are discouraged.

---

### Explicit Lifetimes at Boundaries Only

The agent SHOULD:

* Let the compiler infer lifetimes internally
* Make lifetimes explicit only at public or complex boundaries

---

## Cross-Language Boundaries (Tauri / FFI)

Rust code exposed to:

* JavaScript
* Python
* C#

MUST:

* Validate all external inputs
* Avoid leaking Rust-specific assumptions
* Fail safely and predictably

Never trust external callers.

---

## Performance Considerations

The agent SHOULD:

* Avoid unnecessary allocations
* Be conscious of hot paths

The agent MUST:

* Justify optimizations
* Prefer measurement over assumptions

---

## Scope and Authority

* This document applies to all Rust backend code.
* More specific backend rules may refine it.
* In case of conflict, security rules take precedence.

---

## Expected Outcome

Following these rules results in Rust code that is:

* Safe by default
* Easier to reason about
* Suitable for long-term maintenance
