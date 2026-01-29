---
description: Standard process for adding new gameplay features, ensuring architecture integrity and performance.
---

# Workflow: Create Game Feature

## Purpose

This workflow defines the **standard process for adding a new gameplay feature**
to an existing game while preserving:

- Game architecture integrity
- Performance constraints
- Engine-specific rules (Unity or Tauri-based game)

It ensures gameplay features are:

- Designed before implemented
- Integrated without creating hidden coupling
- Safe to extend and debug later

---

## When to Use This Workflow

Use this workflow when:

- Adding a new gameplay mechanic
- Implementing a new system or rule
- Expanding existing gameplay behavior

Do NOT use this workflow for:

- Pure UI features
- Rendering-only or VFX-only changes
- Small balance tweaks without structural impact

---

## Inputs

The agent SHOULD expect:

- Feature description and design intent
- Affected gameplay systems or state
- Target platform(s): Unity, Tauri-based game
- Performance sensitivity (mobile vs desktop)

If the feature intent or scope is unclear, the agent MUST ask.

---

## Step-by-Step Workflow

### Step 1: Clarify Gameplay Intent

- Identify the **core player-facing behavior**
- Define what the feature:

  - Changes
  - Adds
  - Does NOT affect

- Identify non-goals explicitly

**Rules Applied**

- Game architecture rules
- Gameplay code style rules

**Output**

- Clear gameplay intent statement
- Defined scope and non-scope

---

### Step 2: Identify Affected Game State

- Enumerate game state involved
- Determine which systems:

  - Read this state
  - Modify this state

- Identify invariants that must hold

**Skills Used**

- `design-game-system`

**Output**

- State impact map
- Invariants checklist

---

### Step 3: Design the Game System(s)

- Decide whether:

  - A new system is required
  - An existing system should be extended

- Define system responsibility and boundaries
- Define inputs, outputs, and triggers

**Skills Used**

- `design-game-system`

**Rules Applied**

- Engine-agnostic game architecture
- Performance model

**Output**

- System design outline
- Data flow description

---

### Step 4: Choose Engine-Specific Implementation Path

#### Unity

- Map systems into:

  - Plain C# services
  - MonoBehaviour adapters

- Define DI wiring (VContainer)
- Define async behavior (UniTask)
- Define messaging needs (MessagePipe)

#### Tauri-Based Game

- Define:

  - Game loop integration
  - State ownership outside React
  - Rendering observation strategy

- Confirm no gameplay logic enters React

**Rules Applied**

- Unity architecture rules OR
- Tauri game architecture rules

**Output**

- Engine-specific integration plan

---

### Step 5: Implement Incrementally

- Implement core gameplay logic first
- Add engine adapters next
- Add UI or presentation last

Implementation MUST:

- Avoid logic in Update/render loops
- Avoid premature optimization
- Follow approved stack and rules

---

### Step 6: Validate Gameplay Behavior

- Verify feature behaves as designed
- Check state transitions and invariants
- Validate interaction with existing systems

**Skills Used**

- `debug-runtime-issue`

---

### Step 7: Validate Performance

- Check update frequency
- Validate frame budget impact
- Check allocation and GC behavior

**Skills Used**

- `optimize-mobile-fps`

**Output**

- Performance observations
- Identified risks or regressions

---

### Step 8: Validate Architecture and Boundaries

- Confirm no UI-driven gameplay logic
- Confirm no cross-system hidden coupling
- Confirm engine-specific rules are respected

---

### Step 9: Documentation and Notes

- Document:

  - Feature intent
  - System boundaries
  - Known limitations

- Update design notes if needed

---

## Failure Handling

If during implementation:

- Scope expands uncontrollably
- Performance targets cannot be met
- Architecture constraints are violated

The agent MUST:

- Stop
- Surface the issue
- Propose alternatives or trade-offs

---

## Deliverables

By the end of this workflow, the agent SHOULD produce:

- A new gameplay feature
- Updated game systems
- Verified performance impact
- Updated documentation

---

## Anti-Patterns to Avoid

- UI-driven gameplay logic
- Feature logic scattered across systems
- Frame-dependent gameplay behavior
- Engine-specific hacks leaking into domain logic

---

## Expected Outcome

Following this workflow results in gameplay features that:

- Integrate cleanly into the existing architecture
- Perform well on target platforms
- Are maintainable and extensible
