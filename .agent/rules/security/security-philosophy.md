# Security Philosophy and Threat Model

## Purpose

This document defines the **foundational security philosophy**
for all software assisted by the agent, including:

* Desktop tools (Tauri)
* Mobile applications (Android / iOS)
* Games (Unity and Tauri-based)
* Tooling and automation

It exists to ensure security decisions are:

* Intentional
* Proportionate to risk
* Integrated into architecture, not bolted on later

---

## Core Security Mindset

### Security Is a Design Concern

Security MUST be considered:

* During architecture design
* At system boundaries
* When defining data flow

The agent MUST NOT:

* Treat security as a post-processing step
* Assume trusted environments by default

---

### Threat Model Awareness

The agent MUST reason about:

* Who the attacker could be
* What assets are valuable
* What attack surfaces exist

Default assumptions:

* Client-side code is not trusted
* User input is not trusted
* External data is not trusted

---

## Trust Boundaries

### Explicit Trust Boundaries

All systems MUST define clear trust boundaries.

Common boundaries include:

* Frontend ↔ Backend (IPC)
* Game logic ↔ Native system access
* Tool UI ↔ File system

Crossing a trust boundary MUST:

* Be explicit
* Validate inputs
* Fail safely

---

### Client Is Not Trusted

The agent MUST assume:

* Frontend code can be inspected or modified
* Game clients can be tampered with
* UI-level validation can be bypassed

Security-critical logic MUST:

* Live in trusted layers
* Be enforced server-side or native-side where applicable

---

## Principle of Least Privilege

Systems MUST:

* Expose the minimal required capabilities
* Avoid broad or generic permissions

The agent MUST NOT:

* Expose powerful APIs “just in case”
* Grant blanket access to system resources

---

## Input Validation Discipline

All external inputs MUST:

* Be validated
* Be range-checked
* Be type-checked

This applies to:

* IPC payloads
* File paths
* Network data
* User-generated content

Validation MUST occur at the boundary, not deep in the system.

---

## Failure Behavior

### Fail Closed, Not Open

On error or unexpected input, systems SHOULD:

* Reject the operation
* Preserve integrity
* Avoid partial or undefined states

Silent degradation that compromises security is unacceptable.

---

## Secrets and Sensitive Data

The agent MUST:

* Avoid hardcoding secrets
* Avoid exposing sensitive data to untrusted layers

Sensitive data SHOULD:

* Be stored securely
* Be accessed through controlled interfaces

---

## Logging and Diagnostics

Security-relevant events SHOULD:

* Be logged with sufficient context
* Avoid leaking sensitive information

Logs MUST NOT:

* Expose secrets
* Reveal internal system details unnecessarily

---

## Trade-Off Transparency

When security is relaxed for usability or performance reasons,
the agent MUST:

* Make the trade-off explicit
* Explain the risk
* Avoid silent compromises

---

## Anti-Patterns to Avoid

* Trusting client-side validation
* Exposing generic “execute” APIs
* Over-privileged IPC commands
* Assuming obscurity equals security

---

## Scope and Authority

* This document applies globally to all projects.
* More specific security rules may refine it.
* In case of conflict, stricter security rules take precedence.

---

## Expected Outcome

Following this philosophy results in systems that:

* Have clearly defined attack surfaces
* Fail predictably and safely
* Avoid common architectural security mistakes
