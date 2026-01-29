# Game Architecture (Engine-Agnostic)

## Purpose

This document defines **core architectural principles for game development**
that apply regardless of implementation platform, including:

* Unity (C#)
* React + Tauri (Canvas / WebGL / custom loop)
* Headless simulations or tooling

Its purpose is to ensure game code:

* Is structured around game logic, not engine mechanics
* Remains testable and evolvable
* Scales in complexity without becoming fragile

---

## Fundamental Game Architecture Model

### Game as a State Machine

At its core, a game is a **state machine evolving over time**.

The agent MUST treat game logic as:

* Explicit state
* Explicit transitions
* Deterministic updates (given the same inputs)

Rendering, input, and effects are **secondary concerns** layered on top of state.

---

### Separation of Concerns

Game systems MUST be conceptually separated into:

* **Game State**

  * The authoritative source of truth
  * Serializable and inspectable
* **Game Logic**

  * Rules that transform state
  * Deterministic where possible
* **Input Handling**

  * Translates raw input into game actions
* **Rendering / Presentation**

  * Visual or audio representation of state
* **Side Effects**

  * I/O, networking, persistence

The agent MUST avoid designs where these concerns are tightly coupled.

---

## Engine-Agnostic Principles

### Logic Must Not Depend on Frame Rate

Game logic MUST NOT:

* Depend on variable frame timing
* Encode logic directly into render callbacks

Time-dependent behavior SHOULD:

* Use explicit delta time
* Be modeled in simulation steps where possible

---

### Determinism Where Feasible

The agent SHOULD:

* Prefer deterministic updates for gameplay logic
* Isolate non-deterministic behavior (input timing, randomness, I/O)

Deterministic logic enables:

* Easier debugging
* Replays
* Testing and simulation

---

## Data-Oriented Thinking

### Data Over Behavior

Game state SHOULD:

* Be represented as plain data structures
* Avoid hidden behavior inside state objects

Behavior SHOULD:

* Live in systems that operate on data
* Be explicit and traceable

This applies regardless of:

* OOP (Unity)
* Functional patterns (TypeScript)
* ECS-style architectures

---

### Explicit Ownership of State

Every piece of game state MUST:

* Have a clear owner
* Be modified through explicit logic paths

Avoid:

* Global mutable state
* Implicit cross-system mutation

---

## Game Loop Concept (Abstract)

All games conceptually follow this loop:

1. Collect input
2. Translate input into actions
3. Update game state
4. Produce outputs (render, audio, events)

The agent MUST:

* Respect this logical order
* Avoid mixing steps implicitly

Actual implementation may vary by engine.

---

## Scalability and Complexity Control

### Avoid God Objects

The agent MUST NOT:

* Centralize all logic in a single controller
* Create monolithic “GameManager” classes that own everything

Responsibility SHOULD be distributed across:

* Systems
* Modules
* Clear boundaries

---

### Progressive Complexity

Architectures SHOULD:

* Start simple
* Allow gradual introduction of complexity

Avoid:

* Over-engineering early
* Introducing ECS or complex abstractions without need

---

## Testing and Simulation

Game logic SHOULD:

* Be testable without rendering
* Be executable in isolation

The agent SHOULD:

* Encourage separation that allows headless execution
* Avoid designs that require the engine to run logic

---

## Scope and Authority

* This document defines **domain-level game architecture rules**.
* Engine-specific rules (Unity, Web-based) MUST refine, not contradict it.
* In case of conflict, this document takes precedence for game logic design.

---

## Expected Outcome

Following these principles results in game code that:

* Is engine-agnostic at the logic level
* Is easier to debug and test
* Can be adapted across platforms and technologies
