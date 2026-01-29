# Skill: Debug UI State and Rendering

## Purpose

This skill defines how the agent should **analyze, debug, and reason about UI state,
render behavior, and performance issues** in React-based applications and Tauri-based games.

It exists to ensure UI debugging is:

* Systematic
* Evidence-driven
* Focused on root causes, not symptoms

---

## When to Use This Skill

Use this skill when:

* UI behaves inconsistently or unexpectedly
* State updates do not reflect correctly in the UI
* Performance degrades during interaction or animation
* Debugging render frequency or component updates

Do NOT use this skill for:

* Backend or native debugging
* Core gameplay simulation debugging

---

## Inputs

The agent SHOULD expect:

* Description of the UI issue
* Observed symptoms (flicker, lag, stale data)
* Relevant component or state snippets
* Platform context (desktop, Android, iOS)

If reproduction steps are unclear, the agent MUST request them.

---

## Preconditions

Before executing this skill, the agent MUST:

1. Identify the scope of the issue:

   * Single component
   * Component subtree
   * Global UI behavior
2. Determine whether the issue is:

   * State-related
   * Render-related
   * Effect-related
3. Confirm whether animations or IPC are involved

---

## Debugging Principles

### 1. State Is the First Suspect

The agent MUST:

* Inspect state ownership
* Verify that state changes are intentional and minimal
* Check for duplicated or derived state stored unnecessarily

Unexpected UI behavior often originates from state misuse.

---

### 2. Rendering Must Be Observable

The agent SHOULD:

* Reason about when and why components render
* Identify unnecessary renders
* Correlate renders with state or prop changes

Render frequency MUST NOT be guessed.

---

### 3. Effects and Lifecycles

The agent MUST:

* Inspect `useEffect` dependencies carefully
* Ensure effects are not firing unexpectedly
* Check for missing cleanup logic

Effects with unclear purpose are high-risk areas.

---

## Performance Debugging

### Identifying Hot Paths

The agent SHOULD:

* Identify components that render frequently
* Inspect props that change often
* Look for expensive logic during render

Avoid optimizing without knowing the hot path.

---

### Animation and Frame Issues

When animations are involved, the agent MUST:

* Verify whether animations are GPU-composited
* Check for state-driven animation loops
* Ensure animations are not triggering React re-renders per frame

---

## IPC and External Triggers

If UI issues correlate with IPC:

* Verify IPC call frequency
* Check for unnecessary state updates after IPC responses
* Ensure async results are handled safely

IPC-related UI bugs often manifest as timing issues.

---

## Debugging Tools and Techniques

The agent SHOULD:

* Suggest logging state transitions
* Use profiling tools where appropriate
* Temporarily simplify UI to isolate the issue

Debugging SHOULD progress from:

* Simple observations
* To targeted instrumentation
* To structural fixes

---

## Error and Failure Handling

UI errors MUST:

* Be surfaced clearly
* Avoid cascading failures

Silent UI errors are unacceptable and must be traced.

---

## Postconditions

After executing this skill:

* The root cause of the UI issue should be identified
* Fixes should target the underlying problem
* The UI should behave predictably

---

## Constraints

The agent MUST:

* Follow frontend and performance rules
* Avoid speculative fixes
* Avoid “try this and see” debugging approaches

---

## Anti-Patterns to Avoid

* Blind memoization
* Random dependency array changes
* Guess-based performance fixes
* Masking issues with excessive state resets

---

## Expected Outcome

Following this skill results in UI debugging that:

* Identifies real causes
* Produces durable fixes
* Improves long-term UI stability
