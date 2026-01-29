# Unity Security Rules and Client Trust Model

## Purpose

This document defines **security rules specific to Unity-based games**
assisted by the agent.

It exists to ensure that:

* Unity clients are treated as untrusted
* Security-critical logic is not placed in client code
* Common Unity security anti-patterns are avoided

This document refines the global security philosophy for Unity runtimes.

---

## Fundamental Threat Model

### Unity Client Is Not Trusted

The agent MUST assume:

* Game binaries can be inspected, modified, or patched
* Memory can be read or manipulated
* Game logic can be reverse-engineered

Therefore:

* Client-side code MUST NOT be trusted for enforcement
* Critical rules MUST be validated outside the client when applicable

Security through obscurity is not acceptable.

---

## Scope of Client-Side Responsibility

### What Unity Code MAY Do

Unity client code MAY:

* Present visuals and audio
* Collect player input
* Perform local prediction
* Drive moment-to-moment gameplay experience

---

### What Unity Code MUST NOT Do

Unity client code MUST NOT:

* Enforce authoritative game rules in competitive or persistent contexts
* Protect valuable assets through code-only checks
* Assume client-side validation is sufficient

If cheating matters, authority must live elsewhere.

---

## Data Integrity and Validation

### All External Data Is Untrusted

Data originating from:

* Save files
* Network messages
* Player input
* External tools

MUST be:

* Validated
* Sanitized
* Range-checked

Never assume:

* Correct format
* Honest values
* Stable ordering

---

## Save Data and Persistence

### Save Files Are Attack Vectors

Save data MUST:

* Be treated as untrusted input
* Be validated on load
* Fail safely on corruption

Avoid:

* Blind deserialization
* Assuming save files are untampered

Optional protections (when justified):

* Checksums
* Versioning
* Schema validation

---

## Asset and Resource Protection

### Assets Are Not Secure

The agent MUST assume:

* Assets can be extracted
* Asset formats can be reverse-engineered

Therefore:

* Do not embed secrets in assets
* Do not rely on asset obscurity for protection

Asset protection is a business decision, not a security guarantee.

---

## Networking and Online Features

### Client Is Advisory, Not Authoritative

For online or competitive features:

* The client SHOULD be treated as advisory
* Server-side validation SHOULD be authoritative

The agent MUST NOT:

* Trust client-reported scores
* Trust client-reported state transitions
* Trust client-reported timing

---

## Anti-Tamper and Obfuscation

### Obfuscation Is a Delay, Not a Defense

Code obfuscation MAY:

* Increase effort for attackers
* Deter casual tampering

But it MUST NOT:

* Be relied upon as core security
* Replace proper validation or authority separation

Use obfuscation only when justified by business needs.

---

## Runtime Reflection and Debugging

### Debug Features Must Be Controlled

Debug features MUST:

* Be disabled or gated in production builds
* Not expose internal state unintentionally

The agent MUST NOT:

* Leave debug menus accessible
* Leave cheat or test commands enabled

---

## Third-Party Libraries

### Dependency Trust Discipline

All third-party libraries MUST:

* Be reviewed
* Be version-pinned
* Be justified

The agent MUST NOT:

* Introduce libraries without clear need
* Assume third-party code is safe by default

---

## Error Handling and Failure Modes

Errors in Unity MUST:

* Fail safely
* Avoid crashing the entire game when possible
* Avoid exposing sensitive internal state

Security-related errors SHOULD:

* Be logged internally
* Avoid detailed exposure to players

---

## Anti-Patterns to Avoid

* Client-side authority assumptions
* Hidden “anti-cheat” logic in client code
* Hardcoded secrets or keys
* Trusting save data implicitly

---

## Scope and Authority

* This document applies to all Unity-based game code.
* It refines global security rules.
* In case of conflict, stricter security rules take precedence.

---

## Expected Outcome

Following these rules results in Unity games that:

* Are resilient to common tampering techniques
* Do not rely on false security assumptions
* Separate gameplay experience from security enforcement
