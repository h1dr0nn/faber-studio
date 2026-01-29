# Tauri Mobile and Native SDK Integration Overview

## Why Native Code Is Required

Although Tauri uses a web frontend, **Tauri mobile applications are native apps**.
The WebView is only a rendering layer.

Many mobile capabilities **cannot** be implemented correctly without native code:

* Ads and mediation SDKs
* In-app purchases
* Attribution and analytics
* App Tracking Transparency (iOS)
* Store and OS-level services

---

## Correct Mental Model

Tauri mobile uses a **multi-layer architecture**:

* React handles UI only
* Rust handles rules and policy
* Kotlin / Swift handle SDKs and OS integration

This separation is intentional and required for:

* Security
* Compliance
* Long-term maintainability

---

## Common Mistakes to Avoid

* Treating Tauri as a “web app”
* Calling SDKs from JavaScript
* Encoding business logic in native code
* Letting UI decide compliance behavior

These mistakes lead to:

* Store rejection
* Hard-to-debug bugs
* Architecture collapse over time

---

## Recommended Flow (Conceptual)

1. UI reports an event
2. Rust evaluates policy
3. Native layer invokes SDK
4. Result flows back upward

This ensures:

* Centralized decision-making
* Minimal native complexity
* Platform compliance

---

## Long-Term Benefits

Following this model:

* Makes SDK upgrades easier
* Reduces platform-specific bugs
* Keeps core logic portable
* Protects against architectural drift

---

## Intended Audience

This document is intended for:

* Project maintainers
* New contributors
* AI agents requiring architectural context

It is not a coding guide, but a **conceptual reference**.
