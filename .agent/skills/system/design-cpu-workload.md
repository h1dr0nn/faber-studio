# Skill: Design CPU Workload (Single-Core vs Multi-Core)

## Purpose

This skill defines how the agent should **analyze, design, and choose**
between **single-threaded and multi-threaded execution models** for CPU workloads.

It exists to ensure CPU usage is:

* Intentional
* Measurable
* Appropriate for the target platform

This skill is about **designing the workload**, not implementing threading primitives.

---

## When to Use This Skill

Use this skill when:

* Designing a new system with non-trivial CPU cost
* Refactoring logic that feels “slow” or “heavy”
* Deciding whether to parallelize a task
* Evaluating performance on multi-core devices

Do NOT use this skill for:

* Pure IO-bound work
* UI or rendering logic
* Micro-optimizations

---

## Inputs

The agent SHOULD expect:

* Description of the workload
* Execution frequency (per-frame, per-event, batch)
* Target platform (desktop, mobile)
* Performance symptoms or goals

If frequency or platform is unclear, the agent MUST ask.

---

## Preconditions

Before designing the workload, the agent MUST:

1. Identify where the workload runs:

   * Main thread
   * Background worker
   * Job / executor
2. Determine execution frequency:

   * Once
   * Occasionally
   * Repeated / continuous
3. Estimate cost per execution (rough order of magnitude)

---

## Step 1: Classify the Workload

The agent MUST classify the workload as one of:

### CPU-Bound

* Heavy computation
* Minimal waiting on external resources
* Cost scales with input size

### IO-Bound

* Waiting on disk, network, or IPC
* CPU mostly idle during waits

### Mixed

* Alternating compute and IO phases

Misclassification leads to incorrect design.

---

## Step 2: Decide on Single-Core vs Multi-Core

### Prefer Single-Core When

The agent SHOULD default to **single-threaded execution** when:

* The workload is sequential
* Tasks depend heavily on each other
* Cost per execution is small or medium
* Execution frequency is high (e.g. per-frame)

Single-threaded code is:

* Easier to reason about
* Easier to debug
* Often faster due to cache locality

---

### Consider Multi-Core Only When

Multi-core execution MAY be considered only if:

* The workload is CPU-bound
* Tasks can be partitioned independently
* Synchronization cost is low
* Per-task cost is large enough to amortize overhead

The agent MUST be able to explain:

* Why single-core is insufficient
* What the parallel unit of work is

---

## Step 3: Partitioning Strategy

If multi-core is chosen, the agent MUST define:

* The unit of parallel work
* How work is split
* How results are combined

Avoid:

* Fine-grained parallelism
* Frequent synchronization
* Shared mutable state

Coarse-grained parallelism is preferred.

---

## Step 4: Frequency vs Parallelism Trade-off

The agent MUST consider:

* **High-frequency tasks**
  → Strong bias toward single-core

* **Low-frequency, batch tasks**
  → Better candidates for multi-core

Parallelism in hot paths is high risk.

---

## Step 5: Platform Sensitivity

### Desktop

* Multi-core may help for heavy batch work
* Still bounded by memory bandwidth and cache effects

### Mobile

* Multi-core is thermally constrained
* Sustained parallel load causes throttling
* Burst performance is preferred

The agent MUST bias toward:

* Short, bursty execution
* Early exit and incremental processing

---

## Step 6: Validate With Measurement

Before committing to multi-core design, the agent MUST:

* Measure single-core performance
* Establish a baseline
* Compare with parallel prototype if necessary

Parallelism without baseline comparison is prohibited.

---

## Common Design Mistakes (Avoid)

* Parallelizing logic before simplifying it
* Using threads to hide bad algorithms
* Splitting work too finely
* Assuming “modern CPUs will handle it”

---

## Output

The output of this skill SHOULD include:

* Workload classification
* Execution frequency
* Chosen execution model (single vs multi)
* Justification for the choice
* Identified risks

---

## Postconditions

After executing this skill:

* The workload model should be explicit
* Threading decisions should be justified
* Performance expectations should be realistic

---

## Constraints

The agent MUST:

* Follow `cpu-threading` rules
* Respect platform-specific limits
* Avoid speculative parallelism

---

## Expected Outcome

Following this skill results in CPU workload designs that:

* Match the real performance characteristics of the platform
* Avoid unnecessary complexity
* Scale predictably when parallelism is truly beneficial
