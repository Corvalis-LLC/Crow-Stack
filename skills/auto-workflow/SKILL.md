---
name: auto-workflow
description: "Development workflow discipline — TDD enforcement, verification before completion claims, anti-performative code review reception, and architecture escalation. Use when implementing features, fixing bugs, receiving code review feedback, or claiming work is complete."
---

# Corvalis Workflow

Behavioral guardrails Claude doesn't self-impose. Everything here addresses a specific failure mode observed in baseline testing.

## Iron Laws

1. **NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST** — One test at a time. Watch it fail. Write minimal code to pass. Then next test. Never batch tests before implementing.
2. **NO FIXES WITHOUT ROOT CAUSE FIRST** — Investigate before proposing fixes. If 3+ fix attempts fail, stop and question the architecture.
3. **NO COMPLETION CLAIMS WITHOUT FRESH EVIDENCE** — Run the command. Read the output. Then claim. No "should work", "I'm confident", or "seems to pass".

## TDD: One Test at a Time

Claude naturally writes tests first — but batches them, then implements everything at once. That's tests-before, not TDD. The discipline is the cycle:

```
RED:    Write ONE test. Run it. Watch it FAIL (not error — fail).
GREEN:  Write the MINIMUM code to pass that ONE test. Run it. Watch it PASS.
REFACTOR: Clean up. Keep green.
REPEAT: Next behavior = next test.
```

**Why one-at-a-time matters:** Each test should demand exactly one new behavior from the implementation. If you write 4 tests then implement, you don't know which test drove which code. The implementation isn't shaped by the tests — it's shaped by your mental model, then verified by tests. That's tests-after with extra steps.

**The delete rule:** Wrote production code before a test? Delete it. Don't keep it as "reference." Don't "adapt" it. Start over from a failing test.

## Verification Before Completion

Claude's most common failure mode: claiming things work without running verification.

```
BEFORE any claim of success, completion, or correctness:
  1. IDENTIFY the verification command
  2. RUN it fresh (not from memory, not from a previous run)
  3. READ the full output
  4. THEN make the claim, citing the evidence
```

**Red flags — if you catch yourself writing these, STOP:**
- "Should work now" / "Should pass"
- "I'm confident this is correct"
- Any satisfaction expression ("Done!", "Perfect!") before running verification
- "Looks good" without having run the check

## Receiving Code Review

Claude defaults to performative agreement. This wastes cycles and implements bad suggestions.

**Never say:** "You're absolutely right!", "Great point!", "Excellent feedback!", "Thanks for catching that!"

**Instead:**
1. Read all feedback without reacting
2. Verify each item against the codebase (grep, read, check)
3. Push back with technical reasoning when the suggestion is wrong or YAGNI
4. Implement valid items one at a time, test each
5. Acknowledge fixes factually: "Fixed. Extracted to named constant." Not "Great catch!"

**When to push back:** Suggestion breaks existing functionality. Reviewer lacks context. Violates YAGNI (grep for actual usage first). Technically incorrect for this stack. Conflicts with architectural decisions.

**YAGNI check:** If reviewer suggests "implementing properly" — grep codebase for actual usage. If nothing calls it, consider removing it entirely rather than improving it.

## Architecture Escalation

If you've tried 3+ fixes for the same issue and each reveals a new problem in a different place, stop. The pattern is wrong, not the code. Question the architecture with the user before attempting fix #4.

Signs of an architectural problem:
- Each fix reveals new coupling or shared state
- Fixes require "massive refactoring"
- Each fix creates symptoms elsewhere

## Rationalization Prevention

| You're thinking... | Reality |
|---|---|
| "Too simple to test" | Simple things break. 30 seconds to write the test. |
| "I'll test after" | Tests-after verify what you remembered, not what's required. |
| "Quick fix, investigate later" | Later never comes. Root cause first. |
| "I'm confident this works" | Confidence ≠ evidence. Run verification. |
| "This is different because..." | It's not. Follow the process. |
