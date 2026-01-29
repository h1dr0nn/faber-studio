# Skill: Design Native SDK Bridge (Tauri Mobile)

## Purpose

This skill defines how the agent should **design a clean, minimal bridge**
between **Rust core logic** and **native mobile SDKs** implemented in
**Kotlin (Android)** and **Swift (iOS)**.

The goal is to integrate SDKs without:

* Leaking SDK details upward
* Polluting business logic
* Creating unmaintainable native code

---

## When to Use This Skill

Use this skill when:

* Integrating ads, billing, analytics, or attribution SDKs
* Designing native capabilities for Tauri mobile
* Refactoring existing native glue code

Do NOT use this skill for:

* Pure UI features
* Web-only APIs
* Desktop-only Tauri features

---

## Inputs

The agent SHOULD expect:

* SDK purpose (ads, IAP, analytics, etc.)
* Required SDK capabilities
* Policy rules from Rust
* Platform targets (Android, iOS)

---

## Design Principles

### 1. Rust Owns Decisions

Rust MUST:

* Decide **when** an SDK action is allowed
* Validate inputs and state
* Enforce business and compliance rules

Native code MUST NOT second-guess Rust decisions.

---

### 2. Native Bridge Is Thin

The native bridge SHOULD:

* Accept simple commands
* Call SDK APIs
* Return structured results

The bridge MUST NOT:

* Maintain complex state
* Contain branching business logic

---

### 3. Minimal Surface Area

Each SDK SHOULD expose:

* A small, explicit command set
* Well-defined inputs and outputs

Generic “do everything” bridges are prohibited.

---

## Error and Result Handling

* Native SDK errors MUST be mapped to stable result types
* Rust MUST handle retries or fallback logic
* React MUST only observe outcomes, not raw SDK state

---

## Lifecycle Management

Native code MUST:

* Handle platform lifecycle events
* Initialize and dispose SDKs correctly
* Respect background / foreground transitions

Lifecycle complexity MUST NOT leak upward.

---

## Testing and Validation

The agent SHOULD:

* Enable mock or stub SDK behavior for testing
* Avoid coupling core logic to real SDK availability

---

## Output

The output of this skill SHOULD include:

* Bridge API definition
* Responsibility split explanation
* Error mapping strategy
* Lifecycle handling notes

---

## Constraints

The agent MUST:

* Follow native SDK integration rules
* Respect platform compliance requirements
* Keep native code auditable and minimal

---

## Expected Outcome

Following this skill results in native SDK integrations that:

* Are safe and compliant
* Do not pollute application logic
* Can be replaced or updated easily
