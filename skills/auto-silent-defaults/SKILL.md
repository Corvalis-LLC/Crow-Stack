---
name: auto-silent-defaults
description: "Silent default discipline: when defaults mask errors, when missing data should fail loudly, and the specific swallowing patterns Claude defaults to. Corrects returning empty arrays on failure, defaulting required config, and unwrap_or_default hiding bugs. Use when handling missing data, writing fallbacks, setting defaults, or designing function return types. Triggers: default, fallback, unwrap_or, unwrap_or_default, optional, missing, empty, silent, swallow, coalesce, null coalescing, optional chaining, default value."
---

# Silent Defaults — What Claude Gets Wrong

When data is missing, you silently substitute a default instead of surfacing the problem. The caller can't distinguish "no data" from "error." Bugs hide for weeks behind plausible-looking empty arrays and zero values.

## The Core Rule

**Missing required data is an error. Missing optional data is None/null. Neither is a default value.**

```
REQUIRED field missing  →  Error (crash at startup, return error at runtime)
OPTIONAL field missing  →  None / null / Option::None
KNOWN SAFE default      →  Default (only when the default is genuinely correct)
```

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Empty array on fetch failure | `catch { return [] }` | Propagate the error — let caller decide |
| `unwrap_or_default()` on required data | `config.port.unwrap_or_default()` → `0` | `.ok_or_else(\|\| anyhow!("port is required"))` |
| `unwrap_or(0)` hiding missing values | `price.unwrap_or(0.0)` | Return `Option<f64>` — let caller handle absence |
| Null coalescing required fields | `user.email ?? ""` | If email is required, its absence is a bug |
| Default on parse failure | `parseInt(input) \|\| 0` | Validate and error: `const n = parseInt(input); if (isNaN(n)) throw ...` |
| Silent config fallback | `env.get("DB_URL").unwrap_or("localhost")` | Crash at startup: `env.get("DB_URL").expect("DB_URL must be set")` |
| Catch-all returning fallback data | `try { fetchReports() } catch { return cachedReports }` | Only fallback when cache is explicitly designed for this |
| Optional chaining past required fields | `user?.profile?.email` when user is guaranteed | Don't use `?.` on non-optional paths — it hides bugs |

## The Three Default Categories

### 1. Legitimate Defaults (use these)

The default is correct by design — not a fallback for missing data:

```rust
// Pagination defaults — genuinely optional, sensible default
let page_size = params.page_size.unwrap_or(25);
let sort_order = params.sort.unwrap_or(SortOrder::Descending);
```

```typescript
// Feature flags — default-off is intentional
const enableBeta = config.enableBeta ?? false;
```

### 2. Missing Required Data (error, don't default)

```rust
// You write:
let db_url = env::var("DATABASE_URL").unwrap_or_default(); // Empty string!

// Senior writes:
let db_url = env::var("DATABASE_URL")
    .context("DATABASE_URL must be set")?;
```

```typescript
// You write:
const userId = params.userId ?? "anonymous"; // Masks auth bug

// Senior writes:
const userId = params.userId;
if (!userId) throw new AppError("UNAUTHORIZED", "userId is required");
```

### 3. Fallback by Design (explicit, logged, not silent)

```rust
// You write — silent fallback:
let price = fetch_price("BTC").await.unwrap_or(0.0);

// Senior writes — explicit, logged fallback:
let price = match fetch_price("BTC").await {
    Ok(p) => p,
    Err(err) => {
        warn!(err = %err, "unable to fetch live BTC price, using cached value");
        cached_price_store.get("BTC")
            .ok_or_else(|| anyhow!("no cached price available for BTC"))?
    }
};
```

## The Error vs Default Decision

| Situation | Action |
|-----------|--------|
| Required config missing at startup | **Crash** — fail fast, not at 3am |
| Required field missing in API request | **Error** — 400 Bad Request |
| Optional field missing in API request | **None** — use Option/undefined |
| External API returns error | **Propagate** — or explicit fallback with logging |
| Collection might be empty | **Return empty collection** — emptiness is valid data |
| Parse/conversion fails | **Error** — don't substitute 0/""/false |

## Distinguishing "No Data" from "Error"

```typescript
// You write — caller can't tell what happened:
async function getOrders(userId: string): Promise<Order[]> {
  try {
    return await api.fetchOrders(userId);
  } catch {
    return []; // "No orders" or "API down"? Caller will never know.
  }
}

// Senior writes — result type preserves distinction:
type OrdersResult =
  | { status: "ok"; orders: Order[] }
  | { status: "error"; error: string };

async function getOrders(userId: string): Promise<OrdersResult> {
  try {
    const orders = await api.fetchOrders(userId);
    return { status: "ok", orders };
  } catch (err) {
    return { status: "error", error: String(err) };
  }
}
```

## The Startup Rule

**All required configuration must be validated at startup, not on first use.**

```rust
// You write — fails at 3am when the code path is first hit:
fn get_api_key() -> &str {
    env::var("API_KEY").unwrap_or_default().leak()
}

// Senior writes — fails immediately on startup:
struct Config {
    api_key: String,
    db_url: String,
    port: u16,
}

impl Config {
    fn from_env() -> Result<Self> {
        Ok(Self {
            api_key: env::var("API_KEY").context("API_KEY is required")?,
            db_url: env::var("DATABASE_URL").context("DATABASE_URL is required")?,
            port: env::var("PORT").context("PORT is required")?
                .parse().context("PORT must be a valid u16")?,
        })
    }
}
```

Collect ALL config errors and report them at once — no whack-a-mole.
