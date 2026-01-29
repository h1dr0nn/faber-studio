# Python Scripting and Automation Rules

## Purpose

This document defines **rules and best practices**
for Python code produced or assisted by the agent,
primarily used for scripting, automation, and tooling.

It applies to:

* Build scripts
* Asset pipelines
* Developer tooling
* Auxiliary services supporting Tauri or Unity workflows

The goal is to keep Python code:

* Simple
* Predictable
* Easy to maintain

---

## Role of Python in the System

Python is treated as:

* A glue language
* An automation tool
* A productivity enhancer

It is NOT treated as:

* A high-performance runtime
* A long-lived, stateful backend unless explicitly designed as such

---

## Simplicity First

### Prefer Simple, Explicit Code

The agent MUST:

* Favor straightforward procedural code
* Avoid unnecessary abstraction layers

Complex frameworks or patterns MUST NOT be introduced unless clearly justified.

---

### Script Size Awareness

Python scripts SHOULD:

* Remain small and focused
* Be split when responsibilities grow

Monolithic scripts that perform many unrelated tasks are discouraged.

---

## Dependency Management

### Minimal Dependencies

The agent MUST:

* Minimize third-party dependencies
* Prefer standard library solutions where reasonable

Every dependency MUST:

* Be justified
* Be documented

---

### Explicit Environment Assumptions

Scripts MUST:

* Declare required Python version
* Declare required dependencies
* Avoid relying on globally installed packages

Virtual environments are strongly recommended.

---

## Error Handling

### Fail Loudly and Clearly

Python scripts MUST:

* Handle errors explicitly
* Exit with non-zero status on failure
* Print meaningful error messages

Silent failures are unacceptable.

---

### Defensive Input Handling

All external inputs MUST:

* Be validated
* Be treated as untrusted

Assume scripts may be run with incorrect parameters.

---

## Performance Considerations

The agent SHOULD:

* Avoid unnecessary recomputation
* Cache results when appropriate

The agent MUST:

* Avoid using Python for CPU-intensive workloads unless explicitly acceptable

Heavy computation SHOULD:

* Be delegated to Rust
* Or use multiprocessing when appropriate

---

## Logging and Output

Scripts SHOULD:

* Use structured, readable logging
* Avoid excessive verbosity by default

Output MUST:

* Be suitable for both human reading and automation consumption

---

## Integration with Other Systems

Python code interacting with:

* Rust
* Unity
* Build systems

MUST:

* Use clear interfaces
* Avoid implicit coupling

Scripts SHOULD be replaceable without breaking the system.

---

## Scope and Authority

* This document applies to all Python scripting.
* More specific automation rules may refine it.
* In case of conflict, security rules take precedence.

---

## Expected Outcome

Following these rules ensures Python code that:

* Is easy to understand
* Does not become technical debt
* Supports development workflows reliably
