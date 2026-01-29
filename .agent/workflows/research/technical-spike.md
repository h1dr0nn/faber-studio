---
description: Controlled process for technical research and experimentation without affecting production code.
---

# Workflow: Technical Spike (Controlled Research)

## Purpose

This workflow defines a **controlled process for technical research and experimentation**
using the agent, without polluting production code, architecture, or long-term decisions.

It exists to:

- Explore unknowns safely
- Validate feasibility before commitment
- Capture learnings explicitly

---

## When to Use This Workflow

Use this workflow when:

- Evaluating a new library, framework, or approach
- Investigating performance characteristics
- Testing architectural alternatives
- Exploring engine or platform constraints

Do NOT use this workflow for:

- Production feature implementation
- Refactoring existing systems
- Shipping code directly to users

---

## Inputs

The agent SHOULD expect:

- The research question or hypothesis
- Target platform or stack (Unity, Tauri, Rust, React)
- Evaluation criteria (performance, complexity, risk)
- Time or scope constraints

If the research question is vague, the agent MUST ask.

---

## Step-by-Step Workflow

### Step 1: Define the Research Question

- Clearly state what is being investigated
- Define success and failure criteria
- Identify what decisions this research will inform

**Output**

- Explicit research question
- Decision impact statement

---

### Step 2: Define Constraints and Non-Goals

- Identify what is explicitly out of scope
- Confirm no production guarantees
- Define acceptable shortcuts or mockups

**Output**

- Constraints list
- Non-goals list

---

### Step 3: Choose the Safest Experiment Form

Select one:

- Isolated prototype
- Minimal sandbox project
- Disposable branch or folder
- Standalone script or spike project

Production code MUST NOT be modified directly.

---

### Step 4: Implement the Spike

- Implement the minimal code required to answer the question
- Prefer clarity over polish
- Avoid premature abstraction

The spike SHOULD:

- Be small
- Be disposable
- Be clearly marked as experimental

---

### Step 5: Observe and Measure

- Measure against defined criteria
- Capture quantitative data when possible
- Note qualitative observations

Speculation without observation is unacceptable.

---

### Step 6: Analyze Results

- Summarize findings clearly
- Identify risks, limitations, and unknowns
- Compare results against alternatives (if applicable)

**Output**

- Findings summary
- Pros / cons
- Open questions

---

### Step 7: Make an Explicit Decision

Based on results:

- Adopt
- Reject
- Defer for later investigation

Decisions MUST be explicit, not implied.

---

### Step 8: Document and Cleanup

- Document conclusions and rationale
- Archive or delete spike code
- Ensure no experimental code leaks into production

---

## Failure Handling

If during the spike:

- Scope expands uncontrollably
- Results are inconclusive
- Time constraints are exceeded

The agent MUST:

- Stop the spike
- Report findings so far
- Recommend next steps

---

## Deliverables

By the end of this workflow, the agent SHOULD produce:

- Research summary
- Measured results
- Explicit recommendation
- Archived or removed experimental code

---

## Anti-Patterns to Avoid

- Turning spikes into production code
- “Just trying things” without criteria
- Keeping experimental code indefinitely
- Making architectural decisions without data

---

## Expected Outcome

Following this workflow results in research that:

- Informs decisions clearly
- Minimizes technical debt
- Preserves architectural integrity
