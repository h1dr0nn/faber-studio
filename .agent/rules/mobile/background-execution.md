# Background Execution Rules (Mobile)

## Purpose

This document defines **non-negotiable rules** for **background execution**
on **Android and iOS**, including:

* iOS background modes
* Android foreground services
* Silent push and background triggers

It exists to prevent:

* OS policy violations
* Excessive battery drain
* Unexpected app termination
* Misuse of background capabilities

These rules apply only to apps or features that **explicitly require background execution**.

---

## Fundamental Principle

> **Background execution on mobile is a privilege, not a default capability.**

Mobile OSes aggressively restrict background work.
Any background execution MUST be:

* Explicit
* Justified
* Platform-compliant

---

## Default Assumption

The agent MUST assume:

* Apps are **not allowed** to run in background
* Background execution will be suspended or terminated
* OS policies override application intent

Designs that assume continuous background execution are invalid.

---

## iOS Background Execution Rules

### Background Modes

On iOS, background execution is allowed only via **explicit background modes**, such as:

* Audio
* Location
* Bluetooth
* VoIP
* Background fetch (limited)

The agent MUST ensure:

* Background modes are declared explicitly
* Actual behavior matches declared mode
* No misuse of background modes for unrelated work

Declaring unnecessary background modes is prohibited.

---

### Silent Push Notifications

Silent push MAY be used to:

* Wake the app briefly
* Perform minimal background work
* Refresh state

Silent push MUST:

* Perform bounded work
* Complete quickly
* Not rely on guaranteed delivery

Using silent push as a general scheduler is prohibited.

---

## Android Background Execution Rules

### Foreground Services

On Android, sustained background work MUST use:

* Foreground services
* User-visible notifications

The agent MUST ensure:

* Foreground services are clearly justified
* Notifications accurately describe background activity
* Services stop when no longer required

Hidden or deceptive foreground services are prohibited.

---

### Background Limits

The agent MUST assume:

* Background execution limits vary by OS version
* Background work may be delayed or cancelled

Designs MUST tolerate these limitations.

---

## Cross-Platform Discipline

The agent MUST:

* Design background features with platform asymmetry in mind
* Avoid “lowest common denominator” hacks
* Treat background execution as best-effort

Guaranteed background execution is not possible on mobile.

---

## SDK Background Behavior

Native SDKs that perform background work MUST:

* Comply with platform background rules
* Be paused or disabled when background execution is not allowed
* Not keep the app alive unnecessarily

SDK-induced background abuse is unacceptable.

---

## Error and Termination Handling

The agent MUST assume:

* Background work may be killed at any time
* No cleanup callbacks are guaranteed

Background tasks MUST:

* Be idempotent
* Tolerate abrupt termination

---

## Anti-Patterns (Explicitly Forbidden)

* Using background execution to bypass OS limits
* Running hidden background loops
* Misusing silent push as a scheduler
* Declaring background modes “just in case”
* Assuming background execution is reliable

---

## Scope and Authority

* These rules apply only when background execution is used.
* They override convenience-based background designs.
* Platform-specific stricter rules apply.

---

## Expected Outcome

Following these rules results in mobile apps that:

* Respect OS background policies
* Avoid store rejection
* Minimize battery and system abuse
* Behave predictably under background constraints
