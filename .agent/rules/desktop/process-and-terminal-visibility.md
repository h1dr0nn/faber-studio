# Process Execution and Terminal Visibility Rules (Desktop)

## Purpose

This document defines **non-negotiable rules** for executing **child processes,
bundled scripts, and external tools** in desktop applications on:

* Windows
* macOS
* Linux

It exists to prevent:

* Unwanted terminal or console windows appearing
* Poor user experience and unprofessional behavior
* User confusion or mistrust
* Platform-specific process execution bugs

These rules apply to:

* Bundled Python scripts
* CLI tools
* Background helpers
* Automation binaries
* Any spawned child process

---

## Fundamental Principle

> **Desktop applications must not expose implementation details to users.**

Terminal windows, consoles, and script runners are **implementation artifacts**
and MUST be hidden by default.

---

## Default Process Visibility Rule

The agent MUST ensure:

* All child processes run **without visible terminals**
* No console window appears during normal app operation
* Background helpers are truly background

Visible terminals are allowed **only** for explicit developer/debug modes.

---

## Windows-Specific Rules

On Windows, the agent MUST:

* Disable console window creation for child processes
* Use hidden or detached process flags when spawning
* Avoid relying on default shell behavior

Launching `python.exe`, `cmd.exe`, or CLI tools
without hiding the console is prohibited.

---

## macOS-Specific Rules

On macOS, the agent MUST:

* Avoid launching Terminal.app or visible shells
* Use proper background execution APIs
* Ensure helper tools do not request UI privileges

Console output MUST NOT surface as visible windows.

---

## Linux-Specific Rules

On Linux, the agent MUST:

* Avoid spawning terminal emulators implicitly
* Execute processes detached from any TTY
* Assume diverse desktop environments

Relying on a specific terminal emulator is prohibited.

---

## Bundled Python and Script Execution

When bundling Python or scripts:

* Scripts MUST run headless by default
* STDOUT / STDERR MUST be captured or redirected
* Errors MUST be surfaced via application UI or logs

Letting script output appear in a terminal window is unacceptable.

---

## Error Handling and Logging

The agent MUST ensure:

* Script failures do not spawn terminals
* Errors are reported via structured logs or UI
* Debug output is gated behind explicit flags

Debug convenience MUST NOT leak into production behavior.

---

## User-Initiated CLI Tools (Exception Case)

Visible terminals MAY be allowed only when:

* The user explicitly requests a CLI operation
* The behavior is clearly documented
* The terminal is part of the intended UX

Implicit terminal popups are never acceptable.

---

## Anti-Patterns (Explicitly Forbidden)

* “It’s fine, users can ignore the console”
* Shipping apps that spawn cmd / Terminal windows
* Relying on default shell invocation
* Leaving debug process flags enabled in release builds
* Treating console windows as harmless

---

## Scope and Authority

* These rules apply to all desktop production builds.
* They override convenience-based process execution.
* Platform-specific stricter rules may apply.

---

## Expected Outcome

Following these rules results in desktop applications that:

* Behave professionally
* Hide internal tooling details
* Avoid user confusion or mistrust
* Meet user expectations for native desktop apps
