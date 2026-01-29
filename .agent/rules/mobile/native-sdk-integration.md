# Native Mobile SDK Integration Rules

## Purpose

This document defines **non-negotiable rules** for integrating **native mobile SDKs**
in Tauri-based applications targeting **Android and iOS**.

It exists to prevent:

* Misplaced business logic in native layers
* Direct SDK usage from untrusted layers
* Architecture drift caused by “quick SDK hacks”
* Security and compliance violations

These rules apply when integrating:

* Ads SDKs (AdMob, AppLovin MAX, mediation)
* In-App Purchase / Billing
* Attribution & Analytics SDKs
* Platform-specific services (ATT, StoreKit, Play Services)

---

## Fundamental Principle

> **Native SDKs belong to the platform layer, not the application logic layer.**

Kotlin and Swift code represent **platform authority**, not business logic.

---

## Layer Responsibility Model

The agent MUST enforce the following responsibility split:

### React (Frontend / UI)

* UI rendering
* User interaction
* State display

React MUST NOT:

* Call native SDKs directly
* Contain SDK-specific logic
* Decide compliance-sensitive behavior

---

### Rust (Core / Policy Layer)

* Business rules and policy decisions
* Validation and gating logic
* Cross-platform behavior consistency

Rust MUST NOT:

* Embed native SDK logic
* Contain platform-specific SDK calls

---

### Kotlin / Swift (Native SDK Layer)

* SDK initialization
* SDK lifecycle handling
* OS and store compliance

Native code MUST:

* Be minimal and bounded
* Act only on explicit instructions from Rust
* Avoid business rules or gameplay logic

---

## Trust Boundaries

The agent MUST treat the following as **strict trust boundaries**:

* React → Rust (IPC boundary)
* Rust → Native SDK (FFI / bridge boundary)

No mutable state or decision authority may cross these boundaries implicitly.

---

## SDK Invocation Rules

* SDK calls MUST originate in Kotlin / Swift only
* SDK calls MUST be triggered by Rust decisions
* React may only signal intent or events

Example (conceptual):

* React: “user completed level”
* Rust: “ads allowed by policy?”
* Native: “show rewarded ad”

---

## Compliance and Security

Compliance-sensitive behavior (ads, tracking, billing) MUST:

* Be enforced natively
* Follow platform requirements
* Be gated by explicit policy checks

Frontend assumptions about compliance are prohibited.

---

## Error Handling

Native SDK failures MUST:

* Be isolated
* Be reported back to Rust as structured results
* NOT crash the application

SDK failure MUST NOT propagate directly to UI or game loop.

---

## Anti-Patterns (Explicitly Forbidden)

* Calling SDKs from JavaScript
* Embedding ads or billing logic in React
* Encoding business logic in Kotlin / Swift
* Using native code as a “dumping ground”
* Bypassing Rust policy checks

---

## Scope and Authority

* These rules apply to all Tauri mobile builds.
* They override convenience-based SDK integration.
* Stricter security or platform rules may apply.

---

## Expected Outcome

Following these rules results in:

* Clean separation of concerns
* Safer SDK integrations
* Easier maintenance and compliance updates
* Reduced risk of platform rejection
