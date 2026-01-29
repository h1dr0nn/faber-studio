# Tauri Mobile Layered Responsibility Model

## Overview

This document explains the **layered responsibility model** for **Tauri-based mobile applications**
targeting **Android and iOS**, with a specific focus on **native SDK integration**.

It exists to answer one critical question clearly:

> **Which parts MUST be implemented in native code (Kotlin / Swift), and why?**

This is a conceptual and architectural reference, not a coding guide.

---

## Layered Architecture Recap

A Tauri mobile application is structured as follows:

```
┌──────────────────────────┐
│ React (UI / HUD / Menu)  │  ← untrusted
└──────────▲───────────────┘
           │ IPC
┌──────────┴───────────────┐
│ Rust (Core / Policy)     │  ← trusted
└──────────▲───────────────┘
           │ FFI / Native bridge
┌──────────┴───────────────┐
│ Kotlin / Swift (SDK)     │  ← platform authority
└──────────▲───────────────┘
           │ OS / Store / SDK
┌──────────┴───────────────┐
│ Android / iOS Platform   │
└──────────────────────────┘
```

Each layer has **non-overlapping responsibilities**.

---

## Why Native Code Is Mandatory for Certain Features

Although Tauri uses a web frontend, **mobile platforms do not treat WebViews as first-class citizens**
for many system-level capabilities.

As a result, several feature categories **must be implemented in native code**.

---

## 1. Native SDKs (Ads, IAP, Analytics, Attribution)

### Examples

* Ads SDKs (AdMob, AppLovin, Unity Ads)
* In-App Purchase / Billing
* Analytics SDKs
* Attribution SDKs (AppsFlyer, Adjust)

### Why Native Is Required

These SDKs:

* Are distributed as native libraries
* Depend on OS lifecycle callbacks
* Require tight integration with store APIs
* Are subject to platform compliance rules

They MUST:

* Be initialized natively
* Be invoked from native code
* Handle callbacks in native lifecycle contexts

Implementing these in JavaScript is either:

* Impossible
* Unsupported
* Or non-compliant with store policies

---

## 2. Mediation Layers (AppLovin MAX, AdMob Mediation)

### Why Mediation Is Especially Native-Critical

Mediation frameworks:

* Dynamically load multiple ad SDKs
* Manage waterfall or bidding logic internally
* Depend on precise lifecycle and threading behavior

These frameworks:

* Expect full control of the native environment
* Cannot be reliably orchestrated from JS or Rust

Therefore:

* Mediation logic MUST live entirely in Kotlin / Swift
* Upper layers MUST NOT attempt to “optimize” or bypass mediation

---

## 3. Platform Services (Push, ATT, StoreKit)

### Examples

* Push notifications
* App Tracking Transparency (iOS)
* StoreKit (iOS)
* Google Play Services

### Why Native Is Required

These services:

* Are enforced by the OS
* Require system-level permissions
* Involve modal system dialogs
* Depend on exact OS timing and callbacks

Web layers are:

* Not trusted
* Not granted authority
* Not allowed to bypass these mechanisms

---

## 4. Permissions and OS Lifecycle Handling

### Examples

* Permission prompts
* App background / foreground transitions
* Activity / ViewController lifecycle
* Low-memory warnings

### Why Native Is Required

Only native code:

* Receives authoritative lifecycle callbacks
* Can correctly respond to OS state changes
* Can manage SDK lifecycles safely

Handling these at higher layers leads to:

* Missed callbacks
* Undefined behavior
* Store rejection or runtime crashes

---

## Responsibility Summary Table

| Feature Category        | Must Live In   |
| ----------------------- | -------------- |
| Ads SDK                 | Kotlin / Swift |
| Mediation               | Kotlin / Swift |
| In-App Purchase         | Kotlin / Swift |
| Analytics / Attribution | Kotlin / Swift |
| ATT / StoreKit          | Kotlin / Swift |
| Permissions             | Kotlin / Swift |
| SDK Lifecycle           | Kotlin / Swift |
| Business Rules          | Rust           |
| UI Decisions            | React          |

---

## Role of Rust and React (Important)

### Rust (Core / Policy)

* Decides *whether* an SDK action is allowed
* Enforces business and compliance rules
* Provides a stable, cross-platform decision layer

Rust MUST NOT:

* Implement SDK logic
* Depend on platform-specific APIs

---

### React (UI Layer)

* Displays state
* Collects user input
* Reports events

React MUST NOT:

* Call SDKs directly
* Handle compliance logic
* Depend on platform-specific behavior

---

## Key Takeaway

> **If a feature depends on the OS, the store, or a third-party mobile SDK,
> it belongs in native code.**

Attempting to “lift” these responsibilities upward:

* Breaks platform assumptions
* Increases maintenance cost
* Introduces compliance and security risks

---

## Intended Use of This Document

This document is intended to:

* Align architectural understanding
* Prevent incorrect abstractions
* Serve as onboarding material
* Act as a reference when making integration decisions

It is deliberately strict to protect long-term system integrity.
