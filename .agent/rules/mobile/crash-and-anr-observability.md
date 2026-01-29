# Crash and ANR Observability Rules (Mobile)

## Purpose

This document defines **non-negotiable rules** for **crash, ANR (Application Not Responding),
and native failure observability** on mobile platforms.

It exists to prevent:

* Silent crashes in production
* Undetected ANRs degrading user experience
* Inability to diagnose native or cross-language failures
* Shipping regressions without visibility

These rules apply to:

* Android and iOS mobile applications
* Tauri mobile apps
* Unity mobile games
* Apps with native (Kotlin / Swift) and Rust components

---

## Fundamental Principle

> **If a failure cannot be observed, it cannot be fixed.**

Crashes, freezes, and fatal errors MUST be observable in production,
even if recovery is not possible.

---

## Failure Categories (Must Be Covered)

The agent MUST ensure observability for:

* JavaScript / TypeScript crashes
* Rust panics or fatal errors
* Native crashes (Kotlin / Swift)
* ANRs (Android)
* Hard freezes caused by main-thread blocking

Ignoring any category is unacceptable.

---

## Crash Reporting Discipline

### Mandatory Crash Capture

The agent MUST ensure:

* Unhandled crashes are captured
* Stack traces are preserved
* Crashes are associated with app version and platform

Crash reporting MUST be enabled in production builds.

---

### Cross-Language Crash Mapping

For systems with multiple languages:

* Native crashes MUST be distinguishable from JS or Rust failures
* Error boundaries MUST preserve origin information
* Cross-language calls MUST not erase context

Losing failure origin is unacceptable.

---

## ANR (Android) Rules

The agent MUST assume:

* ANRs are user-visible failures
* ANRs lead to store penalties and uninstalls

The agent MUST ensure:

* Main thread is never blocked by long work
* Long-running tasks are offloaded correctly
* ANR signals are observable and logged

Treating ANRs as “rare edge cases” is prohibited.

---

## Main Thread Freeze Detection

The agent SHOULD:

* Monitor main thread responsiveness
* Correlate freezes with recent operations
* Treat long frames as early warning signals

Freezes are often worse than crashes.

---

## Native Crash Discipline

Native crashes MUST:

* Be reported with symbolicated stack traces where possible
* Not be swallowed or masked
* Not be converted into silent failures

Native crashes are critical signals, not noise.

---

## Error Context and Metadata

Crash and ANR reports SHOULD include:

* App version
* OS version
* Device class
* Session state (foreground/background)
* Recent critical actions (bounded)

Reports without context reduce diagnostic value.

---

## Privacy and Data Safety

The agent MUST ensure:

* Crash data does not include sensitive user data
* Logs are sanitized
* Observability respects privacy policies

Debug convenience MUST NOT violate user privacy.

---

## Release Discipline

The agent MUST ensure:

* Observability is validated before release
* Crash reporting works in production configuration
* Debug-only instrumentation is gated appropriately

Shipping without observability is prohibited.

---

## Anti-Patterns (Explicitly Forbidden)

* Disabling crash reporting in production
* Catching fatal errors and continuing blindly
* Ignoring ANRs
* Logging without aggregation or visibility
* Assuming QA will catch all crashes

---

## Scope and Authority

* These rules apply to all mobile production builds.
* They override convenience-based error handling.
* Platform-specific observability requirements may apply.

---

## Expected Outcome

Following these rules results in mobile apps that:

* Fail loudly and diagnosably
* Allow rapid root-cause analysis
* Reduce time-to-fix for production issues
* Improve long-term stability and user trust
