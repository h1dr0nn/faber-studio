# Tauri Security Rules and IPC Hardening

## Purpose

This document defines **security rules specific to Tauri-based applications**,
including desktop tools and Tauri-based games targeting Android and iOS.

It exists to ensure that:

* Native capabilities are tightly controlled
* IPC boundaries are hardened
* The frontend is treated as an untrusted layer

This document refines the global security philosophy for the Tauri runtime.

---

## Fundamental Threat Model

### Frontend Is Untrusted

The agent MUST assume:

* Frontend code can be inspected, modified, or injected
* JavaScript runtime can be tampered with
* UI-level validation can be bypassed

Therefore:

* All security-critical logic MUST live in the native layer
* The frontend MUST be treated as an untrusted client

---

## IPC Security Rules

### Explicit IPC Surface Definition

All IPC commands MUST:

* Be explicitly declared
* Have a single, well-defined purpose
* Accept structured, typed payloads

The agent MUST NOT:

* Expose generic or multipurpose IPC commands
* Use dynamic command routing
* Accept arbitrary JSON blobs without validation

---

### Input Validation at IPC Boundary

Every IPC command MUST:

* Validate all inputs
* Enforce type, range, and structural checks
* Reject unexpected fields

Never trust:

* Frontend-side validation
* TypeScript types alone

Validation MUST occur in the native layer.

---

### Least-Privilege IPC Design

IPC commands SHOULD:

* Expose minimal functionality
* Avoid broad system access

Examples of good practice:

* `readUserConfig(path)`
* `saveGameState(data)`

Examples to avoid:

* `executeCommand(cmd)`
* `readAnyFile(path)`

---

## File System Access

### Controlled File Access Only

Native file system access MUST:

* Be routed through explicit IPC commands
* Restrict accessible directories
* Normalize and validate paths

The agent MUST NOT:

* Expose raw file system APIs to the frontend
* Allow arbitrary path traversal

---

## Native Capabilities

### No Implicit Privileges

Native code MUST:

* Explicitly request and justify permissions
* Avoid enabling unused capabilities

The agent MUST NOT:

* Enable features “just in case”
* Leave debug capabilities enabled in production builds

---

## Asynchronous and Long-Running Operations

### Non-Blocking IPC

IPC handlers MUST:

* Be asynchronous
* Avoid blocking the main thread
* Provide progress or completion signaling where appropriate

Long-running operations MUST:

* Be cancellable
* Fail safely on interruption

---

## Error Handling and Exposure

### Sanitized Error Reporting

Errors returned to the frontend MUST:

* Be meaningful but sanitized
* Avoid leaking internal details
* Avoid exposing file paths, stack traces, or system state

Detailed diagnostics SHOULD:

* Remain in native logs
* Be gated behind debug modes

---

## Build and Distribution Security

### Environment Separation

The agent MUST:

* Separate development and production configurations
* Disable debug-only IPC commands in production

Secrets MUST NOT:

* Be embedded in frontend code
* Be exposed through IPC

---

## Mobile-Specific Considerations

On Android and iOS:

* Assume hostile environments
* Assume application package inspection
* Assume memory and IPC introspection is possible

Security decisions MUST:

* Not rely on obscurity
* Not rely on client-side checks

---

## Anti-Patterns to Avoid

* Trusting frontend validation
* Over-broad IPC APIs
* File system access without strict validation
* Debug features leaking into release builds

---

## Scope and Authority

* This document applies to all Tauri-based applications.
* It refines the global security rules.
* In case of conflict, stricter security rules take precedence.

---

## Expected Outcome

Following these rules results in Tauri applications that:

* Expose minimal attack surfaces
* Treat native access responsibly
* Are resilient to common IPC-based attacks
