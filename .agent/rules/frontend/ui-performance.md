# Frontend UI Performance Rules

## Purpose

This document defines **performance-related rules and expectations**
for frontend UI code produced or assisted by the agent.

It applies to:

* React + TypeScript applications
* Tauri frontend layers
* Desktop tooling UIs with long-running sessions

The goal is to maintain:

* Smooth interactions
* Predictable responsiveness
* Stable performance over extended usage

---

## Performance Mindset

### UI Responsiveness Is a Feature

UI responsiveness MUST be treated as a core feature.

The agent MUST:

* Avoid blocking the main thread
* Avoid unnecessary rendering work
* Prioritize perceived performance, not just raw metrics

---

## Rendering Rules

### Minimize Re-Renders

The agent MUST:

* Be conscious of component render frequency
* Avoid global state updates that trigger large subtree re-renders

Components SHOULD:

* Be split by responsibility
* Receive only the props they need

---

### Memoization Discipline

Memoization SHOULD be applied:

* Only when there is a measurable or likely performance benefit
* With clear understanding of dependency correctness

The agent MUST NOT:

* Blanket-apply `useMemo` or `useCallback`
* Sacrifice readability without justification

---

## Event Handling

Event handlers MUST:

* Be lightweight
* Avoid expensive synchronous work

Heavy processing SHOULD:

* Be deferred
* Be delegated to backend or workers
* Trigger loading states when appropriate

---

## Lists and Large Data Sets

When rendering large lists, the agent SHOULD:

* Use virtualization
* Avoid rendering off-screen elements
* Minimize DOM complexity per item

Rendering thousands of elements eagerly is unacceptable.

---

## Animation and Visual Effects

Animations SHOULD:

* Be subtle
* Avoid layout thrashing
* Prefer GPU-accelerated properties

Avoid:

* Animations that block interaction
* Complex effects that degrade responsiveness

---

## Long-Running Sessions

Desktop tools often run for hours.

The agent MUST:

* Avoid memory leaks
* Clean up subscriptions and effects
* Prevent unbounded state growth

Resource usage SHOULD remain stable over time.

---

## Measurement and Feedback

The agent SHOULD:

* Encourage profiling before optimizing
* Use built-in tools where appropriate
* Make performance assumptions explicit

---

## Anti-Patterns to Avoid

* Performing heavy computation during render
* Large synchronous state updates
* Excessive DOM depth
* Treating desktop UI as disposable

---

## Scope and Authority

* This document applies to all frontend UI code.
* It complements, not replaces, general coding philosophy.
* In case of conflict, performance-critical rules take precedence.

---

## Expected Outcome

Following these rules ensures frontend UIs that:

* Feel fast and responsive
* Scale with application complexity
* Remain stable in long-running desktop environments
