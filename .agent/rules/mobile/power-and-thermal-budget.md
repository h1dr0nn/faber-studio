# Power and Thermal Budget Rules (Mobile)

## Purpose

This document defines **non-negotiable rules** for managing **power consumption,
thermal behavior, and battery impact** on mobile devices.

It exists to prevent:

* Excessive battery drain
* Thermal throttling
* Performance collapse after prolonged usage
* App store rejections due to power abuse

These rules apply to:

* Tauri mobile applications
* Unity mobile games
* Any mobile app with continuous or repeated execution

---

## Fundamental Principle

> **On mobile, power and heat are primary constraints, not secondary concerns.**

Correct functionality that drains battery or overheats the device
is considered a system failure.

---

## Power Budget Reality

The agent MUST assume:

* Mobile CPUs are thermally constrained
* Sustained high load triggers throttling
* Battery drain directly impacts user retention

Desktop-style performance assumptions are invalid.

---

## CPU Usage Discipline

The agent MUST:

* Avoid sustained high CPU utilization
* Avoid busy-wait loops or polling
* Prefer event-driven execution

Continuous background computation is prohibited.

---

## Frame and Update Discipline

For games or animated UIs:

* Stable frame pacing is preferred over maximum FPS
* Dynamic frame rate reduction MAY be used
* Idle states MUST reduce or halt updates

Rendering without visible benefit is unacceptable.

---

## Background Power Rules

When the app is backgrounded:

* CPU activity MUST be minimized
* Timers and loops MUST be stopped
* Network and sensor usage MUST be paused unless explicitly required

Background power usage without justification is prohibited.

---

## Burst vs Sustained Execution

The agent SHOULD prefer:

* Short bursts of computation
* Followed by idle periods

Sustained parallel execution is high risk on mobile.

---

## Native SDK Power Considerations

Native SDKs (ads, analytics, mediation) MUST:

* Be initialized lazily when possible
* Be paused or informed during backgrounding
* Avoid unnecessary polling or wakeups

SDK misuse that increases power drain is unacceptable.

---

## Thermal Throttling Awareness

The agent MUST assume:

* Performance may degrade under thermal pressure
* Long sessions will trigger throttling

Systems SHOULD:

* Degrade gracefully
* Reduce workload when possible

---

## Measurement Requirement

Power-sensitive behavior MUST be:

* Measured on real devices
* Observed during long sessions
* Validated under worst-case scenarios

Assumptions without measurement are prohibited.

---

## Anti-Patterns (Explicitly Forbidden)

* Infinite background loops
* High-frequency polling
* Forcing maximum FPS at all times
* Ignoring background state
* Assuming throttling will not occur

---

## Scope and Authority

* These rules apply to all mobile builds.
* They override convenience-based performance decisions.
* Platform-specific power policies may apply.

---

## Expected Outcome

Following these rules results in mobile apps that:

* Preserve battery life
* Maintain thermal stability
* Deliver consistent long-session performance
