---
name: auto-evolution
description: "Schema and API evolution discipline: backwards-compatible changes, rolling deploy safety, transition periods, deprecation-over-removal, and consumer impact assessment. Corrects destructive renames, required field additions, atomic switchover assumptions, and missing runtime deprecation warnings. Use when changing database schemas, API contracts, config formats, shared struct definitions, environment variables, or CLI interfaces. Triggers: migration, rename, schema, breaking change, backwards compatible, deprecate, column, field, ALTER, evolution, rolling deploy, config change, env var rename, version, transition."
---

# Evolution — What Claude Gets Wrong

You refactor like the old version vanishes the instant you deploy. It doesn't. Rolling deploys mean old code reads new data (and vice versa) for minutes or hours. Multi-binary systems mean one binary updates while others run the old version. Config files in the wild don't update themselves.

## The Iron Rule

**Every change to a shared interface must be safe for both old and new consumers simultaneously.** Shared interfaces include: database columns, serialized messages (JSON, protobuf, job queue payloads), API response shapes, config file fields, environment variables, CLI flags, and public function signatures in shared libraries.

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Destructive rename | `ALTER TABLE RENAME COLUMN` | Add new column → backfill → dual-write → migrate consumers → drop old |
| Required field on existing data | `model: String` (no default) | `#[serde(default)] model: Option<String>` or `DEFAULT` in migration |
| Atomic switchover | Rename env var everywhere at once | New code reads new name, falls back to old, warns |
| No deprecation warning | Old config field silently ignored | `tracing::warn!("interval_ms is deprecated, use interval_secs")` |
| Missing consumer enumeration | Update struct, miss inline SQL/SSE types/dashboard | List ALL consumers before changing |
| Enum variant removal | Remove `"active"` status, old data breaks | Keep old variants, map to new internally |
| Breaking return type change | `Vec<T>` → `HashMap<K, T>` on public fn | Add new method, deprecate old |

## The Three-Phase Pattern

Every breaking change follows three phases:

### Phase 1: Expand (add new, keep old)
```sql
-- Database: add new column alongside old
ALTER TABLE users ADD COLUMN email_address TEXT;
UPDATE users SET email_address = email;
-- Code: write to BOTH columns
```
```rust
// Config: accept both field names
#[derive(Deserialize)]
struct Config {
    interval_secs: Option<u64>,
    #[serde(rename = "interval_ms")]
    legacy_interval_ms: Option<u64>,
}
impl Config {
    fn interval(&self) -> Duration {
        if let Some(ms) = self.legacy_interval_ms {
            tracing::warn!("'interval_ms' is deprecated, use 'interval_secs'");
            Duration::from_millis(ms)
        } else {
            Duration::from_secs(self.interval_secs.unwrap_or(60))
        }
    }
}
```

### Phase 2: Migrate (move consumers to new)
- Update all consumers to use the new interface
- Old interface still works but warns

### Phase 3: Contract (remove old)
- Only after ALL consumers confirmed migrated
- Remove old column/field/function
- This is a SEPARATE deployment from Phase 1

## New Fields on Existing Data

```rust
// WRONG: existing serialized jobs lack this field — deserialization fails
#[derive(Deserialize)]
struct InferenceJob {
    prompt: String,
    model: String,  // Old jobs in queue don't have this
}

// RIGHT: default for backwards compatibility
#[derive(Deserialize)]
struct InferenceJob {
    prompt: String,
    #[serde(default = "default_model")]
    model: String,
}
fn default_model() -> String { "auto".into() }
```

```sql
-- WRONG: NOT NULL without default on table with existing rows
ALTER TABLE jobs ADD COLUMN model TEXT NOT NULL;

-- RIGHT: default covers existing rows
ALTER TABLE jobs ADD COLUMN model TEXT NOT NULL DEFAULT 'auto';
```

## Environment Variables & CLI Flags

```rust
// WRONG: rename and hope all deploy manifests update simultaneously
let db = env::var("MONOLITH_DB_URL")?;

// RIGHT: fallback chain with warning
let db = env::var("MONOLITH_DB_URL").or_else(|_| {
    let val = env::var("DATABASE_URL")?;
    tracing::warn!("DATABASE_URL is deprecated, use MONOLITH_DB_URL");
    Ok(val)
})?;
```

## Enum Evolution

```rust
// WRONG: unknown variant crashes deserialization
#[derive(Deserialize)]
enum Severity { Low, Medium, High, Critical }

// RIGHT: catch-all for forwards compatibility
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Severity {
    Low, Medium, High, Critical,
    #[serde(other)]
    Unknown,
}
```

## Consumer Enumeration Checklist

Before changing ANY shared interface, find ALL consumers:
- [ ] Rust structs (grep for field name across all crates)
- [ ] SQL queries (inline SQL, not just migration files)
- [ ] API response types (Axum handlers, response DTOs)
- [ ] Dashboard types (Zod schemas, TypeScript interfaces, SSE event types)
- [ ] Config files (TOML, .env, docker-compose, Railway config)
- [ ] Tests (test fixtures, mock data, factory functions)
- [ ] Documentation (README, API docs, comments referencing the field)

## Rolling Deploy Rules

1. **Database migrations must be compatible with the PREVIOUS code version** — the old binary will run against the new schema during deploy
2. **New code must handle data written by old code** — `#[serde(default)]`, `Option<T>`, `DEFAULT` in SQL
3. **Old code must not crash on new data** — avoid `#[serde(deny_unknown_fields)]`, use `#[serde(other)]` on enums
4. **Multi-binary systems deploy independently** — orchestrator and worker are NOT atomic; treat every shared type as a public API
