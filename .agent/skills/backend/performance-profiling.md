# Skill: Backend Performance Profiling

## Purpose

This skill defines how the agent should **analyze, measure, and reason about performance**
in backend, native, and systems code.

It applies to:

* Rust backends (Tauri, native services)
* Python automation and tooling
* Performance-sensitive subsystems used by tools or games

The goal is to ensure performance work is:

* Evidence-driven
* Targeted
* Sustainable

---

## When to Use This Skill

Use this skill when:

* Performance problems are suspected or observed
* Optimizing critical paths
* Validating performance assumptions
* Preparing code for production release

Do NOT use this skill for:

* Premature optimization
* Cosmetic micro-optimizations without evidence

---

## Inputs

The agent SHOULD expect:

* Description of performance symptoms
* Target platform (desktop, Android, iOS)
* Runtime context (long-running, bursty, background)
* Relevant code paths or modules

If no symptoms are described, the agent MUST question the need for optimization.

---

## Preconditions

Before executing this skill, the agent MUST:

1. Clarify what “slow” means (latency, throughput, frame time)
2. Identify the scope of the issue:

   * CPU
   * Memory
   * I/O
3. Determine whether the issue is:

   * Consistent
   * Intermittent
   * Load-dependent

---

## Core Profiling Principles

### 1. Measure Before Optimizing

The agent MUST:

* Collect data before making changes
* Avoid assumptions about bottlenecks

Optimization without measurement is prohibited.

---

### 2. Focus on Hot Paths

The agent SHOULD:

* Identify the smallest set of code responsible for most cost
* Ignore cold paths until hot paths are addressed

Optimizing cold code yields negligible benefit.

---

## Profiling Techniques

### CPU Profiling

The agent SHOULD:

* Measure function-level execution time
* Identify blocking or expensive operations
* Look for unnecessary repetition

For Rust:

* Consider sampling profilers
* Use instrumentation where appropriate

---

### Memory Profiling

The agent MUST:

* Track allocation frequency
* Watch for unbounded growth
* Identify leaks or retention issues

GC or allocator pressure is often a hidden bottleneck.

---

### I/O and Concurrency

The agent SHOULD:

* Measure I/O latency
* Identify blocking calls in async contexts
* Check for excessive synchronization

Concurrency bugs often appear as performance issues.

---

## Hypothesis-Driven Optimization

Optimization SHOULD follow this loop:

1. Observe a symptom
2. Form a hypothesis
3. Measure to validate
4. Apply a targeted change
5. Re-measure to confirm improvement

Skipping steps is unacceptable.

---

## Validation and Regression Safety

After optimization:

* Performance improvements MUST be verified
* Functional correctness MUST be preserved

Optimizations that regress stability or correctness are invalid.

---

## Reporting and Communication

The agent SHOULD:

* Explain what was measured
* Explain why a change helped
* Quantify improvements where possible

Vague claims like “this should be faster” are unacceptable.

---

## Postconditions

After executing this skill:

* Bottlenecks should be clearly identified
* Changes should be justified by data
* Performance behavior should be predictable

---

## Constraints

The agent MUST:

* Respect safety and security rules
* Avoid speculative or aesthetic optimizations
* Avoid introducing complexity without benefit

---

## Anti-Patterns to Avoid

* Optimizing without data
* Chasing micro-optimizations prematurely
* Ignoring platform-specific behavior
* Sacrificing clarity for marginal gains

---

## Expected Outcome

Following this skill results in backend systems that:

* Perform well under real workloads
* Are optimized where it matters
* Remain maintainable and correct
