---
description: Safe and structured process for refactoring existing modules while preserving constraints.
---

# Workflow: Refactor Module Safely

## Purpose

This workflow defines a **safe, structured process for refactoring an existing module**
(frontend, backend, or shared logic) without breaking architecture, performance,
or security guarantees.

It exists to ensure refactors are:

- Intentional
- Incremental
- Verifiable

---

## When to Use This Workflow

Use this workflow when:

- A module has grown too complex
- Responsibilities are unclear or mixed
- Performance or maintainability issues appear
- Preparing a module for reuse or extension

Do NOT use this workflow for:

- Cosmetic code style changes
- Emergency hotfixes (use targeted fixes instead)

---

## Inputs

The agent SHOULD expect:

- The module to be refactored
- Reasons for refactoring (complexity, bugs, performance)
- Known constraints or risks
- Dependent modules or consumers

If the reason for refactoring is vague, the agent MUST ask.

---

## Step-by-Step Workflow

### Step 1: Clarify Refactor Intent

- Identify the **primary problem**:

  - Responsibility overload
  - Tight coupling
  - Performance bottleneck
  - Testability issues

- Define **non-goals** (what must not change)

**Output**

- Clear refactor objective
- List of invariants to preserve

---

### Step 2: Map Current Responsibilities

- Enumerate what the module:

  - Owns
  - Depends on
  - Exposes publicly

- Identify hidden or implicit responsibilities

**Skills Used**

- `design-rust-module` (for backend)
- `ui-state-debug` (for frontend)

**Output**

- Responsibility map
- Dependency graph (conceptual)

---

### Step 3: Identify Safe Boundaries

- Determine what can be refactored independently
- Identify boundaries protected by:

  - Public APIs
  - IPC contracts
  - Data schemas

Breaking boundaries is prohibited unless explicitly approved.

**Output**

- Refactor-safe zones
- Protected interfaces list

---

### Step 4: Design Target Structure

- Propose the **post-refactor structure**
- Split responsibilities where appropriate
- Simplify data flow and ownership

Design MUST:

- Follow existing rules and skills
- Avoid introducing new abstractions unnecessarily

**Skills Used**

- `design-rust-module`
- `build-react-component`
- `design-game-system` (if applicable)

**Output**

- Target module structure
- Migration strategy

---

### Step 5: Refactor Incrementally

- Apply changes in small, reversible steps
- Keep the system buildable at all times
- Avoid large “big bang” refactors

Each step SHOULD:

- Compile
- Pass existing tests (if any)

---

### Step 6: Validate Behavior

- Verify functional behavior matches pre-refactor behavior
- Re-run performance-sensitive scenarios
- Validate error handling and edge cases

**Skills Used**

- `backend-performance-profiling`
- `ui-state-debug`
- `debug-runtime-issue` (if needed)

---

### Step 7: Validate Architecture and Rules

- Confirm all relevant rules still apply:

  - Security
  - Performance
  - Ownership boundaries

- Ensure no shortcuts were introduced

**Output**

- Refactor validation checklist

---

### Step 8: Cleanup and Documentation

- Remove dead code
- Update documentation or comments
- Document rationale for non-obvious decisions

**Rules Applied**

- Documentation standards

---

## Failure Handling

If during refactor:

- Scope expands uncontrollably
- Core invariants are threatened
- Risks exceed expected benefit

The agent MUST:

- Pause the refactor
- Surface the concern
- Propose alternatives

---

## Deliverables

By the end of this workflow, the agent SHOULD produce:

- A cleaner module structure
- Preserved external behavior
- Improved readability or performance
- Updated documentation

---

## Anti-Patterns to Avoid

- Big-bang refactors
- Refactoring while changing behavior unintentionally
- Introducing abstractions “just in case”
- Ignoring downstream consumers

---

## Expected Outcome

Following this workflow results in refactors that:

- Improve code health without destabilizing the system
- Preserve architectural intent
- Reduce long-term maintenance cost
