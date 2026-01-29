# Skill: Python Automation and Tooling

## Purpose

This skill defines how the agent should **design, write, and maintain Python code**
used for automation, pipelines, and internal tooling.

It applies to:

* Build and release pipelines
* Asset processing
* Developer productivity tools
* Glue code between systems (Unity, Tauri, Rust)

The goal is to ensure Python remains:

* A lightweight automation layer
* Easy to understand and replace
* Free from long-term technical debt

---

## When to Use This Skill

Use this skill when:

* Writing or modifying automation scripts
* Designing pipeline steps
* Creating internal developer tools in Python

Do NOT use this skill for:

* Performance-critical systems
* Long-lived backend services (unless explicitly designed as such)

---

## Inputs

The agent SHOULD expect:

* The task the script must perform
* Execution context (CI, local dev, build server)
* Expected inputs and outputs
* Failure tolerance requirements

If execution context is unclear, the agent MUST ask.

---

## Preconditions

Before executing this skill, the agent MUST:

1. Confirm Python is the appropriate tool
2. Define the script’s single responsibility
3. Identify whether the script is:

   * One-off
   * Reusable
   * Part of a pipeline

---

## Core Automation Principles

### 1. Python Is Glue, Not Core Infrastructure

Python SHOULD:

* Orchestrate
* Transform data
* Automate repetitive tasks

Python MUST NOT:

* Become a hidden backend
* Accumulate complex business logic

If complexity grows, the agent SHOULD suggest migration.

---

### 2. Explicit Inputs and Outputs

Automation scripts MUST:

* Accept explicit inputs (arguments, config files)
* Produce explicit outputs (files, logs, exit codes)

Implicit behavior is unacceptable.

---

## Script Structure Guidelines

### Entry Point Discipline

Scripts MUST:

* Have a clear entry point (`main`)
* Avoid executing logic on import

This ensures:

* Testability
* Predictable behavior

---

### Configuration Handling

Configuration SHOULD:

* Be explicit
* Be externalized where reasonable
* Avoid hardcoded paths or values

Environment variables MAY be used but MUST be documented.

---

## Dependency Management

The agent MUST:

* Minimize third-party dependencies
* Prefer standard library solutions

Dependencies MUST:

* Be pinned
* Be documented

---

## Error Handling and Exit Codes

Scripts MUST:

* Fail loudly on error
* Exit with non-zero codes on failure
* Print actionable error messages

Silent failures are unacceptable.

---

## Logging and Output

The agent SHOULD:

* Use structured, readable logging
* Avoid excessive verbosity by default

Output SHOULD:

* Be machine-consumable when part of a pipeline
* Be human-readable when run manually

---

## Performance Awareness

The agent MUST:

* Avoid unnecessary recomputation
* Avoid loading large datasets unnecessarily

Python SHOULD NOT:

* Be used for heavy CPU-bound work

Heavy tasks SHOULD be delegated to:

* Rust
* Native tools
* Multiprocessing (when justified)

---

## Maintainability and Lifespan

Automation code SHOULD:

* Be easy to delete or rewrite
* Avoid tight coupling to internal details

Scripts that become critical infrastructure SHOULD:

* Be redesigned deliberately
* Not evolve accidentally

---

## Postconditions

After executing this skill:

* The script should be easy to run and understand
* Behavior should be explicit and predictable
* Failures should be obvious and actionable

---

## Constraints

The agent MUST:

* Follow backend and security rules
* Avoid over-engineering
* Avoid speculative abstractions

---

## Anti-Patterns to Avoid

* Monolithic “do-everything” scripts
* Hidden side effects
* Silent error handling
* Treating Python as a permanent backend

---

## Expected Outcome

Following this skill results in Python automation that:

* Improves productivity
* Does not become technical debt
* Can be safely evolved or replaced
