# App Lifecycle and Process Death Rules (Mobile)

## Purpose

This document defines **non-negotiable rules** for handling **mobile application lifecycle**
and **process death** on Android and iOS.

It exists to prevent:

* Loss of critical state
* Corrupted sessions
* Incorrect assumptions about app longevity
* Bugs that only appear in real-world mobile usage

These rules apply to:

* Tauri mobile applications
* Unity mobile games
* Any long-running mobile app or game

---

## Fundamental Principle

> **On mobile, process death is normal behavior, not an error case.**

The OS may terminate the app:

* Without warning
* Without callbacks
* While the app is in background

Designs that assume continuous process lifetime are invalid.

---

## Lifecycle Reality on Mobile

The agent MUST assume:

* Background ≠ alive
* Suspended ≠ safe
* Memory state ≠ persistent state

At any moment, the OS may reclaim the process.

---

## State Classification Requirement

All runtime state MUST be classified as:

### Reconstructible State

* Can be rebuilt from deterministic logic
* Can be safely discarded on process death

### Persisted State

* Explicitly saved to disk
* Required to restore user experience

Memory-only state MUST be treated as disposable.

---

## Save Discipline

The agent MUST ensure:

* Critical state is saved **before** backgrounding
* Save operations are idempotent
* Partial saves do not corrupt existing data

Saving only on “app exit” is prohibited.

---

## Restore Discipline

On app start or resume:

* The app MUST assume a cold start
* State restoration MUST be explicit
* Missing or corrupted state MUST be handled gracefully

Implicit restoration assumptions are prohibited.

---

## Background Transitions

On backgrounding:

* Long-running tasks MUST be cancelled or checkpointed
* Native SDKs MUST be paused or informed as required
* No assumption of future resume may be made

---

## Foreground Transitions

On returning to foreground:

* State MUST be validated
* External conditions (network, auth, SDK readiness) MUST be rechecked
* UI MUST tolerate partial or delayed restoration

---

## Cross-Layer Implications

### React (UI)

* MUST tolerate full reload
* MUST not assume preserved JS memory
* MUST derive UI from restored state

### Rust (Core)

* MUST own save/restore logic
* MUST validate restored state
* MUST handle versioning and migration

### Native (Kotlin / Swift)

* MUST forward lifecycle events accurately
* MUST not hide process death from upper layers

---

## Anti-Patterns (Explicitly Forbidden)

* Assuming `onPause` / `onStop` always runs
* Relying on in-memory caches for correctness
* Treating background as “safe idle”
* Skipping restore logic during development

---

## Scope and Authority

* These rules apply to all mobile builds.
* They override convenience-based lifecycle handling.
* Platform-specific stricter rules may apply.

---

## Expected Outcome

Following these rules results in mobile apps that:

* Survive process death reliably
* Restore user experience correctly
* Behave predic
