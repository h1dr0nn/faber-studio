# Skill: Optimize Mobile FPS

## Purpose

This skill defines how the agent should **identify, analyze, and optimize FPS issues**
for games targeting **mobile platforms (Android / iOS)**.

It applies to:

* Unity-based mobile games
* Tauri-based games using React + Canvas / WebGL
* Game tools or editors running on mobile devices

The goal is to achieve:

* Stable frame rate
* Predictable performance
* Acceptable thermal and battery behavior

---

## When to Use This Skill

Use this skill when:

* FPS drops are observed on mobile devices
* Preparing a game for mobile release
* Performance varies significantly across devices
* Optimizing long play sessions

Do NOT use this skill for:

* Desktop-only optimization
* Micro-optimizations without evidence

---

## Inputs

The agent SHOULD expect:

* Target FPS (30 / 60)
* Target devices or device class
* Observed performance symptoms
* Profiling data (if available)

If no target FPS is defined, the agent MUST ask.

---

## Preconditions

Before executing this skill, the agent MUST:

1. Confirm the target frame rate
2. Identify the dominant bottleneck:

   * CPU
   * GPU
   * Memory / GC
3. Confirm whether the issue is:

   * Consistent
   * Thermal (degrades over time)
   * Scene- or feature-specific

---

## Core Mobile Performance Principles

### 1. Mobile ≠ Desktop

The agent MUST assume:

* Lower CPU and GPU throughput
* Aggressive thermal throttling
* Limited memory bandwidth

Desktop-level assumptions are invalid.

---

### 2. Frame Budget Discipline

The agent MUST:

* Treat frame budget as fixed
* Ensure all per-frame work fits within budget

At 60 FPS:

* CPU + GPU + overhead ≤ ~16.6 ms

At 30 FPS:

* ≤ ~33.3 ms

---

## CPU Optimization Strategies

The agent SHOULD:

* Reduce per-frame logic
* Avoid unnecessary Update / loop execution
* Prefer event-driven or batched logic

Avoid:

* Per-frame allocations
* Heavy reflection or dynamic dispatch
* Unbounded async work

---

## GPU Optimization Strategies

The agent SHOULD:

* Reduce draw calls
* Reduce overdraw
* Simplify shaders and materials

For Tauri-based games:

* Avoid excessive Canvas redraws
* Avoid full-screen redraws when partial updates suffice

---

## Memory and GC Control

The agent MUST:

* Avoid per-frame object allocation
* Reuse objects and buffers
* Minimize temporary allocations

GC spikes are a common source of mobile stutter.

---

## Animation Discipline

Animations MUST:

* Be GPU-composited where possible
* Avoid driving gameplay logic
* Avoid triggering excessive re-renders

CPU-bound animations are high risk on mobile.

---

## Thermal and Long-Session Considerations

The agent MUST:

* Consider performance degradation over time
* Avoid designs that push devices at maximum load continuously

Sustainable performance is preferred over peak performance.

---

## Measurement and Validation

The agent SHOULD:

* Profile on real devices
* Observe performance over extended sessions
* Validate improvements with before/after measurements

Simulator-only testing is insufficient.

---

## Postconditions

After executing this skill:

* FPS should meet target on representative devices
* Performance should be stable over time
* Thermal throttling impact should be minimized

---

## Constraints

The agent MUST:

* Follow game performance and architecture rules
* Avoid speculative optimizations
* Avoid sacrificing correctness for FPS

---

## Anti-Patterns to Avoid

* Optimizing without profiling
* Chasing maximum FPS at all costs
* Ignoring thermal behavior
* Treating GC spikes as “acceptable”

---

## Expected Outcome

Following this skill results in mobile games that:

* Run smoothly on target devices
* Maintain stable FPS over time
* Deliver a consistent player experience
