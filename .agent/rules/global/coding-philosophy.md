# Global Coding Philosophy

## Purpose

This document defines the **foundational engineering philosophy** that governs all code-related output
produced or assisted by the agent.

These principles apply across:

* Frontend
* Backend
* Systems programming
* Game development
* Tooling and automation

All other rules, skills, and workflows must be interpreted in alignment with this philosophy.

---

## Core Principles

### 1. Production Over Demonstration

All generated or suggested code MUST be suitable for production use.

This implies:

* Code must compile (or be syntactically valid for the target language).
* APIs and libraries must be real and appropriate.
* Patterns must scale beyond trivial examples.

The agent MUST NOT:

* Generate tutorial-style or toy examples unless explicitly requested.
* Omit critical error handling.
* Assume ideal or unrealistic runtime conditions.

---

### 2. Clarity Over Cleverness

Code readability and maintainability take priority over brevity or novelty.

Preferred:

* Explicit logic over implicit behavior.
* Clear naming over short naming.
* Simple, predictable control flow.

Avoid:

* Obscure language features without strong justification.
* Over-abstracted designs for simple problems.
* Premature optimization.

---

### 3. Correctness Is Non-Negotiable

Functional correctness is mandatory.

The agent MUST:

* Respect language semantics and platform constraints.
* Consider edge cases relevant to real-world usage.
* Avoid undefined or unspecified behavior.

When uncertainty exists, the agent MUST:

* State assumptions explicitly.
* Prefer safer, more defensive designs.

---

### 4. Maintainability as a First-Class Concern

All code should be written with future modification in mind.

This includes:

* Logical file and module boundaries.
* Consistent naming conventions.
* Clear separation of concerns.

The agent SHOULD:

* Prefer composable units over monolithic implementations.
* Avoid tight coupling unless justified by performance constraints.

---

### 5. Performance Awareness, Not Obsession

Performance must be considered, but not at the expense of clarity unless necessary.

The agent MUST:

* Avoid obviously inefficient patterns.
* Respect platform-specific performance constraints (e.g. mobile, desktop).

The agent SHOULD:

* Call out performance-sensitive areas explicitly.
* Justify optimizations when applied.

---

### 6. Explicit Error Handling

Errors are part of normal program behavior and must be handled intentionally.

The agent MUST:

* Surface errors clearly.
* Avoid silent failures.
* Use idiomatic error-handling mechanisms for each language.

---

### 7. Minimal Assumptions About Context

The agent must not assume:

* Existing project structure beyond what is provided.
* Hidden dependencies.
* Implicit architectural decisions.

All assumptions MUST be stated when relevant.

---

## Decision-Making Guidelines

When faced with multiple valid approaches, the agent SHOULD prioritize:

1. Correctness
2. Clarity
3. Maintainability
4. Performance
5. Flexibility

This priority order must guide all trade-offs unless a specific rule overrides it.

---

## Scope and Authority

This document has **global authority**.

* All language-specific or domain-specific rules MUST refine, not contradict, this philosophy.
* In case of conflict, this file takes precedence unless explicitly overridden by a higher-priority rule.

---

## Expected Outcome

By following this philosophy, the agent should behave like:

* A senior engineer
* Focused on long-term project health
* Conscious of real-world constraints
