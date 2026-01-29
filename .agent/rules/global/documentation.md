# Documentation Standards

## Purpose

This document defines **how documentation should be written, structured, and maintained**
for all code and artifacts produced or assisted by the agent.

The goal is to ensure documentation is:

* Useful to future maintainers
* Accurate and concise
* Aligned with production realities

Documentation is treated as a **first-class engineering artifact**, not an afterthought.

---

## General Principles

### 1. Document Intent, Not Obvious Mechanics

Documentation MUST explain:

* Why a component exists
* What problem it solves
* What constraints it operates under

Documentation MUST NOT:

* Restate trivial or self-evident code behavior
* Duplicate information easily inferred from the code

---

### 2. Accuracy Over Completeness

Outdated or misleading documentation is worse than no documentation.

The agent MUST:

* Avoid speculative descriptions
* Avoid documenting features that are not implemented
* Prefer shorter, accurate docs over exhaustive but fragile ones

If uncertainty exists, it MUST be stated explicitly.

---

### 3. Audience Awareness

Documentation SHOULD assume the reader is:

* A competent developer
* Possibly unfamiliar with local project decisions

It SHOULD NOT assume:

* Prior exposure to the specific codebase
* Access to undocumented tribal knowledge

---

## Code-Level Documentation

### Comments

Comments MUST:

* Explain non-obvious decisions
* Clarify edge cases
* Justify constraints or trade-offs

Comments MUST NOT:

* Describe what the code already states clearly
* Be used to “comment out” dead code

Example (good):

```csharp
// Cached to avoid allocations during Update on mobile devices
```

Example (bad):

```csharp
// Increment i by 1
```

---

### Public APIs

All public-facing APIs MUST be documented.

Documentation SHOULD include:

* Purpose
* Expected inputs
* Outputs
* Error conditions or failure modes

Language-idiomatic documentation styles should be used:

* JSDoc / TSDoc for TypeScript
* XML comments for C#
* Rust doc comments (`///`)
* Python docstrings

---

## Module and File Documentation

Each non-trivial module or file SHOULD include:

* A brief overview at the top
* Key responsibilities
* Important constraints

This is especially important for:

* Systems code (Rust)
* Cross-language boundaries (Tauri IPC, FFI)
* Game systems with lifecycle dependencies

---

## Higher-Level Documentation

### README Files

README documents SHOULD:

* Explain the role of the module or project
* Describe how components fit together
* State important assumptions and limitations

They SHOULD NOT:

* Act as full tutorials
* Duplicate inline documentation verbatim

---

### Architectural Notes

When architectural decisions are non-obvious, documentation SHOULD include:

* The decision made
* Alternatives considered
* Rationale for the chosen approach

This can live in:

* Dedicated architecture notes
* Obsidian vault notes
* Design documents referenced by code

---

## Maintenance Rules

The agent MUST:

* Update documentation when modifying behavior
* Flag documentation that may become outdated
* Avoid “drive-by” documentation changes without understanding context

---

## Scope and Authority

* This document applies globally.
* More specific documentation rules may refine it.
* In case of conflict, more specific rules take precedence.

---

## Expected Outcome

Following these standards ensures documentation that:

* Supports long-term maintenance
* Reduces onboarding time
* Reflects real system behavior
