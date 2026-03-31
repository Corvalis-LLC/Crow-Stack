---
name: auto-errors
description: "Error message quality and error handling discipline: actionable messages, audience-appropriate wording, context enrichment on propagation, and the specific error anti-patterns Claude defaults to. Corrects generic messages, developer-speak in user-facing contexts, and bare error propagation. Use when writing error handling, creating error types, or designing error responses. Triggers: error, error message, error handling, Result, anyhow, thiserror, try catch, except, error type, validation error, user-facing error, error response, error format."
---

# Errors — What Claude Gets Wrong

You treat error handling as an afterthought. Your happy paths are elegant; your error paths produce cryptic, generic messages. You write error messages for developers when users will see them, and you propagate errors without adding context.

## The Three-Part Error Message

Every error message answers three questions:

1. **What happened?** — State it clearly
2. **Why?** — The cause
3. **What to do** — Actionable next step

```
BAD:  "Validation error"
GOOD: "Invalid email format. Expected name@domain.com, got 'john@'"

BAD:  "Connection failed"
GOOD: "Unable to connect to PostgreSQL at db.railway.internal:5432: connection refused. Is the database running?"

BAD:  "Internal server error"
GOOD: "Sentinel CHRONOS failed to fetch price data: HTTP 429 Too Many Requests. Retry after 60s."
```

Use "unable to" over "failed to" / "could not" / "can't" — it's neutral and consistent.

## Audience-Appropriate Messages

You write one error message that tries to serve all audiences and serves none. There are three audiences:

**User-facing (UI, API responses):**
- Use the user's language, not internal names
- Never expose stack traces, SQL, or internal service names
- Provide concrete next steps
- Don't blame: "That doesn't look like an email" not "You entered an invalid email"

**Operator-facing (logs):**
- Full technical context: function, module, structured fields
- Correlation IDs for cross-referencing
- Entity IDs, durations, counts

**Programmatic (error types for callers):**
- Matchable variants, not strings
- Typed metadata (the failing path, expected vs actual)
- Stable error codes that won't change when you reword messages

**The boundary rule:** Internal details stay in logs. The user gets a sanitized message + a correlation ID: "Something unexpected happened. Reference: `abc-123`."

## Context Enrichment on Propagation

Your most common Rust failure: bare `?` that loses all context.

```rust
// You write:
let data = fetch_price().await?;

// Senior writes:
let data = fetch_price()
    .await
    .context("fetching BTC price from CoinGecko for sentinel CHRONOS")?;
```

**Rule:** Every `?` at a function boundary should add what THIS function was doing. The result is a causal chain:

```
Error: failed to evaluate triumvirate consensus
  caused by: unable to submit inference job for judge TORUS
    caused by: connection refused to PostgreSQL at db.railway.internal:5432
```

**Exception:** Inside a single function with only one `?`, context is obvious. Don't over-wrap.

## When to Recover vs Propagate vs Crash

| Situation | Action |
|---|---|
| Expected condition with known remediation (retry, fallback, default) | **Recover** |
| This function lacks context to decide | **Propagate** with added context |
| Invalid config at startup | **Crash** — fail fast, not at 3am |
| Invariant violation (programmer error) | **Crash** with `expect("reason this is a bug")` |
| User input validation failure | **Return error** — never crash on user input |

## Structured Error Types Over Strings

```rust
// You write:
Err(anyhow!("invalid input"))

// Senior writes:
#[derive(Debug, thiserror::Error)]
enum SentinelError {
    #[error("unable to fetch {symbol} price from {source}: {reason}")]
    FetchFailed { symbol: String, source: String, reason: String },
    #[error("price threshold exceeded: {delta_pct:.1}% (threshold: {threshold_pct:.1}%)")]
    ThresholdBreached { delta_pct: f64, threshold_pct: f64 },
}
```

Library crates: `thiserror` with meaningful variants. Binary crates: `anyhow` with `.context()`.

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Generic catch-all | `"Something went wrong"` | Be specific about what and why |
| Bare propagation | `fetch_data().await?` | Add `.context("what this function was doing")` |
| Same message, multiple sites | Three places say `"database error"` | Make each unique: include the operation |
| Log-and-propagate | `error!("failed: {e}"); return Err(e)` | Either log OR propagate, never both |
| Lazy expect | `.expect("should have value")` | `.expect("report missing after confirmed insert; this is a bug")` |
| Developer-speak to users | `"Failed to deserialize response body"` | `"Unable to load sentinel reports. Please try again."` |
| Blaming the user | `"You entered an invalid email"` | `"That doesn't look like an email address"` |
| Swallowing silently | `let _ = risky_operation();` | At minimum log at WARN |
| Over-catching | `try { ... } catch (Exception e) { ... }` | Catch the narrowest type you can handle |

## Error Messages Should Be Unique

Every error site should be identifiable from its message alone. If you have three places that can fail with "database error," debugging is impossible. Include the operation: "unable to insert inference_job," "unable to fetch sentinel_reports," "unable to update debate_status."

## Cross-Language Error Patterns

**TypeScript** lacks Rust's `Result` and `thiserror`. Compensate with:
```typescript
// Discriminated union errors (closest to Rust enums)
type AppError =
  | { code: "NOT_FOUND"; resource: string; id: string }
  | { code: "VALIDATION"; field: string; message: string }
  | { code: "INTERNAL"; reference: string };

// Context enrichment via wrapper
function withContext(err: unknown, context: string): Error {
  const wrapped = new Error(context);
  wrapped.cause = err;
  return wrapped;
}
```

**Python** — use custom exception classes, not bare `ValueError` for everything:
```python
class ConfigError(Exception):
    """Collect ALL errors, report at once — no whack-a-mole."""
    def __init__(self, errors: list[str]) -> None:
        super().__init__(f"{len(errors)} config problems:\n" + "\n".join(errors))
        self.errors = errors
```

## Make Invalid States Unrepresentable

The best error handling is no error handling. Use the type system to prevent invalid states:

- Newtypes with validated constructors: `EmailAddress(String)` that validates on creation
- Enums for states: `enum Status { Connecting, Connected, Disconnected }` not boolean flags
- Parse, don't validate: once you have an `EmailAddress`, it's guaranteed valid

Shift error handling to the boundary. The interior of the system should be error-free by construction.
