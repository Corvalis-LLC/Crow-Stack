---
name: auto-edge-cases
description: "Edge case and boundary handling discipline: empty collections, zero/negative inputs, off-by-one, overflow, Unicode, and the specific defensive coding patterns Claude skips. Corrects happy-path-only code that crashes on boundary inputs. Use when writing functions that accept external input, collections, numeric ranges, or string processing. Triggers: edge case, boundary, empty, zero, null, overflow, off-by-one, pagination, guard, defensive, validate input, empty array, empty string, unicode, saturating."
---

# Edge Cases — What Claude Gets Wrong

You write beautiful happy paths that crash on the first empty input. Your code works for the demo case and fails in production. You skip guards because the "obvious" input is non-empty, positive, and ASCII.

## The Boundary Checklist

Before writing any function that accepts input, ask: **"What happens when this is..."**

| Input Type | Check These |
|-----------|-------------|
| Collection | Empty, single-element, very large |
| Number | Zero, negative, MAX, MIN, NaN/Infinity (JS) |
| String | Empty, whitespace-only, Unicode (emoji, RTL, multi-byte), very long |
| Index/Offset | 0, length-1, length (off-by-one), negative |
| Option/Nullable | None/null/undefined — always |
| Pagination | First page, last page (partial), page beyond end, page size 0 |
| Date/Time | Midnight, DST transition, leap day, epoch, far-future |
| Arithmetic | Overflow, underflow, division by zero |

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| No empty guard | `items[0]` without length check | Guard: `if items.is_empty() { return ... }` |
| Unchecked division | `total / count` | `if count == 0 { ... }` or return Option |
| Naive slicing | `&items[start..end]` | `&items[start.min(len)..end.min(len)]` |
| Overflow-prone arithmetic | `page * per_page + per_page` | Use `saturating_mul`, `saturating_add`, or `checked_*` |
| `.length` on Unicode | `str.length` in JS for display width | Use `Intl.Segmenter` or grapheme libraries |
| Truthy checks that miss `0` | `if (value)` when `0` is valid | `if (value != null)` or `if (value !== undefined)` |
| Pagination off-by-one | `Math.ceil(total / pageSize)` with total=0 | Handle total=0 → 0 pages, not 1 |
| Missing last-page truncation | `items.slice(start, start + pageSize)` | Works, but verify `start` doesn't exceed length |
| Negative index acceptance | `items[userInput]` | Validate `>= 0` before indexing |
| Silent NaN propagation | `parseFloat(input) * rate` | Check `isNaN` immediately after parse |

## Language-Specific Patterns

### Rust
```rust
// You write:
fn paginate(items: &[Item], page: usize, per_page: usize) -> &[Item] {
    let start = page * per_page;
    &items[start..start + per_page] // panics on overflow AND out-of-bounds
}

// Senior writes:
fn paginate(items: &[Item], page: usize, per_page: usize) -> &[Item] {
    let start = page.saturating_mul(per_page).min(items.len());
    let end = start.saturating_add(per_page).min(items.len());
    &items[start..end]
}
```

Use `saturating_*` for index math. Use `checked_*` when overflow means an error, not a clamp.

### TypeScript
```typescript
// You write:
function getFirst<T>(items: T[]): T {
  return items[0]; // undefined if empty — caller won't expect it
}

// Senior writes:
function getFirst<T>(items: T[]): T | undefined {
  return items[0]; // Return type tells the truth
}
// Or if empty is an error:
function getFirst<T>(items: T[]): T {
  if (items.length === 0) throw new Error("expected non-empty array");
  return items[0];
}
```

### Python
```python
# You write:
def average(values: list[float]) -> float:
    return sum(values) / len(values)  # ZeroDivisionError on []

# Senior writes:
def average(values: list[float]) -> float:
    if not values:
        return 0.0  # or raise ValueError("cannot average empty list")
    return sum(values) / len(values)
```

## The Off-By-One Discipline

Off-by-one is your second most common boundary failure. Apply this mental model:

- **Inclusive ranges** `[start, end]`: both endpoints are valid. Count = `end - start + 1`
- **Half-open ranges** `[start, end)`: start is valid, end is not. Count = `end - start`
- **Pagination**: page 0 or page 1? Decide once, document it, be consistent.

When writing range logic, always verify with: the empty case (start == end), the single-element case (end == start + 1), and the boundary (last valid index).

## When NOT to Guard

Don't guard against impossible states inside trusted code. If a private function is only called after validation, you don't need to re-validate. Guard at boundaries:

- **Public API endpoints** — always validate
- **Function accepting user input** — always validate
- **Internal helper called from one place** — caller already validated, trust it
- **Struct constructor** — validate, then interior code trusts the type

The rule: validate at the boundary, trust after the boundary.
