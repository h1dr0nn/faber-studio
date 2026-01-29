# Gameplay Code Style (Engine-Agnostic)

## Purpose

This document defines **code style and structural rules for gameplay logic**
that apply across all game implementations, regardless of engine or rendering technology.

It exists to ensure gameplay code:

* Remains readable as complexity grows
* Is resilient to feature additions
* Can be reasoned about independently of engine details

This document governs **gameplay logic**, not rendering or engine integration.

---

## Core Gameplay Code Principles

### 1. Gameplay Logic Is Not UI Logic

Gameplay logic MUST:

* Be independent of rendering
* Be independent of UI frameworks
* Avoid direct references to view or presentation layers

Rendering and UI SHOULD:

* Observe gameplay state
* React to state changes
* Never drive core game rules directly

---

### 2. Explicit State Transitions

Gameplay behavior MUST be expressed as:

* Explicit state changes
* Clear transitions from one state to another

Avoid:

* Hidden mutations
* Implicit side effects
* Logic triggered by incidental engine callbacks

State transitions SHOULD be:

* Easy to trace
* Easy to log
* Easy to test

---

## Structural Guidelines

### Separate “What Happens” From “When It Happens”

Gameplay code SHOULD distinguish between:

* **Rules** (what should happen)
* **Timing / scheduling** (when it happens)

Avoid embedding timing logic directly into gameplay rules.

This enables:

* Deterministic simulation
* Flexible scheduling
* Easier balancing and tuning

---

### Small, Focused Gameplay Units

Gameplay logic SHOULD:

* Be composed of small, focused units
* Have clearly defined responsibilities

Avoid:

* Large classes or modules that control many systems
* God objects coordinating all gameplay behavior

---

## Entity and System Thinking

### Entities Are Data, Not Behavior

Entities SHOULD:

* Represent state
* Be mostly passive

Behavior SHOULD:

* Live in systems
* Operate on entity data explicitly

This applies even when using:

* Object-oriented engines
* Functional or reactive patterns

---

### Systems Have Clear Inputs and Outputs

Gameplay systems MUST:

* Declare what data they read
* Declare what data they modify

Avoid:

* Systems that mutate arbitrary global state
* Implicit dependencies between systems

---

## Event and Messaging Discipline

Gameplay events SHOULD:

* Represent meaningful domain events
* Be named after what occurred, not how it was handled

Avoid:

* Using events as a generic control flow mechanism
* Broadcasting events without clear ownership

Events MUST NOT:

* Hide critical gameplay logic
* Replace explicit state transitions

---

## Determinism and Predictability

Gameplay logic SHOULD:

* Be deterministic where feasible
* Produce the same result given the same inputs

Randomness MUST:

* Be explicit
* Be isolated
* Be controllable (e.g. seeded)

---

## Testing and Debugging Considerations

Gameplay code SHOULD:

* Be executable without rendering
* Be testable in isolation
* Support logging or inspection of state transitions

Avoid designs that:

* Require the full engine runtime to test logic
* Entangle gameplay rules with lifecycle callbacks

---

## Naming and Readability

Gameplay identifiers SHOULD:

* Reflect domain concepts
* Avoid engine-specific terminology

Prefer:

* `ApplyDamage`
* `ResolveTurn`
* `AdvanceWave`

Avoid:

* Names tied to update loops or rendering details

---

## Anti-Patterns to Avoid

* Logic hidden inside update/render callbacks
* Gameplay rules embedded in UI components
* Overuse of events to avoid explicit logic
* Frame-dependent gameplay behavior
* Tight coupling between systems

---

## Scope and Authority

* This document applies to all gameplay logic.
* Engine-specific rules may refine integration details.
* In case of conflict, this document takes precedence for gameplay code.

---

## Expected Outcome

Following these rules results in gameplay code that:

* Scales with feature complexity
* Is easier to reason about and debug
* Can be reused across engines and platforms
