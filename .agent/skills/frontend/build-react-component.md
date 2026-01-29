# Skill: Build React Component

## Purpose

This skill defines how the agent should **design and implement a production-ready React component**
for desktop tools or Tauri-based applications.

It is intended to produce components that are:

* Correct by default
* Easy to maintain
* Performance-conscious
* Consistent with the established frontend rules

This skill applies to **UI components**, not game core logic.

---

## When to Use This Skill

Use this skill when:

* Creating a new React component
* Refactoring an existing component
* Translating UI requirements into implementation

Do NOT use this skill for:

* Core game logic
* Game loop or simulation code
* Non-UI systems

---

## Inputs

The agent SHOULD expect some or all of the following inputs:

* Component purpose and responsibility
* Required props and data sources
* Interaction requirements (events, callbacks)
* Performance considerations
* Tauri integration needs (if any)

If critical inputs are missing, the agent MUST ask for clarification.

---

## Preconditions

Before executing this skill, the agent MUST:

1. Understand the component’s single responsibility
2. Identify whether the component is:

   * Presentational
   * Container / logic-oriented
3. Identify state ownership:

   * Local state
   * External state
4. Confirm that the component is **not part of core gameplay logic**

---

## Execution Guidelines

### Component Structure

The agent MUST:

* Use functional components
* Use TypeScript with explicit typing
* Keep the component focused on a single concern

Avoid:

* Multi-responsibility components
* Implicit dependencies

---

### Props and Typing

Props MUST:

* Be explicitly typed
* Reflect domain intent
* Avoid overly generic shapes

Prefer:

* Narrow, intention-revealing props
* Composition over prop explosion

---

### State Management

The agent SHOULD:

* Prefer local state where possible
* Avoid introducing global state without justification

State MUST:

* Be minimal
* Be explicit
* Avoid duplication of derived data

---

### Side Effects

Side effects MUST:

* Be isolated in effects or callbacks
* Have explicit dependencies
* Be easy to reason about

Avoid:

* Hidden side effects in render paths
* Effects with unclear lifecycles

---

### Performance Discipline

The agent MUST:

* Avoid unnecessary re-renders
* Avoid expensive work during render
* Be conscious of component update frequency

Memoization SHOULD:

* Be applied only when justified
* Be explained if non-obvious

---

### Error Handling

The component SHOULD:

* Handle invalid or missing data gracefully
* Fail visibly but safely

Silent UI failures are unacceptable.

---

## Output

The output of this skill MUST include:

* A complete React component implementation
* Clear TypeScript typings
* Minimal but sufficient comments explaining non-obvious decisions

Optional outputs (when relevant):

* Supporting hooks
* Styles or layout scaffolding
* Basic usage example

---

## Postconditions

After execution:

* The component should compile without errors
* The responsibility should be clearly defined
* The component should integrate cleanly into a larger application

---

## Constraints

The agent MUST:

* Follow all frontend and global rules
* Respect Tauri constraints if applicable
* Avoid introducing unnecessary dependencies

---

## Anti-Patterns to Avoid

* God components
* Implicit global state access
* Excessive prop drilling without structure
* Performance optimizations without justification

---

## Expected Outcome

Following this skill produces React components that:

* Are easy to reason about
* Scale as the UI grows
* Fit cleanly into production desktop or Tauri applications
