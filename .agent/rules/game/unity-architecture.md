# Unity Architecture and Technology Stack

## Purpose

This document defines the **mandatory architectural rules and technology stack**
for Unity-based game development assisted by the agent.

It exists to ensure Unity code:

* Is modular and testable
* Avoids common MonoBehaviour anti-patterns
* Aligns with modern Unity development practices
* Is consistent across projects

This document is **engine-specific** and refines the engine-agnostic game rules.

---

## Official Unity Stack (Non-Negotiable)

Unless explicitly overridden by the developer, the agent MUST assume
the following stack for all Unity game projects:

* **Dependency Injection**: VContainer
* **Async Logic**: UniTask
* **Reactive State / UI**: R3
* **Messaging / Signals**: MessagePipe

Alternative frameworks (Zenject, UniRx, Coroutines, UnityEvent, ad-hoc events)
MUST NOT be introduced.

---

## Dependency Injection (VContainer)

### Composition Root Discipline

All object composition MUST occur in:

* Explicit composition roots
* Lifetime scopes defined by VContainer

The agent MUST NOT:

* Instantiate services manually (`new`)
* Use `FindObjectOfType` or similar reflection-based lookups
* Rely on singletons for core systems

---

### Lifetime Management

The agent MUST:

* Define lifetimes intentionally (Singleton, Scoped, Transient)
* Align lifetimes with gameplay or scene boundaries

Avoid:

* Long-lived objects with hidden dependencies
* Scene objects owning global logic

---

## MonoBehaviour Usage Rules

### MonoBehaviour Is an Integration Layer

MonoBehaviours SHOULD:

* Bridge Unity lifecycle with game systems
* Handle serialization and inspector binding
* Delegate logic to injected services

MonoBehaviours MUST NOT:

* Contain complex gameplay logic
* Act as service locators
* Own cross-system orchestration

---

### Update Discipline

The agent MUST NOT:

* Centralize gameplay logic in `Update`, `FixedUpdate`, or `LateUpdate`

Update loops SHOULD:

* Be thin
* Forward timing information to systems explicitly
* Avoid hidden logic execution

---

## Async Logic (UniTask)

### UniTask Over Coroutines

All asynchronous gameplay or system logic MUST:

* Use UniTask
* Avoid Unity Coroutines

Coroutines MAY be used only for:

* Legacy interop
* Engine-specific visual sequences (with justification)

---

### Cancellation Discipline

All async operations MUST:

* Accept a `CancellationToken`
* Be cancellable based on lifecycle (scene, object, game state)

Fire-and-forget async logic is prohibited unless:

* Explicitly documented
* Non-critical
* Fully isolated

---

## Reactive State and UI (R3)

### Reactive Is for Observation, Not Logic

R3 SHOULD:

* Observe state changes
* Drive UI updates
* React to domain events

R3 MUST NOT:

* Contain core gameplay rules
* Perform state mutations in subscriptions

Gameplay logic MUST remain imperative and explicit.

---

### Subscription Lifecycle Management

All subscriptions MUST:

* Be disposed correctly
* Respect object or scene lifetimes

Leaking subscriptions is unacceptable.

---

## Messaging and Signals (MessagePipe)

### Explicit Event Boundaries

MessagePipe MUST be used for:

* Cross-system communication
* Domain events
* Decoupled notifications

Events SHOULD:

* Represent facts that occurred
* Be named clearly and precisely

Avoid:

* Using messaging as a control-flow replacement
* Broadcasting without clear ownership

---

### No Ad-Hoc Event Systems

The agent MUST NOT:

* Introduce custom event buses
* Use raw C# events for cross-system messaging
* Mix multiple messaging paradigms

MessagePipe is the single messaging mechanism.

---

## Scene and State Transitions

### Explicit Scene Boundaries

Scene changes SHOULD:

* Be treated as lifecycle transitions
* Trigger explicit initialization and teardown

Hidden state persistence across scenes is discouraged unless intentional.

---

## Testing and Tooling

Unity game logic SHOULD:

* Be testable outside Play Mode
* Live in plain C# classes where possible

The agent SHOULD:

* Encourage separation that allows unit testing
* Avoid designs that require MonoBehaviour instantiation to test logic

---

## Performance and Allocation Rules

The agent MUST:

* Avoid allocations in hot paths
* Be conscious of GC pressure
* Avoid per-frame reactive allocations

Async and reactive abstractions MUST be used with performance awareness.

---

## Anti-Patterns to Avoid

* God-object MonoBehaviours
* Logic-heavy Update loops
* Coroutine-driven gameplay
* Singleton service abuse
* UI-driven gameplay rules

---

## Scope and Authority

* This document applies to all Unity game code.
* It refines engine-agnostic game rules.
* In case of conflict, this document takes precedence for Unity projects.

---

## Expected Outcome

Following these rules results in Unity projects that:

* Scale in complexity
* Are easier to test and refactor
* Avoid common architectural traps
* Fully leverage the chosen technology stack
