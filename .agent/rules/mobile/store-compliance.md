# App Store Compliance Rules (Mobile)

## Purpose

This document defines **non-negotiable rules** for ensuring **app store compliance**
on **Android (Google Play)** and **iOS (App Store)**.

It exists to prevent:

* App rejection during review
* Policy violations related to ads, tracking, or billing
* Silent removals or account penalties
* Compliance logic drifting across layers

These rules apply to:

* Tauri mobile applications
* Unity mobile games
* Any mobile app distributed through official stores

---

## Fundamental Principle

> **Store policies are system constraints, not optional guidelines.**

Compliance MUST be enforced architecturally,
not patched after rejection.

---

## Policy Ownership Model

The agent MUST enforce the following ownership:

* **Compliance decisions** → Rust (policy layer)
* **Compliance enforcement** → Native (Kotlin / Swift)
* **Compliance presentation** → React (UI only)

No single layer may own compliance end-to-end.

---

## Ads Compliance Rules

For ad-supported apps:

* Ads MUST respect user consent (GDPR, ATT, etc.)
* Ads MUST NOT be shown before required consent
* Ad frequency MUST follow declared behavior
* Mediation behavior MUST follow SDK policies

Ad logic embedded in UI or game logic is prohibited.

---

## Tracking and Privacy (ATT, GDPR, COPPA)

The agent MUST ensure:

* ATT prompts are shown only at allowed times
* Tracking does not start before consent
* Privacy choices are respected across sessions
* Underage restrictions are enforced when applicable

Assuming user consent is prohibited.

---

## In-App Purchase and Billing

For billing features:

* All purchases MUST use official store APIs
* Purchase flows MUST be native
* Receipts MUST be validated appropriately
* UI MUST not simulate or bypass store flows

Custom or web-based billing is prohibited.

---

## Metadata and Behavior Alignment

The agent MUST ensure:

* App behavior matches store metadata
* Declared permissions match actual usage
* Feature flags do not hide non-compliant behavior

Mismatch between description and behavior is a violation.

---

## Review-Time Behavior

The agent MUST assume:

* Reviewers may test edge cases
* Reviewers may deny permissions
* Reviewers may operate without network

Apps MUST behave gracefully under review conditions.

---

## SDK and Third-Party Compliance

Third-party SDKs MUST:

* Be approved for store usage
* Be configured according to policy
* Be kept up to date when policies change

Using deprecated or banned SDK features is prohibited.

---

## Error Handling and Compliance

On compliance-related errors:

* The app MUST fail safely
* Restricted features MUST remain disabled
* The app MUST not crash or loop

Graceful denial is preferred over partial execution.

---

## Anti-Patterns (Explicitly Forbidden)

* “Fix it after rejection”
* Showing ads before consent
* Hiding tracking behind UI tricks
* Bypassing store billing
* Treating policy as documentation-only

---

## Scope and Authority

* These rules apply to all store-distributed mobile apps.
* They override convenience-based implementation choices.
* Stricter regional or platform rules may apply.

---

## Expected Outcome

Following these rules results in mobile apps that:

* Pass store review reliably
* Avoid compliance-related regressions
* Maintain long-term store eligibility
