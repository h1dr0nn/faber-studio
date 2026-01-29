# Data Ownership and Mutation Rules

## Purpose

This document defines **non-negotiable rules** for **data ownership, mutation,
and serialization** across all systems.

It exists to prevent:

* Accidental data copying
* Hidden shared mutable state
* Lifetime leaks
* Cross-language inconsistency

These rules apply to:

* Rust ↔ TypeScript (Tauri)
* Rust ↔ C# (Unity)
* Shared libraries
* Save/load pipelines
* IPC boundaries

---

## Fundamental Principles

### 1. Every Data Has an Owner

Every piece of data MUST have **exactly one owner** at any point in time.

The owner is responsible for:

* Lifetime
* Validity
* Disposal or serialization

Data without a clear owner is a system design error.

---

### 2. Ownership ≠ Access

The agent MUST distinguish between:

* **Ownership** (who controls lifetime)
* **Access** (who can read)
* **Mutation** (who can modify)

Read access does NOT imply mutation rights.

---

## Ownership Categories

Data MUST be classified into one of the following ownership models:

### Exclusive Ownership

* One owner
* Mutation allowed only by owner
* No shared mutation

Preferred default.

---

### Shared Read-Only

* One owner
* Multiple readers
* No mutation by readers

Mutation requires explicit ownership transfer.

---

### Transferred Ownership

* Ownership moves from one system to another
* Original owner relinquishes control

Implicit ownership transfer is prohibited.

---

### Serialized Ownership

* Data exists only as serialized form
* Ownership applies only during load/save boundaries

---

## Mutation Rules

### Mutation Authority

The agent MUST ensure:

* Only the owner may mutate data
* Mutation happens in controlled locations

Scattered mutation across systems is prohibited.

---

### Immutable-by-Default Bias

Data SHOULD be treated as immutable unless mutation is explicitly required.

Mutation SHOULD be:

* Localized
* Predictable
* Traceable

---

## Cross-System Boundaries

### IPC Boundaries (Tauri)

At IPC boundaries:

* Ownership MUST NOT be shared
* Data MUST be copied or serialized
* Frontend receives a snapshot, not live data

Shared mutable state across IPC is prohibited.

---

### Engine Boundaries (Unity)

Unity systems MUST:

* Own gameplay state explicitly
* Avoid sharing mutable state across MonoBehaviours
* Use messaging or state snapshots for propagation

---

### Shared Libraries

Shared libraries MUST:

* Never assume ownership of application state
* Operate on caller-owned data
* Clearly document mutation behavior

---

## Serialization and Persistence

### Save / Load Discipline

For save data:

* Serialized data has no owner until loaded
* Loading creates a new owned instance
* Save format is not trusted input

Implicit coupling between runtime state and serialized form is prohibited.

---

## Lifetime and Ownership Alignment

Ownership MUST align with:

* Memory lifetime
* Threading model
* System boundaries

Ownership that outlives its system is a critical bug.

---

## Copy vs Reference Rules

The agent MUST:

* Copy data at ownership boundaries
* Avoid copying within the same ownership domain unnecessarily

Passing references across ownership boundaries is prohibited unless explicitly documented.

---

## Error Handling and Ownership

On error:

* Ownership MUST remain well-defined
* Partially mutated data MUST be handled safely

Undefined ownership during failure is unacceptable.

---

## Anti-Patterns (Explicitly Forbidden)

* Global mutable state
* Shared mutable data across systems
* Implicit ownership transfer
* “Everyone can write” data models
* Passing references across IPC

---

## Scope and Authority

* These rules apply globally.
* They override convenience-based designs.
* Stricter domain rules may apply.

---

## Expected Outcome

Following these rules results in systems that:

* Are easier to reason about
* Avoid data corruption
* Scale safely across languages and platforms
