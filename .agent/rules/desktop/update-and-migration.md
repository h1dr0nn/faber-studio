# Update, Migration, and Version Compatibility Rules (Desktop)

## Purpose

This document defines **non-negotiable rules** for handling **application updates,
data migration, and version compatibility** in desktop applications on:

* Windows
* macOS
* Linux

It exists to prevent:

* Applications failing to launch after update
* Data corruption due to schema mismatch
* Users getting stuck on broken versions
* Irreversible update-related regressions

These rules apply to:

* Tauri desktop applications (including auto-update)
* Unity desktop tools
* Any desktop app that evolves over time

---

## Fundamental Principle

> **Updating the application must never destroy the user’s ability to run it or access their data.**

Updates are a high-risk operation and MUST be treated as such.

---

## Update Atomicity Rule

The agent MUST ensure:

* Updates are applied atomically
* Partial updates cannot leave the app in a broken state
* Rollback or recovery is possible if update fails

Half-applied updates are unacceptable.

---

## Version Awareness Requirement

The agent MUST ensure:

* The running app knows its own version
* Persistent data is tagged with version or schema information
* Migrations are version-aware and explicit

Assuming compatibility implicitly is prohibited.

---

## Data Migration Discipline

When data formats or schemas change:

* Migration MUST be explicit
* Migration MUST be idempotent
* Migration MUST handle partial or interrupted runs

Migration code MUST NOT assume a clean state.

---

## Backward Compatibility Bias

Where feasible, the agent SHOULD:

* Read older data formats
* Defer destructive migrations
* Prefer additive changes over breaking ones

Breaking compatibility MUST be intentional and documented.

---

## Update-Time Failure Handling

On update failure:

* The app MUST fail safely
* Existing data MUST remain intact
* The user MUST not be locked out permanently

Crashing after update without recovery is unacceptable.

---

## Auto-Update Discipline (Tauri)

For Tauri auto-update systems:

* Update checks MUST not block startup or UI
* Update downloads MUST be resumable or cancellable
* Update installation MUST be explicit and visible

Silent destructive updates are prohibited.

---

## Cross-Platform Packaging Awareness

The agent MUST consider:

* Platform-specific install locations
* Permission differences
* File locking behavior during updates

Assuming identical behavior across OSes is invalid.

---

## Migration Testing Requirement

The agent SHOULD:

* Test migration paths from older versions
* Validate upgrades across multiple version gaps
* Test interrupted update scenarios

Untested migrations are high risk.

---

## Anti-Patterns (Explicitly Forbidden)

* Overwriting data without migration
* Assuming users always update sequentially
* Blocking app startup on update failure
* Deleting old data before migration success
* Treating update logic as boilerplate

---

## Scope and Authority

* These rules apply to all desktop builds.
* They override convenience-based update handling.
* Platform-specific stricter rules may apply.

---

## Expected Outcome

Following these rules results in desktop applications that:

* Update reliably
* Preserve user data across versions
* Recover gracefully from update failures
* Maintain long-term user trust
