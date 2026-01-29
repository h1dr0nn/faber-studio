# Profile: Shared Library (Cross-Platform)

## Scope

This profile defines **strict constraints** for **shared libraries**
used across multiple platforms, runtimes, or applications.

It applies to shared code used by:

* Desktop and mobile applications
* Unity and Tauri projects
* Rust, TypeScript, and C# consumers

Shared libraries MUST remain **portable, predictable, and platform-agnostic**.

---

## Fundamental Principle

> **Shared libraries must not assume platform, lifecycle, or execution context.**

Any assumption leaks platform-specific behavior into shared code
and causes hidden bugs downstream.

---

## Platform Neutrality Requirement

Shared libraries MUST NOT assume:

* Desktop or mobile environment
* Availability of UI threads
* Stable process lifetime
* Background execution capability
* File system access
* Network availability

All such assumptions MUST be injected or handled externally.

---

## Responsibility Boundaries

Shared libraries MAY:

* Implement pure logic
* Perform deterministic computation
* Define data models and transformations
* Expose abstract interfaces

Shared libraries MUST NOT:

* Access OS APIs directly
* Spawn processes
* Perform file system IO
* Call native SDKs
* Block threads
* Depend on UI or engine lifecycle

---

## Ownership and Data Discipline

Shared libraries MUST:

* Treat all input as caller-owned
* Avoid retaining references beyond call scope
* Return results without hidden side effects

Shared libraries MUST NOT:

* Own long-lived state implicitly
* Mutate shared data structures unexpectedly

Ownership ambiguity is prohibited.

---

## Threading and Concurrency Assumptions

Shared libraries MUST:

* Be safe under different threading models
* Avoid thread affinity assumptions
* Avoid internal thread creation unless explicitly documented

Threading decisions belong to the application, not the library.

---

## Time and Lifecycle Neutrality

Shared libraries MUST:

* Avoid direct use of wall-clock time
* Accept time or scheduling context explicitly if needed
* Avoid timers or background loops

Lifecycle management belongs to the host application.

---

## Error Handling Discipline

Shared libraries MUST:

* Surface errors explicitly
* Avoid fatal termination
* Avoid logging directly to user-visible outputs

Libraries SHOULD return structured error information
and let the host decide how to handle it.

---

## Performance Expectations

Shared libraries SHOULD:

* Avoid hidden allocations in hot paths
* Avoid unbounded memory growth
* Remain predictable under repeated use

Performance characteristics MUST be stable and documented.

---

## Testing and Validation Bias

Shared libraries SHOULD:

* Be testable in isolation
* Avoid reliance on platform mocks
* Be validated under multiple usage patterns

Library correctness MUST not depend on a specific app.

---

## Anti-Patterns (Explicitly Forbidden)

* “Just one platform-specific hack”
* Reading environment variables implicitly
* Touching the file system for convenience
* Creating background threads silently
* Acting as a service locator or global state

---

## Intended Use

This profile is intended to:

* Protect shared code from platform leakage
* Ensure long-term reuse viability
* Constrain AI-generated shared logic
* Prevent subtle cross-platform bugs

It is a **strict contract**, not a suggestion.

---

## Key Takeaway

> **Shared libraries exist to reduce duplication, not to hide platform complexity.**

They must remain clean, explicit, and context-free to be safe.
