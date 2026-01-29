# Network Variability and Offline Rules (Mobile)

## Purpose

This document defines **non-negotiable rules** for handling **network variability,
connectivity loss, and offline behavior** on mobile devices.

It exists to prevent:

* UI freezes or hangs due to network issues
* Data corruption from partial requests
* Poor user experience under real-world mobile conditions
* Hidden assumptions about continuous connectivity

These rules apply to:

* Tauri mobile applications
* Unity mobile games
* Any mobile app or game that communicates with remote services

---

## Fundamental Principle

> **On mobile, the network is unreliable by default.**

Connectivity may be:

* Slow
* Intermittent
* Temporarily unavailable
* Switched frequently between Wi-Fi and cellular

Designs that assume stable connectivity are invalid.

---

## Connectivity Assumptions

The agent MUST assume:

* Network availability can change at any time
* Requests may fail mid-flight
* Latency and bandwidth vary significantly
* Captive portals and restricted networks exist

Network failure is normal, not exceptional.

---

## UI and Main Loop Protection

The agent MUST ensure:

* Network operations never block UI or main loops
* User interaction remains responsive during network failure
* Loading states are explicit and cancellable

Blocking on network calls is prohibited.

---

## Offline-First Bias

Where feasible, systems SHOULD:

* Operate in a degraded offline mode
* Cache essential data locally
* Defer non-critical network operations

Offline behavior MUST be intentional, not accidental.

---

## Request Discipline

Network requests MUST:

* Be cancellable
* Have explicit timeouts
* Fail gracefully

Fire-and-forget requests without error handling are prohibited.

---

## Retry and Backoff Strategy

Retries MAY be used only when:

* Failure is likely transient
* Retry count is bounded
* Backoff strategy exists

Aggressive or infinite retries are prohibited.

---

## Data Integrity Rules

The agent MUST ensure:

* Partial responses do not corrupt state
* Failed requests do not leave data half-updated
* Synchronization logic is idempotent

Network failure MUST NOT corrupt persistent data.

---

## Background and Network Usage

When backgrounded:

* Network activity SHOULD be minimized
* Deferred until foreground unless explicitly required
* Aligned with platform background execution rules

Unnecessary background network usage is prohibited.

---

## Cross-Layer Implications

### React (UI)

* MUST tolerate offline states
* MUST not assume immediate network responses
* MUST provide feedback for loading and failure

---

### Rust (Core)

* SHOULD coordinate retries and caching
* MUST validate remote data before applying
* MUST isolate network failure from core logic

---

### Native (Kotlin / Swift)

* MUST respect OS network policies
* MUST handle connectivity callbacks accurately
* MUST not bypass platform restrictions

---

## Anti-Patterns (Explicitly Forbidden)

* Assuming “always online”
* Blocking UI on network calls
* Retrying endlessly
* Silent network failure
* Coupling gameplay or core logic to network availability

---

## Scope and Authority

* These rules apply to all mobile builds.
* They override convenience-based networking logic.
* Platform-specific stricter rules may apply.

---

## Expected Outcome

Following these rules results in mobile apps that:

* Remain usable under poor connectivity
* Avoid network-induced crashes or freezes
* Provide predictable user experience in real-world conditions
