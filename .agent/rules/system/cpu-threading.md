# CPU Threading and Workload Rules

## Purpose

This document defines **non-negotiable rules** for designing and executing
CPU workloads across **single-core and multi-core environments**.

It exists to prevent:

* Misuse of threads
* Performance collapse due to oversubscription
* Incorrect assumptions about async, parallelism, and scalability

These rules apply to:

* Rust backends
* Unity games
* Tauri applications
* Tooling and automation

---

## Fundamental Principles

### 1. Threads Are a Limited Resource

The agent MUST assume:

* CPU cores are finite
* Context switching is expensive
* More threads ≠ more performance

Creating threads without a workload model is prohibited.

---

### 2. Async ≠ Parallelism

The agent MUST NOT assume:

* Async code runs in parallel
* `await` implies multi-core usage

Async is a **control-flow model**, not a CPU scaling strategy.

Parallelism must be **explicitly designed**.

---

### 3. Workload Classification Is Mandatory

Before introducing concurrency, the agent MUST classify the workload as:

* **CPU-bound**
* **IO-bound**
* **Mixed**

Failure to classify workload type is a rule violation.

---

## Single-Core vs Multi-Core Rules

### Single-Core Preference (Mandatory)

The agent MUST prefer **single-threaded execution** when:

* Workload is sequential by nature
* Data dependencies are strong
* Per-task cost is low
* Overhead of coordination exceeds benefit

Parallelism without measurable benefit is prohibited.

---

### Multi-Core Usage (Conditional)

Multi-core execution MAY be used only when:

* Workload is CPU-bound
* Tasks are independent or weakly coupled
* Cost per task is significant enough to amortize overhead
* Result ordering can be controlled or is irrelevant

The agent MUST justify multi-core usage explicitly.

---

## Thread Creation Discipline

### Thread Explosion Is Prohibited

The agent MUST NOT:

* Create unbounded threads
* Create threads per task
* Create threads per request

Thread pools or executors MUST be used when concurrency is required.

---

### Thread Lifetime Must Be Controlled

Threads MUST:

* Have clear ownership
* Have clear lifetime
* Be reusable when possible

Detached or unmanaged threads are prohibited.

---

## CPU-Bound Work Rules

For CPU-bound workloads, the agent MUST:

* Avoid async executors intended for IO
* Avoid blocking shared runtime threads
* Avoid running heavy computation on:

  * UI threads
  * Game main threads
  * IPC threads

CPU-bound work MUST be isolated intentionally.

---

## IO-Bound Work Rules

For IO-bound workloads, the agent MUST:

* Prefer async or event-driven models
* Avoid blocking threads unnecessarily
* Avoid creating parallelism where latency hiding is sufficient

Parallelism is not a substitute for proper async IO.

---

## Main Thread Protection Rule

The following threads are **strictly protected**:

* Unity main thread
* UI / render threads
* Tauri WebView thread
* IPC dispatch threads

The agent MUST NOT:

* Execute CPU-heavy work on protected threads
* Block protected threads
* Schedule long-running tasks on them

Violations here are critical performance bugs.

---

## Scaling Assumptions

The agent MUST NOT assume:

* Linear scaling with core count
* Identical performance across devices
* Desktop-class CPU availability on mobile

Mobile CPUs MUST be treated as:

* Thermal-constrained
* Burst-oriented
* Highly sensitive to sustained load

---

## Measurement Requirement

Any introduction of multi-threading MUST be:

* Measured
* Justified
* Revalidated after changes

Threading without profiling is prohibited.

---

## Anti-Patterns (Explicitly Forbidden)

* “Just use threads to make it faster”
* Running CPU-heavy logic in async callbacks
* Parallelizing trivial workloads
* Treating thread count as a tuning knob
* Assuming more cores always help

---

## Scope and Authority

* These rules apply globally.
* They override convenience-based decisions.
* Stricter platform-specific rules may apply on top.

---

## Expected Outcome

Following these rules results in systems that:

* Use CPU resources intentionally
* Avoid thread-related performance collapse
* Scale predictably across platforms and devices
