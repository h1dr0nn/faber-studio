# Skill: Implement Unity Feature (Stack-Constrained)

## Purpose

This skill defines how the agent should **design and implement a gameplay or system feature**
in Unity while **strictly adhering to the approved Unity technology stack**:

* Dependency Injection: **VContainer**
* Async logic: **UniTask**
* Reactive UI / state: **R3**
* Messaging / signals: **MessagePipe**

The goal is to implement features that:

* Integrate cleanly into the existing architecture
* Respect lifecycle and performance constraints
* Avoid common Unity anti-patterns

---

## When to Use This Skill

Use this skill when:

* Implementing a new gameplay feature in Unity
* Adding a new system or mechanic
* Integrating UI with gameplay state
* Refactoring legacy Unity logic into the approved stack

Do NOT use this skill for:

* Engine-agnostic game system design (use `design-game-system`)
* Rendering-only or VFX-only tasks
* Non-Unity platforms

---

## Inputs

The agent SHOULD expect:

* Feature description and intent
* Affected game systems or state
* Lifecycle constraints (scene-based, global, temporary)
* Performance sensitivity

If the feature’s scope or lifecycle is unclear, the agent MUST ask.

---

## Preconditions

Before implementing the feature, the agent MUST:

1. Identify the **gameplay system(s)** involved
2. Determine where the feature lives in the DI graph
3. Identify async requirements
4. Identify whether messaging is required
5. Confirm UI involvement (if any)

---

## Implementation Rules

### 1. Dependency Injection First (VContainer)

All services and systems MUST:

* Be constructed by VContainer
* Declare dependencies explicitly via constructors

The agent MUST NOT:

* Use `new` for services
* Use static singletons
* Use `FindObjectOfType` or similar patterns

---

### 2. MonoBehaviour as Adapter Only

MonoBehaviours SHOULD:

* Receive injected dependencies
* Bridge Unity lifecycle to systems
* Handle serialization and inspector wiring

MonoBehaviours MUST NOT:

* Contain core gameplay logic
* Act as orchestration hubs

---

### 3. Async Logic via UniTask

Async operations MUST:

* Use UniTask
* Respect cancellation tokens
* Be lifecycle-aware

The agent MUST NOT:

* Use Coroutines for gameplay logic
* Fire-and-forget async tasks without justification

---

### 4. Messaging via MessagePipe

Cross-system communication MUST:

* Use MessagePipe
* Publish explicit domain events

The agent MUST NOT:

* Use ad-hoc C# events
* Introduce additional event systems

Events SHOULD:

* Represent facts, not commands
* Be named after what happened

---

### 5. Reactive UI via R3

R3 SHOULD:

* Observe state changes
* Drive UI updates

R3 MUST NOT:

* Contain gameplay logic
* Mutate gameplay state inside subscriptions

UI logic MUST remain reactive, not imperative.

---

## State and Lifecycle Management

### Explicit Lifecycle Boundaries

The agent MUST:

* Respect scene boundaries
* Initialize and dispose systems explicitly

Hidden persistence across scenes is prohibited unless documented.

---

### Cancellation and Cleanup

All async and reactive subscriptions MUST:

* Be cancellable
* Be disposed correctly

Leaks are unacceptable.

---

## Performance Discipline

The agent MUST:

* Avoid allocations in hot paths
* Avoid logic in `Update` where possible
* Avoid reactive chains in per-frame logic

Performance-sensitive features SHOULD:

* Be measured
* Be justified

---

## Error Handling

Errors MUST:

* Be handled explicitly
* Fail safely
* Avoid crashing the entire game

Silent failure is unacceptable.

---

## Output

The output of this skill SHOULD include:

* A clear implementation plan
* Key classes or systems involved
* DI wiring strategy
* Async and messaging flow

Code SHOULD be:

* Modular
* Readable
* Consistent with existing architecture

---

## Postconditions

After execution:

* The feature should integrate cleanly
* Architecture constraints should be respected
* Performance and lifecycle risks should be minimal

---

## Constraints

The agent MUST:

* Follow all game, Unity, and security rules
* Avoid introducing alternative frameworks
* Avoid speculative abstractions

---

## Anti-Patterns to Avoid

* Logic-heavy MonoBehaviours
* Coroutine-based gameplay logic
* Singleton abuse
* UI-driven gameplay rules

---

## Expected Outcome

Following this skill results in Unity features that:

* Are consistent with the approved stack
* Are easier to test and maintain
* Scale as the project grows
