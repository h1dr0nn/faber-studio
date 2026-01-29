# Skill: Optimize React Performance (GPU-First)

## Purpose

This skill defines how the agent should **analyze, optimize, and design React UI**
with a **performance-first and GPU-first mindset**, especially for:

* Tauri desktop tools
* Tauri-based games (UI / HUD / menus)
* Long-running applications

The goal is to ensure UI performance remains stable under real-world usage,
including animation-heavy scenarios.

---

## When to Use This Skill

Use this skill when:

* UI performance degrades or is at risk
* Introducing animations or transitions
* Rendering large or frequently updating UI
* Targeting mobile platforms via Tauri

Do NOT use this skill for:

* Core gameplay logic
* Non-UI system performance

---

## Inputs

The agent SHOULD expect:

* Description of UI behavior
* Animation requirements (if any)
* Performance symptoms or constraints
* Target platform (desktop, Android, iOS)

If animation intent is unclear, the agent MUST ask before proceeding.

---

## Preconditions

Before executing this skill, the agent MUST:

1. Identify hot paths in the UI
2. Determine render frequency expectations
3. Classify updates as:

   * Event-driven
   * Continuous
4. Confirm whether animations are required or optional

---

## Core Optimization Principles

### 1. Rendering Is Not Free

The agent MUST:

* Treat React re-renders as a measurable cost
* Avoid re-rendering large trees unnecessarily

React reconciliation MUST NOT be treated as a game loop.

---

### 2. GPU-First Animation Rule (Mandatory)

All animations MUST prefer **GPU-composited properties**.

Allowed (preferred):

* `transform`
* `opacity`
* `filter` (with caution)

Disallowed by default:

* Layout-affecting properties (`top`, `left`, `width`, `height`)
* Per-frame JS-driven style updates
* State-driven animation loops

CPU-bound animations are prohibited unless explicitly justified.

---

### 3. Animation Ownership

Animations SHOULD:

* Be owned by the presentation layer
* Not modify application or gameplay state

State changes MUST NOT:

* Be driven by animation frames
* Be coupled to visual timing

---

## Implementation Guidelines

### CSS and Compositor Animations

The agent SHOULD:

* Prefer CSS transitions or keyframes
* Use the Web Animations API when appropriate
* Ensure animations are offloaded to the compositor thread

`will-change` MAY be used:

* Sparingly
* Only when animation is imminent

---

### React State Discipline

The agent MUST:

* Avoid updating React state every frame
* Avoid animation loops tied to `setState`

If per-frame updates are required:

* They MUST bypass React rendering
* They MUST be isolated from the component tree

---

### Animation Libraries

Animation libraries MAY be used only if:

* They respect GPU-composited properties
* They do not force per-frame React re-renders

Libraries that rely on continuous React state updates
are strongly discouraged.

---

## Large UI and Lists

The agent SHOULD:

* Virtualize large lists
* Avoid animating large numbers of DOM nodes simultaneously

Animating thousands of elements is unacceptable.

---

## Measurement and Validation

The agent SHOULD:

* Use profiling tools to confirm improvements
* Validate animation smoothness on target devices
* Observe CPU and memory usage during animation

Optimizations MUST be validated, not assumed.

---

## Failure and Fallback Strategy

If performance goals cannot be met:

* Reduce animation scope
* Reduce animation frequency
* Prefer static UI over degraded animation

Visual fidelity MUST NOT compromise usability.

---

## Postconditions

After applying this skill:

* UI interactions should feel smooth
* CPU usage should remain stable
* Animations should not degrade long-session performance

---

## Constraints

The agent MUST:

* Follow frontend and game performance rules
* Respect Tauri and mobile constraints
* Avoid speculative optimization without evidence

---

## Anti-Patterns to Avoid

* State-driven animations
* Layout-thrashing animations
* Overuse of animation libraries
* Animations that drive logic

---

## Expected Outcome

Following this skill results in UIs that:

* Feel responsive and polished
* Scale across platforms
* Avoid common performance traps in React-based environments
