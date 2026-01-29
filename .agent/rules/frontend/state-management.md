# Frontend State Management Rules

## Purpose

This document defines **how application state should be modeled, stored, and updated**
in frontend applications assisted by the agent.

It applies to:

* React + TypeScript projects
* Frontend layers of Tauri applications
* UI tooling with non-trivial interaction logic

The goal is to avoid:

* Over-engineered state solutions
* Hidden state coupling
* Fragile or untraceable state changes

---

## State Classification

All state MUST be classified before implementation.

### 1. Local UI State

Characteristics:

* Scoped to a single component or small component tree
* Short-lived
* Not shared broadly

Examples:

* Input values
* Toggle flags
* Local UI modes

This SHOULD be implemented using:

* `useState`
* `useReducer` (when logic is non-trivial)

---

### 2. Shared Application State

Characteristics:

* Used by multiple distant components
* Represents application-level concepts

Examples:

* Auth state
* User preferences
* Active workspace or session

This SHOULD be introduced cautiously and only when necessary.

---

### 3. Server or External State

Characteristics:

* Originates outside the frontend
* Can be re-fetched or synchronized

Examples:

* Backend data
* Tauri IPC responses
* File system data

This SHOULD be treated as:

* Cacheable
* Potentially stale
* Explicitly synchronized

---

## Rules for Choosing a State Solution

### Default Rule

The agent MUST:

* Start with local state
* Escalate to shared state only when justified

Introducing global state without justification is a violation of this rule.

---

### Global State Constraints

When global state is required, it MUST:

* Have a clear ownership model
* Expose explicit update mechanisms
* Avoid implicit mutation

Global state SHOULD:

* Be documented
* Be minimal in surface area

---

## Reducers and Immutability

When state logic becomes complex:

* `useReducer` SHOULD be preferred over multiple `useState` calls.

Reducers MUST:

* Be pure
* Avoid side effects
* Handle default cases explicitly

---

## Derived State

Derived state MUST NOT be stored explicitly.

Instead:

* Compute it from source state
* Memoize if performance requires

Storing derived state increases risk of inconsistency.

---

## Synchronization and Side Effects

Side effects MUST:

* Be isolated from state definitions
* Live in effects or explicit action handlers

State updates MUST NOT:

* Trigger cascading side effects implicitly
* Depend on hidden external conditions

---

## Debuggability Requirements

State changes SHOULD be:

* Traceable
* Predictable

The agent SHOULD:

* Favor explicit actions
* Avoid magic setters or implicit subscriptions

---

## Performance Considerations

State design MUST:

* Minimize unnecessary re-renders
* Avoid large, frequently changing global state objects

The agent SHOULD:

* Split state by responsibility
* Localize high-frequency updates

---

## Anti-Patterns to Avoid

* Introducing Redux-like complexity for simple apps
* Using context as a global dumping ground
* Mutating state objects directly
* Storing transient UI state globally

---

## Scope and Authority

* This document applies to all frontend state management.
* Framework-specific rules may refine it.
* In case of conflict, more specific rules take precedence.

---

## Expected Outcome

Following these rules results in state management that is:

* Simple by default
* Scalable when needed
* Easier to reason about and debug
