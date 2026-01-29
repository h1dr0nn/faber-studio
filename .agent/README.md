# System Rules, Skills, and Workflows

## Purpose

The `.agent` directory defines a **system-level contract** that governs how AI agents
design, implement, and evolve software projects.

It exists to:

* Prevent architectural drift
* Encode production-grade engineering constraints
* Ensure consistency across platforms, languages, and project types
* Make implicit senior-engineer knowledge explicit and enforceable

This is **not documentation for end users**.
It is a **control surface for agent behavior**.

---

## Core Philosophy

> **Correctness, stability, and long-term maintainability outweigh short-term convenience.**

All rules, skills, and workflows are designed to:

* Bias toward safety
* Avoid hidden assumptions
* Prevent common failure modes in real-world production systems

---

## Structure Overview

```
.agent/
├─ rules/        # Non-negotiable system and platform constraints
├─ skills/       # Design-time reasoning guides
├─ workflows/    # Step-by-step execution processes
├─ profiles/     # Project-type specific constraints
└─ documents/    # Conceptual and onboarding references
```

Each category has a distinct role and authority.

---

## Rules (rules/)

Rules define **hard constraints**.
They MUST be followed and MUST NOT be violated for convenience.

### System Rules

Apply across all platforms:

* CPU & threading discipline
* Memory and allocation rules
* Data ownership and mutation
* Time and scheduling
* Failure isolation

### Platform Rules

#### Mobile (Android / iOS)

* App lifecycle and process death
* Power and thermal budget
* Network variability and offline behavior
* Store compliance
* Native SDK integration (Kotlin / Swift)
* Background execution
* Crash and ANR observability

#### Desktop (Windows / macOS / Linux)

* UI responsiveness and main-thread discipline
* Process execution and terminal visibility
* File system safety and user data protection
* Long-running stability
* Update, migration, and version compatibility

Rules override skills and workflows.

---

## Skills (skills/)

Skills define **how the agent should think** when designing systems.

They are used:

* Before writing code
* When refactoring or redesigning
* When evaluating trade-offs

Examples:

* Designing memory lifecycles
* Designing low-power mobile behavior
* Designing cross-platform desktop behavior
* Designing native SDK bridges

Skills do not override rules.

---

## Workflows (workflows/)

Workflows define **repeatable execution processes**.

They are used:

* When performing multi-step tasks
* When consistency and safety matter
* When rules must be actively checked

All workflows:

* Start with a `description` front-matter
* End with a **System Safety Checklist**
* Must respect applicable rules

---

## Profiles (profiles/)

Profiles apply **context-specific constraints** based on project type.

Examples:

* Tauri application (desktop + mobile)
* Unity mobile game
* Shared libraries

Profiles combine:

* Relevant rules
* Relevant skills
* Platform assumptions

Profiles refine, but do not weaken, rules.

---

## Documents (documents/)

Documents provide:

* Conceptual explanations
* Architectural mental models
* Onboarding context

Documents:

* Do NOT define rules
* Do NOT override behavior
* Exist to align understanding

---

## Precedence and Authority

When conflicts exist:

1. **Rules** (highest authority)
2. **Profiles**
3. **Workflows**
4. **Skills**
5. **Documents** (informational only)

Lower layers MUST NOT contradict higher layers.

---

## How to Use `.agent`

* Read relevant **profiles** first
* Follow **workflows** when executing tasks
* Apply **skills** during design
* Obey **rules** at all times

If a task cannot be completed without violating a rule,
the correct action is to **surface the conflict**, not bypass it.

---

## Intended Audience

This system is intended for:

* Senior engineers
* System designers
* AI agents operating in production contexts

It encodes assumptions typically held implicitly by experienced engineers.

---

## Key Takeaway

> **`.agent` is a guardrail system, not a suggestion system.**

It exists to keep projects:

* Stable
* Maintainable
* Production-ready
