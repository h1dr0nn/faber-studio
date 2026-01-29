# React Architecture and Patterns

## Purpose

This document defines **approved architectural patterns and best practices**
for React applications assisted by the agent.

It applies to:

* React + TypeScript projects
* Frontend layers of Tauri applications
* UI tooling used alongside Unity or backend systems

The goal is to produce React code that is:

* Predictable
* Performant
* Maintainable at scale

---

## Core Architectural Principles

### 1. Functional Components as the Default

All React components MUST be written as functional components.

Class components MUST NOT be used unless:

* Interfacing with legacy code
* Explicitly required by a third-party library

Hooks are the standard mechanism for:

* State
* Side effects
* Lifecycle logic

---

### 2. Clear Separation of Responsibilities

Components SHOULD be categorized by responsibility:

* **Presentational components**

  * Render UI
  * Receive data via props
  * Contain minimal logic

* **Container / logic components**

  * Own state
  * Handle side effects
  * Coordinate data flow

Avoid mixing complex business logic directly into UI-rendering code.

---

### 3. Controlled Data Flow

Data flow MUST be:

* Explicit
* Unidirectional
* Traceable

Preferred pattern:

* State is lifted to the lowest common owner
* Props flow downward
* Events flow upward via callbacks

Avoid:

* Implicit shared state
* Hidden dependencies between components

---

## State Management Rules

### Local State First

Local component state SHOULD be the default.

Global state SHOULD be introduced only when:

* Multiple distant components depend on it
* State must survive navigation or reload
* Synchronization is required

---

### Avoid Over-Engineering State

The agent MUST NOT:

* Introduce global state libraries unnecessarily
* Use complex state machines for simple UI flows

State management solutions must be justified by real complexity.

---

## Hooks Usage Rules

### Custom Hooks

Reusable logic SHOULD be extracted into custom hooks.

Custom hooks MUST:

* Start with `use`
* Have a single, clear responsibility
* Avoid hidden side effects

Example:

```ts
function useWindowSize() {}
```

---

### useEffect Discipline

Effects MUST:

* Be minimal
* Declare all dependencies explicitly
* Avoid acting as general-purpose lifecycle hooks

Avoid:

* Large `useEffect` blocks with mixed concerns
* Effects that replicate derived state

---

## Performance Considerations

### Rendering Discipline

The agent MUST:

* Avoid unnecessary re-renders
* Be conscious of render frequency

Techniques include:

* Memoization (`useMemo`, `useCallback`) when justified
* Splitting large components
* Avoiding inline object/function creation in hot paths

---

### Avoid Premature Optimization

Performance optimizations MUST:

* Be motivated by real cost
* Be explained when non-obvious

Readability takes precedence unless performance constraints demand otherwise.

---

## File and Component Structure

### File Organization

Files SHOULD be organized by feature rather than by type.

Preferred:

```text
features/user-profile/
├─ UserProfile.tsx
├─ useUserProfile.ts
└─ userProfile.types.ts
```

Avoid:

```text
components/
hooks/
utils/
```

when it leads to fragmented feature logic.

---

## Error Handling

UI-level errors SHOULD:

* Be surfaced clearly
* Fail gracefully

Critical failures SHOULD:

* Be logged
* Provide fallback UI where appropriate

Silent UI failures are unacceptable.

---

## Scope and Authority

* This document applies to all React code.
* More specific frontend rules may refine it.
* In case of conflict, the more specific rule takes precedence.

---

## Expected Outcome

Following these patterns ensures React code that:

* Scales with feature complexity
* Is easier to debug
* Integrates cleanly with Tauri and backend systems
