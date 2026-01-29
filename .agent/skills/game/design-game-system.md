# Skill: Design Game System

## Purpose

This skill defines how the agent should **design a gameplay system**
that is robust, scalable, and independent of engine-specific details.

It applies to:

* Unity-based games
* Tauri-based games (React as rendering layer)
* Shared game logic reused across platforms or tools

The goal is to produce game systems that:

* Have clear responsibilities
* Are easy to reason about and extend
* Respect performance and architectural constraints

---

## When to Use This Skill

Use this skill when:

* Designing a new gameplay feature or mechanic
* Refactoring existing gameplay logic into clearer systems
* Evaluating the structure of game logic

Do NOT use this skill for:

* UI-only features
* Rendering or animation logic
* Engine integration glue

---

## Inputs

The agent SHOULD expect:

* Description of the gameplay mechanic
* Relevant game rules and constraints
* Performance expectations
* Platform considerations (mobile, desktop)

If gameplay intent is unclear, the agent MUST ask.

---

## Preconditions

Before executing this skill, the agent MUST:

1. Clearly define the system’s responsibility
2. Identify what game state the system reads
3. Identify what game state the system modifies
4. Determine how the system is triggered (events, ticks, actions)

---

## Core Design Principles

### 1. One System, One Responsibility

A game system MUST:

* Solve a single gameplay concern
* Avoid owning unrelated mechanics

Large systems SHOULD be decomposed.

---

### 2. Explicit Inputs and Outputs

The agent MUST:

* Make all data dependencies explicit
* Avoid implicit coupling with other systems

A system SHOULD be understandable by examining:

* Its inputs
* Its outputs
* Its invariants

---

### 3. Data-Driven Behavior

Game systems SHOULD:

* Operate on data
* Avoid embedding hardcoded rules where data-driven configuration is appropriate

This enables:

* Easier balancing
* Reuse across modes or platforms

---

## State and Mutation Discipline

### Controlled State Mutation

The agent MUST:

* Mutate state in well-defined locations
* Avoid scattered state changes

State transitions SHOULD:

* Be explicit
* Be traceable

---

### Determinism Where Possible

Systems SHOULD:

* Produce deterministic results given the same inputs
* Isolate randomness explicitly

This improves:

* Debugging
* Testing
* Replayability

---

## Interaction With Other Systems

### Clear Boundaries

Systems SHOULD:

* Communicate through well-defined events or interfaces
* Avoid direct manipulation of other systems’ internals

The agent MUST NOT:

* Create hidden dependencies between systems

---

## Performance Considerations

The agent MUST:

* Consider update frequency
* Avoid unnecessary per-frame work
* Design for amortized or event-driven execution

Performance-sensitive systems SHOULD:

* Document assumptions
* Be measured in real scenarios

---

## Testing and Validation

Game systems SHOULD:

* Be testable without rendering
* Be executable in isolation

The agent SHOULD:

* Encourage unit or simulation tests
* Avoid designs that require full engine runtime

---

## Postconditions

After executing this skill:

* The system should have a clear purpose
* Its data flow should be explicit
* It should integrate cleanly with other systems

---

## Constraints

The agent MUST:

* Follow game architecture and performance rules
* Avoid engine-specific assumptions
* Avoid premature abstraction

---

## Anti-Patterns to Avoid

* God systems
* Hidden cross-system mutation
* Frame-dependent logic
* Hardcoded balancing values

---

## Expected Outcome

Following this skill results in game systems that:

* Scale with game complexity
* Are easier to maintain and debug
* Can be reused across engines and platforms
