---
name: auto-coding
description: "Language-agnostic code quality standards for clean, maintainable code. Prevents over-engineering, enforces clarity, and guides refactoring. Use when refactoring code, reviewing code quality, simplifying logic, cleaning up implementations, writing new functions or modules, or fixing code smells. Triggers: refactor, simplify, clean up, improve, review code, code quality, readability, maintainability, complexity, nesting, abstraction, over-engineering, code smell, tech debt."
---

# Corvalis Coding Standards

## Core Principle

**Clarity over cleverness. Simplicity over abstraction. Explicit over compact.**

Write code that a new team member can read and understand without context. If you need a comment to explain what code does, rewrite the code.

---

## Quantified Thresholds

| Metric | Limit | Action |
|--------|-------|--------|
| Function length | 30 lines of logic | Split at logical boundaries |
| Nesting depth | 3 levels max | Early returns or extract function |
| Cyclomatic complexity | 10 per function | Simplify conditions |
| Cognitive complexity | 15 per function | Reduce branching, flatten logic |
| Line length | 100 characters | Break into multiple lines |
| File length | 300 lines (excl. tests) | Split by domain |
| Parameters per function | 4 max | Use options/struct/dataclass beyond 4 |

---

## Comments: Why, Not What

Comments explain **why** code exists, never what it does.

**Comment these:** Non-obvious business logic, workarounds with bug references, performance-critical decisions, non-trivial regex, TODOs with ticket/issue numbers.

**Never comment:** What the code already says, obvious operations, closing braces, journal entries (use git blame), commented-out code (delete it).

---

## Preserve Functionality

When refining or refactoring, never change what the code does — only how it does it. All original behaviors must remain intact. Verify before and after. Do not expand scope beyond what was requested.

---

## Enhance Clarity

### Reduce Complexity

Flatten deeply nested code with early returns and guard clauses. This applies across all languages — Rust's `?` operator, Python's early `return`, TypeScript's guard clauses.

### Eliminate Redundancy

Remove tautological comparisons (`== true`, `== false`), unnecessary wrappers, and verbose expressions that add no information.

### Prefer Data Over Code

When you see repeated conditionals mapping values, replace with a lookup table (map, dict, match, Record, HashMap). Add new entries by adding data, not code.

---

## Maintain Balance

- **Don't combine unrelated concerns** into one function just to reduce line count
- **Don't create dense one-liners** that require mental unpacking
- **Don't sacrifice debuggability** — a slightly longer form that's easier to step through is better
- **Don't create premature abstractions** — three similar lines are better than a helper used once
- **Extract at 3+ occurrences** (Rule of Three). Two instances may be coincidental; three indicates a pattern.

---

## Avoid Over-Engineering

This is the most important section. Over-engineering is the most common failure mode when a skill is active.

- **Don't add features that weren't asked for.** A bug fix doesn't need surrounding code cleaned up. A cleanup task doesn't need new features added.
- **Don't change the public API** unless explicitly asked. Preserve function signatures, return types, and exported interfaces.
- **Don't add error handling for impossible scenarios.** Trust internal code; only validate at system boundaries.
- **Don't create config/options for one-time operations.** If there's only one way it's used, hardcode it.
- **Don't add backwards-compatibility shims.** If something is unused, delete it completely.
- **Don't add docstrings, comments, or type annotations to code you didn't modify.**

---

## Code Smells to Fix on Sight

| Smell | Fix |
|---|---|
| Magic numbers/strings | Named constant |
| Deep nesting (>3 levels) | Early returns or extract function |
| God functions (does multiple unrelated things) | Split by responsibility |
| Commented-out code | Delete it (version control exists) |
| Empty catch/except blocks | Log or handle; don't swallow |
| Copy-pasted blocks (3+) | Extract to shared function |
| Boolean parameters | Options object/struct or separate functions |
| Functions over 30 lines | Split at logical boundaries |
| Mutating function arguments | Return new values instead |
| Catch-and-rethrow with no transformation | Remove the try/catch |

---

## Refinement Process

1. **Read first** — Understand existing patterns, reuse existing code and types before creating new ones
2. **Identify opportunities** — Nesting, redundancy, unclear names, missing early returns
3. **Follow the language** — Use conventions from the relevant language skill (auto-rust, auto-typescript, auto-python) rather than applying one language's idioms to another
4. **Verify functionality** — Code must do exactly what it did before, or what was requested
5. **Keep scope tight** — Only refine code recently modified or explicitly targeted

---

## Technical Debt Markers

Use severity markers to track debt. Each marker should reference a ticket/issue when merging to main.

| Marker | Severity | Meaning |
|--------|----------|---------|
| `FIXME` | Critical | Must fix before production — security, data integrity |
| `TODO` | Medium | Should fix soon — performance, UX, maintainability |
| `HACK` | Low | Workaround that should be replaced |

---

## Reference Files

For detailed patterns, see:
- **[references/detailed-patterns.md](references/detailed-patterns.md)** — File organization, commenting deep dive, future-proofing patterns (abstractions, composition, data over code, framework-agnostic logic, deletion-friendly design)
