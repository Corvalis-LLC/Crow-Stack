---
name: auto-comments
description: "Code commenting discipline: when to comment, when silence IS the comment, and the specific anti-patterns Claude defaults to. Corrects over-commenting, restating-the-obvious, and missing high-value 'why not' comments. Use when writing new code, reviewing code, or refactoring. Triggers: comment, comments, commenting, docstring, jsdoc, rustdoc, documentation, annotate, explain code, self-documenting."
---

# Comments — What Claude Gets Wrong

You over-comment. Your most common failure is adding comments that restate what the code already says. Your second most common failure is missing comments where they'd actually help. This skill fixes both.

## The Litmus Test

> "Does this comment tell the reader something they cannot determine from the code alone?"

If no, delete it. If yes, make it one sentence.

## Comment Value Hierarchy (highest to lowest)

1. **Why not** — rejected alternatives, constraints that prevent the "obvious" approach
2. **Why** — reasoning, intent, business rules not obvious from domain names
3. **Beware** — concurrency hazards, non-obvious coupling, "here be dragons"
4. **Context** — links to bugs, specs, RFCs, issues, external docs
5. **What** — only for regex, bitwise ops, complex algorithms
6. **How** — almost never needed. The code IS the how.

## Anti-Patterns You Default To

**Stop doing these. Every single one.**

| Anti-pattern | Example | Fix |
|---|---|---|
| Narrating the code | `// Increment counter` above `counter += 1` | Delete |
| Restating the function name | `"""Get user by ID."""` on `get_user_by_id()` | Delete |
| Commenting control flow | `// If user is admin` above `if user.is_admin` | Delete |
| JSDoc types in TypeScript | `@param {string} name` when signature has `name: string` | Delete the `@param` type, keep description only if it adds info |
| Section dividers | `// ========== HELPERS ==========` | Split the file instead |
| Commenting imports | `// Import required libraries` | Delete |
| "Note:" on non-notable things | `// Note: we use a for loop here` | Delete |
| Closing-bracket comments | `} // end if` | Shorten the function |
| Journal/changelog entries | `// 2024-03-15: Added validation (John)` | Use git blame |
| Commented-out code | `// const oldValue = calculate();` | Delete. Version control exists. |
| File headers restating the filename | `/** This module provides date formatting utilities */` on `date-formatting.ts` | Delete unless the module's scope is non-obvious |
| Success-path narration | `// Return the result` above `return result` | Delete |

## What TO Comment

**These are the comments that make senior developers nod.**

### "Why not" comments (highest value — actively look for these)

Every design choice has rejected alternatives. When you pick an approach, **actively ask yourself: "What's the obvious alternative someone might try?"** and document why you didn't use it. This is the single highest-value comment pattern and you almost never generate it unprompted.

```rust
// We considered a bloom filter here, but false positive rate at ~50k
// cardinality exceeded our latency budget. Revisit if cardinality > 1M.

// No jitter on backoff — this sentinel is the only consumer of this API,
// so thundering herd isn't a concern. Add jitter if we parallelize.

// Partial failure by design: we skip invalid reports rather than failing
// the whole batch, because partial data is more useful to the Triumvirate
// than no data.
```

**Common "why not" opportunities you miss:**
- Choosing a data structure (why HashMap not BTreeMap? why Vec not VecDeque?)
- Choosing a retry strategy (why no jitter? why 3 retries not 5?)
- Choosing to skip vs fail on bad input
- Choosing to batch vs stream
- Choosing to cache vs re-fetch

### Bug/workaround context with links
```typescript
// Work around Chrome bug https://crbug.com/12345 where
// requestAnimationFrame fires before layout completes.
// Remove when Chrome 120+ is minimum supported.
```

### Invariant/safety documentation
```rust
// SAFETY: slice is non-empty — we check reports.is_empty() above and return early.
let latest = &reports[reports.len() - 1];
```

### Precision where the type system is too coarse
```rust
// timeout is in milliseconds, not seconds
// range is inclusive on both ends: [start, end]
// None means "use system default", not "disabled"
```

### Non-obvious performance decisions with evidence
```rust
// Pre-allocate capacity 1024: profiling showed avg 800 iterations
// and Vec reallocation was 12% of runtime. See benches/pipeline.rs
```

### "Here be dragons" warnings
```rust
// WARNING: called from both main thread and worker pool.
// Do NOT acquire state_lock here — deadlocks with commit_batch().
```

### Regex (always comment regex)
```python
# Match ISO date with optional time: YYYY-MM-DD [HH:MM:SS]
# Groups: 1=year, 2=month, 3=day, 4=time (optional)
pattern = r'(\d{4})-(\d{2})-(\d{2})(?:\s+(\d{2}:\d{2}:\d{2}))?'
```

## Docstring Rules

- **Public library APIs**: Always document. Callers can't read your implementation.
- **Internal functions with clear names**: No docstring. `fn send_alert(alert: &Alert)` doesn't need `/// Sends an alert.`
- **When you DO write a docstring**: Document behavior, edge cases, panics, errors — not what the name already says.

## TODO/FIXME/HACK

Always include a ticket or issue reference. Bare TODOs are promises no one keeps.

```rust
// TODO(MONO-1234): migrate to streaming API after v2 launch
// FIXME(MONO-5678): race condition under concurrent writes — needs lock
// HACK: CoinGecko returns timestamps in seconds, not milliseconds
```

## The Verbosity Trap

When you DO write a comment, you tend toward 3-4 sentences. One sentence is almost always enough. If you need a paragraph, the code is too complex — simplify the code, don't explain it.
