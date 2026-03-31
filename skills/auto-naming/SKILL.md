---
name: auto-naming
description: "Variable and function naming discipline: domain vocabulary over generic words, verb semantics, scope-proportional length, and the specific naming anti-patterns Claude defaults to. Corrects generic names, naming bankruptcy words, and inconsistent verb prefixes. Use when writing new code, naming variables/functions/types, or reviewing naming quality. Triggers: naming, name, rename, variable name, function name, refactor names, naming convention, domain language, ubiquitous language."
---

# Naming — What Claude Gets Wrong

Your names are acceptable but rarely great. You default to naming things by their structural role (`result`, `data`, `response`) instead of their domain meaning. This skill fixes that.

## The Core Rule

**Name by what it IS in the domain, not what it IS in the code.**

```rust
// You write:
let result = db.query(...).await?;
let data = response.json().await?;
let items = fetch_all().await?;

// Senior writes:
let sentinel_reports = db.query(...).await?;
let price_history = response.json().await?;
let pending_alerts = fetch_all().await?;
```

## Generic Names You Default To — Stop Using These

| Generic | Name by domain meaning instead |
|---|---|
| `result` | `consensus_outcome`, `parsed_config`, `matched_users` |
| `data` | `sentinel_report`, `price_candle`, `whale_transfer` |
| `response` | `user_profile`, `alert_details` (name by content, not transport) |
| `items` / `list` | `pending_jobs`, `active_sentinels`, `failed_evaluations` |
| `value` / `val` | `threshold`, `confidence_score`, `strike_price` |
| `temp` / `tmp` | Name what it temporarily holds: `unsorted_scores` |
| `input` / `output` | `raw_html` / `parsed_tokens` |
| `info` | Merge into the noun: `UserInfo` → `User` or `UserProfile` |

## Function Verb Semantics

Each verb implies different things about performance, side effects, and failure modes. Use the right one.

| Verb | Implies | Failure |
|---|---|---|
| `get` | Cheap accessor, O(1), possibly cached | Infallible or panics |
| `fetch` | Remote/external source, I/O, network | Returns Result, can fail |
| `find` | Searches a collection, absence is normal | Returns Option |
| `load` | Reads from storage, deserializes | Returns Result |
| `query` | Structured lookup (SQL, API) | Returns collection |
| `compute` / `calculate` | Derives via computation, no I/O | Usually infallible |
| `build` / `create` | Constructs a new instance | Returns the new thing |
| `parse` | Text/bytes → structured data | Returns Result |
| `resolve` | Ambiguous reference → concrete value | Can fail |
| `ensure` | Idempotent guarantee (create if missing) | Usually infallible |
| `validate` | Checks correctness, no mutation | Returns bool or Result |
| `emit` / `send` / `dispatch` | Pushes data outward | Fire-and-forget or Result |

**Your specific failure:** You use `get`, `fetch`, `load`, and `retrieve` interchangeably. `get_user_from_database()` should be `fetch_user()`. `fetch_name()` for a field accessor should be `name()` or `get_name()`.

## Scope-Proportional Length

**Variables:** Longer names for wider scopes.
- Loop body: `i`, `u`, `e` — fine
- Single function: `user`, `count`, `path` — fine
- Module/struct field: `active_subscription_count`, `unprocessed_reports` — fully specific

**Functions:** Shorter names for wider scopes (inverse).
- Public API: `push`, `send`, `save`, `close`
- Private helper: `normalize_whale_alert_timestamp`, `calculate_weighted_consensus_score`

## Boolean Naming

Always phrase as a yes/no question that reads naturally in `if`:

| Prefix | Semantics | Example |
|---|---|---|
| `is_` | Current state | `is_active`, `is_connected` |
| `has_` | Possession | `has_permission`, `has_children` |
| `can_` | Capability | `can_edit`, `can_retry` |
| `should_` | Policy/recommendation | `should_notify`, `should_escalate` |
| `needs_` | Requirement | `needs_review`, `needs_migration` |

Always positive form. `is_valid` not `is_not_valid`. Negating a negative (`!is_not_valid`) is cognitive poison.

For function params: `include_archived: bool` reads better than `is_archived: bool` — name by what the caller is choosing.

## Naming Bankruptcy Words — Never Use These for Classes/Modules

`Manager`, `Handler`, `Processor`, `Service`, `Helper`, `Utils`, `Data`, `Info`, `Base`, `Common`, `Core`, `Engine`, `System`

These words mean nothing. Name by what the thing actually does:
- `UserManager` → `UserRepository`, `UserAuthenticator`, `UserRegistration`
- `DataProcessor` → `SentinelReportAggregator`, `WhaleAlertNormalizer`
- `Utils` → Break into specific modules: `formatting`, `validation`, `parsing`

## Collection and Map Naming

- Plural for collections: `users`, `reports`, `alerts`
- Qualified when filtered: `active_users`, `pending_alerts`
- Maps by key-to-value: `price_by_symbol`, `reports_by_sentinel`
- Never: `map`, `dict`, `lookup`, `cache` as the full name

## Domain Vocabulary

When a codebase has established terms, use them. Don't introduce synonyms.
- If the codebase says `sentinel`, don't write `crawler`
- If it says `debate`, don't write `analysis`
- If it says `alert`, don't write `notification`

Grep the codebase for existing terminology before naming new things.

## Naming Reveals Design Problems

If you can't name it, the design is wrong:
- **Can't name without "And"?** It does too much — split it
- **Need "Manager"?** Unclear responsibility — narrow the scope
- **Two things with similar names?** (`UserData` / `UserInfo`) Duplicated concept — merge or differentiate
- **Named by implementation?** (`string_array`, `filtered_list`) Name by meaning: `tags`, `active_subscriptions`

## Don't Encode Types in Names

```python
# You write:          # Senior writes:
user_dict = ...       user = ...
config_map = ...      config = ...
name_str = ...        name = ...
items_list = ...      items = ...
```

The type system handles types. Names handle meaning.
