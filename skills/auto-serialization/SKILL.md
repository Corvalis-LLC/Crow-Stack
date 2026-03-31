---
name: auto-serialization
description: "Serialization discipline: decimal precision, timezone-aware datetimes, forwards-compatible enums, null vs missing distinction, and deterministic output. Corrects float financial values, naive datetimes, crashing enum deserialization, and non-canonical serialization. Use when defining serializable types, API payloads, job queue messages, or database JSON columns. Triggers: serialize, deserialize, serde, json, toml, pydantic, zod, schema, payload, precision, float, decimal, timestamp, timezone, enum, variant, unknown, forwards compatible, canonical, deterministic."
---

# Serialization — What Claude Gets Wrong

You serialize data like it will only ever be read by the exact code that wrote it. In reality, serialized data crosses service boundaries, survives schema changes, gets cached, and is read by code versions that don't exist yet.

## The Five Rules

1. **Financial/precise values → string or decimal type, never float**
2. **Datetimes → always UTC, always with timezone suffix**
3. **Enums → unknown variants must not crash deserialization**
4. **New fields → must be optional or defaulted for old data**
5. **Canonical form → same input produces same bytes**

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Float for money | `price: f64` | `#[serde(with = "rust_decimal::serde::str")] price: Decimal` |
| Naive datetime | `dt.isoformat()` → `"2026-03-06T10:00:00"` | `dt.astimezone(UTC).isoformat()` → `"2026-03-06T10:00:00+00:00"` |
| Crashing enum | `z.enum(["low","high"])` rejects `"critical"` | Add catchall: `#[serde(other)] Unknown` / `.catch("unknown")` |
| No null vs missing | `field: Option<T> = None` conflates both | `#[serde(skip_serializing_if = "Option::is_none")]` + distinguish |
| Non-deterministic | `HashMap` key order varies per run | Sort keys, normalize decimals, use `BTreeMap` |
| Implicit format | No docs on wire format | Doc comment with example JSON |

## Decimal Precision

```rust
// WRONG: 0.1 + 0.2 ≠ 0.3 in IEEE 754
#[derive(Serialize)]
struct Trade { price: f64 }

// RIGHT: string representation preserves exact decimal
use rust_decimal::Decimal;
#[derive(Serialize)]
struct Trade {
    #[serde(with = "rust_decimal::serde::str")]
    price: Decimal, // Serializes as "0.000000001", not 9.99999999e-10
}
```
```typescript
// WRONG: number loses precision beyond ~15 digits
const amount = z.number();

// RIGHT: string with validation
const amount = z.string().regex(/^\d+(\.\d+)?$/);
```
```python
# WRONG: float accumulates error
class Trade(BaseModel):
    price: float

# RIGHT: Decimal with string serialization
from decimal import Decimal
class Trade(BaseModel):
    price: Decimal
    class Config:
        json_encoders = {Decimal: str}
```

## Timezone-Aware Datetimes

```rust
// WRONG: NaiveDateTime has no timezone — is this UTC? Local? Unknown.
pub created_at: NaiveDateTime

// RIGHT: DateTime<Utc> serializes with +00:00
pub created_at: DateTime<Utc>
```
```python
# WRONG: naive datetime, isoformat() omits timezone
{"ts": datetime.now().isoformat()}  # "2026-03-06T10:00:00"

# RIGHT: always UTC, always aware
from datetime import datetime, timezone
{"ts": datetime.now(timezone.utc).isoformat()}  # "2026-03-06T10:00:00+00:00"
```

## Forwards-Compatible Enums

```rust
// WRONG: new variant added by producer crashes old consumer
#[derive(Deserialize)]
enum Status { Pending, Running, Completed, Failed }

// RIGHT: catch-all variant
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Pending, Running, Completed, Failed,
    #[serde(other)]
    Unknown,
}
```
```typescript
// WRONG: z.enum() rejects unknown values
const Status = z.enum(["pending", "running", "completed"]);

// RIGHT: parse known, catch unknown
function parseStatus(raw: string): Status | "unknown" {
    const known = ["pending", "running", "completed"] as const;
    return known.includes(raw as any) ? (raw as Status) : "unknown";
}
```
```python
# WRONG: str Enum crashes on unknown
class Status(str, Enum):
    PENDING = "pending"

# RIGHT: classmethod fallback
class Status(str, Enum):
    PENDING = "pending"
    UNKNOWN = "unknown"
    @classmethod
    def _missing_(cls, value):
        return cls.UNKNOWN
```

## Null vs Missing

These are different: `{"name": null}` means "explicitly no name." `{}` means "name was not provided." Your types should reflect this.

```rust
// Distinguish with skip_serializing_if
#[derive(Serialize, Deserialize)]
struct Update {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>, // Missing from JSON = None; explicit null = None
    // For true three-state: use a custom deserializer or wrapper
}
```

## Deterministic Output

When serialized data is hashed, compared, or deduplicated, output must be canonical:

```rust
// WRONG: HashMap iteration order is random
use std::collections::HashMap;
#[derive(Serialize)]
struct Report { metadata: HashMap<String, String> }

// RIGHT: BTreeMap guarantees sorted keys
use std::collections::BTreeMap;
#[derive(Serialize)]
struct Report { metadata: BTreeMap<String, String> }
```

## Format Documentation

Every serializable type should have a doc comment showing the wire format:

```rust
/// Serialized as JSON:
/// ```json
/// {"price": "0.00123", "symbol": "BTC", "timestamp": "2026-03-06T10:00:00+00:00"}
/// ```
#[derive(Serialize)]
struct PriceUpdate { /* ... */ }
```
