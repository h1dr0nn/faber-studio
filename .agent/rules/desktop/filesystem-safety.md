# File System Safety and User Data Protection (Desktop)

## Purpose

This document defines **non-negotiable rules** for interacting with the **local file system**
in desktop applications on:

* Windows
* macOS
* Linux

It exists to prevent:

* Accidental data loss
* Silent file corruption
* Destructive overwrite of user files
* Irreversible operations without user awareness

These rules apply to:

* Tauri desktop applications
* Unity desktop tools
* Any desktop app that reads or writes user files

---

## Fundamental Principle

> **User data is sacred and irreplaceable.**

Any file system operation that can modify or delete user data
MUST be treated as a potentially destructive action.

---

## Explicit Intent Requirement

The agent MUST ensure:

* File write, overwrite, move, or delete operations require explicit intent
* Implicit or accidental modification is prohibited
* Defaults must be safe and non-destructive

Operations without clear user intent are invalid.

---

## Read vs Write Discipline

The agent MUST distinguish clearly between:

* Read-only operations
* Mutating operations (write, overwrite, delete)

Code paths MUST NOT blur this distinction.

---

## Overwrite and Delete Rules

Before overwriting or deleting files, the agent MUST:

* Confirm target path explicitly
* Avoid silent overwrite
* Prefer atomic replace strategies where possible

Blind overwrite is prohibited.

---

## Atomic Write Requirement

For file writes, the agent SHOULD:

* Write to a temporary file first
* Validate write success
* Atomically replace the target file

Partial writes MUST NOT corrupt existing data.

---

## Path Safety and Validation

The agent MUST:

* Validate all file paths
* Avoid path traversal vulnerabilities
* Avoid assumptions about path format or separators

User-supplied paths MUST be treated as untrusted input.

---

## Platform-Specific Path Discipline

The agent MUST assume:

* Case sensitivity differs by platform
* Path length limits vary
* Special characters may be present

Hard-coded path assumptions are prohibited.

---

## Permissions and Access Errors

The agent MUST handle:

* Permission denial gracefully
* Locked or in-use files
* Read-only volumes

Crashing or hanging on file access errors is unacceptable.

---

## Backup and Recovery Bias

For destructive operations, the agent SHOULD:

* Offer backup or undo paths
* Preserve original data when feasible
* Fail safely rather than destructively

Irreversible operations must be deliberate.

---

## Temporary and Cache Files

Temporary files MUST:

* Be isolated from user data
* Be cleaned up appropriately
* Never overwrite user files accidentally

Cache cleanup MUST NOT touch user-owned files.

---

## Cross-Layer Responsibilities

### React (UI)

* MUST surface file operation intent clearly
* MUST not perform file logic directly

---

### Rust / Native Core

* MUST enforce safety checks
* MUST own file system operations
* MUST validate paths and outcomes

---

## Anti-Patterns (Explicitly Forbidden)

* Silent overwrite of user files
* Deleting files without confirmation or backup
* Assuming writable paths
* Using user home directory casually
* Treating file IO errors as rare edge cases

---

## Scope and Authority

* These rules apply to all desktop builds.
* They override convenience-based file handling.
* Stricter OS-specific rules may apply.

---

## Expected Outcome

Following these rules results in desktop applications that:

* Protect user data reliably
* Avoid destructive mistakes
* Behave predictably across platforms
* Earn long-term user trust
