# Tauri Frontend Constraints

## Purpose

This document defines **frontend-specific constraints and best practices**
when building applications using **Tauri**.

It exists to ensure that React-based UIs:

* Respect the native boundary
* Communicate correctly with the backend (Rust / Python)
* Avoid patterns that break Tauri’s security or performance model

---

## Fundamental Tauri Model

Tauri applications are **not traditional web apps**.

Key implications:

* The frontend runs in a WebView
* The backend has native system access
* Communication occurs via IPC

The agent MUST treat the frontend as:

* A constrained UI layer
* Not a system-level execution environment

---

## IPC Communication Rules

### Explicit IPC Boundaries

All communication between frontend and backend MUST:

* Go through explicit IPC commands
* Use clearly defined payloads
* Handle errors explicitly

The agent MUST NOT:

* Assume synchronous behavior
* Assume unlimited bandwidth
* Assume backend calls are cheap

---

### Command Design

IPC commands SHOULD:

* Be coarse-grained
* Represent meaningful operations
* Avoid chatty, high-frequency calls

Avoid:

* IPC calls inside tight render loops
* IPC calls triggered by every keystroke

---

## Security Constraints

### No Direct System Access

The frontend MUST NOT:

* Access the file system directly
* Execute shell commands
* Perform privileged operations

All such operations MUST be delegated to the backend.

---

### Minimal Surface Area

Exposed IPC commands SHOULD:

* Be minimal
* Be purpose-driven
* Validate all inputs on the backend side

Never expose generic “execute” or “eval”-style commands.

---

## Performance Considerations

### IPC Cost Awareness

IPC calls are significantly more expensive than in-process calls.

The agent MUST:

* Batch operations when possible
* Cache results appropriately
* Avoid unnecessary round-trips

---

### UI Responsiveness

Long-running backend operations MUST:

* Be asynchronous
* Provide progress or loading feedback
* Never block the UI thread

---

## State Synchronization

The frontend SHOULD:

* Treat backend state as authoritative
* Avoid duplicating backend logic
* Synchronize explicitly

Local UI state MUST NOT:

* Drift silently from backend state
* Implicitly mirror backend internals

---

## Error Handling

IPC errors MUST:

* Be surfaced to the frontend explicitly
* Be handled gracefully in the UI
* Include meaningful error messages

Silent failures are unacceptable.

---

## Development and Debugging

The agent SHOULD:

* Encourage logging at IPC boundaries
* Separate dev-only tooling from production code
* Avoid leaking debug-only commands into release builds

---

## Scope and Authority

* This document applies to all Tauri frontend code.
* Backend rules may further constrain behavior.
* In case of conflict, security rules take precedence.

---

## Expected Outcome

Following these constraints ensures:

* Secure Tauri applications
* Predictable frontend-backend interaction
* Good performance and user experience
