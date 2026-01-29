# Skill: Design Memory Lifecycle and Reuse

## Purpose

This skill defines how the agent should **design memory lifetime, reuse strategies,
and allocation boundaries** before writing implementation code.

It exists to ensure memory behavior is:

* Predictable
* Stable over long sessions
* Aligned with performance and platform constraints

This skill focuses on **design-time decisions**, not micro-optimizations.

---

## When to Use This Skill

Use this skill when:

* Designing a new system with non-trivial data flow
* Refactoring code with GC spikes or memory growth
* Handling frequently updated data structures
* Building long-running tools or games

Do NOT use this skill for:

* Trivial data objects
* One-off scripts
* Cold-path logic with negligible lifetime impact

---

## Inputs

The agent SHOULD expect:

* Description of the data being managed
* Update frequency (per-frame, per-event, batch)
* Target platform (desktop, mobile)
* Observed or expected memory symptoms

If update frequency or platform is unclear, the agent MUST ask.

---

## Step 1: Identify Data Categories

The agent MUST classify data into one of the following:

### Frame-Local

* Valid for a single frame or tick
* Discarded immediately after use

### Short-Lived

* Valid for a short interaction or operation
* Lifetime measured in milliseconds or seconds

### Long-Lived

* Valid across scenes, screens, or sessions
* Owned by a system or service

### Persistent

* Serialized or cached
* Survives restarts or reloads

Misclassification leads to memory bugs.

---

## Step 2: Define Ownership

For each data category, the agent MUST define:

* Who creates the data
* Who owns it
* Who is allowed to mutate it
* When it must be released or invalidated

Ambiguous ownership is prohibited.

---

## Step 3: Choose Allocation Strategy

The agent SHOULD choose allocation strategy based on lifetime:

* **Frame-Local**
  → Stack allocation, reuse buffers, scratch space

* **Short-Lived**
  → Object pools or recyclable containers

* **Long-Lived**
  → Stable heap allocation with clear disposal

* **Persistent**
  → Explicit serialization boundaries

Allocation strategy MUST align with lifetime.

---

## Step 4: Reuse vs Allocate Trade-off

Reuse SHOULD be preferred when:

* Object shape is stable
* Allocation cost is non-trivial
* Lifetime repeats frequently

Allocation MAY be acceptable when:

* Object creation is rare
* Lifetime is long and stable
* Complexity of reuse outweighs benefit

Reuse without clear lifetime is prohibited.

---

## Step 5: Growth and Upper Bounds

The agent MUST define:

* Upper bounds for collections
* Eviction or reset strategy
* Behavior under memory pressure

Unbounded growth is unacceptable.

---

## Step 6: Platform Sensitivity

### Desktop

* More tolerant but still finite
* Memory leaks accumulate over long sessions

### Mobile

* Strict memory limits
* OS may terminate process on pressure
* GC pauses are highly visible

Mobile designs MUST bias toward:

* Reuse
* Predictable memory footprints
* Early release

---

## Step 7: Validation Strategy

The agent SHOULD plan:

* How to observe memory behavior
* How to detect leaks or growth
* How to test long-session stability

Memory behavior must be observable.

---

## Common Design Mistakes (Avoid)

* Treating all data as short-lived
* Pooling everything without bounds
* Retaining references for convenience
* Mixing ownership responsibilities
* Optimizing without understanding lifetime

---

## Output

The output of this skill SHOULD include:

* Data lifetime classification
* Ownership model
* Allocation and reuse strategy
* Identified risks

---

## Postconditions

After executing this skill:

* Memory behavior should be explicit
* Allocation decisions should be justified
* Long-session stability risks should be reduced

---

## Constraints

The agent MUST:

* Follow memory and allocation rules
* Respect platform-specific limits
* Avoid speculative pooling

---

## Expected Outcome

Following this skill results in systems that:

* Avoid GC spikes and memory bloat
* Are easier to reason about
* Remain stable over long-running use
