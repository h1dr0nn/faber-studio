# Skill: Design Low-Power and Thermal-Safe Behavior (Mobile)

## Purpose

This skill defines how the agent should **design application and gameplay behavior**
to minimize **battery drain and thermal stress** on mobile devices.

It exists to ensure mobile systems:

* Conserve power by default
* Degrade gracefully under load
* Remain stable during long sessions

This skill focuses on **behavioral design**, not low-level micro-optimizations.

---

## When to Use This Skill

Use this skill when:

* Designing systems that run continuously or frequently
* Implementing background-capable features
* Building long-session games or tools
* Integrating native SDKs that may wake the CPU

Do NOT use this skill for:

* One-off cold-path logic
* Desktop-only features
* Short-lived utilities

---

## Inputs

The agent SHOULD expect:

* Description of the system or feature
* Execution frequency (idle, event-driven, continuous)
* Target platform (Android, iOS)
* Expected session duration

If execution frequency or session duration is unclear, the agent MUST ask.

---

## Core Principles

### 1. Idle Is the Default State

The agent MUST design systems so that:

* Doing nothing consumes near-zero CPU
* No work runs without a reason

Polling without necessity is prohibited.

---

### 2. Event-Driven Over Polling

The agent SHOULD prefer:

* Event callbacks
* OS or engine signals
* Explicit user actions

Polling MAY be used only when:

* Events are unavailable
* Polling frequency is low and bounded

---

## Step 1: Identify Activity States

The agent MUST identify distinct activity states, such as:

* Idle
* Active interaction
* Backgrounded
* Suspended
* Recovering / restoring

Each state MUST have defined behavior and power expectations.

---

## Step 2: Define Behavior Per State

For each activity state, the agent SHOULD define:

* Which systems are active
* Which systems are paused
* Which timers or loops run
* Which SDKs are enabled

Undefined behavior in any state is unacceptable.

---

## Step 3: Reduce Work During Idle

When idle, the agent SHOULD:

* Halt simulation updates
* Reduce render frequency
* Pause background jobs
* Avoid unnecessary IPC

Idle systems consuming CPU are a design failure.

---

## Step 4: Prefer Burst Execution

The agent SHOULD design work as:

* Short, bounded bursts
* Followed by idle periods

Continuous execution SHOULD be avoided.

---

## Step 5: Degradation Strategies

Under power or thermal pressure, the agent SHOULD:

* Reduce update frequency
* Lower visual fidelity
* Skip non-critical work
* Delay background tasks

Failure to degrade gracefully is unacceptable.

---

## Step 6: Native SDK Power Discipline

When designing SDK usage, the agent SHOULD:

* Initialize SDKs lazily
* Disable or pause SDKs when not needed
* Avoid redundant SDK calls

SDK activity MUST align with actual user-visible value.

---

## Step 7: Validation Plan

The agent SHOULD plan to:

* Observe battery usage during long sessions
* Monitor thermal throttling behavior
* Test idle and background scenarios

Power behavior must be observable.

---

## Common Design Mistakes (Avoid)

* “Always-on” background logic
* High-frequency timers during idle
* Polling for state that rarely changes
* Treating background as a low-priority foreground
* Optimizing FPS while ignoring battery

---

## Output

The output of this skill SHOULD include:

* Activity state definitions
* Per-state behavior description
* Degradation strategy
* Identified power risks

---

## Postconditions

After executing this skill:

* Systems should consume minimal power when idle
* Long sessions should remain stable
* Power-related issues should be predictable

---

## Constraints

The agent MUST:

* Follow power and thermal budget rules
* Respect platform lifecycle constraints
* Avoid speculative optimization

---

## Expected Outcome

Following this skill results in mobile apps that:

* Preserve battery life
* Avoid overheating
* Provide consistent long-session user experience
