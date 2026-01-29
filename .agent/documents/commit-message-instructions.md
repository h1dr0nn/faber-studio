---
description: Guidelines for writing standardized commit messages
---

# Commit Message Instructions

## Standard Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

## Types

| Type       | Description                                                        |
| ---------- | ------------------------------------------------------------------ |
| `feat`     | A new feature                                                      |
| `fix`      | A bug fix                                                          |
| `docs`     | Documentation only changes                                         |
| `style`    | Code formatting, no logic changes (spaces, formatting, semicolons) |
| `refactor` | Code refactoring, neither fixes a bug nor adds a feature           |
| `perf`     | Performance improvements                                           |
| `test`     | Adding or updating tests                                           |
| `chore`    | Changes to build process, auxiliary tools, libraries               |
| `ci`       | CI/CD configuration changes                                        |
| `build`    | Changes affecting build system or external dependencies            |
| `revert`   | Reverts a previous commit                                          |

## Scope (optional)

Scope describes the affected code section:

- `ui` - User interface
- `api` - API endpoints
- `auth` - Authentication/Authorization
- `db` - Database
- `config` - Configuration
- `deps` - Dependencies
- Specific component name (e.g., `SensePage`, `JourneyPage`, `BottomNav`)

## Subject

- Use present tense, imperative mood: "add" not "added" or "adds"
- Do not capitalize the first letter
- No period at the end
- Limit to 50 characters

## Body (optional)

- Explain **what** and **why**, not **how**
- Wrap at 72 characters
- Separate from subject with a blank line

## Footer (optional)

- Reference issues: `Closes #123`, `Fixes #456`
- Breaking changes: `BREAKING CHANGE: description`

---

## Examples

### Simple commit

```
feat(ui): add emotional resonance chart to Sense page
```

### Commit with body

```
fix(SensePage): correct chart scaling on small screens

The radial chart was overflowing on devices with screen width
less than 375px. Added responsive scaling based on viewport.
```

### Breaking change

```
refactor(api)!: change emotion data structure

BREAKING CHANGE: emotion object now uses `intensity` instead of `level`
```

### Multiple scopes

```
feat(ui,theme): implement dark mode for Sense and Journey pages
```

---

## Additional Rules

1. **One commit = one logical change**
   - Do not bundle unrelated changes into a single commit

2. **Commit frequently**
   - Commit after each small milestone is completed

3. **Verify before committing**
   - Ensure code builds successfully
   - Run tests if available

4. **Do not commit directly to main/master**
   - Use feature branches
   - Create Pull Requests for review

---

## Quick Reference

```bash
# New feature
git commit -m "feat(component): add new feature"

# Bug fix
git commit -m "fix(component): resolve issue with X"

# Refactor
git commit -m "refactor(module): simplify logic in function"

# Documentation
git commit -m "docs: update README with setup instructions"

# Styling
git commit -m "style: format code with prettier"

# Performance
git commit -m "perf(api): optimize database queries"

# Chore
git commit -m "chore: update dependencies"
```
