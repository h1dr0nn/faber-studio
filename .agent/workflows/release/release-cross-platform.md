---
description: End-to-end release process for desktop and mobile applications, ensuring stability, compliance, observability, and safe update/migration behavior.
---

# Workflow: Release Cross-Platform Application (Desktop & Mobile)

## Purpose

This workflow defines a **safe, end-to-end release process** for applications
targeting **desktop and/or mobile platforms**.

It exists to prevent:

* Shipping unstable builds
* Store rejections
* Broken auto-updates
* Missing crash or ANR visibility
* Irreversible data loss during migration

---

## When to Use This Workflow

Use this workflow when:

* Preparing a production release
* Submitting to app stores
* Enabling auto-update
* Shipping breaking or non-trivial changes

Do NOT use this workflow for:

* Internal debug builds
* Experimental or throwaway releases

---

## Inputs

* Target platforms (desktop, mobile, or both)
* Release version
* Changelog and breaking changes
* Migration requirements
* Store submission requirements (if mobile)

---

## Step-by-Step Workflow

### Step 1: Declare Release Scope

Explicitly declare:

* Target platforms
* Release type (patch / minor / major)
* Whether migrations are included

Undeclared scope leads to incomplete validation.

---

### Step 2: Freeze System Interfaces

Before release:

* Freeze IPC interfaces
* Freeze save-data schemas
* Freeze native bridge APIs

Changing interfaces during release is prohibited.

---

### Step 3: Validate Long-Running Stability (Desktop)

For desktop builds:

* Run extended sessions
* Observe memory and resource usage
* Verify no gradual degradation

Skipping long-session validation is unacceptable.

---

### Step 4: Validate Update & Migration Paths (Desktop)

If auto-update or data migration exists:

* Test update from previous versions
* Test interrupted update scenarios
* Verify rollback or recovery behavior

Assuming updates “just work” is invalid.

---

### Step 5: Validate Mobile Lifecycle & Power Behavior

For mobile builds:

* Test background / foreground transitions
* Test process death and restore
* Observe power and thermal behavior

Lifecycle bugs often surface only in real usage.

---

### Step 6: Validate Store Compliance (Mobile)

Before submission:

* Verify ATT and consent flows
* Verify ads and mediation behavior
* Verify IAP flows
* Verify permissions match declared usage

Compliance checks are mandatory, not optional.

---

### Step 7: Enable Observability

Ensure:

* Crash reporting is enabled
* ANR detection is active (Android)
* Native crash mapping is available
* Debug-only logs are gated

Shipping without observability is prohibited.

---

### Step 8: Network and Offline Validation

Validate:

* Poor or missing network conditions
* Offline startup behavior
* Retry and backoff behavior

Network failure is a normal user scenario.

---

### Step 9: Final Build and Packaging

For desktop:

* Verify packaging per OS
* Verify no visible terminal windows
* Verify correct install locations

For mobile:

* Verify signing and provisioning
* Verify store build configuration

---

### Step 10: Release and Monitor

After release:

* Monitor crash and ANR reports
* Monitor user feedback
* Be prepared to hotfix or rollback

Release is the start of observation, not the end.

---

## System Safety Checklist (Mandatory)

Before completing this workflow, the agent MUST verify:

### Desktop

* [ ] UI remains responsive under load
* [ ] No visible terminal or console windows appear
* [ ] Auto-update and rollback paths are validated
* [ ] Long-running stability is confirmed

### Mobile

* [ ] Lifecycle and process death are handled
* [ ] Power and thermal behavior are acceptable
* [ ] Store compliance requirements are met
* [ ] Background execution rules are respected

### Cross-Cutting

* [ ] Data migrations are explicit and safe
* [ ] Network failure does not corrupt state
* [ ] Crash and ANR observability is enabled

---

## Expected Outcome

Following this workflow results in releases that:

* Pass store review
* Update safely
* Preserve user data
* Surface failures quickly
* Maintain user trust
