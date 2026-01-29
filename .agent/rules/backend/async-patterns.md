# Async and Concurrency Patterns

## Purpose

This document defines **approved async and concurrency patterns**
for backend and systems code produced or assisted by the agent.

It applies to:

* Rust async code (Tauri backend, services)
* Python async or threaded utilities
* Cross-language orchestration where concurrency is involved

The goal is to avoid:

* Deadlocks
* Starvation
* Blocking async runtimes
* Unpredictable concurrency behavior

---

## General Async Principles

### Async Is a Tool, Not a Default

The agent MUST NOT:

* Introduce async without a concrete need
* Assume concurrency improves performance automatically

Async SHOULD be used when:

* I/O-bound operations dominate
* Parallelism improves responsiveness

---

## Rust Async Rules

### No Blocking in Async Contexts

Async functions MUST NOT:

* Call blocking I/O
* Use `std::thread::sleep`
* Perform heavy CPU-bound work

Blocking work MUST:

* Be moved to a dedicated thread pool
* Be wrapped using appropriate runtime utilities

---

### Explicit Runtime Assumptions

The agent MUST:

* Be explicit about the async runtime (e.g. Tokio)
* Avoid relying on implicit runtime behavior

Async boundaries SHOULD be visible and intentional.

---

### Task Spawning Discipline

Spawning tasks SHOULD:

* Be controlled
* Be limited in scope

The agent MUST NOT:

* Spawn unbounded background tasks
* Fire-and-forget critical work without supervision

All spawned tasks SHOULD:

* Have clear ownership
* Be cancellable where appropriate

---

## Synchronization Rules

### Avoid Shared Mutable State

The agent SHOULD:

* Prefer message passing or ownership transfer
* Minimize shared mutable state across threads or tasks

When shared state is unavoidable:

* Synchronization MUST be explicit
* Lock scope MUST be minimal

---

### Lock Usage Discipline

Locks MUST:

* Be held for the shortest possible duration
* Avoid async `.await` while holding a lock

Nested locks are strongly discouraged.

---

## Python Concurrency Rules

### Explicit Model Selection

The agent MUST:

* Be explicit about concurrency model:

  * Async (`asyncio`)
  * Threads
  * Processes

Mixing models without clear boundaries is discouraged.

---

### GIL Awareness

The agent MUST:

* Acknowledge the Global Interpreter Lock (GIL)
* Avoid CPU-bound work in threads when it does not scale

CPU-heavy work SHOULD:

* Use multiprocessing
* Or be delegated to native extensions

---

## Error Handling in Concurrent Code

Errors in async or concurrent contexts MUST:

* Be propagated or logged explicitly
* Not be silently dropped

The agent MUST NOT:

* Ignore task failures
* Assume success without verification

---

## Cancellation and Shutdown

Async systems MUST:

* Support graceful shutdown
* Respect cancellation signals

The agent SHOULD:

* Design tasks to be interruptible
* Clean up resources on cancellation

---

## Performance Considerations

Concurrency SHOULD:

* Improve throughput or responsiveness
* Be measured, not assumed

Over-parallelization is an anti-pattern.

---

## Anti-Patterns to Avoid

* Blocking async runtimes
* Unbounded task spawning
* Long-lived locks
* Hidden concurrency

---

## Scope and Authority

* This document applies to all async and concurrent backend code.
* More specific rules may refine it.
* In case of conflict, safety rules take precedence.

---

## Expected Outcome

Following these patterns results in backend systems that are:

* Predictable
* Scalable
* Easier to debug and maintain
