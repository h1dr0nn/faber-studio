# Global Naming Conventions

## Purpose

This document defines **consistent, cross-language naming standards**
for all code produced or assisted by the agent.

Consistent naming improves:

* Readability
* Maintainability
* Cross-team collaboration
* Cognitive load when switching languages or domains

These conventions apply unless a more specific rule overrides them.

---

## General Principles

### 1. Names Must Convey Intent

All identifiers MUST clearly communicate their purpose.

Good names answer:

* What does this represent?
* What responsibility does it have?
* What kind of thing is it?

Avoid:

* Ambiguous abbreviations
* Overly generic names
* Context-dependent meanings

---

### 2. Avoid Abbreviations Unless Universally Accepted

Abbreviations are allowed ONLY when they are:

* Industry-standard
* Widely understood in context

Examples of acceptable abbreviations:

* `id`
* `ui`
* `api`
* `cpu`
* `fps`
* `config`

Avoid:

* Custom shorthand
* Single-letter names outside of trivial scopes

---

### 3. Consistency Over Local Preference

Once a naming pattern is established, it MUST be applied consistently.

The agent MUST NOT:

* Mix naming styles within the same scope
* Introduce new synonyms for existing concepts

Example:

* Choose either `UserProfile` or `PlayerProfile`, not both.

---

## Case Conventions by Language

### TypeScript / JavaScript

* **Variables & functions**: `camelCase`
* **Classes & types**: `PascalCase`
* **Constants**: `SCREAMING_SNAKE_CASE`
* **Files**: `kebab-case.ts` or `PascalCase.tsx` (project-consistent)

Examples:

```ts
const maxRetryCount = 3;
function fetchUserData() {}
class GameSession {}
```

---

### Rust

* **Variables & functions**: `snake_case`
* **Types & traits**: `PascalCase`
* **Constants**: `SCREAMING_SNAKE_CASE`
* **Modules & files**: `snake_case.rs`

Examples:

```rust
const MAX_RETRY_COUNT: u32 = 3;

fn fetch_user_data() {}

struct GameSession;
```

---

### Python

* **Variables & functions**: `snake_case`
* **Classes**: `PascalCase`
* **Constants**: `SCREAMING_SNAKE_CASE`
* **Files & modules**: `snake_case.py`

---

### C# (Unity)

* **Classes, structs, enums**: `PascalCase`
* **Methods**: `PascalCase`
* **Fields (private)**: `camelCase`
* **Properties**: `PascalCase`
* **Constants**: `PascalCase`

Examples:

```csharp
private int currentHealth;

public int CurrentHealth { get; }

public void ApplyDamage(int amount) {}
```

---

## Domain-Specific Guidelines

### Boolean Names

Boolean identifiers MUST read naturally as true/false.

Preferred prefixes:

* `is`
* `has`
* `can`
* `should`

Examples:

* `isInitialized`
* `hasAuthority`
* `canRetry`

---

### Collections

Collection names MUST be plural nouns.

Examples:

* `users`
* `activeSessions`
* `enemyUnits`

---

### Events and Handlers

Event-related names SHOULD describe the action that occurred.

Examples:

* `OnPlayerDied`
* `HandleConnectionLost`
* `onWindowResized`

---

## Anti-Patterns to Avoid

* Reusing the same name for different concepts
* Encoding type information into variable names unnecessarily
* Naming based on current implementation details

---

## Scope and Authority

* This document applies globally.
* Language- or framework-specific rules may refine it but MUST NOT contradict it.
* In case of conflict, the more specific rule takes precedence.

---

## Expected Outcome

Following these conventions ensures:

* Faster code comprehension
* Easier refactoring
* Reduced mental overhead when switching stacks
