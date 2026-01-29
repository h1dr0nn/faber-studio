# Profile: Unity Mobile Game (Android / iOS)

## Scope

This profile defines **architectural constraints, design expectations, and system rules**
for **Unity-based mobile games** targeting:

* Android
* iOS

It applies to:

* Gameplay code
* UI systems
* SDK integrations (ads, IAP, analytics)
* Background and lifecycle handling

This profile assumes a **production-grade mobile game**, not a prototype.

---

## Core Architectural Principles

Unity mobile games MUST be designed as **lifecycle-driven systems**, not
continuously running simulations.

The system MUST tolerate:

* App backgrounding
* Process death
* Resource pressure
* Power and thermal constraints

---

## Technology Stack Assumptions

This profile assumes the following stack:

* **Dependency Injection**: VContainer
* **Async Logic**: UniTask
* **Reactive UI / State**: R3
* **Messaging / Signals**: MessagePipe

These choices influence architectural constraints.

---

## Dependency Injection (VContainer)

* All long-lived systems MUST be registered explicitly
* Lifetime scopes MUST align with:

  * App lifetime
  * Scene lifetime
  * Feature lifetime

Hidden singletons outside DI are prohibited.

---

## Async and Concurrency (UniTask)

* Async work MUST be cancellable
* Cancellation MUST align with lifecycle events
* Fire-and-forget async logic is prohibited

Async completion MUST NOT mutate gameplay state directly.

---

## Reactive State (R3)

* Reactive streams MUST be bounded
* Subscriptions MUST be disposed correctly
* UI MUST tolerate re-subscription after lifecycle changes

Leaking subscriptions leads to performance and memory issues.

---

## Messaging (MessagePipe)

* Messages MUST represent events, not state
* Message handlers MUST be fast and non-blocking
* Message lifetimes MUST be scoped

Using messaging as a state container is prohibited.

---

## Game Loop and Time Model

* Gameplay logic MUST be frame-rate independent
* Simulation time MUST be explicit
* Long-running logic MUST not depend on Update frequency

Using `Update()` as a universal driver is prohibited.

---

## Mobile Lifecycle Discipline

The game MUST:

* Handle background / foreground transitions
* Save critical progress before suspension
* Restore state on cold start

Assuming uninterrupted runtime is invalid.

---

## Power and Thermal Budget

The game MUST:

* Reduce activity when idle
* Pause unnecessary systems when backgrounded
* Avoid sustained high CPU/GPU usage

Visual fidelity MUST degrade gracefully when needed.

---

## SDK Integration (Ads, IAP, Analytics)

SDKs MUST:

* Be integrated natively
* Respect store and privacy policies
* Be gated by explicit game logic decisions

UI or gameplay code MUST NOT call SDKs directly.

---

## Network and Offline Behavior

The game MUST:

* Tolerate network loss
* Avoid blocking gameplay on network
* Sync opportunistically

Network failure MUST NOT corrupt game state.

---

## Store Compliance

The game MUST:

* Respect ATT, GDPR, COPPA where applicable
* Follow ad and monetization policies
* Match store metadata with actual behavior

Compliance violations are system failures.

---

## Crash and ANR Observability

The game MUST:

* Capture crashes and ANRs
* Preserve stack traces
* Avoid silent failure

Observability MUST be enabled before release.

---

## Anti-Patterns (Explicitly Forbidden)

* Global static state outside DI
* Async logic without cancellation
* Heavy work in Update()
* Memory-only progress
* SDK logic in gameplay code
* Ignoring lifecycle callbacks

---

## Intended Use

This profile is intended to:

* Guide Unity mobile game architecture
* Constrain AI-generated code
* Prevent common mobile game failures
* Align implementation with store expectations

It is a **system-level contract**, not a tutorial.

---

## Key Takeaway

> **Unity mobile games are lifecycle-bound systems under strict resource constraints.**

Designing with this assumption is mandatory for production success.
