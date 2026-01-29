# Memory and Allocation Rules

## Purpose

This document defines **non-negotiable rules** for **memory usage, allocation,
and object lifetime** across all systems.

It exists to prevent:

* GC spikes
* Memory bloat
* Fragmentation
* Performance collapse in long-running applications

These rules apply to:

* Unity (C#)
* Tauri frontend (TypeScript / JS)
* Rust backend
* Shared libraries
* Tooling and automation

---

## Fundamental Principles

### 1. Allocation Is a Performance Cost

The agent MUST assume:

* Memory allocation is not free
* Deallocation is not free
* GC and allocators introduce unpredictable latency

Any allocation in hot paths MUST be justified.

---

### 2. Lifetime Must Be Designed Explicitly

Every allocated object MUST have a **clear lifetime**:

* Frame-local
* Short-lived
* Long-lived
* Persistent

Implicit or accidental lifetimes are prohibited.

---

## Allocation Rules by Context

### Hot Paths (Per-Frame / Per-Tick)

The agent MUST NOT:

* Allocate objects
* Allocate closures
* Allocate temporary collections
* Box values
* Create strings dynamically

Hot paths MUST be allocation-free.

Violations here are critical performance bugs.

---

### Medium-Frequency Paths

Allocations MAY be allowed only if:

* They are amortized
* They do not scale with frame count
* They do not grow unbounded

Caching or reuse SHOULD be preferred.

---

### Cold Paths

Allocations in cold paths are acceptable,
but lifetime and ownership MUST still be explicit.

---

## Language-Specific Constraints

### Unity / C#

The agent MUST:

* Avoid allocations in `Update`, `LateUpdate`, `FixedUpdate`
* Avoid LINQ in hot paths
* Avoid implicit allocations (boxing, closures, foreach on alloc-heavy types)

Object pooling SHOULD be preferred for:

* Gameplay entities
* Reusable data containers
* Temporary buffers

---

### TypeScript / JavaScript (Tauri)

The agent MUST:

* Avoid per-frame object creation
* Avoid creating new arrays or objects during render loops
* Avoid excessive immutable cloning in hot paths

Memory churn in JS leads to GC pauses and UI stutter.

---

### Rust

The agent MUST:

* Avoid unnecessary cloning
* Avoid heap allocation when stack allocation suffices
* Avoid allocating in tight loops

Allocation strategy MUST be intentional
(e.g. reuse buffers, preallocate collections).

---

## Reuse and Pooling Discipline

Reuse SHOULD be preferred when:

* Object shape is stable
* Lifetime is predictable
* Allocation cost is non-trivial

Pooling MUST:

* Have clear ownership
* Have bounded size
* Avoid unbounded growth

Pooling without bounds is prohibited.

---

## Memory Growth and Stability

The agent MUST:

* Avoid unbounded memory growth
* Avoid retaining references longer than necessary
* Release resources deterministically when possible

Long-running tools and games MUST maintain stable memory usage.

---

## Mobile-Specific Constraints

On mobile platforms, the agent MUST assume:

* Limited memory
* Aggressive memory pressure
* OS-level process termination under stress

Memory spikes are unacceptable.

---

## Measurement Requirement

If memory behavior is non-trivial, the agent MUST:

* Measure allocation frequency
* Observe GC behavior
* Validate long-session stability

Memory optimization without observation is prohibited.

---

## Anti-Patterns (Explicitly Forbidden)

* “It’s just a small allocation”
* Allocating in render or update loops
* Creating garbage to “let GC handle it”
* Pooling everything blindly
* Retaining data “just in case”

---

## Scope and Authority

* These rules apply globally.
* They override convenience-based coding.
* Stricter platform rules may apply.

---

## Expected Outcome

Following these rules results in systems that:

* Maintain stable memory usage
* Avoid GC-induced stutter
* Scale safely in long-running scenarios
